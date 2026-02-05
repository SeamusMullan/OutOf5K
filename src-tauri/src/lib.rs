//! OutOf5K - CS2 Gameplay Analysis Tool
//!
//! This is the main library for the Tauri backend, handling:
//! - IPC commands from the frontend
//! - SQLite database operations
//! - Python subprocess management for demo parsing
//! - External API integrations (Steam, HLTV, Faceit)

pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod services;
pub mod state;

use state::AppState;
use tauri::Manager;

/// Initialize and run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Initialize logging in debug mode
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .build(),
                )?;
            }

            // Initialize application state
            let app_state = AppState::new(app.handle())?;
            app.manage(app_state);

            log::info!("OutOf5K initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::demo_commands::import_demos,
            commands::demo_commands::list_demos,
            commands::demo_commands::get_demo,
            commands::demo_commands::delete_demo,
            commands::stats_commands::get_player_stats,
            commands::stats_commands::get_player_stats_by_map,
            commands::settings_commands::get_settings,
            commands::settings_commands::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
