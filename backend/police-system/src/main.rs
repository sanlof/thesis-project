mod api;
mod database;
mod models;
mod middleware;
mod utils;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("🚔 Police System Starting...");
    
    // Validate required security configuration
    validate_security_config();
    
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
    
    // Configure rate limiting
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10)  // Allow 10 requests per second
        .burst_size(20)  // Allow bursts up to 20
        .finish()
        .expect("Failed to configure rate limiter");
    
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
    
    let hospital_origin = env::var("HOSPITAL_ORIGIN")
        .unwrap_or_else(|_| {
            log::warn!("HOSPITAL_ORIGIN not set, using localhost:8001");
            "http://localhost:8001".to_string()
        });
    
    log::info!("🔗 CORS enabled for hospital system at {}", hospital_origin);
    log::info!("🔐 API key authentication enabled for shared endpoints");
    log::info!("⏱️  Rate limiting: 10 req/s, burst 20");
    
    // Create and run HTTP server
    HttpServer::new(move || {
        // Configure CORS for cross-origin requests from hospital system
        let cors = if cfg!(debug_assertions) {
            log::warn!("⚠️  Running in DEBUG mode with relaxed CORS");
            Cors::default()
                .allowed_origin("http://localhost:8001")
                .allowed_origin("http://127.0.0.1:8001")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::CONTENT_TYPE,
                    actix_web::http::header::AUTHORIZATION,
                ])
                .expose_headers(vec![actix_web::http::header::CONTENT_TYPE])
                .max_age(3600)
        } else {
            Cors::default()
                .allowed_origin(&hospital_origin)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::CONTENT_TYPE,
                    actix_web::http::header::AUTHORIZATION,
                ])
                .expose_headers(vec![actix_web::http::header::CONTENT_TYPE])
                .max_age(3600)
        };
        
        App::new()
            // Add middleware
            .wrap(actix_middleware::Logger::default())
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            
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

/// Validate that required security configuration is present
fn validate_security_config() {
    // Check for API key in production
    if !cfg!(debug_assertions) {
        env::var("HOSPITAL_API_KEY")
            .expect("HOSPITAL_API_KEY must be set in production");
        
        log::info!("✅ Security configuration validated");
    } else {
        log::warn!("⚠️  Running in DEBUG mode - ensure HOSPITAL_API_KEY is set for production");
    }
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "police-system"
    }))
}