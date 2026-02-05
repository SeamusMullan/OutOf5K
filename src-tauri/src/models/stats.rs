//! Statistics models

use serde::{Deserialize, Serialize};

/// Aggregated player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub player_id: String,
    pub player_name: String,
    pub matches: i32,
    pub total_kills: i32,
    pub total_deaths: i32,
    pub total_assists: i32,
    pub avg_adr: f64,
    pub avg_rating: f64,
    pub avg_kast: f64,
    pub hs_percentage: f64,
}

/// Player statistics grouped by map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatsByMap {
    pub map_name: String,
    pub matches: i32,
    pub kills: i32,
    pub deaths: i32,
    pub avg_rating: f64,
    pub avg_adr: f64,
}

/// Performance over time (weekly buckets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    pub week: String,
    pub matches: i32,
    pub avg_rating: f64,
    pub avg_adr: f64,
    pub avg_kast: f64,
}

/// Weapon statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponStats {
    pub weapon: String,
    pub kills: i32,
    pub headshots: i32,
    pub hs_percentage: f64,
}

/// User settings stored in the database
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub demo_directory: Option<String>,
    pub steam_id: Option<String>,
    pub theme: String,
    pub auto_parse: bool,
    pub parse_positions: bool,
    pub position_interval: i32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            demo_directory: None,
            steam_id: None,
            theme: "dark".to_string(),
            auto_parse: true,
            parse_positions: true,
            position_interval: 16,
        }
    }
}
