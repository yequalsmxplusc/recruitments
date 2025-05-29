use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_cors::Cors;
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
    let admin_username = std::env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");//future superadmin use
    let admin_password_hash = std::env::var("ADMIN_PASSWORD_HASH").expect("ADMIN_PASSWORD_HASH must be set");//future superadmin use
     let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    // Pass all three arguments to AuthConfig::new
    let auth_config = AuthConfig::new(&jwt_secret, admin_username, admin_password_hash);

    // Load initial data for applicants
    let initial_data = load_applicants().await;
    let app_state = web::Data::new(AppState {
        applicants: Mutex::new(initial_data),
    });

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
        /*
         * Services Used: POST /login,
         *  GET /api/applicants,
         * GET /api/applicants/all,  
         * PATCH /api/applicants/{id}
         */

     println!("[LOG] Port running on {}",bind_address);
    // Start the Actix server
    HttpServer::new(move || {

    //  let allowed_origin = frontend_url.clone();

     let cors = Cors::default()
    .allowed_origin(&frontend_url)
    .allowed_methods(vec!["GET", "POST", "PATCH", "OPTIONS"])
    .allowed_headers(vec![
        actix_web::http::header::AUTHORIZATION,
        actix_web::http::header::CONTENT_TYPE,
    ]) 
    .max_age(3600) 
    .supports_credentials(); 

        let auth_middleware = HttpAuthentication::bearer(validator);
        // println!("CORS AT: {}",frontend_url);
        App::new()
        .wrap(cors)
            .app_data(app_state.clone())
            .app_data(web::Data::new(auth_config.clone()))
            .service(login)
            .service(
                web::scope("/api")
                .wrap(auth_middleware)
                    .service(
                        web::scope("/applicants")
                            .service(applicants::service::get_all_applicants)
                            .service(applicants::service::get_applicant)
                            .service(applicants::service::update_applicant),
                    ),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}