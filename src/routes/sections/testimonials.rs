use actix_web::{get, web, HttpResponse};

use crate::error::ApiError;
use crate::models::Testimonials;
use crate::routes::sections::handler::serve_section_with;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/testimonials", skip(state))]
#[get("/testimonials")]
pub async fn get_testimonials(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    serve_section_with::<Testimonials, _>(state.get_ref(), "TESTIMONIALS", |section| {
        section.data.items.sort_by_key(|item| item.order);
    })
    .await
}
