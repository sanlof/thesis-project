mod api;
mod database;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use database::create_pool;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "hospital-system"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("🏥 Hospital System starting...");

    let pool = create_pool()
        .await
        .expect("Failed to create database pool");

    log::info!("✓ Database connection established");

    let server_port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8001".to_string());
    let bind_address = format!("127.0.0.1:{}", server_port);

    log::info!("🚀 Server starting on http://{}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/api/patients", web::get().to(api::patients::get_all_patients))
            .route("/api/patients/{id}", web::get().to(api::patients::get_patient_by_id))
            .route("/api/patients", web::post().to(api::patients::create_patient))
            .route(
                "/api/patients/{patient_id}/records",
                web::get().to(api::records::get_records_by_patient)
            )
            .route(
                "/api/patients/{patient_id}/records",
                web::post().to(api::records::create_record)
            )
            .route(
                "/api/shared/patient/{personal_id}",
                web::get().to(api::shared::get_patient_by_personal_id)
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}