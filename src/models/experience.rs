use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub items: Vec<ExperienceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub company: String,
    pub company_url: String,
    pub start: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    pub current: bool,
    pub order: i32,
    pub career: Vec<ExperienceCareer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceCareer {
    pub position: String,
    pub location: String,
    pub start: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    pub current: bool,
    pub order: i32,
    pub description: Vec<String>,
}
