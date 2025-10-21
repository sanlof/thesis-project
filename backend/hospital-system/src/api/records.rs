use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::{DbPool, queries};

#[derive(Deserialize)]
pub struct CreateRecordRequest {
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
}

pub async fn get_records_by_patient(
    pool: web::Data<DbPool>,
    patient_id: web::Path<i32>
) -> impl Responder {
    match queries::get_records_by_patient(pool.get_ref(), patient_id.into_inner()).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch records"
            }))
        }
    }
}

pub async fn create_record(
    pool: web::Data<DbPool>,
    patient_id: web::Path<i32>,
    request: web::Json<CreateRecordRequest>
) -> impl Responder {
    let patient_id = patient_id.into_inner();
    
    match queries::get_patient_by_id(pool.get_ref(), patient_id).await {
        Ok(_) => {
            match queries::create_record(
                pool.get_ref(),
                patient_id,
                request.diagnosis.clone(),
                request.treatment.clone()
            ).await {
                Ok(record) => HttpResponse::Created().json(record),
                Err(e) => {
                    eprintln!("Database error: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to create record"
                    }))
                }
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to verify patient"
            }))
        }
    }
}