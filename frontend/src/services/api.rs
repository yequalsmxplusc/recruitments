use crate::models::applicant::Applicant;
use gloo_net::http::{Request, Response};
// use serde_json::from_str;

const API_BASE: &str = "http://localhost:8000";

// Change fetch_applicants to return a single Applicant (not Vec<Applicant>)
pub async fn fetch_applicant(token: String) -> Result<Applicant, String> {
    let response = Request::get(&format!("{}/api/applicants/", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    parse_response(response).await
}

pub async fn fetch_all_applicants(token: String) -> Result<Vec<Applicant>, String> {
    let response = Request::get(&format!("{}/api/applicants/all", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    if text.contains("Admin access required") {
        return Err("not_admin".to_string()); // custom signal to show 404
    }
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub async fn update_applicant(
    applicant: &Applicant,
    token: String,
) -> Result<Applicant, String> {
    let response = Request::patch(&format!("{}/api/applicants/{}", API_BASE, applicant.id))
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