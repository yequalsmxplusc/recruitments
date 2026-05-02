use crate::auth::models::{Applicant, AuthConfig, Claims, InterviewSlot};
use crate::google_api::GoogleClient;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, get, patch, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::{StreamExt, TryStreamExt};
use jsonwebtoken::{Validation, decode};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Deserialize)]
struct ApplicantUpdate {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
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
    pub is_selected: Option<bool>,
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,
    pub status: Option<String>,
    pub round: Option<String>,
}

pub struct AppState {
    pub applicants: Mutex<Vec<Applicant>>,
    pub google_client: GoogleClient,
}

pub async fn load_applicants(client: &GoogleClient) -> Vec<Applicant> {
    client.fetch_applicants().await.unwrap_or_else(|e| {
        println!("Error fetching from Google Sheets: {}", e);
        Vec::new()
    })
}

// save_applicants removed — data is now persisted via Google Sheets API
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
    if let Some(applicant) = applicants
        .iter()
        .find(|a| &a.id == user_id || &a.email == user_id)
    {
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

    let decoded = decode::<Claims>(token, &config.decoding_key, &Validation::default());

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
    let user_id = &decoded.claims.sub;
    let update = patch.into_inner();
    
    // Security check: Prevent non-admins from updating other users
    if !is_admin && user_id != &id && user_id != &update.email {
        return HttpResponse::Forbidden().body("You can only modify your own profile");
    }

    // Validation
    if let Some(mobile) = &update.mobile {
        if mobile.len() != 10 || !mobile.chars().all(|c| c.is_numeric()) {
            return HttpResponse::BadRequest().body("Mobile must be 10 digits");
        }
    }
    if let Some(why_apply) = &update.why_apply {
        if why_apply.split_whitespace().count() > 150 {
            return HttpResponse::BadRequest().body("Why apply must be 150 words or less");
        }
    }
    if let Some(event_experience) = &update.event_experience {
        if event_experience.split_whitespace().count() > 50 {
            return HttpResponse::BadRequest().body("Event experience must be 50 words or less");
        }
    }
    if let Some(grad_year) = &update.grad_year {
        if grad_year != "2028" && grad_year != "2029" {
            return HttpResponse::BadRequest().body("Grad year must be 2028 or 2029");
        }
    }
    // Check capacity if interview_slot is being set
    if let Some(interview_slot) = &update.interview_slot {
        let applicants = data.applicants.lock().unwrap();
        let booked = applicants
            .iter()
            .filter(|a| a.interview_slot.as_deref() == Some(interview_slot) && a.id != id)
            .count() as u32;
        if booked >= 60 {
            return HttpResponse::BadRequest().body("Slot is full");
        }
    }
    // Lock, modify, and release the lock before any .await
    let (updated_applicant, is_new) = {
        let mut applicants = data.applicants.lock().unwrap();
        if let Some(applicant) = applicants.iter_mut().find(|a| a.id == id) {
            println!("Applicant found: {}", applicant.id);
            if let Some(grad_year) = update.grad_year {
                applicant.grad_year = Some(grad_year);
            }
            if let Some(mobile) = update.mobile {
                applicant.mobile = Some(mobile);
            }
            if let Some(gender) = update.gender {
                applicant.gender = Some(gender);
            }
            if let Some(faculty) = update.faculty {
                applicant.faculty = Some(faculty);
            }
            if let Some(department) = update.department {
                applicant.department = Some(department);
            }
            if let Some(skill) = update.skill {
                applicant.skill = Some(skill);
            }
            if let Some(event_participation) = update.event_participation {
                applicant.event_participation = Some(event_participation);
            }
            if let Some(why_apply) = update.why_apply {
                applicant.why_apply = Some(why_apply);
            }
            if let Some(event_experience) = update.event_experience {
                applicant.event_experience = Some(event_experience);
            }
            if let Some(submission1_url) = update.submission1_url {
                applicant.submission1_url = Some(submission1_url);
            }
            if let Some(submission2_url) = update.submission2_url {
                applicant.submission2_url = Some(submission2_url);
            }
            if let Some(interview_slot) = update.interview_slot {
                applicant.interview_slot = Some(interview_slot);
            }
            if let Some(is_selected_value) = update.is_selected {
                if is_admin {
                    applicant.is_selected = is_selected_value;
                }
            }
            if let Some(is_admin_value) = update.is_admin {
                if is_admin {
                    applicant.is_admin = is_admin_value;
                }
            }
            if let Some(status) = update.status {
                if is_admin {
                    applicant.status = Some(status);
                }
            }
            if let Some(round) = update.round {
                if is_admin {
                    applicant.round = Some(round);
                }
            }
            (Some(applicant.clone()), false)
        } else {
            // New applicant
            let new_applicant = Applicant {
                id: update.id.clone(),
                name: update.name.clone(),
                email: update.email.clone(),
                password: update
                    .password
                    .clone()
                    .unwrap_or_else(|| "default".to_string()),
                grad_year: update.grad_year.clone(),
                mobile: update.mobile.clone(),
                gender: update.gender.clone(),
                faculty: update.faculty.clone(),
                department: update.department.clone(),
                skill: update.skill.clone(),
                event_participation: update.event_participation,
                why_apply: update.why_apply.clone(),
                event_experience: update.event_experience.clone(),
                submission1_url: update.submission1_url.clone(),
                submission2_url: update.submission2_url.clone(),
                interview_slot: update.interview_slot.clone(),
                is_selected: update.is_selected.unwrap_or(false),
                is_admin: update.is_admin.unwrap_or(false),
                status: update.status.clone().or(Some("In Consideration".to_string())),
                round: update.round.clone().or(Some("Applied".to_string())),
            };
            applicants.push(new_applicant.clone());
            (Some(new_applicant), true)
        }
    }; // MutexGuard dropped here, before the .await

    match updated_applicant {
        Some(applicant) => {
            let result = if is_new {
                data.google_client.append_applicant_row(&applicant).await
            } else {
                data.google_client.update_applicant_row(&applicant).await
            };
            if let Err(e) = result {
                return HttpResponse::InternalServerError().json(e.to_string());
            }
            HttpResponse::Ok().json(applicant)
        }
        None => HttpResponse::NotFound().json("Applicant not found"),
    }
}

