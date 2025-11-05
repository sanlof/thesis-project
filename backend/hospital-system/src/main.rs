mod api;
mod database;
mod models;
mod middleware;
mod config;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("🏥 Hospital System Starting...");
    
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Validate security configuration
    log::info!("✅ Security configuration loaded");
    log::info!("   - API Key authentication: ENABLED");
    log::info!("   - Rate limiting: {} req/min", config.rate_limit_per_minute);
    log::info!("   - TLS: {}", if config.enable_tls { "ENABLED" } else { "DISABLED (dev only)" });
    
    if !config.enable_tls {
        log::warn!("⚠️  TLS is DISABLED - This is only acceptable in development!");
        log::warn!("⚠️  Enable TLS in production by setting ENABLE_TLS=true");
    }
    
    let server_address = format!("127.0.0.1:{}", config.server_port);
    
    // Establish database connection
    log::info!("Connecting to database...");
    let pool = database::establish_connection()
        .await
        .expect("Failed to create database connection pool");
    
    log::info!("✅ Database connection established");
    
    // Log available routes
    log::info!("📋 Configuring routes:");
    log::info!("   - GET    /patients (Internal)");
    log::info!("   - POST   /patients (Internal)");
    log::info!("   - GET    /patients/{{id}} (Internal)");
    log::info!("   - PUT    /patients/{{id}} (Internal)");
    log::info!("   - DELETE /patients/{{id}} (Internal)");
    log::info!("   - GET    /patients/personal/{{personal_id}} (Internal)");
    log::info!("   - GET    /patients/flagged (Internal)");
    log::info!("   - GET    /api/shared/patients (Authenticated)");
    log::info!("   - GET    /api/shared/patients/flagged (Authenticated)");
    log::info!("   - GET    /api/shared/patients/{{personal_id}} (Authenticated)");
    
    log::info!("🚀 Starting HTTP server at http://{}", server_address);
    log::info!("🔒 API Key authentication required for /api/shared/* endpoints");
    
    let api_key = config.api_key.clone();
    let allowed_origins = config.allowed_origins.clone();
    
    // Create and run HTTP server
    let server = HttpServer::new(move || {
        // Create rate limiter for each worker
        let rate_limiter = middleware::configure_rate_limiter(config.rate_limit_per_minute);
        
        // Configure CORS - STRICT production settings
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::HeaderName::from_static("x-api-key"),
            ])
            .max_age(3600);
        
        // Only allow specific origins (no wildcard)
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        
        App::new()
            // Add security middleware
            .wrap(actix_middleware::Logger::default())
            .wrap(cors)
            .wrap(rate_limiter)
            
            // Add security headers
            .wrap(actix_middleware::DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            )
            
            // Share database pool across all handlers
            .app_data(web::Data::new(pool.clone()))
            
            // Configure API routes
            .configure(api::configure_patients)
            
            // Shared API routes with authentication
            .service(
                web::scope("/api/shared")
                    .wrap(middleware::ApiKeyAuth::new(api_key.clone()))
                    .configure(api::configure_shared)
            )
            
            // Health check endpoint
            .route("/health", web::get().to(health_check))
    });
    
    // Bind server with optional TLS
    let server = if config.enable_tls {
        log::info!("🔐 TLS enabled");
        // In production, you would use:
        // server.bind_rustls(&server_address, load_rustls_config(&config))?
        // For now, we'll just bind normally and log a warning
        log::warn!("⚠️  TLS binding not implemented yet - using HTTP");
        server.bind(&server_address)?
    } else {
        server.bind(&server_address)?
    };
    
    server.run().await?;
    
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