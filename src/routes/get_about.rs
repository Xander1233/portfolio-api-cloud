use actix_web::web;
use crate::dynamodb::{DynamoDbAppState};
use crate::queries::get_section::get_section;

pub async fn get_about(state: web::Data<DynamoDbAppState>) -> actix_web::HttpResponse {
    
    let id: i64 = 1;
    let section_type = "ABOUT";

    match get_section::<serde_json::Value>(&state.ddb, &state.table, id, section_type, state.cache.clone(), state.cache_ttl).await {
        Ok(Some(stored_section)) => {
            actix_web::HttpResponse::Ok().json(stored_section)
        }
        Ok(None) => {
            actix_web::HttpResponse::NotFound().body("About section not found")
        }
        Err(err) => {
            tracing::error!(
                task = "Get About Section",
                result = "failure",
                error = %err,
                "Failed to retrieve about section from DynamoDB"
            );
            actix_web::HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}