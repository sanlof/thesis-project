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
    
    // Validate required security configuration
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
    
    let hospital_origin = env::var("HOSPITAL_ORIGIN")
        .unwrap_or_else(|_| {
            log::warn!("HOSPITAL_ORIGIN not set, using localhost:8001");
            "http://localhost:8001".to_string()
        });
    
    log::info!("🔗 CORS enabled for hospital system at {}", hospital_origin);
    log::info!("🔐 API key authentication enabled for shared endpoints");
    log::info!("⏱️  Rate limiting: 10 req/s, burst 20");
    
    // Create HTTP server
    let server = HttpServer::new(move || {
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
    });
    
    // Bind server with or without TLS
    if enable_tls {
        log::info!("🔐 TLS enabled - loading certificates...");
        
        let tls_config = load_tls_config()?;
        
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
        log::warn!("⚠️  TLS is DISABLED - using HTTP only");
        log::warn!("⚠️  This is acceptable only in development!");
        log::warn!("⚠️  Set ENABLE_TLS=true in production");
        
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