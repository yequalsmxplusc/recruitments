use actix_web::{get, patch, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode,Validation};
use crate::auth::models::{AuthConfig, Claims, Applicant};
use serde::{Deserialize, Serialize};
use std::{sync::Mutex,fs};

#[derive(Debug, Clone, Deserialize)]
struct ApplicantUpdate {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub contact_number: Option<String>,
    #[serde(rename = "isSelected")]
    pub is_selected: Option<bool>,
    pub department: Option<String>,
    pub year: Option<String>,
    pub interview_slot: Option<String>,
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,
}

pub struct AppState {
    pub applicants: Mutex<Vec<Applicant>>,
}

pub async fn load_applicants() -> Vec<Applicant> {
    let data = match fs::read_to_string("data/data.json") {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            return Vec::new();
        }
    };
    // println!("Raw JSON data:{}",data);

    let parsed: Vec<ApplicantUpdate> = match serde_json::from_str(&data) {
        Ok(applicants) => applicants,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return Vec::new();
        }
    };

    parsed
        .into_iter()
        .map(|a| Applicant {
            id: a.id,
            name: a.name,
            email: a.email,
            password: a.password.unwrap_or_else(|| "default_password".to_string()), // fallback
            contact_number: a.contact_number.unwrap_or_default(),
            is_selected: a.is_selected.unwrap_or(false),
            department: a.department.unwrap_or_default(),
            year: a.year.unwrap_or_default(),
            interview_slot: a.interview_slot.unwrap_or_else(|| "Not Assigned".to_string()),
            is_admin: a.is_admin.unwrap_or(false),
        })
        .collect()
}

pub async fn save_applicants(applicants: &[Applicant]) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(applicants)?;
    fs::write("data/data.json", data)
}
/**
 * REQ: GET api/applicants/
 * What it does: returns JSON list of user data in data/data.json
 * RES: JSON {id}
 */
#[get("/")]
pub async fn get_applicant(
    auth: BearerAuth,
    data: web::Data<AppState>,
    config: web::Data<AuthConfig>,
) -> impl Responder {
    // Decode the token
    let token = auth.token();
    let decoded = match decode::<Claims>(&token, &config.decoding_key, &Validation::default()) {
        Ok(token_data) => token_data,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired token"),
    };

    let user_id = &decoded.claims.sub;
    let applicants = data.applicants.lock().unwrap();

    // Match either by ID or email (as used during login)
    if let Some(applicant) = applicants.iter().find(|a| &a.id == user_id || &a.email == user_id) {
        HttpResponse::Ok().json(applicant.clone())
    } else {
        HttpResponse::NotFound().body("Applicant not found")
    }
}

/**
 * REQ: GET/api/applicants
 * RES: JSON
 * What it does: returns entire JSON list of users in data/data.json if you are ADMIN only
 */
#[get("/all")]
pub async fn get_all_applicants(
    _auth: BearerAuth,
    data: web::Data<AppState>,
    config: web::Data<AuthConfig>,
) -> impl Responder {
    let token = _auth.token();

    let decoded = decode::<Claims>(
        token,
        &config.decoding_key,
        &Validation::default(),
    );

    match decoded {
        Ok(token_data) => {
            if !token_data.claims.is_admin {
                return HttpResponse::Unauthorized().body("Admin access required");
            } else {
                let applicants = data.applicants.lock().unwrap();
                return HttpResponse::Ok().json(applicants.clone());
            }
        }
        Err(err) => {
            println!("[DEBUG] JWT decode failed: {:?}", err);
            return HttpResponse::Unauthorized().body("Invalid token");
        }
    }
}

/**
 * REQ: PATCH api/applicants/{id}
 * REs: JSON [{id}]
 * What it does: Modifies the entire details of the users as needed
 */
#[patch("/{id}")]
pub async fn update_applicant(
    auth: BearerAuth,
    id: web::Path<String>,
    patch: web::Json<ApplicantUpdate>,
    data: web::Data<AppState>,
    config: web::Data<AuthConfig>,
) -> impl Responder {
    let id = id.into_inner();
    // Decode JWT
    let token = auth.token();
    let decoded = match decode::<Claims>(&token, &config.decoding_key, &Validation::default()) {
        Ok(d) => d,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token or Session Timed Out"),
    };
    let is_admin = decoded.claims.is_admin;
    // Lock and modify in one pass
    let mut applicants = data.applicants.lock().unwrap();
    if let Some(applicant) = applicants.iter_mut().find(|a| a.id == id) {
        println!("Applicant found: {}", applicant.id);
        let update = patch.into_inner();
        if let Some(password) = update.password {
            applicant.password = password;
        }
        if let Some(contact_number) = update.contact_number {
            applicant.contact_number = contact_number;
        }
        if let Some(department) = update.department {
            applicant.department = department;
        }
        if let Some(year) = update.year {
            applicant.year = year;
        }
        if let Some(interview_slot) = update.interview_slot {
            applicant.interview_slot = interview_slot;
        }
        if let Some(is_admin_value) = update.is_admin {
            if is_admin {
                applicant.is_admin = is_admin_value;
            }
        }
        if let Some(is_selected_value) = update.is_selected {
            if is_admin {
                applicant.is_selected = is_selected_value;
            }
        }
        let updated_applicant = applicant.clone();

        if let Err(e) = save_applicants(&*applicants).await {
            return HttpResponse::InternalServerError().json(e.to_string());
        }

        return HttpResponse::Ok().json(updated_applicant);
    }

    HttpResponse::NotFound().json("Applicant not found")
}