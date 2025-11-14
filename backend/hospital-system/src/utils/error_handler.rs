use actix_web::HttpResponse;
use uuid::Uuid;
use serde_json::json;

/// Generate a unique correlation ID for error tracking
pub fn generate_correlation_id() -> String {
    Uuid::new_v4().to_string()
}

/// Handle database errors with logging and safe error response
pub fn handle_database_error<E: std::fmt::Display>(error: E, context: &str) -> HttpResponse {
    let correlation_id = generate_correlation_id();
    
    // Log full error details server-side
    log::error!(
        "Database error [{}] in {}: {}",
        correlation_id,
        context,
        error
    );
    
    // Return generic error to client
    HttpResponse::InternalServerError().json(json!({
        "error": "Service temporarily unavailable",
        "correlation_id": correlation_id
    }))
}

/// Handle not found errors
pub fn handle_not_found(resource: &str, identifier: &str) -> HttpResponse {
    let correlation_id = generate_correlation_id();
    
    log::warn!(
        "Resource not found [{}]: {} with identifier: {}",
        correlation_id,
        resource,
        identifier
    );
    
    HttpResponse::NotFound().json(json!({
        "error": "Resource not found",
        "correlation_id": correlation_id
    }))
}

/// Handle validation errors with safe error information
pub fn handle_validation_error(message: &str, context: &str) -> HttpResponse {
    let correlation_id = generate_correlation_id();
    
    log::warn!(
        "Validation error [{}] in {}: {}",
        correlation_id,
        context,
        message
    );
    
    HttpResponse::BadRequest().json(json!({
        "error": "Invalid request format",
        "correlation_id": correlation_id
    }))
}

/// Handle unauthorized access attempts
pub fn handle_unauthorized(context: &str) -> HttpResponse {
    let correlation_id = generate_correlation_id();
    
    log::warn!(
        "Unauthorized access attempt [{}] in {}",
        correlation_id,
        context
    );
    
    HttpResponse::Unauthorized().json(json!({
        "error": "Authentication required",
        "correlation_id": correlation_id
    }))
}

/// Handle service unavailable errors
pub fn handle_service_unavailable(service: &str) -> HttpResponse {
    let correlation_id = generate_correlation_id();
    
    log::error!(
        "Service unavailable [{}]: {}",
        correlation_id,
        service
    );
    
    HttpResponse::ServiceUnavailable().json(json!({
        "error": "Service temporarily unavailable",
        "correlation_id": correlation_id
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_correlation_id() {
        let id1 = generate_correlation_id();
        let id2 = generate_correlation_id();
        
        assert_ne!(id1, id2);
        assert!(Uuid::parse_str(&id1).is_ok());
    }
}