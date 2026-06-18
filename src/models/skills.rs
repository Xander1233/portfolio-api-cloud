use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub items: Vec<SkillItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillItem {
    pub name: String,
    pub level: i32,
    pub logo: String,
    pub logo_color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}
