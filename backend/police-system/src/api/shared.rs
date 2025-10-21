use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::Serialize;
use crate::database::DbPool;

#[derive(Serialize)]
pub struct SuspectInfo {
    pub name: String,
    pub personal_id: String,
    pub has_active_cases: bool,
}

fn verify_api_key(req: &HttpRequest) -> bool {
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return key_str == "dev_key_12345";
        }
    }
    false
}

pub async fn get_suspect_by_personal_id(
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
    
    let suspect_query = sqlx::query!(
        "SELECT name, personal_id FROM suspects WHERE personal_id = $1 LIMIT 1",
        personal_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match suspect_query {
        Ok(Some(suspect)) => {
            let has_active_cases_result = sqlx::query!(
                "SELECT COUNT(*) as count FROM cases c 
                 INNER JOIN suspects s ON c.id = s.case_id 
                 WHERE s.personal_id = $1 AND c.status = 'active'",
                personal_id
            )
            .fetch_one(pool.get_ref())
            .await;

            let has_active_cases = match has_active_cases_result {
                Ok(result) => result.count.unwrap_or(0) > 0,
                Err(_) => false,
            };

            let suspect_info = SuspectInfo {
                name: suspect.name,
                personal_id: suspect.personal_id.unwrap_or_default(),
                has_active_cases,
            };

            HttpResponse::Ok().json(suspect_info)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Suspect not found"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to lookup suspect"
            }))
        }
    }
}