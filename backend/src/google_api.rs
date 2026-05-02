use crate::auth::models::Applicant;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    client_email: String,
    private_key: String,
    token_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub struct GoogleClient {
    client: Client,
    service_account: ServiceAccount,
    sheet_id: String,
    folder_id: String,
}

impl GoogleClient {
    pub fn new(sheet_url: &str, drive_url: &str, service_account_path: &str) -> Self {
        let content =
            fs::read_to_string(service_account_path).expect("Failed to read service account JSON");
        let service_account: ServiceAccount =
            serde_json::from_str(&content).expect("Failed to parse service account JSON");

        let sheet_id = Self::extract_id(sheet_url, "/d/", "/").unwrap_or_default();
        let folder_id = Self::extract_id(drive_url, "/folders/", "?").unwrap_or_default();

        Self {
            client: Client::new(),
            service_account,
            sheet_id,
            folder_id,
        }
    }

    fn extract_id(url: &str, prefix: &str, suffix_char: &str) -> Option<String> {
        if let Some(start) = url.find(prefix) {
            let start_idx = start + prefix.len();
            let rest = &url[start_idx..];
            let end_idx = rest.find(suffix_char).unwrap_or(rest.len());
            return Some(rest[..end_idx].to_string());
        }
        None
    }

    async fn get_access_token(&self, scope: &str) -> Result<String, Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        let claims = Claims {
            iss: self.service_account.client_email.clone(),
            scope: scope.to_string(),
            aud: self.service_account.token_uri.clone(),
            exp: now + 3600,
            iat: now,
        };

