//! Nova Mind - Tauri 主入口

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(test)]
mod tests;

mod db;
mod sandbox;
mod commands;
mod health;
mod export;
mod security;

use tauri::Manager;
use crate::db::Database;
use crate::sandbox::SandboxManager;
use crate::security::SecurityManager;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Nova Mind 启动中...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("获取应用数据目录失败");
            std::fs::create_dir_all(&app_data_dir).ok();

            let db = Database::new(&app_data_dir.join("nova-mind.db"))
                .expect("数据库初始化失败");
            app.manage(db);
            log::info!("数据库初始化完成");

            let sandbox_manager = SandboxManager::new(app.handle().clone());
            app.manage(sandbox_manager);
            log::info!("沙箱管理器初始化完成");

            let security_manager = SecurityManager::new();
            app.manage(security_manager);
            log::info!("安全管理器初始化完成");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            health::health_check,
            commands::get_mindmaps,
            commands::create_mindmap,
            commands::get_mindmap,
            commands::update_mindmap,
            commands::delete_mindmap,
            commands::get_skills,
            commands::get_skill,
            commands::create_skill,
            commands::update_skill,
            commands::delete_skill,
            commands::get_conversations,
            commands::get_conversation,
            commands::create_conversation,
            commands::update_conversation,
            commands::delete_conversation,
            commands::get_messages,
            commands::create_message,
            commands::update_message_content,
            commands::delete_message,
            commands::initialize_sandbox,
            commands::start_openclaw,
            commands::stop_openclaw,
            commands::get_sandbox_status,
            commands::check_for_updates,
            commands::export_markdown,
            commands::export_word,
            commands::save_api_key,
            commands::get_api_key_masked,
            commands::delete_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("Nova Mind 启动失败");
}
