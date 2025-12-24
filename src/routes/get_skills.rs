use actix_web::{web, HttpResponse};
use crate::common::{Skills, StoredSection};
use crate::dynamodb::{DynamoDbAppState};
use crate::queries::get_section::get_section;

pub async fn get_skills(state: web::Data<DynamoDbAppState>) -> actix_web::Result<HttpResponse> {
    let id: i64 = 1;
    let section_type = "SKILLS";

    match get_section::<Skills>(&state.ddb, &state.table, id, section_type, state.cache.clone(), state.cache_ttl).await {
        Ok(Some(stored_section)) => {

            // TODO Might have to refactor this later to avoid overhead
            let skills = Skills {
                items: stored_section.data.items.into_iter().map(|mut skill| {
                    skill.logo = format!("https://cdn.david-neidhart.de/{}", skill.logo);
                    skill
                }).collect(),
            };

            let stored_section = StoredSection {
                id: stored_section.id,
                section_type: stored_section.section_type,
                updated_at: stored_section.updated_at,
                data: skills,
            };

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