use super::models::{AuthConfig, Claims, LoginRequest, LoginResponse};
use crate::applicants::service::AppState;
use actix_web::{HttpResponse, Responder, post, web};
use chrono::Utc;
use jsonwebtoken::{Header, encode};

/**
 * REQ: POST /login
 * RES: JWT Token Bearer <your_jwt_token>
 */
#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginRequest>,
    auth_config: web::Data<AuthConfig>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Read applicants from in-memory state (loaded from Google Sheets at startup)
    let applicants = data.applicants.lock().unwrap();

    // Check if the provided credentials match any applicant
    let applicant = applicants
        .iter()
        .find(|a| a.id == credentials.username || a.email == credentials.username);

    if let Some(applicant) = applicant {
        // println!("Applicant found: {:?}", applicant);

        // Directly compare the passwords (not secure for production)
        if applicant.password != credentials.password {
            return HttpResponse::Unauthorized().json("Invalid credentials: Password Mismatch");
        }

        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: credentials.username.clone(),
            exp: expiration as usize,
            is_admin: applicant.is_admin,
        };

        let token = encode(&Header::default(), &claims, &auth_config.encoding_key).unwrap();

        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        // println!("No applicant found with the provided username.");
        HttpResponse::Unauthorized().json("Invalid Credentials: else block")
    }
}
