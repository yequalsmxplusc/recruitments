use gloo_storage::{LocalStorage, Storage};

pub fn set<T: serde::Serialize>(key: &str, value: &T) -> Result<(), String> {
    LocalStorage::set(key, value).map_err(|e| e.to_string())
}

pub fn get<T: serde::de::DeserializeOwned>(key: &str) -> Result<T, String> {
    LocalStorage::get(key).map_err(|e| e.to_string())
}

pub fn delete(key: &str) {
    LocalStorage::delete(key);
}
