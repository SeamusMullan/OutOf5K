//! Player-related models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A CS2 player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub steam_id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub is_local_user: bool,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Player's participation in a demo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoPlayer {
    pub demo_id: String,
    pub player_id: String,
    pub team: String,
    pub start_team: String,
}

/// Player with their match stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerWithStats {
    pub player: Player,
    pub team: String,
    pub stats: PlayerMatchStats,
}

/// Per-match player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchStats {
    pub demo_id: String,
    pub player_id: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub total_damage: i32,
    pub headshots: i32,
    pub adr: Option<f64>,
    pub kast: Option<f64>,
    pub rating: Option<f64>,
    pub first_kills: i32,
    pub first_deaths: i32,
    pub clutches_won: i32,
    pub clutches_lost: i32,
}

/// Per-round player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRoundStats {
    pub demo_id: String,
    pub round_id: i64,
    pub player_id: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub damage: i32,
    pub headshots: i32,
    pub flash_assists: i32,
}
