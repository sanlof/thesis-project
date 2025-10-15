use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    println!("Hospital System starting on http://localhost:8001");
    
    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .route("/health", web::get().to(health_check))
            .route("/api/patients", web::get().to(get_patients))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

async fn get_patients() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!([
        {"id": 1, "patient_id": "H-2024-001", "name": "John Doe"}
    ]))
}