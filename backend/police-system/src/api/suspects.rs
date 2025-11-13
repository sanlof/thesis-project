use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use crate::database;
use crate::models::{CreateSuspect, UpdateSuspect, Suspect};
use crate::utils::logging::hash_for_logging;
use crate::utils::error_handler::{
    handle_database_error,
    handle_not_found,
    handle_validation_error,
};
use crate::utils::audit::{AuditLog, EventType, Action, AuditResult};

/// Request body for flag updates - now includes personal_id
#[derive(Deserialize)]
struct FlagUpdateRequest {
    personal_id: String,
    flag: bool,
}

/// GET /suspects - Retrieve all suspects
async fn get_all_suspects(pool: web::Data<PgPool>) -> HttpResponse {
    match database::get_all_suspects(&pool).await {
        Ok(suspects) => {
            log::info!("Retrieved {} suspects", suspects.len());
            HttpResponse::Ok().json(suspects)
        }
        Err(e) => handle_database_error(e, "get_all_suspects"),
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
        Ok(None) => handle_not_found("suspect", &suspect_id.to_string()),
        Err(e) => handle_database_error(e, "get_suspect_by_id"),
    }
}

/// GET /suspects/personal/{personal_id} - Retrieve a suspect by Swedish personal ID
async fn get_suspect_by_personal_id(
    pool: web::Data<PgPool>,
    personal_id: web::Path<String>,
) -> HttpResponse {
    let pid = personal_id.into_inner();
    
    // Validate personal ID format
    if !Suspect::validate_personal_id(&pid) {
        return handle_validation_error(
            &format!("Invalid personal_id format: {}", hash_for_logging(&pid)),
            "get_suspect_by_personal_id"
        );
    }
    
    match database::get_suspect_by_personal_id(&pool, &pid).await {
        Ok(Some(suspect)) => {
            log::info!("Retrieved suspect with personal_id hash: {}", hash_for_logging(&pid));
            HttpResponse::Ok().json(suspect)
        }
        Ok(None) => handle_not_found("suspect", &hash_for_logging(&pid)),
        Err(e) => handle_database_error(e, "get_suspect_by_personal_id"),
    }
}

