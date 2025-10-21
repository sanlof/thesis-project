mod api;
mod database;
mod models;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("🏥 Hospital System Starting...");
    
    // Read server configuration
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8001".to_string());
    let server_address = format!("127.0.0.1:{}", server_port);
    
    // Establish database connection
    log::info!("Connecting to database...");
    let pool = database::establish_connection()
        .await
        .expect("Failed to create database connection pool");
    
    log::info!("✅ Database connection established");
    
    // Log available routes
    log::info!("📋 Configuring routes:");
    log::info!("   - GET    /patients");
    log::info!("   - POST   /patients");
    log::info!("   - GET    /patients/{{id}}");
    log::info!("   - PUT    /patients/{{id}}");
    log::info!("   - DELETE /patients/{{id}}");
    log::info!("   - GET    /patients/personal/{{personal_id}}");
    log::info!("   - GET    /patients/flagged");
    log::info!("   - GET    /api/shared/patients");
    log::info!("   - GET    /api/shared/patients/flagged");
    log::info!("   - GET    /api/shared/patients/{{personal_id}}");
    
    log::info!("🚀 Starting HTTP server at http://{}", server_address);
    log::info!("🔗 CORS enabled for police system at http://localhost:8000");
    
    // Create and run HTTP server
    HttpServer::new(move || {
        // Configure CORS for cross-origin requests from police system
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")  // Police system
            .allowed_origin("http://127.0.0.1:8000")  // Police system (alternative)
            .allow_any_origin()  // Allow all origins in development
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .max_age(3600);
        
        App::new()
            // Add middleware
            .wrap(middleware::Logger::default())
            .wrap(cors)
            
            // Share database pool across all handlers
            .app_data(web::Data::new(pool.clone()))
            
            // Configure API routes
            .configure(api::configure_patients)
            .configure(api::configure_shared)
            
            // Health check endpoint
            .route("/health", web::get().to(health_check))
    })
    .bind(&server_address)
    .map_err(|e| {
        log::error!("❌ Failed to bind server to {}: {}", server_address, e);
        e
    })?
    .run()
    .await?;
    
    log::info!("🛑 Hospital System shut down");
    Ok(())
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "hospital-system"
    }))
}