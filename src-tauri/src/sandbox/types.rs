//! OpenClaw 类型定义

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawConfig {
    pub gateway_url: String,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub thinking_level: String,
}

impl Default for OpenClawConfig {
    fn default() -> Self {
        Self {
            gateway_url: "ws://127.0.0.1:18789".to_string(),
            api_key: None,
            model: Some("anthropic/claude-opus-4-6".to_string()),
            thinking_level: "medium".to_string(),
        }
    }
}

impl OpenClawConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.gateway_url.is_empty() {
            return Err("Gateway URL 不能为空".to_string());
        }
        if !self.gateway_url.starts_with("ws://") && !self.gateway_url.starts_with("wss://") {
            return Err("Gateway URL 必须以 ws:// 或 wss:// 开头".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AgentRequest {
    pub message: String,
    pub session_id: Option<String>,
    pub thinking_level: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub connected: bool,
    pub status: ConnectionStatus,
    pub version: Option<String>,
    pub error: Option<String>,
}

impl Default for GatewayStatus {
    fn default() -> Self {
        Self {
            connected: false,
            status: ConnectionStatus::Disconnected,
            version: None,
            error: None,
        }
    }
}
