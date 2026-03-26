//! OpenClaw Desktop - Tauri 主入口

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sandbox;
mod commands;

use tauri::Manager;
use sandbox::SandboxManager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let manager = SandboxManager::new(app.handle().clone());
            app.manage(manager);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::initialize_sandbox,
            commands::start_openclaw,
            commands::stop_openclaw,
            commands::get_sandbox_status,
            commands::check_for_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
