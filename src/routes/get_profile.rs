use actix_web::{web, HttpResponse};
use aws_sdk_dynamodb::types::AttributeValue;
use crate::dynamodb::DynamoDbAppState;

pub async fn get_profile(state: web::Data<DynamoDbAppState>) -> actix_web::Result<HttpResponse> {

    let id: i64 = 1;

    let ddb_response = state
        .ddb
        .query()
        .table_name(&state.table)
        .key_condition_expression("id = :id")
        .expression_attribute_values(":id", AttributeValue::N(id.to_string()))
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    #[derive(serde::Deserialize)]
    struct RawSection {
        id: i64,
        #[serde(rename = "type")]
        section_type: String,
        updated_at: i64,
        data: serde_json::Value,
    }

    let mut response_data = serde_json::Map::new();

    for item in ddb_response.items.unwrap_or_default() {
        let raw: RawSection = serde_dynamo::from_item(item).map_err(actix_web::error::ErrorInternalServerError)?;
        response_data.insert(raw.section_type, raw.data);
    }

    Ok(HttpResponse::Ok().json(response_data))
}