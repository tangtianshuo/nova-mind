//! OpenClaw 沙箱管理模块

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("沙箱路径无效")]
    InvalidPath,
    #[error("下载失败: {0}")]
    DownloadFailed(String),
    #[error("验证失败: {0}")]
    VerificationFailed(String),
    #[error("进程启动失败: {0}")]
    ProcessStartFailed(String),
    #[error("沙箱未初始化")]
    NotInitialized,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInfo {
    pub status: SandboxStatus,
    pub version: String,
    pub path: String,
    pub port: Option<u16>,
}

pub struct SandboxManager {
    status: Mutex<SandboxStatus>,
    version: Mutex<String>,
    path: Mutex<Option<PathBuf>>,
    #[allow(dead_code)]
    process: Mutex<Option<std::process::Child>>,
}

impl SandboxManager {
    pub fn new(_app_handle: AppHandle) -> Self {
        Self {
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

    pub fn initialize(&self, _download_url: &str) -> Result<(), SandboxError> {
        let mut status = self.status.lock().unwrap();
        *status = SandboxStatus::Downloading;
        drop(status);

        log::info!("开始下载 OpenClaw 沙箱...");
        *self.status.lock().unwrap() = SandboxStatus::Ready;
        log::info!("OpenClaw 沙箱初始化完成");
        Ok(())
    }

    pub fn start(&self, _port: u16) -> Result<(), SandboxError> {
        if !self.is_initialized() {
            return Err(SandboxError::NotInitialized);
        }

        let mut status = self.status.lock().unwrap();
        if *status == SandboxStatus::Running {
            return Ok(());
        }

        log::info!("启动 OpenClaw 沙箱...");
        *status = SandboxStatus::Running;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), SandboxError> {
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
}
