use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::{DbPool, queries};

#[derive(Deserialize)]
pub struct CreateSuspectRequest {
    pub name: String,
    pub personal_id: Option<String>,
}

pub async fn get_suspects_by_case(
    pool: web::Data<DbPool>,
    case_id: web::Path<i32>
) -> impl Responder {
    match queries::get_suspects_by_case(pool.get_ref(), case_id.into_inner()).await {
        Ok(suspects) => HttpResponse::Ok().json(suspects),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch suspects"
            }))
        }
    }
}

pub async fn create_suspect(
    pool: web::Data<DbPool>,
    case_id: web::Path<i32>,
    request: web::Json<CreateSuspectRequest>
) -> impl Responder {
    let case_id = case_id.into_inner();
    
    match queries::get_case_by_id(pool.get_ref(), case_id).await {
        Ok(_) => {
            match queries::create_suspect(
                pool.get_ref(),
                case_id,
                request.name.clone(),
                request.personal_id.clone()
            ).await {
                Ok(suspect) => HttpResponse::Created().json(suspect),
                Err(e) => {
                    eprintln!("Database error: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to create suspect"
                    }))
                }
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Case not found"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to verify case"
            }))
        }
    }
}