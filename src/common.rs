use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::writer::Tee;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredSection<T> {
    pub id: i64,
    #[serde(rename = "type")]
    pub section_type: String,
    pub updated_at: i64,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct About {
    pub headline: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: i64,
    pub email: String,
    pub location: String,
    pub location_city: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub items: Vec<EducationItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EducationItem {
    pub institution: String,
    pub institution_url: String,
    pub degree: String,
    pub field_of_study: String,
    pub start: String,
    pub end: Option<String>,
    pub current: bool,
    pub order: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub items: Vec<ExperienceItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub company: String,
    pub company_url: String,
    pub start: String,
    pub end: Option<String>,
    pub current: bool,
    pub order: i32,
    pub career: Vec<ExperienceCareer>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperienceCareer {
    pub position: String,
    pub location: String,
    pub start: String,
    pub end: Option<String>,
    pub current: bool,
    pub order: i32,
    pub description: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skills {
    pub items: Vec<SkillsItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsItem {
    pub name: String,
    pub level: String,
    pub logo: String
}
