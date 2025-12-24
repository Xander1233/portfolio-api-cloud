use actix_web::{web, HttpResponse};
use crate::dynamodb::{DynamoDbAppState};
use crate::queries::get_section::get_section;

pub async fn get_skills(state: web::Data<DynamoDbAppState>) -> actix_web::Result<HttpResponse> {
    let id: i64 = 1;
    let section_type = "SKILLS";

    match get_section::<serde_json::Value>(&state.ddb, &state.table, id, section_type).await {
        Ok(Some(stored_section)) => {
            Ok(HttpResponse::Ok().json(stored_section))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().body("Skills section not found"))
        }
        Err(err) => {
            tracing::error!(
                task = "Get Skills Section",
                result = "failure",
                error = %err,
                "Failed to retrieve skills section from DynamoDB"
            );
            Ok(HttpResponse::InternalServerError().body("Internal Server Error"))
        }
    }
}