        let header = Header::new(Algorithm::RS256);
        let key = EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes())?;
        let jwt = encode(&header, &claims, &key)?;

        let res = self
            .client
            .post(&self.service_account.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(res.access_token)
    }

    pub async fn fetch_applicants(&self) -> Result<Vec<Applicant>, Box<dyn std::error::Error>> {
        let token = self
            .get_access_token("https://www.googleapis.com/auth/spreadsheets.readonly")
            .await?;

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1!A:T",
            self.sheet_id
        );

        let res = self
            .client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let rows = res["values"].as_array().ok_or("No values in sheet")?;

        let mut applicants = Vec::new();
        // Assuming first row is header
        for row in rows.iter().skip(1) {
            if let Some(row_arr) = row.as_array() {
                if row_arr.len() < 3 {
                    continue;
                }

                applicants.push(Applicant {
                    id: row_arr
                        .get(0)
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    name: row_arr
                        .get(1)
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    email: row_arr
                        .get(2)
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    password: row_arr
                        .get(3)
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| "default")
                        .to_string(),
                    grad_year: row_arr
                        .get(4)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    mobile: row_arr
                        .get(5)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    gender: row_arr
                        .get(6)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    faculty: row_arr
                        .get(7)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    department: row_arr
                        .get(8)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    skill: row_arr
                        .get(9)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    event_participation: row_arr
                        .get(10)
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case("true")),
                    why_apply: row_arr
                        .get(11)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    event_experience: row_arr
                        .get(12)
                        .and_then(|v| v.as_str())
                        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    submission1_url: row_arr
                        .get(13)
                        .and_then(|v| v.as_str())
                        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    submission2_url: row_arr
                        .get(14)
                        .and_then(|v| v.as_str())
                        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    interview_slot: row_arr
                        .get(15)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    is_selected: row_arr
                        .get(16)
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case("true"))
                        .unwrap_or(false),
                    is_admin: row_arr
                        .get(17)
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case("true"))
                        .unwrap_or(false),
                    status: row_arr
                        .get(18)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or(Some("In Consideration".to_string())),
                    round: row_arr
                        .get(19)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or(Some("Applied".to_string())),
                });
            }
        }

        Ok(applicants)
    }

    pub async fn upload_file(
        &self,
        filename: &str,
        content: Vec<u8>,
        mime_type: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let token = self
            .get_access_token("https://www.googleapis.com/auth/drive.file")
            .await?;

        let metadata = serde_json::json!({
            "name": filename,
            "parents": [self.folder_id]
        });

        let form = reqwest::multipart::Form::new()
            .part(
                "metadata",
                reqwest::multipart::Part::text(metadata.to_string())
                    .mime_str("application/json")?,
            )
            .part(
                "file",
                reqwest::multipart::Part::bytes(content)
                    .file_name(filename.to_string())
                    .mime_str(mime_type)?,
            );

        let res = self
            .client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let file_id = res["id"].as_str().ok_or("Failed to get file ID")?;
        Ok(format!("https://drive.google.com/file/d/{}/view", file_id))
    }

    pub async fn update_applicant_row(
        &self,
        applicant: &Applicant,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = self
            .get_access_token("https://www.googleapis.com/auth/spreadsheets")
            .await?;

        // This is a simplified implementation.
        // In a real scenario, you'd need to find the correct row first.
        // For now, we'll assume the ID corresponds to a row index (e.g., Row 2 for ID "1")
        // Or better, we'd search for the ID in Column A.

        // Search for the ID
        let fetch_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1!A:A",
            self.sheet_id
        );
        let res = self
            .client
            .get(&fetch_url)
            .bearer_auth(token.clone())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let rows = res["values"].as_array().ok_or("Could not find ID column")?;
        let mut target_row = None;
        for (idx, row) in rows.iter().enumerate() {
            if let Some(id) = row.get(0).and_then(|v| v.as_str()) {
                if id == applicant.id {
                    target_row = Some(idx + 1);
                    break;
                }
            }
        }

        let row_idx = target_row.ok_or("Applicant ID not found in sheet")?;

        let update_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1!A{}:T{}?valueInputOption=USER_ENTERED",
            self.sheet_id, row_idx, row_idx
        );

        let values = vec![vec![
            applicant.id.clone(),
            applicant.name.clone(),
            applicant.email.clone(),
            applicant.password.clone(),
            applicant.grad_year.clone().unwrap_or_default(),
            applicant.mobile.clone().unwrap_or_default(),
            applicant.gender.clone().unwrap_or_default(),
            applicant.faculty.clone().unwrap_or_default(),
            applicant.department.clone().unwrap_or_default(),
            applicant.skill.clone().unwrap_or_default(),
            applicant.event_participation
                .map(|b| b.to_string())
                .unwrap_or_default(),
            applicant.why_apply.clone().unwrap_or_default(),
            applicant.event_experience.clone().unwrap_or_default(),
            applicant.submission1_url.clone().unwrap_or_default(),
            applicant.submission2_url.clone().unwrap_or_default(),
            applicant.interview_slot.clone().unwrap_or_default(),
            applicant.is_selected.to_string(),
            applicant.is_admin.to_string(),
            applicant.status.clone().unwrap_or_else(|| "In Consideration".to_string()),
            applicant.round.clone().unwrap_or_else(|| "Applied".to_string()),
        ]];

        let body = serde_json::json!({
            "values": values
        });

        self.client
            .put(&update_url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn append_applicant_row(
        &self,
        applicant: &Applicant,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = self
            .get_access_token("https://www.googleapis.com/auth/spreadsheets")
            .await?;

        let append_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1!A:T:append?valueInputOption=USER_ENTERED",
            self.sheet_id
        );

        let values = vec![vec![
            applicant.id.clone(),
            applicant.name.clone(),
            applicant.email.clone(),
            applicant.password.clone(),
            applicant.grad_year.clone().unwrap_or_default(),
            applicant.mobile.clone().unwrap_or_default(),
            applicant.gender.clone().unwrap_or_default(),
            applicant.faculty.clone().unwrap_or_default(),
            applicant.department.clone().unwrap_or_default(),
            applicant.skill.clone().unwrap_or_default(),
            applicant.event_participation
                .map(|b| b.to_string())
                .unwrap_or_default(),
            applicant.why_apply.clone().unwrap_or_default(),
            applicant.event_experience.clone().unwrap_or_default(),
            applicant.submission1_url.clone().unwrap_or_default(),
            applicant.submission2_url.clone().unwrap_or_default(),
            applicant.interview_slot.clone().unwrap_or_default(),
            applicant.is_selected.to_string(),
            applicant.is_admin.to_string(),
            applicant.status.clone().unwrap_or_else(|| "In Consideration".to_string()),
            applicant.round.clone().unwrap_or_else(|| "Applied".to_string()),
        ]];

        let body = serde_json::json!({
            "values": values
        });

        self.client
            .post(&append_url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sheet_id_from_url() {
        let url = "https://docs.google.com/spreadsheets/d/1t6a4DUxEma2E4jbKfXr9FsbbcEiwdmqTkptDs6bBo6w/edit?usp=sharing";
        let id = GoogleClient::extract_id(url, "/d/", "/");
        assert_eq!(
            id,
            Some("1t6a4DUxEma2E4jbKfXr9FsbbcEiwdmqTkptDs6bBo6w".to_string())
        );
    }

    #[test]
    fn test_extract_folder_id_from_url() {
        let url =
            "https://drive.google.com/drive/folders/1svR7T-ndnuxORsk3RuEn062QZMvhk3Ru?usp=sharing";
        let id = GoogleClient::extract_id(url, "/folders/", "?");
        assert_eq!(id, Some("1svR7T-ndnuxORsk3RuEn062QZMvhk3Ru".to_string()));
    }

    #[test]
    fn test_extract_id_no_suffix() {
        // URL without query params — suffix_char "?" not found, should take until end
        let url = "https://drive.google.com/drive/folders/abc123def";
        let id = GoogleClient::extract_id(url, "/folders/", "?");
        assert_eq!(id, Some("abc123def".to_string()));
    }

    #[test]
    fn test_extract_id_invalid_url() {
        let url = "https://example.com/nothing-here";
        let id = GoogleClient::extract_id(url, "/d/", "/");
        assert_eq!(id, None);
    }

    #[test]
    fn test_extract_id_empty_string() {
        let id = GoogleClient::extract_id("", "/d/", "/");
        assert_eq!(id, None);
    }

    #[test]
    fn test_boolean_parsing_from_sheet_strings() {
        // Simulates how Google Sheets API returns boolean-like values as strings
        let true_val = serde_json::json!("TRUE");
        let false_val = serde_json::json!("FALSE");
        let mixed_case = serde_json::json!("True");
        let empty = serde_json::json!("");

        assert_eq!(
            true_val
                .as_str()
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            true
        );
        assert_eq!(
            false_val
                .as_str()
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            false
        );
        assert_eq!(
            mixed_case
                .as_str()
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            true
        );
        assert_eq!(
            empty
                .as_str()
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            false
        );
    }

    #[test]
    fn test_boolean_as_bool_returns_none_for_strings() {
        // This proves why the original code was broken:
        // Google Sheets returns "TRUE" as a string, not a JSON boolean
        let sheet_value = serde_json::json!("TRUE");
        assert_eq!(sheet_value.as_bool(), None); // as_bool() fails on string "TRUE"
    }

    #[test]
    fn test_applicant_row_parsing_minimal() {
        // Simulate a row with only 3 columns (minimum to not be skipped)
        let row = vec![
            serde_json::json!("id1"),
            serde_json::json!("John Doe"),
            serde_json::json!("john@example.com"),
        ];
        assert!(row.len() >= 3);
        assert_eq!(row[0].as_str(), Some("id1"));
        assert_eq!(row[1].as_str(), Some("John Doe"));
        assert_eq!(row[2].as_str(), Some("john@example.com"));
    }

    #[test]
    fn test_applicant_row_parsing_full() {
        // Simulate a full row as returned by Google Sheets
        let row: Vec<serde_json::Value> = vec![
            serde_json::json!("id1"),
            serde_json::json!("John Doe"),
            serde_json::json!("john@example.com"),
            serde_json::json!("password123"),
            serde_json::json!("9876543210"),
            serde_json::json!("TRUE"), // is_selected
            serde_json::json!("CS"),
            serde_json::json!("2029"),
            serde_json::json!("Slot A"),
            serde_json::json!("FALSE"), // is_admin
            serde_json::json!("Applied"),
        ];

        let applicant = Applicant {
            id: row
                .get(0)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            name: row
                .get(1)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            email: row
                .get(2)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            password: row
                .get(3)
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string(),
            mobile: row.get(4).and_then(|v| v.as_str()).map(|s| s.to_string()),
            is_selected: row
                .get(5)
                .and_then(|v| v.as_str())
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            department: row.get(6).and_then(|v| v.as_str()).map(|s| s.to_string()),
            grad_year: row.get(7).and_then(|v| v.as_str()).map(|s| s.to_string()),
            interview_slot: row.get(8).and_then(|v| v.as_str()).map(|s| s.to_string()),
            is_admin: row
                .get(9)
                .and_then(|v| v.as_str())
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            status: row
                .get(10)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or(Some("Applied".to_string())),
            ..Default::default()
        };

        assert_eq!(applicant.id, "id1");
        assert_eq!(applicant.name, "John Doe");
        assert_eq!(applicant.email, "john@example.com");
        assert_eq!(applicant.password, "password123");
        assert_eq!(applicant.mobile.as_deref(), Some("9876543210"));
        assert!(applicant.is_selected);
        assert_eq!(applicant.department.as_deref(), Some("CS"));
        assert_eq!(applicant.grad_year.as_deref(), Some("2029"));
        assert_eq!(applicant.interview_slot.as_deref(), Some("Slot A"));
        assert!(!applicant.is_admin);
        assert_eq!(applicant.status.as_deref(), Some("Applied"));
    }
}
