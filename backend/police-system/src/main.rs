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
    
    log::info!("🚔 Police System Starting...");
    
    // Read server configuration
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8000".to_string());
    let server_address = format!("127.0.0.1:{}", server_port);
    
    // Establish database connection
    log::info!("Connecting to database...");
    let pool = database::establish_connection()
        .await
        .expect("Failed to create database connection pool");
    
    log::info!("✅ Database connection established");
    
    // Log available routes
    log::info!("📋 Configuring routes:");
    log::info!("   - GET    /suspects");
    log::info!("   - POST   /suspects");
    log::info!("   - GET    /suspects/{{id}}");
    log::info!("   - PUT    /suspects/{{id}}");
    log::info!("   - DELETE /suspects/{{id}}");
    log::info!("   - GET    /suspects/personal/{{personal_id}}");
    log::info!("   - PUT    /suspects/{{personal_id}}/flag");
    log::info!("   - GET    /api/shared/suspects");
    log::info!("   - GET    /api/shared/suspects/{{personal_id}}");
    
    log::info!("🚀 Starting HTTP server at http://{}", server_address);
    log::info!("🔗 CORS enabled for hospital system at http://localhost:8001");
    
    // Create and run HTTP server
    HttpServer::new(move || {
        // Configure CORS for cross-origin requests from hospital system
        let cors = Cors::default()
            .allowed_origin("http://localhost:8001")  // Hospital system
            .allowed_origin("http://127.0.0.1:8001")  // Hospital system (alternative)
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
            .configure(api::configure_suspects)
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
    
    log::info!("🛑 Police System shut down");
    Ok(())
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "police-system"
    }))
}