//! OpenClaw 消息处理器和流式响应

use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{AppHandle, Emitter};
use serde::Serialize;

use crate::sandbox::protocol::{parse_sse_data, SSEEvent};

#[derive(Debug, Clone, Serialize)]
pub struct StreamDelta {
    pub conversation_id: i64,
    pub delta_type: String,
    pub content: String,
    pub message_id: String,
}

pub struct MessageHandler {
    app_handle: AppHandle,
    conversation_id: i64,
    message_id: Arc<RwLock<Option<String>>>,
    accumulated_text: Arc<RwLock<String>>,
    accumulated_thinking: Arc<RwLock<String>>,
}

impl MessageHandler {
    pub fn new(app_handle: AppHandle, conversation_id: i64) -> Self {
        Self {
            app_handle,
            conversation_id,
            message_id: Arc::new(RwLock::new(None)),
            accumulated_text: Arc::new(RwLock::new(String::new())),
            accumulated_thinking: Arc::new(RwLock::new(String::new())),
        }
    }

    pub async fn handle_event(&self, event: SSEEvent) {
        match event {
            SSEEvent::MessageStart { message } => {
                let msg_id = message.id.clone();
                *self.message_id.write().await = Some(msg_id.clone());
                log::info!("开始处理消息: {}", msg_id);
            }
            
            SSEEvent::ContentBlockStart { index, content_block } => {
                log::debug!("内容块开始 (index: {}): {:?}", index, content_block.block_type);
            }
            
            SSEEvent::ContentBlockDelta { index: _, delta } => {
                if let Some(text) = &delta.text {
                    {
                        let mut accumulated = self.accumulated_text.write().await;
                        accumulated.push_str(text);
                    }
                    
                    let msg_id = self.message_id.read().await.clone().unwrap_or_default();
                    let delta_event = StreamDelta {
                        conversation_id: self.conversation_id,
                        delta_type: "text".to_string(),
                        content: text.clone(),
                        message_id: msg_id,
                    };
                    
                    let _ = self.app_handle.emit("openclaw:stream", delta_event);
                }
                
                if let Some(thinking) = &delta.thinking {
                    {
                        let mut accumulated = self.accumulated_thinking.write().await;
                        accumulated.push_str(thinking);
                    }
                    
                    let msg_id = self.message_id.read().await.clone().unwrap_or_default();
                    let delta_event = StreamDelta {
                        conversation_id: self.conversation_id,
                        delta_type: "thinking".to_string(),
                        content: thinking.clone(),
                        message_id: msg_id,
                    };
                    
                    let _ = self.app_handle.emit("openclaw:stream", delta_event);
                }
            }
            
            SSEEvent::ContentBlockStop { index: _ } => {
                log::debug!("内容块结束");
            }
            
            SSEEvent::MessageDelta { delta, usage: _ } => {
                if let Some(text) = &delta.text {
                    log::debug!("消息增量: {}", text);
                }
            }
            
            SSEEvent::MessageStop => {
                let msg_id = self.message_id.read().await.clone().unwrap_or_default();
                let final_event = StreamDelta {
                    conversation_id: self.conversation_id,
                    delta_type: "done".to_string(),
                    content: String::new(),
                    message_id: msg_id,
                };
                
                let _ = self.app_handle.emit("openclaw:stream", final_event);
                
                log::info!("消息处理完成");
                
                self.reset().await;
            }
            
            SSEEvent::Error { code, message } => {
                let msg_id = self.message_id.read().await.clone().unwrap_or_default();
                let error_event = StreamDelta {
                    conversation_id: self.conversation_id,
                    delta_type: "error".to_string(),
                    content: format!("{}: {}", code, message),
                    message_id: msg_id,
                };
                
                let _ = self.app_handle.emit("openclaw:stream", error_event);
                
                log::error!("流式响应错误: {} - {}", code, message);
            }
            
            SSEEvent::Ping => {
                log::debug!("收到 Ping");
            }
        }
    }

    pub async fn handle_raw_data(&self, data: &str) {
        let events = parse_sse_data(data);
        
        for parsed_event in events {
            if let Some(event) = parsed_event.event {
                self.handle_event(event).await;
            }
        }
    }

    #[allow(dead_code)]
    pub async fn reset(&self) {
        *self.message_id.write().await = None;
        *self.accumulated_text.write().await = String::new();
        *self.accumulated_thinking.write().await = String::new();
    }

    #[allow(dead_code)]
    pub async fn get_accumulated_text(&self) -> String {
        self.accumulated_text.read().await.clone()
    }

    #[allow(dead_code)]
    pub async fn get_accumulated_thinking(&self) -> String {
        self.accumulated_thinking.read().await.clone()
    }
}

#[allow(dead_code)]
pub struct StreamManager {
    handlers: RwLock<std::collections::HashMap<i64, Arc<MessageHandler>>>,
    app_handle: AppHandle,
}

#[allow(dead_code)]
impl StreamManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            handlers: RwLock::new(std::collections::HashMap::new()),
            app_handle,
        }
    }

    pub async fn create_handler(&self, conversation_id: i64) -> Arc<MessageHandler> {
        let mut handlers = self.handlers.write().await;
        
        if let Some(existing) = handlers.get(&conversation_id) {
            existing.reset().await;
            return existing.clone();
        }
        
        let handler = Arc::new(MessageHandler::new(
            self.app_handle.clone(),
            conversation_id,
        ));
        
        handlers.insert(conversation_id, handler.clone());
        
        handler
    }

    pub async fn remove_handler(&self, conversation_id: i64) {
        let mut handlers = self.handlers.write().await;
        handlers.remove(&conversation_id);
    }

    pub async fn handle_message(&self, conversation_id: i64, data: &str) {
        let handlers = self.handlers.read().await;
        
        if let Some(handler) = handlers.get(&conversation_id) {
            handler.handle_raw_data(data).await;
        }
    }
}

#[allow(dead_code)]
pub type SharedStreamManager = Arc<StreamManager>;

#[allow(dead_code)]
pub fn create_stream_manager(app_handle: AppHandle) -> SharedStreamManager {
    Arc::new(StreamManager::new(app_handle))
}
