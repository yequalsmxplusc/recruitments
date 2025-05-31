use crate::models::applicant::Applicant;
use gloo_net::http::{Request, Response};
// use dotenv::dotenv;
// use std::env;
// use std::sync::OnceLock;

// //env call in WASM (non cli-static)
// static API_BASE: OnceLock<String> = OnceLock::new();

// pub fn get_api_base() -> &'static str {
//     API_BASE.get_or_init(|| {
//         option_env!("API_BASE")
//             .expect("API_BASE must be set at compile time")
//             .to_string()
//     })
// }

const get_api_base: &str = "https://recruitments-backend-a55x.onrender.com";


// Change fetch_applicants to return a single Applicant (not Vec<Applicant>)
pub async fn fetch_applicant(token: String) -> Result<Applicant, String> {
    let response = Request::get(&format!("{}/api/applicants/", get_api_base))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    parse_response(response).await
}

pub async fn fetch_all_applicants(token: String) -> Result<Vec<Applicant>, String> {
    let response = Request::get(&format!("{}/api/applicants/all", get_api_base))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    if text.contains("Admin access required") {
        return Err("not_admin".to_string()); // custom signal to show 404
    }
    // Log raw JSON to browser console
    web_sys::console::log_1(&format!("Applicants JSON: {}", text).into());
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub async fn update_applicant(
    applicant: &Applicant,
    token: String,
) -> Result<Applicant, String> {
    let response = Request::patch(&format!("{}/api/applicants/{}", get_api_base, applicant.id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(applicant)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    parse_response(response).await
}

async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, String> {
    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Server error: {}", response.status_text()))
    }
}