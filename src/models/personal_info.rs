use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: i64,
    pub email: String,
    pub location: String,
    pub location_city: String,
    pub links: Vec<PersonalInfoLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfoLink {
    pub name: String,
    pub url: String,
}
