use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::database;
use crate::models::{CreatePatient, UpdatePatient};

/// GET /patients - Retrieve all patients
async fn get_all_patients(pool: web::Data<PgPool>) -> HttpResponse {
    match database::get_all_patients(&pool).await {
        Ok(patients) => {
            log::info!("Retrieved {} patients", patients.len());
            HttpResponse::Ok().json(patients)
        }
        Err(e) => {
            log::error!("Failed to retrieve patients: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve patients"
            }))
        }
    }
}

/// GET /patients/{id} - Retrieve a patient by ID
async fn get_patient_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    let patient_id = id.into_inner();
    
    match database::get_patient_by_id(&pool, patient_id).await {
        Ok(Some(patient)) => {
            log::info!("Retrieved patient with ID {}", patient_id);
            HttpResponse::Ok().json(patient)
        }
        Ok(None) => {
            log::warn!("Patient with ID {} not found", patient_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to retrieve patient {}: {}", patient_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve patient"
            }))
        }
    }
}

/// GET /patients/personal/{personal_id} - Retrieve a patient by Swedish personal ID
async fn get_patient_by_personal_id(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    match database::get_patient_by_personal_id(&pool, &pid).await {
        Ok(Some(patient)) => {
            log::info!("Retrieved patient with personal_id {}", pid);
            HttpResponse::Ok().json(patient)
        }
        Ok(None) => {
            log::warn!("Patient with personal_id {} not found", pid);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to retrieve patient with personal_id {}: {}", pid, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve patient"
            }))
        }
    }
}

/// POST /patients - Create a new patient
async fn create_patient(
    pool: web::Data<PgPool>,
    patient: web::Json<CreatePatient>,
) -> HttpResponse {
    let patient_data = patient.into_inner();
    
    match database::create_patient(&pool, patient_data).await {
        Ok(created_patient) => {
            log::info!("Created patient with ID {}", created_patient.id);
            HttpResponse::Created().json(created_patient)
        }
        Err(e) => {
            log::error!("Failed to create patient: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create patient"
            }))
        }
    }
}

/// PUT /patients/{id} - Update an existing patient
async fn update_patient(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    patient: web::Json<UpdatePatient>,
) -> HttpResponse {
    let patient_id = id.into_inner();
    let patient_data = patient.into_inner();
    
    match database::update_patient(&pool, patient_id, patient_data).await {
        Ok(Some(updated_patient)) => {
            log::info!("Updated patient with ID {}", patient_id);
            HttpResponse::Ok().json(updated_patient)
        }
        Ok(None) => {
            log::warn!("Patient with ID {} not found for update", patient_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to update patient {}: {}", patient_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update patient"
            }))
        }
    }
}

/// DELETE /patients/{id} - Delete a patient
async fn delete_patient(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    let patient_id = id.into_inner();
    
    match database::delete_patient(&pool, patient_id).await {
        Ok(true) => {
            log::info!("Deleted patient with ID {}", patient_id);
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            log::warn!("Patient with ID {} not found for deletion", patient_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to delete patient {}: {}", patient_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete patient"
            }))
        }
    }
}

/// GET /patients/flagged - Retrieve all patients flagged by police system
/// Flags are automatically synchronized from the police database via postgres_fdw triggers
async fn get_flagged_patients(pool: web::Data<PgPool>) -> HttpResponse {
    match database::get_flagged_patients(&pool).await {
        Ok(flagged_patients) => {
            log::info!("Retrieved {} flagged patients", flagged_patients.len());
            HttpResponse::Ok().json(flagged_patients)
        }
        Err(e) => {
            log::error!("Failed to retrieve flagged patients: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve flagged patients"
            }))
        }
    }
}

/// Configure all patient-related routes
/// 
/// Routes are ordered with literal paths first to avoid conflicts:
/// - /patients (GET, POST)
/// - /patients/flagged (GET)
/// - /patients/personal/{personal_id} (GET)
/// - /patients/{id} (GET, PUT, DELETE)
pub fn configure_patients(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patients")
            .route("", web::get().to(get_all_patients))
            .route("", web::post().to(create_patient))
            .route("/flagged", web::get().to(get_flagged_patients))
            .route("/personal/{personal_id}", web::get().to(get_patient_by_personal_id))
            .route("/{id}", web::get().to(get_patient_by_id))
            .route("/{id}", web::put().to(update_patient))
            .route("/{id}", web::delete().to(delete_patient))
    );
}