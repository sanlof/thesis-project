use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::{DbPool, queries};

#[derive(Deserialize)]
pub struct CreatePatientRequest {
    pub patient_id: String,
    pub name: String,
    pub personal_id: Option<String>,
}

pub async fn get_all_patients(pool: web::Data<DbPool>) -> impl Responder {
    match queries::get_all_patients(pool.get_ref()).await {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch patients"
            }))
        }
    }
}

pub async fn get_patient_by_id(
    pool: web::Data<DbPool>,
    patient_id: web::Path<i32>
) -> impl Responder {
    match queries::get_patient_by_id(pool.get_ref(), patient_id.into_inner()).await {
        Ok(patient) => HttpResponse::Ok().json(patient),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch patient"
            }))
        }
    }
}

pub async fn create_patient(
    pool: web::Data<DbPool>,
    request: web::Json<CreatePatientRequest>
) -> impl Responder {
    match queries::create_patient(
        pool.get_ref(),
        request.patient_id.clone(),
        request.name.clone(),
        request.personal_id.clone()
    ).await {
        Ok(patient) => HttpResponse::Created().json(patient),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create patient"
            }))
        }
    }
}