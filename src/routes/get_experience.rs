use actix_web::{web, HttpResponse};
use crate::dynamodb::{DynamoDbAppState, get_section};

pub async fn get_experience(state: web::Data<DynamoDbAppState>) -> actix_web::Result<HttpResponse> {
    let id: i64 = 1;
    let section_type = "EXPERIENCE";

    match get_section::<serde_json::Value>(&state.ddb, &state.table, id, section_type).await {
        Ok(Some(stored_section)) => {
            Ok(HttpResponse::Ok().json(stored_section))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().body("Experience section not found"))
        }
        Err(err) => {
            tracing::error!(
                task = "Get Experience Section",
                result = "failure",
                error = %err,
                "Failed to retrieve experience section from DynamoDB"
            );
            Ok(HttpResponse::InternalServerError().body("Internal Server Error"))
        }
    }
}