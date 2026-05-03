use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Applicant {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub grad_year: Option<String>, // 2028 or 2029
    pub mobile: Option<String>,    // 10-digit
    pub gender: Option<String>,
    pub faculty: Option<String>,                  // arts/science/engineering/ISLM
    pub department: Option<String>,               // based on faculty
    pub skill: Option<String>,                    // design/tech variants
    pub event_participation: Option<bool>,        // yes/no
    pub why_apply: Option<String>,                // 150 words
    pub event_experience: Option<String>, // 50 words if yes
    pub submission1_url: Option<String>,  // case study 1
    pub submission2_url: Option<String>,  // case study 2 (for skill)
    pub interview_slot: Option<String>,
    #[serde(rename = "isSelected")]
    pub is_selected: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    pub status: Option<String>,
    pub round: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewSlot {
    pub date_time: String,
    pub capacity: u32,
    pub remaining: Option<u32>,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl AuthConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }
}
