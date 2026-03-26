//! OpenClaw 错误类型

use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum OpenClawError {
    #[error("连接失败: {0}")]
    ConnectionFailed(String),
    
    #[error("连接超时")]
    ConnectionTimeout,
    
    #[error("Gateway 未运行，请先启动 OpenClaw Gateway")]
    GatewayNotRunning,
    
    #[error("WebSocket 错误: {0}")]
    WebSocketError(String),
    
    #[error("协议解析错误: {0}")]
    ProtocolError(String),
    
    #[error("消息发送失败: {0}")]
    SendFailed(String),
    
    #[error("消息接收失败: {0}")]
    ReceiveFailed(String),
    
    #[error("流式响应错误: {0}")]
    StreamError(String),
    
    #[error("无效的配置: {0}")]
    InvalidConfig(String),
    
    #[error("认证失败: {0}")]
    AuthFailed(String),
    
    #[error("会话不存在: {0}")]
    SessionNotFound(String),
    
    #[error("API 错误: {0}")]
    ApiError(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for OpenClawError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        match err {
            tokio_tungstenite::tungstenite::Error::Io(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    OpenClawError::ConnectionTimeout
                } else {
                    OpenClawError::WebSocketError(e.to_string())
                }
            }
            tokio_tungstenite::tungstenite::Error::Url(e) => {
                OpenClawError::InvalidConfig(format!("URL 错误: {}", e))
            }
            tokio_tungstenite::tungstenite::Error::Protocol(e) => {
                OpenClawError::ProtocolError(e.to_string())
            }
            _ => OpenClawError::WebSocketError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for OpenClawError {
    fn from(err: serde_json::Error) -> Self {
        OpenClawError::ProtocolError(format!("JSON 解析错误: {}", err))
    }
}

impl From<tokio::time::error::Elapsed> for OpenClawError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        OpenClawError::ConnectionTimeout
    }
}
