use actix_web::{get, web};
use serde::{Serialize};
use crate::dynamodb::{DynamoDbAppState};

#[derive(Serialize)]
struct HealthResponse {
    #[serde(rename = "status")]
    pub status: String,
}

#[get("/health")]
pub async fn health(state: web::Data<DynamoDbAppState>) -> actix_web::HttpResponse {
    let ddb_status = check_ddb_health(&state).await;

    let response = HealthResponse {
        status: match ddb_status {
            Ok(_) => "OK".to_string(),
            Err(_) => "degraded".to_string()
        }
    };
    actix_web::HttpResponse::Ok().json(response)
}

async fn check_ddb_health(state: &DynamoDbAppState) -> Result<(), String> {
    state.ddb
        .list_tables()
        .limit(0)
        .send()
        .await
        .map(|_| ())
        .map_err(|e| format!("DynamoDB health check failed: {}", e.into_service_error()))
}