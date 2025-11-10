mod api;
mod database;
mod models;
mod middleware;
mod config;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use config::Config;
use std::fs::File;
use std::io::BufReader;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

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
    
    log::info!("🔒 API Key authentication required for /api/shared/* endpoints");
    
    let api_key = config.api_key.clone();
    let allowed_origins = config.allowed_origins.clone();
    let enable_tls = config.enable_tls;
    
    // Create HTTP server
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
    
    // Bind server with or without TLS
    if enable_tls {
        log::info!("🔐 TLS enabled - loading certificates...");
        
        let tls_config = load_tls_config(&config)?;
        
        log::info!("🚀 Starting HTTPS server at https://{}", server_address);
        
        server
            .bind_rustls_021(&server_address, tls_config)
            .map_err(|e| {
                log::error!("❌ Failed to bind HTTPS server to {}: {}", server_address, e);
                e
            })?
            .run()
            .await?;
    } else {
        log::info!("🚀 Starting HTTP server at http://{}", server_address);
        
        server
            .bind(&server_address)
            .map_err(|e| {
                log::error!("❌ Failed to bind HTTP server to {}: {}", server_address, e);
                e
            })?
            .run()
            .await?;
    }
    
    log::info!("🛑 Hospital System shut down");
    Ok(())
}

/// Load TLS configuration from certificate and key files
fn load_tls_config(config: &Config) -> std::io::Result<ServerConfig> {
    let cert_path = config.tls_cert_path.as_ref()
        .ok_or_else(|| {
            log::error!("TLS_CERT_PATH not configured");
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TLS_CERT_PATH not set"
            )
        })?;
    
    let key_path = config.tls_key_path.as_ref()
        .ok_or_else(|| {
            log::error!("TLS_KEY_PATH not configured");
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TLS_KEY_PATH not set"
            )
        })?;
    
    log::info!("Loading certificate from: {}", cert_path);
    log::info!("Loading private key from: {}", key_path);
    
    // Load certificate chain
    let cert_file = File::open(cert_path)
        .map_err(|e| {
            log::error!("Failed to open certificate file '{}': {}", cert_path, e);
            e
        })?;
    let mut cert_reader = BufReader::new(cert_file);
    
    let cert_chain: Vec<Certificate> = certs(&mut cert_reader)
        .map_err(|e| {
            log::error!("Failed to parse certificate file: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?
        .into_iter()
        .map(Certificate)
        .collect();
    
    if cert_chain.is_empty() {
        log::error!("No certificates found in '{}'", cert_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No certificates found in certificate file"
        ));
    }
    
    log::info!("✅ Loaded {} certificate(s)", cert_chain.len());
    
    // Load private key
    let key_file = File::open(key_path)
        .map_err(|e| {
            log::error!("Failed to open private key file '{}': {}", key_path, e);
            e
        })?;
    let mut key_reader = BufReader::new(key_file);
    
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key_reader)
        .map_err(|e| {
            log::error!("Failed to parse private key file: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?
        .into_iter()
        .map(PrivateKey)
        .collect();
    
    if keys.is_empty() {
        log::error!("No private keys found in '{}'", key_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No private keys found in key file"
        ));
    }
    
    let private_key = keys.remove(0);
    log::info!("✅ Loaded private key");
    
    // Build TLS configuration
    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .map_err(|e| {
            log::error!("Failed to build TLS configuration: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
        })?;
    
    log::info!("✅ TLS configuration loaded successfully");
    
    Ok(tls_config)
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "hospital-system"
    }))
}