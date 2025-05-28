use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applicant {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub contact_number: String,
    #[serde(rename = "isSelected")]
    pub is_selected: bool,
    pub department: String,
    pub year: String,
    pub interview_slot: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    pub status: String,
}