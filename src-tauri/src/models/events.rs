//! Game event models

use serde::{Deserialize, Serialize};

/// Types of game events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    Kill,
    Death,
    Assist,
    FlashAssist,
    GrenadeThrow,
    GrenadeDetonate,
    BombPlant,
    BombDefuse,
    BombExplode,
    RoundStart,
    RoundEnd,
    PlayerHurt,
    WeaponFire,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Kill => write!(f, "KILL"),
            EventType::Death => write!(f, "DEATH"),
            EventType::Assist => write!(f, "ASSIST"),
            EventType::FlashAssist => write!(f, "FLASH_ASSIST"),
            EventType::GrenadeThrow => write!(f, "GRENADE_THROW"),
            EventType::GrenadeDetonate => write!(f, "GRENADE_DETONATE"),
            EventType::BombPlant => write!(f, "BOMB_PLANT"),
            EventType::BombDefuse => write!(f, "BOMB_DEFUSE"),
            EventType::BombExplode => write!(f, "BOMB_EXPLODE"),
            EventType::RoundStart => write!(f, "ROUND_START"),
            EventType::RoundEnd => write!(f, "ROUND_END"),
            EventType::PlayerHurt => write!(f, "PLAYER_HURT"),
            EventType::WeaponFire => write!(f, "WEAPON_FIRE"),
        }
    }
}

/// A game event from a demo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: i64,
    pub demo_id: String,
    pub round_id: i64,
    pub tick: i64,
    pub event_type: EventType,
    pub data: serde_json::Value,
}

/// Kill event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillEventData {
    pub attacker_id: String,
    pub victim_id: String,
    pub assister_id: Option<String>,
    pub weapon: String,
    pub headshot: bool,
    pub penetrated: bool,
    pub through_smoke: bool,
    pub no_scope: bool,
    pub attacker_blind: bool,
    pub attacker_position: Position,
    pub victim_position: Position,
}

/// 3D position in game world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Player position snapshot for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPosition {
    pub tick: i64,
    pub player_id: String,
    pub position: Position,
    pub view_x: f64,
    pub view_y: f64,
    pub health: i32,
    pub armor: i32,
    pub is_alive: bool,
}

/// Detected highlight/notable moment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub id: i64,
    pub demo_id: String,
    pub round_id: Option<i64>,
    pub player_id: String,
    pub highlight_type: HighlightType,
    pub start_tick: i64,
    pub end_tick: i64,
    pub description: Option<String>,
}

/// Types of highlights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HighlightType {
    Ace,
    QuadKill,
    TripleKill,
    Clutch1v2,
    Clutch1v3,
    Clutch1v4,
    Clutch1v5,
    NinjaDefuse,
    Wallbang,
    CollateralKill,
}
