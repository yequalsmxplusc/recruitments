use actix_web::{post, web, HttpResponse, Responder};
use jsonwebtoken::{encode, Header};
use chrono::Utc;
use std::fs;
use super::models::{LoginRequest, LoginResponse, Claims, AuthConfig, Applicant};

/**
 * REQ: GET /login
 * RES: JWT Token Bearer <your_jwt_token>
 */
#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginRequest>,
    auth_config: web::Data<AuthConfig>,
) -> impl Responder {
    // println!("Attempting to read data.json file...");

    // Load applicants data from data.json
    let applicants_data = fs::read_to_string("data/data.json")
        .expect("Unable to read file");
    // println!("File read successfully. Data: {}", applicants_data);

    let applicants: Vec<Applicant> = serde_json::from_str(&applicants_data).unwrap_or_default();
    // println!("Deserialized applicants: {:?}", applicants);

    // Check if the provided credentials match any applicant
    let applicant = applicants.iter().find(|a: &&Applicant| a.id == credentials.username || a.email == credentials.username);
    // println!("Searching for applicant with username: {}", credentials.username);

    if let Some(applicant) = applicant {
        println!("Applicant found: {:?}", applicant);

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

        let token = encode(
            &Header::default(),
            &claims,
            &auth_config.encoding_key,
        ).unwrap();

        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        // println!("No applicant found with the provided username.");
        HttpResponse::Unauthorized().json("Invalid Credentials: else block")
    }
}
