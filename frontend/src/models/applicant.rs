use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applicant {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub contact_number: String,
    pub is_selected: bool,
    pub department: String,
    pub date: String,
}
