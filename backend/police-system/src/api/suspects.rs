use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use crate::database;
use crate::models::{CreateSuspect, UpdateSuspect};

/// Request body for flag updates
#[derive(Deserialize)]
struct FlagUpdate {
    flag: bool,
}

/// GET /suspects - Retrieve all suspects
async fn get_all_suspects(pool: web::Data<PgPool>) -> HttpResponse {
    match database::get_all_suspects(&pool).await {
        Ok(suspects) => {
            log::info!("Retrieved {} suspects", suspects.len());
            HttpResponse::Ok().json(suspects)
        }
        Err(e) => {
            log::error!("Failed to retrieve suspects: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve suspects"
            }))
        }
    }
}

/// GET /suspects/{id} - Retrieve a suspect by ID
async fn get_suspect_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    let suspect_id = id.into_inner();
    
    match database::get_suspect_by_id(&pool, suspect_id).await {
        Ok(Some(suspect)) => {
            log::info!("Retrieved suspect with ID {}", suspect_id);
            HttpResponse::Ok().json(suspect)
        }
        Ok(None) => {
            log::warn!("Suspect with ID {} not found", suspect_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Suspect not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to retrieve suspect {}: {}", suspect_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve suspect"
            }))
        }
    }
}

/// GET /suspects/personal/{personal_id} - Retrieve a suspect by Swedish personal ID
async fn get_suspect_by_personal_id(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    match database::get_suspect_by_personal_id(&pool, &pid).await {
        Ok(Some(suspect)) => {
            log::info!("Retrieved suspect with personal_id {}", pid);
            HttpResponse::Ok().json(suspect)
        }
        Ok(None) => {
            log::warn!("Suspect with personal_id {} not found", pid);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Suspect not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to retrieve suspect with personal_id {}: {}", pid, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve suspect"
            }))
        }
    }
}

/// POST /suspects - Create a new suspect
async fn create_suspect(
    pool: web::Data<PgPool>,
    suspect: web::Json<CreateSuspect>,
) -> HttpResponse {
    let suspect_data = suspect.into_inner();
    
    match database::create_suspect(&pool, suspect_data).await {
        Ok(created_suspect) => {
            log::info!("Created suspect with ID {}", created_suspect.id);
            HttpResponse::Created().json(created_suspect)
        }
        Err(e) => {
            log::error!("Failed to create suspect: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create suspect"
            }))
        }
    }
}

/// PUT /suspects/{id} - Update an existing suspect
async fn update_suspect(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    suspect: web::Json<UpdateSuspect>,
) -> HttpResponse {
    let suspect_id = id.into_inner();
    let suspect_data = suspect.into_inner();
    
    match database::update_suspect(&pool, suspect_id, suspect_data).await {
        Ok(Some(updated_suspect)) => {
            log::info!("Updated suspect with ID {}", suspect_id);
            HttpResponse::Ok().json(updated_suspect)
        }
        Ok(None) => {
            log::warn!("Suspect with ID {} not found for update", suspect_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Suspect not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to update suspect {}: {}", suspect_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update suspect"
            }))
        }
    }
}

/// DELETE /suspects/{id} - Delete a suspect
async fn delete_suspect(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    let suspect_id = id.into_inner();
    
    match database::delete_suspect(&pool, suspect_id).await {
        Ok(true) => {
            log::info!("Deleted suspect with ID {}", suspect_id);
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            log::warn!("Suspect with ID {} not found for deletion", suspect_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Suspect not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to delete suspect {}: {}", suspect_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete suspect"
            }))
        }
    }
}

/// PUT /suspects/{personal_id}/flag - Update flag status
/// This triggers automatic synchronization to the hospital database via postgres_fdw
async fn update_flag(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
    flag_data: web::Json<FlagUpdate>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    match database::update_flag(&pool, &pid, flag_data.flag).await {
        Ok(Some(updated_suspect)) => {
            log::info!(
                "Updated flag to {} for suspect with personal_id {} (will auto-sync to hospital)",
                flag_data.flag,
                pid
            );
            HttpResponse::Ok().json(updated_suspect)
        }
        Ok(None) => {
            log::warn!("Suspect with personal_id {} not found for flag update", pid);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Suspect not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to update flag for suspect {}: {}", pid, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update flag"
            }))
        }
    }
}

/// Configure all suspect-related routes
/// 
/// Routes are ordered with literal paths first to avoid conflicts:
/// - /suspects (GET, POST)
/// - /suspects/personal/{personal_id} (GET)
/// - /suspects/{personal_id}/flag (PUT)
/// - /suspects/{id} (GET, PUT, DELETE)
pub fn configure_suspects(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/suspects")
            .route("", web::get().to(get_all_suspects))
            .route("", web::post().to(create_suspect))
            .route("/personal/{personal_id}", web::get().to(get_suspect_by_personal_id))
            .route("/{personal_id}/flag", web::put().to(update_flag))
            .route("/{id}", web::get().to(get_suspect_by_id))
            .route("/{id}", web::put().to(update_suspect))
            .route("/{id}", web::delete().to(delete_suspect))
    );
}