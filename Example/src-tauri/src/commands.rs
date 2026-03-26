//! Tauri 命令接口

use tauri::State;
use crate::sandbox::{SandboxManager, LaunchConfig, OpenClawConfig};

#[tauri::command]
pub async fn initialize_sandbox(
    state: State<'_, SandboxManager>,
) -> Result<serde_json::Value, String> {
    let status = state.initialize().await.map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(status).unwrap())
}

#[tauri::command]
pub async fn start_openclaw(
    state: State<'_, SandboxManager>,
    port: Option<u16>,
    config: Option<OpenClawConfig>,
) -> Result<serde_json::Value, String> {
    let launch_config = LaunchConfig {
        port: port.unwrap_or(18789),
        openclaw: config.unwrap_or_default(),
    };

    let status = state.start(launch_config).await.map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(status).unwrap())
}

#[tauri::command]
pub async fn stop_openclaw(state: State<'_, SandboxManager>) -> Result<(), String> {
    state.stop().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sandbox_status(state: State<'_, SandboxManager>) -> Result<serde_json::Value, String> {
    let status = state.status().await;
    Ok(serde_json::to_value(status).unwrap())
}

#[tauri::command]
pub async fn check_for_updates(state: State<'_, SandboxManager>) -> Result<Option<String>, String> {
    state.check_update().await.map_err(|e| e.to_string())
}
