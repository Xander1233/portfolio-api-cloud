use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projects {
    pub items: Vec<ProjectItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub order: i32,
    pub title: String,
    pub description: String,
    pub url: String,
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_variant: Option<ProjectBadgeVariant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preview_lines: Vec<ProjectPreviewLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectBadgeVariant {
    Green,
    Neutral,
    Orange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPreviewLine {
    pub segments: Vec<ProjectPreviewSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPreviewSegment {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
