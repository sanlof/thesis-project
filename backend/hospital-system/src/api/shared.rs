use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::Serialize;
use crate::database::DbPool;

#[derive(Serialize)]
pub struct PatientInfo {
    pub name: String,
    pub personal_id: String,
    pub has_records: bool,
}

fn verify_api_key(req: &HttpRequest) -> bool {
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return key_str == "dev_key_12345";
        }
    }
    false
}

pub async fn get_patient_by_personal_id(
    pool: web::Data<DbPool>,
    personal_id: web::Path<String>,
    req: HttpRequest
) -> impl Responder {
    if !verify_api_key(&req) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Unauthorized - Invalid or missing API key"
        }));
    }

    let personal_id = personal_id.into_inner();
    
    let patient_query = sqlx::query!(
        "SELECT name, personal_id FROM patients WHERE personal_id = $1 LIMIT 1",
        personal_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match patient_query {
        Ok(Some(patient)) => {
            let has_records_result = sqlx::query!(
                "SELECT COUNT(*) as count FROM medical_records 
                 WHERE patient_id = (SELECT id FROM patients WHERE personal_id = $1)",
                personal_id
            )
            .fetch_one(pool.get_ref())
            .await;

            let has_records = match has_records_result {
                Ok(result) => result.count.unwrap_or(0) > 0,
                Err(_) => false,
            };

            let patient_info = PatientInfo {
                name: patient.name,
                personal_id: patient.personal_id.unwrap_or_default(),
                has_records,
            };

            HttpResponse::Ok().json(patient_info)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Patient not found"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to lookup patient"
            }))
        }
    }
}