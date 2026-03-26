use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub masked_key: String,
    pub created_at: String,
    pub last_used: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveApiKeyRequest {
    pub key_name: String,
    pub api_key: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResult {
    pub success: bool,
    pub error: Option<String>,
}

pub struct SecurityManager {
    keys: Mutex<HashMap<String, (String, String, Option<String>)>>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            keys: Mutex::new(HashMap::new()),
        }
    }

    pub fn save_api_key(&self, key_name: &str, api_key: &str) -> Result<(), String> {
        let mut keys = self.keys.lock().map_err(|e| e.to_string())?;

        let masked = mask_api_key(api_key);
        let created_at = chrono::Utc::now().to_rfc3339();

        keys.insert(
            key_name.to_string(),
            (api_key.to_string(), created_at, None),
        );

        log::info!("API Key '{}' saved successfully (masked: {})", key_name, masked);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_api_key(&self, key_name: &str) -> Option<String> {
        let keys = self.keys.lock().ok()?;
        keys.get(key_name).map(|(key, _, _)| key.clone())
    }

    pub fn get_api_key_info(&self, key_name: &str) -> Option<ApiKeyInfo> {
        let keys = self.keys.lock().ok()?;

        keys.get(key_name).map(|(key, created_at, last_used)| {
            ApiKeyInfo {
                masked_key: mask_api_key(key),
                created_at: created_at.clone(),
                last_used: last_used.clone(),
            }
        })
    }

    pub fn delete_api_key(&self, key_name: &str) -> Result<(), String> {
        let mut keys = self.keys.lock().map_err(|e| e.to_string())?;

        if keys.remove(key_name).is_some() {
            log::info!("API Key '{}' deleted successfully", key_name);
            Ok(())
        } else {
            Err(format!("API Key '{}' not found", key_name))
        }
    }

    #[allow(dead_code)]
    pub fn list_api_keys(&self) -> Vec<String> {
        let keys = self.keys.lock().unwrap();
        keys.keys().cloned().collect()
    }

    #[allow(dead_code)]
    pub fn update_last_used(&self, key_name: &str) {
        let mut keys = match self.keys.lock() {
            Ok(keys) => keys,
            Err(e) => e.into_inner(),
        };

        if let Some((_, _, last_used)) = keys.get_mut(key_name) {
            *last_used = Some(chrono::Utc::now().to_rfc3339());
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

fn mask_api_key(api_key: &str) -> String {
    if api_key.len() <= 8 {
        return "*".repeat(api_key.len());
    }

    let prefix = &api_key[..4];
    let suffix = &api_key[api_key.len() - 4..];

    format!("{}****{}", prefix, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_api_key() {
        assert_eq!(mask_api_key("sk-1234567890"), "sk-1****7890");
        assert_eq!(mask_api_key("short"), "*****");
        assert_eq!(mask_api_key(""), "");
    }

    #[test]
    fn test_save_and_get_api_key() {
        let manager = SecurityManager::new();

        manager.save_api_key("test", "sk-1234567890").unwrap();

        assert_eq!(
            manager.get_api_key("test"),
            Some("sk-1234567890".to_string())
        );
    }

    #[test]
    fn test_delete_api_key() {
        let manager = SecurityManager::new();

        manager.save_api_key("test", "sk-1234567890").unwrap();
        assert!(manager.get_api_key("test").is_some());

        manager.delete_api_key("test").unwrap();
        assert!(manager.get_api_key("test").is_none());
    }

    #[test]
    fn test_get_api_key_info() {
        let manager = SecurityManager::new();

        manager.save_api_key("test", "sk-1234567890").unwrap();

        let info = manager.get_api_key_info("test").unwrap();
        assert_eq!(info.masked_key, "sk-1****7890");
    }
}
