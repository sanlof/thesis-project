use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::database;
use crate::utils::error_handler::{
    handle_database_error,
    handle_not_found,
};
use crate::utils::audit::{AuditLog, EventType, Action, AuditResult, extract_actor_from_request};

/// GET /api/shared/patients/{personal_id} - Retrieve patient info by Swedish personal ID
/// 
/// This endpoint allows the police system to check if a suspect has medical records
/// by querying their personal_id (Swedish format: YYYYMMDD-XXXX)
/// 
/// **REQUIRES AUTHENTICATION**: X-API-Key header must be present
async fn get_shared_patient_info(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    let actor = extract_actor_from_request(&req);
    let ip = req.peer_addr().map(|a| a.ip());
    
    // Sanitize log output - redact personal ID
    let sanitized_pid = if pid.len() >= 9 {
        format!("{}-****", &pid[..8])
    } else {
        "INVALID-****".to_string()
    };
    
    log::info!("Shared API: Authenticated query for patient {}", sanitized_pid);
    
    match database::get_patient_by_personal_id(&pool, &pid).await {
        Ok(Some(patient)) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("patient:{}", sanitized_pid),
                AuditResult::Success,
            )
            .with_ip(ip)
            .write();
            
            log::info!("Shared API: Patient record found for {}", sanitized_pid);
            HttpResponse::Ok().json(patient)
        }
        Ok(None) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("patient:{}", sanitized_pid),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details("Patient not found".to_string())
            .write();
            
            log::info!("Shared API: No patient record for {}", sanitized_pid);
            handle_not_found("patient", &sanitized_pid)
        }
        Err(e) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("patient:{}", sanitized_pid),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "get_shared_patient_info")
        }
    }
}

/// GET /api/shared/patients - Retrieve all patients
/// 
/// This endpoint allows the police system to retrieve a complete list of all patients
/// for cross-referencing with their suspect database
/// 
/// **REQUIRES AUTHENTICATION**: X-API-Key header must be present
async fn get_all_shared_patients(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let actor = extract_actor_from_request(&req);
    let ip = req.peer_addr().map(|a| a.ip());
    
    log::info!("Shared API: Authenticated request for all patients");
    
    match database::get_all_patients(&pool).await {
        Ok(patients) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                format!("patients:all (count: {})", patients.len()),
                AuditResult::Success,
            )
            .with_ip(ip)
            .write();
            
            log::info!("Shared API: Returning {} patient records", patients.len());
            HttpResponse::Ok().json(patients)
        }
        Err(e) => {
            AuditLog::new(
                EventType::SharedApiAccess,
                actor,
                Action::Read,
                "patients:all".to_string(),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "get_all_shared_patients")
        }
    }
}

/// GET /api/shared/patients/flagged - Retrieve all flagged patients
/// 
/// This endpoint allows the police system to see which patients in the hospital
/// have been flagged. Flags are automatically synchronized from the police database
/// via postgres_fdw triggers.
/// 
/// **REQUIRES AUTHENTICATION**: X-API-Key header must be present
async fn get_shared_flagged_patients(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let actor = extract_actor_from_request(&req);
    let ip = req.peer_addr().map(|a| a.ip());
    
    log::info!("Shared API: Authenticated request for flagged patients");
    
    match database::get_flagged_patients(&pool).await {
        Ok(flagged_patients) => {
            AuditLog::new(
                EventType::FlaggedPatientAccess,
                actor,
                Action::Read,
                format!("patients:flagged (count: {})", flagged_patients.len()),
                AuditResult::Success,
            )
            .with_ip(ip)
            .write();
            
            log::info!("Shared API: Returning {} flagged records", flagged_patients.len());
            HttpResponse::Ok().json(flagged_patients)
        }
        Err(e) => {
            AuditLog::new(
                EventType::FlaggedPatientAccess,
                actor,
                Action::Read,
                "patients:flagged".to_string(),
                AuditResult::Failure,
            )
            .with_ip(ip)
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "get_shared_flagged_patients")
        }
    }
}

/// Configure shared/inter-system API routes
/// 
/// These endpoints are designed to be called by the police system
/// to check if patients have medical records or to view flagged patients.
/// 
/// **ALL ROUTES REQUIRE API KEY AUTHENTICATION**
/// 
/// Routes:
/// - GET /patients - List all patients
/// - GET /patients/flagged - List flagged patients (auto-synced from police)
/// - GET /patients/{personal_id} - Check specific person
/// 
/// Note: This function is now called within a scope that has ApiKeyAuth middleware applied
pub fn configure_shared(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/patients", web::get().to(get_all_shared_patients))
        .route("/patients/flagged", web::get().to(get_shared_flagged_patients))
        .route("/patients/{personal_id}", web::get().to(get_shared_patient_info));
}