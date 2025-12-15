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
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub items: Vec<EducationItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EducationItem {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub start_year: i32,
    pub end_year: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub items: Vec<ExperienceItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub company: String,
    pub position: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub responsibilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skills {
    pub items: Vec<SkillsItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsItem {
    pub name: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    pub items: Vec<ProjectsItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectsItem {
    pub name: String,
    pub description: String,
    pub url: Option<String>,
    pub technologies: Vec<String>,
    pub status: String,
}
