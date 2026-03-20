use actix_web::{get};
use serde::{Serialize};

#[derive(Serialize)]
struct HealthResponse {
    #[serde(rename = "status")]
    pub status: String,
}

#[get("/health")]
pub async fn health() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}