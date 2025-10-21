use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::database;

/// GET /api/shared/patients/{personal_id} - Retrieve patient info by Swedish personal ID
/// 
/// This endpoint allows the police system to check if a suspect has medical records
/// by querying their personal_id (Swedish format: YYYYMMDD-XXXX)
async fn get_shared_patient_info(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    log::info!("Shared API: Police system querying patient info for personal_id {}", pid);
    
    match database::get_patient_by_personal_id(&pool, &pid).await {
        Ok(Some(patient)) => {
            log::info!("Shared API: Found patient record for personal_id {}", pid);
            HttpResponse::Ok().json(patient)
        }
        Ok(None) => {
            log::info!("Shared API: No patient record found for personal_id {}", pid);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "No patient record found",
                "personal_id": pid
            }))
        }
        Err(e) => {
            log::error!("Shared API: Database error querying patient {}: {}", pid, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to query patient information"
            }))
        }
    }
}

/// GET /api/shared/patients - Retrieve all patients
/// 
/// This endpoint allows the police system to retrieve a complete list of all patients
/// for cross-referencing with their suspect database
async fn get_all_shared_patients(pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Shared API: Police system requesting all patients");
    
    match database::get_all_patients(&pool).await {
        Ok(patients) => {
            log::info!("Shared API: Returning {} patient records", patients.len());
            HttpResponse::Ok().json(patients)
        }
        Err(e) => {
            log::error!("Shared API: Database error retrieving patients: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve patients"
            }))
        }
    }
}

/// GET /api/shared/patients/flagged - Retrieve all flagged patients
/// 
/// This endpoint allows the police system to see which patients in the hospital
/// have been flagged. Flags are automatically synchronized from the police database
/// via postgres_fdw triggers.
async fn get_shared_flagged_patients(pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Shared API: Police system requesting flagged patients");
    
    match database::get_flagged_patients(&pool).await {
        Ok(flagged_patients) => {
            log::info!("Shared API: Returning {} flagged patient records", flagged_patients.len());
            HttpResponse::Ok().json(flagged_patients)
        }
        Err(e) => {
            log::error!("Shared API: Database error retrieving flagged patients: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve flagged patients"
            }))
        }
    }
}

/// Configure shared/inter-system API routes
/// 
/// These endpoints are designed to be called by the police system
/// to check if suspects have medical records or to view flagged patients.
/// 
/// Routes:
/// - GET /api/shared/patients - List all patients
/// - GET /api/shared/patients/flagged - List flagged patients (auto-synced from police)
/// - GET /api/shared/patients/{personal_id} - Check specific person
/// 
/// # CORS Configuration
/// 
/// These endpoints should be configured with CORS to allow requests from:
/// - http://localhost:8000 (Police system)
/// 
/// CORS should be configured at the application level in main.rs using actix-cors
pub fn configure_shared(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/shared")
            .route("/patients", web::get().to(get_all_shared_patients))
            .route("/patients/flagged", web::get().to(get_shared_flagged_patients))
            .route("/patients/{personal_id}", web::get().to(get_shared_patient_info))
    );
}