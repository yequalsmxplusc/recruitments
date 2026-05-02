use crate::models::applicant::{Applicant, InterviewSlot};
use gloo_net::http::{Request, Response};
pub fn get_api_base() -> &'static str {
    option_env!("API_BASE").unwrap_or("http://127.0.0.1:8000")
}

pub async fn fetch_applicant(token: String) -> Result<Applicant, String> {
    let response = Request::get(&format!("{}/api/applicants/", get_api_base()))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    parse_response(response).await
}

pub async fn fetch_all_applicants(token: String) -> Result<Vec<Applicant>, String> {
    let response = Request::get(&format!("{}/api/applicants/all", get_api_base()))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    if text.contains("Admin access required") {
        return Err("not_admin".to_string());
    }
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub async fn update_applicant(
    applicant: &Applicant,
    token: String,
) -> Result<Applicant, String> {
    let response = Request::patch(&format!("{}/api/applicants/{}", get_api_base(), applicant.id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(applicant)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    parse_response(response).await
}

pub async fn submit_case_study(
    file_data: Vec<u8>,
    _filename: String,
    case_study: u32,
    token: String,
) -> Result<SubmissionResponse, String> {
    use web_sys::FormData;
    use web_sys::Blob;
    use web_sys::BlobPropertyBag;
    
    let blob_property_bag = BlobPropertyBag::new();
    blob_property_bag.set_type("application/pdf");
    let uint8_array = js_sys::Uint8Array::from(file_data.as_slice());
    let array = js_sys::Array::new();
    array.push(&uint8_array);
    let blob = Blob::new_with_u8_array_sequence_and_options(&array.into(), &blob_property_bag)
        .map_err(|_| "Failed to create blob".to_string())?;
    
    let form_data = FormData::new()
        .map_err(|_| "Failed to create FormData".to_string())?;
    
    form_data
        .append_with_blob(&"file", &blob)
        .map_err(|_| "Failed to append file to FormData".to_string())?;
    
    let response = Request::post(&format!(
        "{}/api/applicants/submit?case_study={}",
        get_api_base(), case_study
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .body(form_data)
    .map_err(|e| e.to_string())?
    .send()
    .await
    .map_err(|e| e.to_string())?;
    
    parse_response(response).await
}

pub async fn get_available_slots(
    token: String,
) -> Result<Vec<InterviewSlot>, String> {
    let response = Request::get(&format!("{}/api/applicants/slots", get_api_base()))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    parse_response(response).await
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubmissionResponse {
    pub status: String,
    pub file_url: String,
    pub message: String,
}

async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, String> {
    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Server error ({}): {}", status, text))
    }
}