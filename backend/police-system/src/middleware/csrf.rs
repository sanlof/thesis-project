use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
    body::{BoxBody, MessageBody},
    cookie::{Cookie, SameSite},
    http::Method,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use rand::Rng;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use subtle::ConstantTimeEq;

const CSRF_COOKIE_NAME: &str = "csrf_token";
const CSRF_HEADER_NAME: &str = "x-csrf-token";
const TOKEN_LENGTH: usize = 32;

/// Generate a cryptographically secure random CSRF token
fn generate_csrf_token() -> String {
    let mut rng = rand::thread_rng();
    let token_bytes: Vec<u8> = (0..TOKEN_LENGTH).map(|_| rng.gen()).collect();
    BASE64.encode(token_bytes)
}

/// CSRF protection middleware
pub struct CsrfProtection {
    enable_tls: bool,
}

impl CsrfProtection {
    pub fn new(enable_tls: bool) -> Self {
        Self { enable_tls }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CsrfProtection
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = CsrfProtectionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfProtectionMiddleware {
            service,
            enable_tls: self.enable_tls,
        }))
    }
}

pub struct CsrfProtectionMiddleware<S> {
    service: S,
    enable_tls: bool,
}

impl<S, B> Service<ServiceRequest> for CsrfProtectionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let enable_tls = self.enable_tls;
        
        // Check if this request should be exempt from CSRF protection
        let should_skip_csrf = should_skip_csrf_check(&req);
        
        if should_skip_csrf {
            // Generate and set CSRF token cookie for GET requests
            if req.method() == Method::GET {
                let token = generate_csrf_token();
                let mut cookie = Cookie::new(CSRF_COOKIE_NAME, token);
                cookie.set_http_only(true);
                cookie.set_same_site(SameSite::Lax); // Changed from Strict to Lax for better proxy compatibility
                cookie.set_path("/");
                
                // Only set Secure flag if TLS is enabled
                if enable_tls {
                    cookie.set_secure(true);
                }
                
                // Store cookie in request extensions for response
                req.extensions_mut().insert(cookie.clone());
                
                log::debug!("CSRF: Generated token for GET request to {}", req.path());
            }
            
            let fut = self.service.call(req);
            return Box::pin(async move {
                let mut res = fut.await?;
                
                // Add cookie to response if present in extensions
                let cookie_opt = res.request().extensions().get::<Cookie>().cloned();
                if let Some(cookie) = cookie_opt {
                    if let Err(e) = res.response_mut().add_cookie(&cookie) {
                        log::warn!("Failed to set CSRF cookie: {}", e);
                    } else {
                        log::debug!("CSRF: Cookie set successfully");
                    }
                }
                
                Ok(res.map_into_boxed_body())
            });
        }
        
        // For state-changing methods, validate CSRF token
        let cookie_token = req
            .cookie(CSRF_COOKIE_NAME)
            .map(|c| c.value().to_string());
        
        let header_token = req
            .headers()
            .get(CSRF_HEADER_NAME)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        log::debug!(
            "CSRF validation for {} {}: cookie={:?}, header={:?}",
            req.method(),
            req.path(),
            cookie_token.as_ref().map(|t| &t[..8.min(t.len())]),
            header_token.as_ref().map(|t| &t[..8.min(t.len())])
        );
        
        match (cookie_token, header_token) {
            (Some(cookie), Some(header)) if constant_time_eq(&cookie, &header) => {
                // Valid CSRF token
                log::debug!("CSRF: Token validated successfully");
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_boxed_body())
                })
            }
            (None, _) => {
                // Missing CSRF cookie
                log::warn!("CSRF validation failed: missing cookie from {}", 
                    req.peer_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()));
                
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Forbidden()
                            .json(serde_json::json!({
                                "error": "CSRF token missing",
                                "code": "CSRF_TOKEN_MISSING"
                            }))
                            .map_into_boxed_body()
                    ))
                })
            }
            (_, None) => {
                // Missing CSRF header
                log::warn!("CSRF validation failed: missing header from {}", 
                    req.peer_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()));
                
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Forbidden()
                            .json(serde_json::json!({
                                "error": "CSRF token required in X-CSRF-Token header",
                                "code": "CSRF_HEADER_MISSING"
                            }))
                            .map_into_boxed_body()
                    ))
                })
            }
            _ => {
                // Token mismatch
                log::warn!("CSRF validation failed: token mismatch from {}", 
                    req.peer_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()));
                
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Forbidden()
                            .json(serde_json::json!({
                                "error": "CSRF token validation failed",
                                "code": "CSRF_TOKEN_INVALID"
                            }))
                            .map_into_boxed_body()
                    ))
                })
            }
        }
    }
}

/// Determine if a request should skip CSRF validation
fn should_skip_csrf_check(req: &ServiceRequest) -> bool {
    let path = req.path();
    let method = req.method();
    
    // Skip CSRF for:
    // 1. All GET requests
    // 2. Health check endpoint
    // 3. Shared API endpoints (use API key auth instead)
    method == Method::GET
        || path == "/health"
        || path.starts_with("/api/shared/")
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    
    if a_bytes.len() != b_bytes.len() {
        return false;
    }
    
    a_bytes.ct_eq(b_bytes).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_csrf_token() {
        let token1 = generate_csrf_token();
        let token2 = generate_csrf_token();
        
        // Tokens should be non-empty
        assert!(!token1.is_empty());
        assert!(!token2.is_empty());
        
        // Tokens should be different
        assert_ne!(token1, token2);
        
        // Token should be valid base64
        assert!(BASE64.decode(&token1).is_ok());
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq("test123", "test123"));
        assert!(!constant_time_eq("test123", "test124"));
        assert!(!constant_time_eq("short", "longer_string"));
    }
}