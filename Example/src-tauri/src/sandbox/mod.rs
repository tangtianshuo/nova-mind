//! OpenClaw 沙箱自动部署管理器

use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;

pub mod download;
pub mod verify;
pub mod process;
pub mod update;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<PathBuf>,
    pub running: bool,
    pub pid: Option<u32>,
    pub port: u16,
}

pub struct SandboxManager {
    app: AppHandle,
    status: Arc<RwLock<SandboxStatus>>,
    process_handle: Arc<RwLock<Option<process::ProcessHandle>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LaunchConfig {
    pub port: u16,
    pub openclaw: OpenClawConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenClawConfig {
    pub channels: Option<serde_json::Value>,
    pub messages: Option<serde_json::Value>,
    pub plugins: Option<Vec<String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("OpenClaw 未安装")]
    NotInstalled,
    #[error("下载失败: {0}")]
    DownloadFailed(String),
    #[error("校验失败: {0}")]
    VerificationFailed(String),
    #[error("启动超时")]
    StartupTimeout,
    #[error("进程错误: {0}")]
    ProcessError(String),
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP 错误: {0}")]
    Http(#[from] reqwest::Error),
    #[error("序列化错误: {0}")]
    Serde(#[from] serde_json::Error),
}

// 实现代码省略，详见完整文档
