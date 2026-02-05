//! Statistics service

use crate::db::Database;
use crate::error::{AppError, AppResult};
use crate::models::{PlayerStats, PlayerStatsByMap};
use std::sync::Arc;

/// Service for calculating and retrieving statistics
pub struct StatsService {
    db: Arc<Database>,
}

impl StatsService {
    /// Create a new StatsService
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Get aggregated stats for a player
    pub async fn get_player_stats(&self, player_id: &str) -> AppResult<PlayerStats> {
        self.db.get_player_stats(player_id).await?.ok_or_else(|| {
            AppError::NotFound(format!("No stats found for player: {}", player_id))
        })
    }

    /// Get player stats grouped by map
    pub async fn get_player_stats_by_map(&self, player_id: &str) -> AppResult<Vec<PlayerStatsByMap>> {
        self.db.get_player_stats_by_map(player_id).await
    }
}
