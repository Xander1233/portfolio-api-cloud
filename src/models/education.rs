use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub items: Vec<EducationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationItem {
    pub institution: String,
    pub institution_url: String,
    pub degree: String,
    pub field_of_study: String,
    pub start: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    pub current: bool,
    pub order: i32,
}
