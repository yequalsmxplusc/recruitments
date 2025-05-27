use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, Validation};
use crate::auth::models::{Claims, AuthConfig};
use actix_web::web;

//Validates jwt authenticator token
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let auth_config = req
        .app_data::<web::Data<AuthConfig>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Auth config not found"))?;

    let token = credentials.token();
    decode::<Claims>(
        token,
        &auth_config.decoding_key,
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    Ok(req)
}