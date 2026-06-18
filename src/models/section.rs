use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSection<T> {
    pub id: i64,
    #[serde(rename = "type")]
    pub section_type: String,
    pub updated_at: i64,
    pub data: T,
}