/// POST /suspects - Create a new suspect
async fn create_suspect(
    pool: web::Data<PgPool>,
    suspect: web::Json<CreateSuspect>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let suspect_data = suspect.into_inner();
    let resource_hash = hash_for_logging(&suspect_data.personal_id);
    
    // Validate personal ID format
    if !Suspect::validate_personal_id(&suspect_data.personal_id) {
        // Audit failure
        AuditLog::new(
            EventType::SuspectCreate,
            "internal".to_string(),
            Action::Create,
            format!("suspect:{}", resource_hash),
            AuditResult::Failure,
        )
        .with_ip(req.peer_addr().map(|a| a.ip()))
        .with_details("Invalid personal_id format".to_string())
        .write();
        
        return handle_validation_error(
            &format!("Invalid personal_id format: {}", resource_hash),
            "create_suspect"
        );
    }
    
    match database::create_suspect(&pool, suspect_data).await {
        Ok(created_suspect) => {
            let pid_hash = created_suspect.personal_id
                .as_ref()
                .map(|pid| hash_for_logging(pid))
                .unwrap_or_else(|| "unknown".to_string());
            
            // Audit success
            AuditLog::new(
                EventType::SuspectCreate,
                "internal".to_string(),
                Action::Create,
                format!("suspect:{}", pid_hash),
                AuditResult::Success,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .write();
            
            log::info!("Created suspect with ID {} (personal_id hash: {})", 
                created_suspect.id, pid_hash);
            HttpResponse::Created().json(created_suspect)
        }
        Err(e) => {
            // Audit failure
            AuditLog::new(
                EventType::SuspectCreate,
                "internal".to_string(),
                Action::Create,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "create_suspect")
        }
    }
}

/// PUT /suspects/{id} - Update an existing suspect
async fn update_suspect(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    suspect: web::Json<UpdateSuspect>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let suspect_id = id.into_inner();
    let suspect_data = suspect.into_inner();
    let resource_hash = hash_for_logging(&suspect_data.personal_id);
    
    // Validate personal ID format if provided
    if !Suspect::validate_personal_id(&suspect_data.personal_id) {
        AuditLog::new(
            EventType::SuspectUpdate,
            "internal".to_string(),
            Action::Update,
            format!("suspect:{}", resource_hash),
            AuditResult::Failure,
        )
        .with_ip(req.peer_addr().map(|a| a.ip()))
        .with_details("Invalid personal_id format".to_string())
        .write();
        
        return handle_validation_error(
            &format!("Invalid personal_id format: {}", resource_hash),
            "update_suspect"
        );
    }
    
    match database::update_suspect(&pool, suspect_id, suspect_data).await {
        Ok(Some(updated_suspect)) => {
            let pid_hash = updated_suspect.personal_id
                .as_ref()
                .map(|pid| hash_for_logging(pid))
                .unwrap_or_else(|| "unknown".to_string());
            
            AuditLog::new(
                EventType::SuspectUpdate,
                "internal".to_string(),
                Action::Update,
                format!("suspect:{}", pid_hash),
                AuditResult::Success,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .write();
            
            log::info!("Updated suspect with ID {}", suspect_id);
            HttpResponse::Ok().json(updated_suspect)
        }
        Ok(None) => {
            AuditLog::new(
                EventType::SuspectUpdate,
                "internal".to_string(),
                Action::Update,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details("Suspect not found".to_string())
            .write();
            
            handle_not_found("suspect", &suspect_id.to_string())
        }
        Err(e) => {
            AuditLog::new(
                EventType::SuspectUpdate,
                "internal".to_string(),
                Action::Update,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "update_suspect")
        }
    }
}

/// DELETE /suspects/{id} - Delete a suspect
async fn delete_suspect(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let suspect_id = id.into_inner();
    
    match database::delete_suspect(&pool, suspect_id).await {
        Ok(true) => {
            AuditLog::new(
                EventType::SuspectDelete,
                "internal".to_string(),
                Action::Delete,
                format!("suspect:id_{}", suspect_id),
                AuditResult::Success,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .write();
            
            log::info!("Deleted suspect with ID {}", suspect_id);
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            AuditLog::new(
                EventType::SuspectDelete,
                "internal".to_string(),
                Action::Delete,
                format!("suspect:id_{}", suspect_id),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details("Suspect not found".to_string())
            .write();
            
            handle_not_found("suspect", &suspect_id.to_string())
        }
        Err(e) => {
            AuditLog::new(
                EventType::SuspectDelete,
                "internal".to_string(),
                Action::Delete,
                format!("suspect:id_{}", suspect_id),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "delete_suspect")
        }
    }
}

/// POST /suspects/flag - Update flag status
/// 
/// SECURITY IMPROVEMENT: Moved personal_id from URL path to request body
/// to prevent logging of sensitive data in browser history and server logs.
/// 
/// This triggers automatic synchronization to the hospital database via postgres_fdw
async fn update_flag(
    pool: web::Data<PgPool>,
    flag_data: web::Json<FlagUpdateRequest>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let request = flag_data.into_inner();
    let resource_hash = hash_for_logging(&request.personal_id);
    
    // Validate personal ID format
    if !Suspect::validate_personal_id(&request.personal_id) {
        AuditLog::new(
            EventType::FlagUpdate,
            "internal".to_string(),
            Action::Update,
            format!("suspect:{}", resource_hash),
            AuditResult::Failure,
        )
        .with_ip(req.peer_addr().map(|a| a.ip()))
        .with_details("Invalid personal_id format".to_string())
        .write();
        
        return handle_validation_error(
            &format!("Invalid personal_id format: {}", resource_hash),
            "update_flag"
        );
    }
    
    match database::update_flag(&pool, &request.personal_id, request.flag).await {
        Ok(Some(updated_suspect)) => {
            AuditLog::new(
                EventType::FlagUpdate,
                "internal".to_string(),
                Action::Update,
                format!("suspect:{}", resource_hash),
                AuditResult::Success,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details(format!("Flag updated to {}", request.flag))
            .write();
            
            log::info!(
                "Updated flag to {} for suspect with personal_id hash: {} (will auto-sync to hospital)",
                request.flag,
                resource_hash
            );
            HttpResponse::Ok().json(updated_suspect)
        }
        Ok(None) => {
            AuditLog::new(
                EventType::FlagUpdate,
                "internal".to_string(),
                Action::Update,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details("Suspect not found".to_string())
            .write();
            
            handle_not_found("suspect", &resource_hash)
        }
        Err(e) => {
            AuditLog::new(
                EventType::FlagUpdate,
                "internal".to_string(),
                Action::Update,
                format!("suspect:{}", resource_hash),
                AuditResult::Failure,
            )
            .with_ip(req.peer_addr().map(|a| a.ip()))
            .with_details(format!("Database error: {}", e))
            .write();
            
            handle_database_error(e, "update_flag")
        }
    }
}

/// Configure all suspect-related routes
/// 
/// Routes are ordered with literal paths first to avoid conflicts:
/// - /suspects (GET, POST)
/// - /suspects/flag (POST) - UPDATED: no longer includes personal_id in path
/// - /suspects/personal/{personal_id} (GET)
/// - /suspects/{id} (GET, PUT, DELETE)
pub fn configure_suspects(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/suspects")
            .route("", web::get().to(get_all_suspects))
            .route("", web::post().to(create_suspect))
            .route("/flag", web::post().to(update_flag))  // Changed from PUT with path param to POST
            .route("/personal/{personal_id}", web::get().to(get_suspect_by_personal_id))
            .route("/{id}", web::get().to(get_suspect_by_id))
            .route("/{id}", web::put().to(update_suspect))
            .route("/{id}", web::delete().to(delete_suspect))
    );
}