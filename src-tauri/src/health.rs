//! 健康检查模块

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub components: Components,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub frontend: bool,
    pub backend: bool,
    pub sandbox: bool,
    pub database: bool,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            components: Components {
                frontend: true,
                backend: true,
                sandbox: false,
                database: false,
            },
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[tauri::command]
pub fn health_check() -> Result<HealthStatus, String> {
    Ok(HealthStatus::default())
}
