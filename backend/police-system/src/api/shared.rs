use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::database;
use crate::utils::logging::hash_for_logging;
use crate::utils::error_handler::{
    handle_database_error,
    handle_not_found,
    handle_validation_error,
};
use crate::models::Suspect;

/// GET /api/shared/suspects/{personal_id} - Retrieve suspect info by Swedish personal ID
/// 
/// This endpoint allows the hospital system to check if a patient has a police record
/// by querying their personal_id (Swedish format: YYYYMMDD-XXXX)
/// 
/// SECURITY: Requires valid API key in X-API-Key header
async fn get_shared_suspect_info(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    // Validate personal ID format
    if !Suspect::validate_personal_id(&pid) {
        return handle_validation_error(
            &format!("Invalid personal_id format: {}", hash_for_logging(&pid)),
            "get_shared_suspect_info"
        );
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
            handle_not_found("suspect", &hash_for_logging(&pid))
        }
        Err(e) => handle_database_error(e, "get_shared_suspect_info"),
    }
}

/// GET /api/shared/suspects - Retrieve all suspects
/// 
/// This endpoint allows the hospital system to retrieve a complete list of all suspects
/// for cross-referencing with their patient database
/// 
/// SECURITY: Requires valid API key in X-API-Key header
async fn get_all_shared_suspects(
    pool: web::Data<PgPool>
) -> HttpResponse {
    log::info!("Shared API: Hospital system requesting all suspects");
    
    match database::get_all_suspects(&pool).await {
        Ok(suspects) => {
            log::info!("Shared API: Returning {} suspect records", suspects.len());
            HttpResponse::Ok().json(suspects)
        }
        Err(e) => handle_database_error(e, "get_all_shared_suspects"),
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
    cfg
        .route("/suspects", web::get().to(get_all_shared_suspects))
        .route("/suspects/{personal_id}", web::get().to(get_shared_suspect_info));
}