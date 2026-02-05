//! Settings-related Tauri commands

use crate::error::CommandError;
use crate::models::Settings;
use crate::state::AppState;
use tauri::State;

/// Get current settings
#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<Settings, CommandError> {
    log::debug!("Getting settings");
    state.settings_service.get_settings().await.map_err(Into::into)
}

/// Update settings
#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), CommandError> {
    log::info!("Updating settings");
    state.settings_service.update_settings(&settings).await.map_err(Into::into)
}
