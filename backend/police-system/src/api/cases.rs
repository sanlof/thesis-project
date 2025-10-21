use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::{DbPool, queries};

#[derive(Deserialize)]
pub struct CreateCaseRequest {
    pub case_number: String,
    pub status: String,
    pub description: Option<String>,
}

pub async fn get_all_cases(pool: web::Data<DbPool>) -> impl Responder {
    match queries::get_all_cases(pool.get_ref()).await {
        Ok(cases) => HttpResponse::Ok().json(cases),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch cases"
            }))
        }
    }
}

pub async fn get_case_by_id(
    pool: web::Data<DbPool>,
    case_id: web::Path<i32>
) -> impl Responder {
    match queries::get_case_by_id(pool.get_ref(), case_id.into_inner()).await {
        Ok(case) => HttpResponse::Ok().json(case),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Case not found"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch case"
            }))
        }
    }
}

pub async fn create_case(
    pool: web::Data<DbPool>,
    request: web::Json<CreateCaseRequest>
) -> impl Responder {
    match queries::create_case(
        pool.get_ref(),
        request.case_number.clone(),
        request.status.clone(),
        request.description.clone()
    ).await {
        Ok(case) => HttpResponse::Created().json(case),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create case"
            }))
        }
    }
}