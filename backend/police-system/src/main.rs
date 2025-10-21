mod api;
mod database;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use database::create_pool;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "police-system"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("🚔 Police System starting...");

    let pool = create_pool()
        .await
        .expect("Failed to create database pool");

    log::info!("✓ Database connection established");

    let server_port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8000".to_string());
    let bind_address = format!("127.0.0.1:{}", server_port);

    log::info!("🚀 Server starting on http://{}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/api/cases", web::get().to(api::cases::get_all_cases))
            .route("/api/cases/{id}", web::get().to(api::cases::get_case_by_id))
            .route("/api/cases", web::post().to(api::cases::create_case))
            .route(
                "/api/cases/{case_id}/suspects",
                web::get().to(api::suspects::get_suspects_by_case)
            )
            .route(
                "/api/cases/{case_id}/suspects",
                web::post().to(api::suspects::create_suspect)
            )
            .route(
                "/api/shared/suspect/{personal_id}",
                web::get().to(api::shared::get_suspect_by_personal_id)
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}