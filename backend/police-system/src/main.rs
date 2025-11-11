mod api;
mod database;
mod models;
mod middleware;
mod utils;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use std::env;
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
    
    log::info!("🚔 Police System Starting...");
    
    // Validate security configuration
    validate_security_config();
    
    // Read server configuration
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8000".to_string());
    let server_address = format!("127.0.0.1:{}", server_port);
    
    // Check if TLS is enabled
    let enable_tls = env::var("ENABLE_TLS")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    
    // Get API key for validating incoming requests from hospital
    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set for shared endpoint authentication");
    
    if api_key.len() < 32 {
        panic!("API_KEY must be at least 32 characters long");
    }
    
    // Parse allowed origins from environment variable
    let allowed_origins_str = env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| {
            log::warn!("ALLOWED_ORIGINS not set, using default development origins");
            "http://localhost:8001,http://localhost:3000,http://127.0.0.1:8001,http://127.0.0.1:3000".to_string()
        });
    
    let allowed_origins: Vec<String> = allowed_origins_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    if allowed_origins.is_empty() {
        panic!("ALLOWED_ORIGINS must contain at least one valid origin");
    }
    
    log::info!("✅ Security configuration loaded");
    log::info!("   - API Key authentication: ENABLED for shared endpoints");
    log::info!("   - TLS: {}", if enable_tls { "ENABLED" } else { "DISABLED (dev only)" });
    log::info!("   - Allowed CORS origins: {:?}", allowed_origins);
    
    // Validate origins in production
    if !cfg!(debug_assertions) {
        for origin in &allowed_origins {
            if origin.starts_with("http://") {
                log::error!("❌ PRODUCTION ERROR: HTTP origin detected: {}", origin);
                panic!("Production mode requires HTTPS origins only. Found HTTP origin: {}", origin);
            }
            if origin.contains("localhost") || origin.contains("127.0.0.1") {
                log::error!("❌ PRODUCTION ERROR: localhost origin detected: {}", origin);
                panic!("Production mode cannot use localhost origins. Found: {}", origin);
            }
        }
        log::info!("✅ All origins validated for production (HTTPS only)");
    }
    
    if !enable_tls {
        log::warn!("⚠️  TLS is DISABLED - This is only acceptable in development!");
        log::warn!("⚠️  Enable TLS in production by setting ENABLE_TLS=true");
    }
    
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
    log::info!("   - GET    /api/shared/suspects (Authenticated)");
    log::info!("   - GET    /api/shared/suspects/{{personal_id}} (Authenticated)");
    
    log::info!("🔒 API Key authentication required for /api/shared/* endpoints");
    log::info!("⏱️  Rate limiting: 10 req/s, burst 20");
    
    // Clone variables for move into closure
    let allowed_origins_clone = allowed_origins.clone();
    
    // Create HTTP server
    let server = HttpServer::new(move || {
        // Configure CORS with strict origin whitelist
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::HeaderName::from_static("x-api-key"),
            ])
            .expose_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600)
            .supports_credentials();
        
        // Add each allowed origin explicitly (no wildcards)
        for origin in &allowed_origins_clone {
            cors = cors.allowed_origin(origin);
        }
        
        App::new()
            // Add security middleware
            .wrap(actix_middleware::Logger::default())
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            
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
            .configure(api::configure_suspects)
            
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
        
        let tls_config = load_tls_config()?;
        
        log::info!("🚀 Starting HTTPS server at https://{}", server_address);
        log::info!("🔒 CORS restricted to: {:?}", allowed_origins);
        
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
        log::info!("🔒 CORS restricted to: {:?}", allowed_origins);
        
        server
            .bind(&server_address)
            .map_err(|e| {
                log::error!("❌ Failed to bind HTTP server to {}: {}", server_address, e);
                e
            })?
            .run()
            .await?;
    }
    
    log::info!("🛑 Police System shut down");
    Ok(())
}

/// Load TLS configuration from certificate and key files
fn load_tls_config() -> std::io::Result<ServerConfig> {
    // Get certificate and key paths from environment
    let cert_path = env::var("TLS_CERT_PATH")
        .map_err(|_| {
            log::error!("TLS_CERT_PATH environment variable not set");
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TLS_CERT_PATH not set"
            )
        })?;
    
    let key_path = env::var("TLS_KEY_PATH")
        .map_err(|_| {
            log::error!("TLS_KEY_PATH environment variable not set");
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TLS_KEY_PATH not set"
            )
        })?;
    
    log::info!("Loading certificate from: {}", cert_path);
    log::info!("Loading private key from: {}", key_path);
    
    // Load certificate chain
    let cert_file = File::open(&cert_path)
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
    let key_file = File::open(&key_path)
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
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .map_err(|e| {
            log::error!("Failed to build TLS configuration: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
        })?;
    
    log::info!("✅ TLS configuration loaded successfully");
    
    Ok(config)
}

/// Validate that required security configuration is present
fn validate_security_config() {
    // Check for API key
    let api_key = env::var("API_KEY");
    
    if !cfg!(debug_assertions) {
        // Production mode - API key is required
        api_key.expect("API_KEY must be set in production");
        log::info!("✅ Security configuration validated");
    } else {
        // Debug mode - warn if API key is missing
        if api_key.is_err() {
            log::warn!("⚠️  Running in DEBUG mode - API_KEY not set");
            log::warn!("⚠️  Set API_KEY for production deployment");
        } else {
            log::info!("✅ API_KEY configured (debug mode)");
        }
    }
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "police-system"
    }))
}