use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use applicants::service::{AppState, load_applicants};
use auth::{middleware::validator, models::AuthConfig, service::login};
use dotenv::dotenv;
use std::sync::Mutex;

mod google_api;
mod auth {
    pub mod middleware;
    pub mod models;
    pub mod service;
}

mod applicants {
    pub mod service;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get the environment variables
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let admin_username = std::env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
    let admin_password_hash =
        std::env::var("ADMIN_PASSWORD_HASH").expect("ADMIN_PASSWORD_HASH must be set");
    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let frontend_url_prod =
        std::env::var("FRONTEND_URL_PROD").expect("FRONTEND_URL_PROD must be set");
    let sheet_url = std::env::var("SHEET_URL").expect("SHEET_URL must be set");
    let drive_url = std::env::var("DRIVE_URL").expect("DRIVE_URL must be set");
    let service_account_path = std::env::var("GOOGLE_SERVICE_ACCOUNT_JSON")
        .expect("GOOGLE_SERVICE_ACCOUNT_JSON must be set");
    let upload_proxy_url = std::env::var("UPLOAD_PROXY_URL").ok();

    let auth_config = AuthConfig::new(&jwt_secret, admin_username, admin_password_hash);
    let google_client =
        google_api::GoogleClient::new(&sheet_url, &drive_url, &service_account_path, upload_proxy_url);

    // Load initial data for applicants
    let initial_data = load_applicants(&google_client).await;
    let app_state = web::Data::new(AppState {
        applicants: Mutex::new(initial_data),
        google_client,
    });

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    println!("[LOG] Port running on {}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin(&frontend_url_prod)
            .allowed_methods(vec!["GET", "POST", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
            .supports_credentials();

        let auth_middleware = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .app_data(web::Data::new(auth_config.clone()))
            .service(login)
            .service(
                web::scope("/api").wrap(auth_middleware).service(
                    web::scope("/applicants")
                        .service(applicants::service::get_all_applicants)
                        .service(applicants::service::get_applicant)
                        .service(applicants::service::update_applicant)
                        .service(applicants::service::submit_file)
                        .service(applicants::service::get_available_slots),
                ),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
