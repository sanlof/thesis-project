use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::database;

/// GET /api/shared/suspects/{personal_id} - Retrieve suspect info by Swedish personal ID
/// 
/// This endpoint allows the hospital system to check if a patient has a police record
/// by querying their personal_id (Swedish format: YYYYMMDD-XXXX)
async fn get_shared_suspect_info(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    log::info!("Shared API: Hospital system querying suspect info for personal_id {}", pid);
    
    match database::get_suspect_by_personal_id(&pool, &pid).await {
        Ok(Some(suspect)) => {
            log::info!("Shared API: Found suspect record for personal_id {}", pid);
            HttpResponse::Ok().json(suspect)
        }
        Ok(None) => {
            log::info!("Shared API: No suspect record found for personal_id {}", pid);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "No suspect record found",
                "personal_id": pid
            }))
        }
        Err(e) => {
            log::error!("Shared API: Database error querying suspect {}: {}", pid, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to query suspect information"
            }))
        }
    }
}

/// GET /api/shared/suspects - Retrieve all suspects
/// 
/// This endpoint allows the hospital system to retrieve a complete list of all suspects
/// for cross-referencing with their patient database
async fn get_all_shared_suspects(pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Shared API: Hospital system requesting all suspects");
    
    match database::get_all_suspects(&pool).await {
        Ok(suspects) => {
            log::info!("Shared API: Returning {} suspect records", suspects.len());
            HttpResponse::Ok().json(suspects)
        }
        Err(e) => {
            log::error!("Shared API: Database error retrieving suspects: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve suspects"
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
/// - GET /api/shared/suspects - List all suspects
/// - GET /api/shared/suspects/{personal_id} - Check specific person
/// 
/// # CORS Configuration
/// 
/// These endpoints should be configured with CORS to allow requests from:
/// - http://localhost:8001 (Hospital system)
/// 
/// CORS should be configured at the application level in main.rs using actix-cors
pub fn configure_shared(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/shared")
            .route("/suspects", web::get().to(get_all_shared_suspects))
            .route("/suspects/{personal_id}", web::get().to(get_shared_suspect_info))
    );
}