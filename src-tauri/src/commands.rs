//! Tauri 命令模块

use crate::db::{
    CreateConversationRequest, CreateMessageRequest, CreateSkillRequest, Database,
    UpdateConversationRequest, UpdateSkillRequest,
};
use crate::sandbox::{GatewayStatus, OpenClawConfig, SandboxInfo, SandboxManager};
use tauri::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

// ============ Mindmap Commands ============
#[tauri::command]
pub fn get_mindmaps(db: State<'_, Database>) -> Result<CommandResult<Vec<crate::db::Mindmap>>, String> {
    match db.get_mindmaps() {
        Ok(mindmaps) => Ok(CommandResult::ok(mindmaps)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn create_mindmap(title: String, db: State<'_, Database>) -> Result<CommandResult<i64>, String> {
    match db.create_mindmap(&title) {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn get_mindmap(id: i64, db: State<'_, Database>) -> Result<CommandResult<crate::db::Mindmap>, String> {
    match db.get_mindmap(id) {
        Ok(mindmap) => Ok(CommandResult::ok(mindmap)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn update_mindmap(
    id: i64,
    title: String,
    content: String,
    db: State<'_, Database>,
) -> Result<CommandResult<()>, String> {
    match db.update_mindmap(id, &title, &content) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn delete_mindmap(id: i64, db: State<'_, Database>) -> Result<CommandResult<()>, String> {
    match db.delete_mindmap(id) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

// ============ Skill Commands ============
#[tauri::command]
pub fn get_skills(
    category: Option<String>,
    db: State<'_, Database>,
) -> Result<CommandResult<Vec<crate::db::Skill>>, String> {
    match db.get_skills(category.as_deref()) {
        Ok(skills) => Ok(CommandResult::ok(skills)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn get_skill(id: i64, db: State<'_, Database>) -> Result<CommandResult<crate::db::Skill>, String> {
    match db.get_skill(id) {
        Ok(skill) => Ok(CommandResult::ok(skill)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn create_skill(
    name: String,
    description: Option<String>,
    content: String,
    category: Option<String>,
    db: State<'_, Database>,
) -> Result<CommandResult<i64>, String> {
    let req = CreateSkillRequest {
        name,
        description,
        content,
        category,
    };
    match db.create_skill(&req) {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn update_skill(
    id: i64,
    name: String,
    description: Option<String>,
    content: String,
    category: Option<String>,
    db: State<'_, Database>,
) -> Result<CommandResult<()>, String> {
    let req = UpdateSkillRequest {
        id,
        name,
        description,
        content,
        category,
    };
    match db.update_skill(&req) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn delete_skill(id: i64, db: State<'_, Database>) -> Result<CommandResult<()>, String> {
    match db.delete_skill(id) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

// ============ Conversation Commands ============
#[tauri::command]
pub fn get_conversations(
    mindmap_id: Option<i64>,
    db: State<'_, Database>,
) -> Result<CommandResult<Vec<crate::db::Conversation>>, String> {
    match db.get_conversations(mindmap_id) {
        Ok(conversations) => Ok(CommandResult::ok(conversations)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn get_conversation(id: i64, db: State<'_, Database>) -> Result<CommandResult<crate::db::Conversation>, String> {
    match db.get_conversation(id) {
        Ok(conversation) => Ok(CommandResult::ok(conversation)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn create_conversation(
    mindmap_id: Option<i64>,
    title: String,
    db: State<'_, Database>,
) -> Result<CommandResult<i64>, String> {
    let req = CreateConversationRequest { mindmap_id, title };
    match db.create_conversation(&req) {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn update_conversation(
    id: i64,
    title: String,
    db: State<'_, Database>,
) -> Result<CommandResult<()>, String> {
    let req = UpdateConversationRequest { id, title };
    match db.update_conversation(&req) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn delete_conversation(id: i64, db: State<'_, Database>) -> Result<CommandResult<()>, String> {
    match db.delete_conversation(id) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

// ============ Message Commands ============
#[tauri::command]
pub fn get_messages(
    conversation_id: i64,
    db: State<'_, Database>,
) -> Result<CommandResult<Vec<crate::db::Message>>, String> {
    match db.get_messages(conversation_id) {
        Ok(messages) => Ok(CommandResult::ok(messages)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn create_message(
    conversation_id: i64,
    role: String,
    content: String,
    db: State<'_, Database>,
) -> Result<CommandResult<i64>, String> {
    let req = CreateMessageRequest {
        conversation_id,
        role,
        content,
    };
    match db.create_message(&req) {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn update_message_content(
    id: i64,
    content: String,
    db: State<'_, Database>,
) -> Result<CommandResult<()>, String> {
    match db.update_message_content(id, &content) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn delete_message(id: i64, db: State<'_, Database>) -> Result<CommandResult<()>, String> {
    match db.delete_message(id) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

// ============ Sandbox Commands ============
#[tauri::command]
pub fn initialize_sandbox(
    download_url: Option<String>,
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    let url = download_url.unwrap_or_else(|| "https://api.github.com/repos/openclaw/openclaw/releases/latest".to_string());
    match manager.initialize(&url) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn start_openclaw(
    port: Option<u16>,
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    let port = port.unwrap_or(8080);
    match manager.start(port) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn stop_openclaw(
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    match manager.stop() {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub fn get_sandbox_status(
    manager: State<'_, SandboxManager>,
) -> Result<SandboxInfo, String> {
    Ok(manager.get_info())
}

#[tauri::command]
pub fn check_for_updates(
    _manager: State<'_, SandboxManager>,
) -> Result<CommandResult<String>, String> {
    Ok(CommandResult::ok("1.0.0".to_string()))
}

// ============ Export Commands ============
#[tauri::command]
pub fn export_markdown(
    conversation_id: i64,
    title: String,
    file_path: String,
    include_metadata: Option<bool>,
    db: State<'_, Database>,
) -> Result<CommandResult<String>, String> {
    let messages = db.get_messages(conversation_id).map_err(|e| e.to_string())?;

    let message_tuples: Vec<(String, String, String)> = messages
        .into_iter()
        .map(|m| (m.role, m.content, m.created_at))
        .collect();

    let include_meta = include_metadata.unwrap_or(true);

    match crate::export::export_to_markdown(
        &std::path::PathBuf::from(&file_path),
        &title,
        &message_tuples,
        include_meta,
    ) {
        Ok(path) => Ok(CommandResult::ok(path)),
        Err(e) => Ok(CommandResult::err(e)),
    }
}

#[tauri::command]
pub fn export_word(
    conversation_id: i64,
    title: String,
    file_path: String,
    db: State<'_, Database>,
) -> Result<CommandResult<String>, String> {
    let messages = db.get_messages(conversation_id).map_err(|e| e.to_string())?;

    let message_tuples: Vec<(String, String, String)> = messages
        .into_iter()
        .map(|m| (m.role, m.content, m.created_at))
        .collect();

    match crate::export::export_to_word(
        &std::path::PathBuf::from(&file_path),
        &title,
        &message_tuples,
    ) {
        Ok(path) => Ok(CommandResult::ok(path)),
        Err(e) => Ok(CommandResult::err(e)),
    }
}

// ============ Security Commands ============
#[tauri::command]
pub fn save_api_key(
    key_name: String,
    api_key: String,
    manager: State<'_, crate::security::SecurityManager>,
) -> Result<CommandResult<()>, String> {
    match manager.save_api_key(&key_name, &api_key) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e)),
    }
}

#[tauri::command]
pub fn get_api_key_masked(
    key_name: String,
    manager: State<'_, crate::security::SecurityManager>,
) -> Result<CommandResult<crate::security::ApiKeyInfo>, String> {
    match manager.get_api_key_info(&key_name) {
        Some(info) => Ok(CommandResult::ok(info)),
        None => Ok(CommandResult::err(format!("API Key '{}' not found", key_name))),
    }
}

#[tauri::command]
pub fn delete_api_key(
    key_name: String,
    manager: State<'_, crate::security::SecurityManager>,
) -> Result<CommandResult<()>, String> {
    match manager.delete_api_key(&key_name) {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e)),
    }
}

// ============ OpenClaw Gateway Commands ============
#[tauri::command]
pub async fn openclaw_check_status(
    manager: State<'_, SandboxManager>,
) -> Result<GatewayStatus, String> {
    Ok(manager.get_gateway_status().await)
}

#[tauri::command]
pub async fn openclaw_connect(
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    match manager.connect().await {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn openclaw_disconnect(
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    match manager.disconnect().await {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn openclaw_retry_connect(
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    match manager.retry_connect().await {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn openclaw_configure(
    config: OpenClawConfig,
    manager: State<'_, SandboxManager>,
) -> Result<CommandResult<()>, String> {
    match manager.update_config(config).await {
        Ok(()) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn openclaw_get_config(
    manager: State<'_, SandboxManager>,
) -> Result<OpenClawConfig, String> {
    Ok(manager.get_config().await)
}
