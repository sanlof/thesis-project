use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

/// Establishes a connection pool to the PostgreSQL database
/// 
/// Reads the DATABASE_URL from environment variables and creates
/// a connection pool with a maximum of 5 connections.
/// 
/// # Returns
/// 
/// * `Result<PgPool, sqlx::Error>` - Connection pool on success, error on failure
/// 
/// # Environment Variables
/// 
/// * `DATABASE_URL` - PostgreSQL connection string (e.g., postgresql://postgres@localhost/police_db)
/// 
/// # Example
/// 
/// ```rust
/// let pool = establish_connection().await?;
/// ```

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    // Read database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    log::info!("Attempting to connect to database...");
    
    // Extract and log only the host (not credentials)
    if let Some(host_start) = database_url.find('@') {
        let host_part = &database_url[host_start + 1..];
        log::debug!("Database host: {}", host_part);
    } else {
        log::debug!("Connecting to database (local socket)");
    }
    
    // Create connection pool with configuration
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| {
            log::error!("Failed to connect to database: {}", e);
            log::error!("Please verify DATABASE_URL is correct and PostgreSQL is running");
            e
        })?;
    
    log::info!("Successfully established connection pool to police_db");
    
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database to be running
    async fn test_establish_connection() {
        dotenv::dotenv().ok();
        let result = establish_connection().await;
        assert!(result.is_ok());
    }
}