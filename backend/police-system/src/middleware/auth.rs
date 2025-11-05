use actix_web::{HttpRequest, Error};
use actix_web::error::ErrorUnauthorized;
use std::env;
use subtle::ConstantTimeEq;

/// Verify API key from request header
/// 
/// Checks the X-API-Key header against the HOSPITAL_API_KEY environment variable
/// using constant-time comparison to prevent timing attacks.
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
pub async fn verify_api_key(req: &HttpRequest) -> Result<(), Error> {
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
            log::warn!("Missing X-API-Key header");
            ErrorUnauthorized("Missing API key")
        })?;
    
    // Get valid API key from environment
    let valid_key = env::var("HOSPITAL_API_KEY")
        .map_err(|_| {
            log::error!("HOSPITAL_API_KEY not configured");
            ErrorUnauthorized("Authentication configuration error")
        })?;
    
    // Constant-time comparison to prevent timing attacks
    let is_valid = provided_key.as_bytes()
        .ct_eq(valid_key.as_bytes())
        .into();
    
    if !is_valid {
        log::warn!("Invalid API key provided");
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
}