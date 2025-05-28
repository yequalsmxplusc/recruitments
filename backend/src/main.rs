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
        /*
         * Services Used: POST /login,
         *  GET /api/applicants,
         * GET /api/applicants/all,  
         * PATCH /api/applicants/{id}
         */
    // Start the Actix server
    HttpServer::new(move || {

     let allowed_origin = frontend_url.clone();

     let cors = Cors::default()
    .allowed_origin_fn({
        let allowed_origin = allowed_origin.clone();
        move |origin, _req_head| {
            origin.as_bytes() == allowed_origin.as_bytes()
                || origin == "http://localhost:3000"
                || origin == "http://127.0.0.1:3000"
                ||origin == "https://mrtlhcd9-3000.inc1.devtunnels.ms/"
        }
    })
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
    .bind("0.0.0.0:8000")?
    .run()
    .await
}