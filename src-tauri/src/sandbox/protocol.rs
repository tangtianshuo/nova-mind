//! OpenClaw SSE 协议解析器

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SSEEvent {
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: u32,
        #[serde(rename = "content_block")]
        content_block: ContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: u32,
        delta: ContentDelta,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop {
        index: u32,
    },
    #[serde(rename = "message_start")]
    MessageStart {
        message: MessageStart,
    },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: MessageDelta,
        #[serde(default)]
        usage: Option<Usage>,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "error")]
    Error {
        code: String,
        message: String,
    },
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDelta {
    #[serde(rename = "type")]
    pub delta_type: String,
    pub text: Option<String>,
    pub thinking: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStart {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: String,
    pub content: Option<Vec<ContentBlock>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    #[serde(rename = "type")]
    pub delta_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "input_tokens")]
    pub input_tokens: Option<u32>,
    #[serde(rename = "output_tokens")]
    pub output_tokens: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParsedSSEEvent {
    pub event: Option<SSEEvent>,
    pub raw_data: String,
}

pub fn parse_sse_line(line: &str) -> Option<ParsedSSEEvent> {
    if line.is_empty() || line == "\r" || line == "\n" || line == "\r\n" {
        return None;
    }

    if line.starts_with("event:") {
        return None;
    }

    if line.starts_with("data:") {
        let data = line.trim_start_matches("data:").trim();
        if data.is_empty() || data == "[DONE]" {
            return Some(ParsedSSEEvent {
                event: None,
                raw_data: String::new(),
            });
        }

        let event = match serde_json::from_str::<SSEEvent>(data) {
            Ok(e) => Some(e),
            Err(e) => {
                log::warn!("SSE 事件解析失败: {}", e);
                None
            }
        };

        return Some(ParsedSSEEvent {
            event,
            raw_data: data.to_string(),
        });
    }

    None
}

pub fn parse_sse_data(data: &str) -> Vec<ParsedSSEEvent> {
    let mut events = Vec::new();
    
    for line in data.lines() {
        if let Some(event) = parse_sse_line(line) {
            events.push(event);
        }
    }
    
    events
}

#[allow(dead_code)]
pub fn extract_text_from_events(events: &[ParsedSSEEvent]) -> String {
    let mut text = String::new();
    
    for event in events {
        if let Some(SSEEvent::ContentBlockDelta { delta, .. }) = &event.event {
            if let Some(delta_text) = &delta.text {
                text.push_str(delta_text);
            }
        }
    }
    
    text
}

#[allow(dead_code)]
pub fn extract_thinking_from_events(events: &[ParsedSSEEvent]) -> String {
    let mut thinking = String::new();
    
    for event in events {
        if let Some(SSEEvent::ContentBlockDelta { delta, .. }) = &event.event {
            if let Some(thinking_text) = &delta.thinking {
                thinking.push_str(thinking_text);
            }
        }
    }
    
    thinking
}

#[allow(dead_code)]
pub fn get_message_id_from_events(events: &[ParsedSSEEvent]) -> Option<String> {
    for event in events {
        if let Some(SSEEvent::MessageStart { message, .. }) = &event.event {
            return Some(message.id.clone());
        }
    }
    None
}
