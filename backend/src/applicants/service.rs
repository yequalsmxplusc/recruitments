use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applicant {
    pub id: String,
    pub name: String,
    // pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub contact_number: String,
    pub is_selected: bool,
    pub department: String,
    pub year: String,
    pub interview_slot: String,
    pub is_admin: bool,
    // pub date: String,
}

pub struct AppState {
    pub applicants: Mutex<Vec<Applicant>>,
}

pub async fn load_applicants() -> Vec<Applicant> {
    let data = fs::read_to_string("data/data.json")
        .expect("Unable to read file");
    serde_json::from_str(&data).unwrap_or_default()
}

pub async fn save_applicants(applicants: &[Applicant]) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(applicants)?;
    fs::write("data/data.json", data)
}

#[get("")]
pub async fn get_applicants(
    _auth: BearerAuth,
    data: web::Data<AppState>,
) -> impl Responder {
    let applicants = data.applicants.lock().unwrap();
    HttpResponse::Ok().json(applicants.clone())
}

#[post("/{id}")]
pub async fn update_applicant(
    _auth: BearerAuth,
    id: web::Path<String>,
    updated: web::Json<Applicant>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut applicants = data.applicants.lock().unwrap();
    let id = id.into_inner(); // move happens here, outside closure
    if let Some(index) = applicants.iter().position(|a| a.id == id) {
        applicants[index] = updated.into_inner();
        if let Err(e) = save_applicants(&*applicants).await {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
        HttpResponse::Ok().json(&applicants[index])
    } else {
        HttpResponse::NotFound().json("Applicant not found")
    }
}