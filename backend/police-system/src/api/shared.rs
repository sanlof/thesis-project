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
use crate::utils::audit::{AuditLog, EventType, Action, AuditResult, extract_actor_from_request};

/// GET /api/shared/suspects/{personal_id} - Retrieve suspect info by Swedish personal ID
/// 
/// This endpoint allows the hospital system to check if a suspect has a police record
/// by querying their personal_id (Swedish format: YYYYMMDD-XXXX)
/// 
/// SECURITY: Requires valid API key in X-API-Key header
async fn get_shared_suspect_info(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    let resource_hash = hash_for_logging(&pid);
    let actor = extract_actor_from_request(&req);
    let ip = req.peer_addr().map(|a| a.ip());
    
    // Validate personal ID format
    if !Suspect::validate_personal_id(&pid) {
        AuditLog::new(
            EventType::SharedApiAccess,
            actor,
            Action::Read,
            format!("suspect:{}", resource_hash),
            AuditResult::Failure,
        )
        .with_ip(ip)
        .with_details("Invalid personal_id format".to_string())
        .write();
        
        return handle_validation_error(
            &format!("Invalid personal_id format: {}", resource_hash),
            "get_shared_suspect_info"
        );
    }
    
    log::info!("Shared API: Hospital system querying suspect info for personal_id hash: {}", 
        resource_hash);
    
    match database::get_suspect_by_personal_id(&pool, &pid).await {
        Ok(Some(suspect)) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("suspect:{}", resource_hash),
                AuditResult::Success,
            )
            .with_ip(ip)
            .write();
            
            log::info!("Shared API: Found suspect record for personal_id hash: {}", 
                resource_hash);
            HttpResponse::Ok().json(suspect)
        }
        Ok(None) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details("Suspect not found".to_string())
            .write();
            
            log::info!("Shared API: No suspect record found for personal_id hash: {}", 
                resource_hash);
            handle_not_found("suspect", &resource_hash)
        }
        Err(e) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "get_shared_suspect_info")
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
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let actor = extract_actor_from_request(&req);
    let ip = req.peer_addr().map(|a| a.ip());
    
    log::info!("Shared API: Hospital system requesting all suspects");
    
    match database::get_all_suspects(&pool).await {
        Ok(suspects) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("suspects:all (count: {})", suspects.len()),
                AuditResult::Success,
            )
            .with_ip(ip)
            .write();
            
            log::info!("Shared API: Returning {} suspect records", suspects.len());
            HttpResponse::Ok().json(suspects)
        }
        Err(e) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                "suspects:all".to_string(),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "get_all_shared_suspects")
        }
    }
}

/// Configure shared/inter-system API routes
/// 
/// These endpoints are designed to be called by the hospital system
/// to check if suspects have police records.
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