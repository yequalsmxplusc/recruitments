use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub admin_username: String,
    pub admin_password_hash: String,
}

impl AuthConfig {
    pub fn new(secret: &str, username: String, password_hash: String) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            admin_username: username,
            admin_password_hash: password_hash,
        }
    }
}
