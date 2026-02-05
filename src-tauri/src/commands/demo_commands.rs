//! Demo-related Tauri commands

use crate::error::CommandError;
use crate::models::{Demo, DemoSummary, ImportResult};
use crate::state::AppState;
use tauri::State;

/// Import demo files for parsing
#[tauri::command]
pub async fn import_demos(
    state: State<'_, AppState>,
    paths: Vec<String>,
) -> Result<Vec<ImportResult>, CommandError> {
    log::info!("Importing {} demo files", paths.len());
    state.demo_service.import_demos(&paths).await.map_err(Into::into)
}

/// List all demos with pagination
#[tauri::command]
pub async fn list_demos(
    state: State<'_, AppState>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<DemoSummary>, CommandError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    log::debug!("Listing demos: limit={}, offset={}", limit, offset);
    state.demo_service.list_demos(limit, offset).await.map_err(Into::into)
}

/// Get a single demo with full details
#[tauri::command]
pub async fn get_demo(
    state: State<'_, AppState>,
    id: String,
) -> Result<Demo, CommandError> {
    log::debug!("Getting demo: {}", id);
    state.demo_service.get_demo(&id).await.map_err(Into::into)
}

/// Delete a demo
#[tauri::command]
pub async fn delete_demo(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    log::info!("Deleting demo: {}", id);
    state.demo_service.delete_demo(&id).await.map_err(Into::into)
}
