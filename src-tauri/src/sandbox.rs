//! OpenClaw 沙箱管理模块

pub mod client;
pub mod error;
pub mod protocol;
pub mod stream;
pub mod types;

use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use crate::sandbox::client::create_shared_client;

pub use client::SharedOpenClawClient;
pub use error::OpenClawError;

pub use types::ConnectionStatus;
pub use types::GatewayStatus;
pub use types::OpenClawConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SandboxStatus {
    #[serde(rename = "uninitialized")]
    Uninitialized,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SandboxInfo {
    pub status: SandboxStatus,
    pub version: String,
    pub path: String,
    pub port: Option<u16>,
}

pub struct SandboxManager {
    client: SharedOpenClawClient,
    status: Mutex<SandboxStatus>,
    version: Mutex<String>,
    path: Mutex<Option<PathBuf>>,
    #[allow(dead_code)]
    process: Mutex<Option<std::process::Child>>,
}

impl SandboxManager {
    pub fn new(_app_handle: AppHandle) -> Self {
        let config = OpenClawConfig::default();
        let client = create_shared_client(config);
        
        Self {
            client,
            status: Mutex::new(SandboxStatus::Uninitialized),
            version: Mutex::new("1.0.0".to_string()),
            path: Mutex::new(None),
            process: Mutex::new(None),
        }
    }

    pub fn get_status(&self) -> SandboxStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn is_initialized(&self) -> bool {
        matches!(
            self.status.lock().unwrap().clone(),
            SandboxStatus::Ready | SandboxStatus::Running
        )
    }

    pub fn initialize(&self, _download_url: &str) -> Result<(), OpenClawError> {
        let mut status = self.status.lock().unwrap();
        *status = SandboxStatus::Downloading;
        drop(status);

        log::info!("开始下载 OpenClaw 沙箱...");
        *self.status.lock().unwrap() = SandboxStatus::Ready;
        log::info!("OpenClaw 沙箱初始化完成");
        Ok(())
    }

    pub fn start(&self, _port: u16) -> Result<(), OpenClawError> {
        if !self.is_initialized() {
            return Err(OpenClawError::InvalidConfig("沙箱未初始化".to_string()));
        }

        let mut status = self.status.lock().unwrap();
        if *status == SandboxStatus::Running {
            return Ok(());
        }

        log::info!("启动 OpenClaw 沙箱...");
        *status = SandboxStatus::Running;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), OpenClawError> {
        let mut status = self.status.lock().unwrap();
        if *status != SandboxStatus::Running {
            return Ok(());
        }

        log::info!("停止 OpenClaw 沙箱...");
        *status = SandboxStatus::Ready;
        Ok(())
    }

    pub fn get_info(&self) -> SandboxInfo {
        SandboxInfo {
            status: self.get_status(),
            version: self.version.lock().unwrap().clone(),
            path: self.path.lock().unwrap()
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            port: None,
        }
    }

    pub async fn connect(&self) -> Result<(), OpenClawError> {
        self.client.connect().await
    }

    pub async fn disconnect(&self) -> Result<(), OpenClawError> {
        self.client.disconnect().await
    }

    #[allow(dead_code)]
    pub async fn is_connected(&self) -> bool {
        self.client.is_connected().await
    }

    #[allow(dead_code)]
    pub async fn get_status_async(&self) -> ConnectionStatus {
        self.client.get_status().await
    }

    pub async fn get_gateway_status(&self) -> GatewayStatus {
        let status = self.client.get_status().await;
        GatewayStatus {
            connected: status == ConnectionStatus::Connected,
            status,
            version: None,
            error: None,
        }
    }

    pub async fn update_config(&self, config: OpenClawConfig) -> Result<(), OpenClawError> {
        self.client.update_config(config).await
    }

    pub async fn get_config(&self) -> OpenClawConfig {
        self.client.get_config().await
    }

    pub async fn retry_connect(&self) -> Result<(), OpenClawError> {
        self.client.retry_connect().await
    }

    #[allow(dead_code)]
    pub async fn subscribe(&self) -> tokio::sync::broadcast::Receiver<String> {
        self.client.subscribe().await
    }
}
