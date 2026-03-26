//! 进程管理模块

use std::path::Path;
use tauri::AppHandle;
use tokio::process::{Child, Command};
use serde::Serialize;

pub struct ProcessHandle {
    child: Child,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
}

pub async fn spawn(
    bin_path: &Path,
    port: u16,
    config_path: &Path,
    app: AppHandle,
) -> Result<ProcessHandle, super::SandboxError> {
    // 实现代码省略
    unimplemented!()
}
