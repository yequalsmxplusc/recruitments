use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use applicants::service::{AppState, load_applicants};
use auth::{service::login, models::AuthConfig, middleware::validator};
use std::sync::Mutex;

mod auth {
    pub mod models;
    pub mod middleware;
    pub mod service;
}

mod applicants {
    pub mod service;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get the environment variables for secret, admin username, and password hash
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let admin_username = std::env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
    let admin_password_hash = std::env::var("ADMIN_PASSWORD_HASH").expect("ADMIN_PASSWORD_HASH must be set");

    // Pass all three arguments to AuthConfig::new
    let auth_config = AuthConfig::new(&jwt_secret, admin_username, admin_password_hash);

    // Load initial data for applicants
    let initial_data = load_applicants().await;
    let app_state = web::Data::new(AppState {
        applicants: Mutex::new(initial_data),
    });

    // Start the Actix server
    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::bearer(validator);

        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(auth_config.clone()))
            .service(login)
            .service(
                web::scope("/api")
                    .wrap(auth_middleware)
                    .service(
                        web::scope("/applicants")
                            .service(applicants::service::get_applicants)
                            .service(applicants::service::update_applicant),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}