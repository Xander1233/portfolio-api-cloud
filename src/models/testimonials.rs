use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonials {
    pub items: Vec<TestimonialItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestimonialItem {
    pub order: i32,
    pub quote: String,
    pub author_name: String,
    pub author_role: String,
    pub initials: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gradient_start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gradient_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_color: Option<String>,
}
