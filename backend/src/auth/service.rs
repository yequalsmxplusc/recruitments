use actix_web::{post, web, HttpResponse, Responder};
use jsonwebtoken::{encode, Header};
use chrono::Utc;
use argon2::verify_encoded;
use super::models::{LoginRequest, LoginResponse, Claims, AuthConfig};

#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginRequest>,
    auth_config: web::Data<AuthConfig>,
) -> impl Responder {
    let stored_username = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".into());
    let stored_hash = std::env::var("ADMIN_PASSWORD_HASH")
        .expect("ADMIN_PASSWORD_HASH must be set");
    
    if credentials.username != stored_username {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }

    // 🔐 Using Argon2 instead of bcrypt
    if !verify_encoded(&stored_hash, credentials.password.as_bytes()).unwrap_or(false) {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: credentials.username.clone(),
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &auth_config.encoding_key,
    ).unwrap();

    HttpResponse::Ok().json(LoginResponse { token })
}