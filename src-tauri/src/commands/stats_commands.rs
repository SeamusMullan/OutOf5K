//! Statistics-related Tauri commands

use crate::error::CommandError;
use crate::models::{PlayerStats, PlayerStatsByMap};
use crate::state::AppState;
use tauri::State;

/// Get aggregated stats for a player
#[tauri::command]
pub async fn get_player_stats(
    state: State<'_, AppState>,
    player_id: String,
) -> Result<PlayerStats, CommandError> {
    log::debug!("Getting stats for player: {}", player_id);
    state.stats_service.get_player_stats(&player_id).await.map_err(Into::into)
}

/// Get player stats grouped by map
#[tauri::command]
pub async fn get_player_stats_by_map(
    state: State<'_, AppState>,
    player_id: String,
) -> Result<Vec<PlayerStatsByMap>, CommandError> {
    log::debug!("Getting stats by map for player: {}", player_id);
    state.stats_service.get_player_stats_by_map(&player_id).await.map_err(Into::into)
}
