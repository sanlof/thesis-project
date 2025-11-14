use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor, PeerIpKeyExtractor, governor::middleware::NoOpMiddleware};
use actix_web::{dev::ServiceRequest, http::StatusCode, HttpResponse, ResponseError};
use sha2::{Sha256, Digest};
use std::fmt;

/// Standard rate limiter using IP address
pub fn configure_rate_limiter(requests_per_second: u64, burst_size: u32) -> Governor<PeerIpKeyExtractor, NoOpMiddleware> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(requests_per_second)
        .burst_size(burst_size)
        .finish()
        .unwrap();
    
    Governor::new(&governor_conf)
}

/// Custom error for API key extraction
#[derive(Debug)]
pub struct ApiKeyError {
    message: String,
}

impl fmt::Display for ApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ApiKeyError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(serde_json::json!({
            "error": self.message
        }))
    }
}

impl ApiKeyError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Custom key extractor that uses API key from X-API-Key header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiKeyExtractor;

impl KeyExtractor for ApiKeyExtractor {
    type Key = String;
    type KeyExtractionError = ApiKeyError;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        // Extract API key from header
        let api_key = req
            .headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| ApiKeyError::new("Missing X-API-Key header"))?;
        
        // Hash the API key for privacy in rate limiting storage
        // This ensures the actual key isn't stored in memory
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        // Use first 32 characters of hash as key
        let key = hash[..32].to_string();
        
        // Log with sanitized key for monitoring
        log::debug!("Rate limit key extracted: api_key:{}", &key[..16]);
        
        Ok(key)
    }
}

/// Rate limiter for shared API endpoints using API key
pub fn configure_shared_api_rate_limiter(
    requests_per_second: u64,
    burst_size: u32,
) -> Governor<ApiKeyExtractor, NoOpMiddleware> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(requests_per_second)
        .burst_size(burst_size)
        .key_extractor(ApiKeyExtractor)
        .finish()
        .unwrap();
    
    log::info!(
        "Configured shared API rate limiter: {} req/s, burst: {}",
        requests_per_second,
        burst_size
    );
    
    Governor::new(&governor_conf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn test_api_key_extractor_success() {
        let extractor = ApiKeyExtractor;
        let req = TestRequest::default()
            .insert_header(("X-API-Key", "test_key_12345"))
            .to_srv_request();
        
        let result = extractor.extract(&req);
        assert!(result.is_ok());
        
        let key = result.unwrap();
        // Should be a hash, not the original key
        assert_ne!(key, "test_key_12345");
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_api_key_extractor_missing_header() {
        let extractor = ApiKeyExtractor;
        let req = TestRequest::default().to_srv_request();
        
        let result = extractor.extract(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_extractor_consistency() {
        let extractor = ApiKeyExtractor;
        let api_key = "consistent_key_test";
        
        let req1 = TestRequest::default()
            .insert_header(("X-API-Key", api_key))
            .to_srv_request();
        
        let req2 = TestRequest::default()
            .insert_header(("X-API-Key", api_key))
            .to_srv_request();
        
        let key1 = extractor.extract(&req1).unwrap();
        let key2 = extractor.extract(&req2).unwrap();
        
        // Same API key should produce same hash
        assert_eq!(key1, key2);
    }
}