/**
 * REQ: POST api/applicants/submit?case_study=1 or 2
 * What it does: Receives a file and uploads it to Google Drive, updates applicant's submission URL
 */
#[actix_web::post("/submit")]
pub async fn submit_file(
    auth: BearerAuth,
    query: web::Query<HashMap<String, String>>,
    mut payload: Multipart,
    data: web::Data<AppState>,
    config: web::Data<AuthConfig>,
) -> impl Responder {
    // Decode JWT
    let token = auth.token();
    let decoded = match jsonwebtoken::decode::<Claims>(
        &token,
        &config.decoding_key,
        &jsonwebtoken::Validation::default(),
    ) {
        Ok(d) => d,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let user_id = decoded.claims.sub;

    let case_study = query
        .get("case_study")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1);

    let mut applicants = data.applicants.lock().unwrap();
    let applicant = applicants
        .iter_mut()
        .find(|a| &a.id == &user_id || &a.email == &user_id);
    if applicant.is_none() {
        return HttpResponse::NotFound().body("Applicant not found");
    }
    let applicant = applicant.unwrap();

    // Check if skill applicant (has skill field set)
    let is_skill = applicant.skill.as_deref().map_or(false, |s| !s.is_empty());
    let max_submissions = if is_skill { 2 } else { 1 };
    if case_study > max_submissions {
        return HttpResponse::BadRequest().body(format!(
            "Invalid case study number. Max allowed: {}",
            max_submissions
        ));
    }

    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .unwrap_or("unknown")
            .to_string();
        let mime_type = field
            .content_type()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let mut body = Vec::new();
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(data) => body.extend_from_slice(&data),
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            }
        }

        match data
            .google_client
            .upload_file(&filename, body, &mime_type)
            .await
        {
            Ok(url) => {
                // Update applicant's submission URL
                if case_study == 1 {
                    applicant.submission1_url = Some(url.clone());
                } else if case_study == 2 {
                    applicant.submission2_url = Some(url.clone());
                }
                // Update in Sheets
                if let Err(e) = data.google_client.update_applicant_row(applicant).await {
                    return HttpResponse::InternalServerError().body(e.to_string());
                }
                return HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "file_url": url,
                    "message": format!("Case study {} uploaded successfully for {}", case_study, user_id)
                }));
            }
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    HttpResponse::BadRequest().body("No file found in request")
}

/**
 * REQ: GET api/applicants/slots
 * What it does: Returns available interview slots with remaining capacity
 */
#[get("/slots")]
pub async fn get_available_slots(data: web::Data<AppState>) -> impl Responder {
    // Hardcoded slots for simplicity
    let predefined_slots = vec![
        InterviewSlot {
            date_time: "2026-05-10T10:00:00Z".to_string(),
            capacity: 60,
        },
        InterviewSlot {
            date_time: "2026-05-10T14:00:00Z".to_string(),
            capacity: 60,
        },
        InterviewSlot {
            date_time: "2026-05-11T10:00:00Z".to_string(),
            capacity: 60,
        },
        // Add more as needed
    ];

    let applicants = data.applicants.lock().unwrap();
    let mut available_slots = Vec::new();

    for slot in predefined_slots {
        let booked = applicants
            .iter()
            .filter(|a| a.interview_slot.as_deref() == Some(&slot.date_time))
            .count() as u32;
        if booked < slot.capacity {
            available_slots.push(serde_json::json!({
                "date_time": slot.date_time,
                "remaining_capacity": slot.capacity - booked
            }));
        }
    }

    HttpResponse::Ok().json(available_slots)
}
