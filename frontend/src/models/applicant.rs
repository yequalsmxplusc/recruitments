use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub struct Applicant {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub grad_year: Option<String>,
    pub mobile: Option<String>,
    pub gender: Option<String>,
    pub faculty: Option<String>,
    pub department: Option<String>,
    pub skill: Option<String>,
    pub event_participation: Option<bool>,
    pub why_apply: Option<String>,
    pub event_experience: Option<String>,
    pub submission1_url: Option<String>,
    pub submission2_url: Option<String>,
    pub interview_slot: Option<String>,
    #[serde(rename = "isSelected")]
    pub is_selected: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    pub status: Option<String>,
    pub round: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewSlot {
    pub date_time: String,
    pub capacity: u32,
    pub remaining: Option<u32>,
}