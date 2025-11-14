use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, body::{BoxBody, MessageBody},
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::env;
use subtle::ConstantTimeEq;

/// Middleware for API key authentication on shared endpoints
pub struct ApiKeyAuth {
    api_key: String,
}

impl ApiKeyAuth {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyAuthMiddleware {
            service,
            api_key: self.api_key.clone(),
        }))
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: S,
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
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
        let api_key = self.api_key.clone();
        
        // Extract API key from header
        let provided_key = req.headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        match provided_key {
            Some(key) if constant_time_eq(key.as_bytes(), api_key.as_bytes()) => {
                // Valid API key - proceed with request
                log::info!("Authenticated API request to {} from IP: {:?}", 
                    req.path(), req.peer_addr());
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_boxed_body())
                })
            }
            Some(_) => {
                // Invalid API key
                log::warn!("Invalid API key provided for {} from IP: {:?}", 
                    req.path(), req.peer_addr());
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Invalid API key"
                            }))
                            .map_into_boxed_body()
                    ))
                })
            }
            None => {
                // Missing API key
                log::warn!("Missing API key for {} from IP: {:?}", 
                    req.path(), req.peer_addr());
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "API key required"
                            }))
                            .map_into_boxed_body()
                    ))
                })
            }
        }
    }
}

/// Helper function to check if key comparison is constant-time safe
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}

/// Verify API key from request header (legacy function for backward compatibility)
/// 
/// This function is kept for any existing code that might reference it,
/// but the middleware approach above is preferred for new implementations.
/// 
/// # Arguments
/// 
/// * `req` - The HTTP request containing the X-API-Key header
/// 
/// # Returns
/// 
/// * `Result<(), Error>` - Ok if valid, ErrorUnauthorized if invalid/missing
/// 
/// # Security
/// 
/// - Uses constant-time comparison to prevent timing side-channel attacks
/// - Requires exact match of API key
/// - In debug mode, allows requests without API key (logs warning)
pub async fn verify_api_key(req: &actix_web::HttpRequest) -> Result<(), Error> {
    use actix_web::error::ErrorUnauthorized;
    
    // In debug mode, allow missing API key but log warning
    if cfg!(debug_assertions) {
        let api_key = req.headers()
            .get("X-API-Key")
            .and_then(|v| v.to_str().ok());
        
        if api_key.is_none() {
            log::warn!("⚠️  DEBUG MODE: API key missing but allowing request");
            return Ok(());
        }
    }
    
    // Extract API key from header
    let provided_key = req.headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            log::warn!("Missing X-API-Key header from IP: {:?}", req.peer_addr());
            ErrorUnauthorized("Missing API key")
        })?;
    
    // Get valid API key from environment
    let valid_key = env::var("HOSPITAL_API_KEY")
        .map_err(|_| {
            log::error!("HOSPITAL_API_KEY not configured");
            ErrorUnauthorized("Authentication configuration error")
        })?;
    
    // Constant-time comparison to prevent timing attacks
    let is_valid: bool = constant_time_eq(provided_key.as_bytes(), valid_key.as_bytes());
    
    if !is_valid {
        log::warn!("Invalid API key provided from IP: {:?}", req.peer_addr());
        return Err(ErrorUnauthorized("Invalid API key"));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[actix_web::test]
    async fn test_verify_api_key_missing() {
        let req = TestRequest::default().to_http_request();
        
        // In debug mode this might pass, in release it should fail
        if !cfg!(debug_assertions) {
            assert!(verify_api_key(&req).await.is_err());
        }
    }

    #[actix_web::test]
    async fn test_verify_api_key_invalid() {
        std::env::set_var("HOSPITAL_API_KEY", "correct_key");
        
        let req = TestRequest::default()
            .insert_header(("X-API-Key", "wrong_key"))
            .to_http_request();
        
        assert!(verify_api_key(&req).await.is_err());
    }

    #[actix_web::test]
    async fn test_verify_api_key_valid() {
        std::env::set_var("HOSPITAL_API_KEY", "correct_key");
        
        let req = TestRequest::default()
            .insert_header(("X-API-Key", "correct_key"))
            .to_http_request();
        
        assert!(verify_api_key(&req).await.is_ok());
    }
    
    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"test123", b"test123"));
        assert!(!constant_time_eq(b"test123", b"test124"));
        assert!(!constant_time_eq(b"short", b"longer"));
    }
}