//! OpenClaw WebSocket 客户端

use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::sandbox::error::OpenClawError;
use crate::sandbox::types::{ConnectionStatus, OpenClawConfig};

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRY_ATTEMPTS: u32 = 3;
const BASE_RETRY_DELAY: Duration = Duration::from_secs(2);

pub struct OpenClawClient {
    config: RwLock<OpenClawConfig>,
    status: RwLock<ConnectionStatus>,
    sender: RwLock<Option<broadcast::Sender<String>>>,
    shutdown_tx: RwLock<Option<broadcast::Sender<()>>>,
}

impl OpenClawClient {
    pub fn new(config: OpenClawConfig) -> Self {
        Self {
            config: RwLock::new(config),
            status: RwLock::new(ConnectionStatus::Disconnected),
            sender: RwLock::new(None),
            shutdown_tx: RwLock::new(None),
        }
    }

    pub async fn connect(&self) -> Result<(), OpenClawError> {
        let config = self.config.read().await;
        let url = config.gateway_url.clone();
        drop(config);

        self.set_status(ConnectionStatus::Connecting).await;
        log::info!("正在连接到 OpenClaw Gateway: {}", url);

        match timeout(CONNECTION_TIMEOUT, connect_async(&url)).await {
            Ok(Ok((ws_stream, _))) => {
                log::info!("成功连接到 OpenClaw Gateway");
                self.set_status(ConnectionStatus::Connected).await;
                self.start_message_handler(ws_stream);
                Ok(())
            }
            Ok(Err(e)) => {
                log::error!("连接失败: {}", e);
                self.set_status(ConnectionStatus::Error).await;
                Err(OpenClawError::ConnectionFailed(e.to_string()))
            }
            Err(_) => {
                log::error!("连接超时");
                self.set_status(ConnectionStatus::Error).await;
                Err(OpenClawError::ConnectionTimeout)
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn start_message_handler(&self, ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>) {
        use futures_util::SinkExt;
        let (mut write, mut read) = ws_stream.split();

        let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);
        
        let (sender, _) = broadcast::channel::<String>(100);

        let read_tx = sender.clone();
        let _read_handle = tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        log::debug!("Received message: {}", text);
                        if read_tx.send(text).is_err() {
                            log::warn!("Failed to send message to channel");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        log::info!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        log::error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        let _write_handle = tokio::spawn(async move {
            let _ = write.close().await;
        });

        let shutdown_tx_clone = shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = shutdown_rx.recv().await;
            log::info!("Message handler shutting down");
        });
        
        let _ = shutdown_tx_clone;
    }

    pub async fn disconnect(&self) -> Result<(), OpenClawError> {
        if let Some(shutdown_tx) = self.shutdown_tx.write().await.take() {
            let _ = shutdown_tx.send(());
        }
        *self.sender.write().await = None;
        self.set_status(ConnectionStatus::Disconnected).await;
        log::info!("已断开与 OpenClaw Gateway 的连接");
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        *self.status.read().await == ConnectionStatus::Connected
    }

    pub async fn get_status(&self) -> ConnectionStatus {
        *self.status.read().await
    }

    async fn set_status(&self, status: ConnectionStatus) {
        *self.status.write().await = status;
    }

    pub async fn update_config(&self, config: OpenClawConfig) -> Result<(), OpenClawError> {
        config.validate().map_err(OpenClawError::InvalidConfig)?;
        *self.config.write().await = config;
        Ok(())
    }

    pub async fn get_config(&self) -> OpenClawConfig {
        self.config.read().await.clone()
    }

    #[allow(dead_code)]
    pub async fn send_message(&self, message: &str) -> Result<(), OpenClawError> {
        if !self.is_connected().await {
            return Err(OpenClawError::GatewayNotRunning);
        }

        if let Some(sender) = self.sender.read().await.as_ref() {
            sender.send(message.to_string()).map_err(|_| {
                OpenClawError::SendFailed("Failed to send message".to_string())
            })?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn subscribe(&self) -> broadcast::Receiver<String> {
        if let Some(sender) = self.sender.read().await.as_ref() {
            sender.subscribe()
        } else {
            broadcast::channel::<String>(100).1
        }
    }

    #[allow(dead_code)]
    pub async fn retry_connect(&self) -> Result<(), OpenClawError> {
        let mut attempts = 0;
        let mut delay = BASE_RETRY_DELAY;

        while attempts < MAX_RETRY_ATTEMPTS {
            attempts += 1;
            log::info!("尝试连接 (第 {} / {} 次)", attempts, MAX_RETRY_ATTEMPTS);

            match self.connect().await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    log::warn!("连接失败: {}", e);
                    if attempts < MAX_RETRY_ATTEMPTS {
                        log::info!("{} 秒后重试...", delay.as_secs());
                        tokio::time::sleep(delay).await;
                        delay *= 2;
                    }
                }
            }
        }

        Err(OpenClawError::ConnectionFailed(format!(
            "重试 {} 次后仍然失败",
            MAX_RETRY_ATTEMPTS
        )))
    }
}

pub type SharedOpenClawClient = Arc<OpenClawClient>;

pub fn create_shared_client(config: OpenClawConfig) -> SharedOpenClawClient {
    Arc::new(OpenClawClient::new(config))
}
