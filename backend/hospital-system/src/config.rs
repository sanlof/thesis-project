use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: String,
    pub api_key: String,
    pub allowed_origins: Vec<String>,
    pub rate_limit_per_minute: u64,
    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8001".to_string());
        
        let api_key = env::var("API_KEY")
            .map_err(|_| "API_KEY must be set for security".to_string())?;
        
        // Validate API key length
        if api_key.len() < 32 {
            return Err("API_KEY must be at least 32 characters long".to_string());
        }
        
        let allowed_origins_str = env::var("ALLOWED_SERVICE_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:8000".to_string());
        
        let allowed_origins: Vec<String> = allowed_origins_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let rate_limit_per_minute = env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);
        
        let enable_tls = env::var("ENABLE_TLS")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
        
        let tls_cert_path = env::var("TLS_CERT_PATH").ok();
        let tls_key_path = env::var("TLS_KEY_PATH").ok();
        
        if enable_tls && (tls_cert_path.is_none() || tls_key_path.is_none()) {
            return Err("TLS_CERT_PATH and TLS_KEY_PATH must be set when ENABLE_TLS=true".to_string());
        }
        
        Ok(Config {
            database_url,
            server_port,
            api_key,
            allowed_origins,
            rate_limit_per_minute,
            enable_tls,
            tls_cert_path,
            tls_key_path,
        })
    }
}