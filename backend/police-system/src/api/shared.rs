use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use crate::database;
use crate::middleware::auth::verify_api_key;
use crate::utils::logging::hash_for_logging;
use crate::models::Suspect;

/// GET /api/shared/suspects/{personal_id} - Retrieve suspect info by Swedish personal ID
/// 
/// This endpoint allows the hospital system to check if a patient has a police record
/// by querying their personal_id (Swedish format: YYYYMMDD-XXXX)
/// 
/// SECURITY: Requires valid API key in X-API-Key header
async fn get_shared_suspect_info(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    // Verify API key authentication
    if let Err(e) = verify_api_key(&req).await {
        log::warn!("Unauthorized shared API access attempt");
        return e.into();
    }
    
    let pid = personal_id.into_inner();
    
    // Validate personal ID format
    if !Suspect::validate_personal_id(&pid) {
        log::warn!("Invalid personal_id format in shared API request");
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid personal_id format. Expected: YYYYMMDD-XXXX"
        }));
    }
    
    log::info!("Shared API: Hospital system querying suspect info for personal_id hash: {}", 
        hash_for_logging(&pid));
    
    match database::get_suspect_by_personal_id(&pool, &pid).await {
        Ok(Some(suspect)) => {
            log::info!("Shared API: Found suspect record for personal_id hash: {}", 
                hash_for_logging(&pid));
            HttpResponse::Ok().json(suspect)
        }
        Ok(None) => {
            log::info!("Shared API: No suspect record found for personal_id hash: {}", 
                hash_for_logging(&pid));
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "No suspect record found"
            }))
        }
        Err(e) => {
            log::error!("Shared API: Database error querying suspect: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Service temporarily unavailable"
            }))
        }
    }
}

/// GET /api/shared/suspects - Retrieve all suspects
/// 
/// This endpoint allows the hospital system to retrieve a complete list of all suspects
/// for cross-referencing with their patient database
/// 
/// SECURITY: Requires valid API key in X-API-Key header
async fn get_all_shared_suspects(
    req: HttpRequest,
    pool: web::Data<PgPool>
) -> HttpResponse {
    // Verify API key authentication
    if let Err(e) = verify_api_key(&req).await {
        log::warn!("Unauthorized shared API access attempt");
        return e.into();
    }
    
    log::info!("Shared API: Hospital system requesting all suspects");
    
    match database::get_all_suspects(&pool).await {
        Ok(suspects) => {
            log::info!("Shared API: Returning {} suspect records", suspects.len());
            HttpResponse::Ok().json(suspects)
        }
        Err(e) => {
            log::error!("Shared API: Database error retrieving suspects: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Service temporarily unavailable"
            }))
        }
    }
}

/// Configure shared/inter-system API routes
/// 
/// These endpoints are designed to be called by the hospital system
/// to check if patients have police records.
/// 
/// Routes:
/// - GET /api/shared/suspects - List all suspects (requires API key)
/// - GET /api/shared/suspects/{personal_id} - Check specific person (requires API key)
/// 
/// # Security
/// 
/// All endpoints require API key authentication via X-API-Key header.
/// Rate limiting is applied at the application level.
/// Input validation enforces Swedish personal ID format.
pub fn configure_shared(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/shared")
            .route("/suspects", web::get().to(get_all_shared_suspects))
            .route("/suspects/{personal_id}", web::get().to(get_shared_suspect_info))
    );
}