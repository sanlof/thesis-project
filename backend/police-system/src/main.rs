use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    println!("Police System starting on http://localhost:8000");
    
    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .route("/health", web::get().to(health_check))
            .route("/api/cases", web::get().to(get_cases))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

async fn get_cases() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!([
        {"id": 1, "case_number": "P-2024-001", "status": "active"}
    ]))
}