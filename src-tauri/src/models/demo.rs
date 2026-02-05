//! Demo-related models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Status of a demo file in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DemoStatus {
    Pending,
    Parsing,
    Parsed,
    Error,
}

impl Default for DemoStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for DemoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DemoStatus::Pending => write!(f, "pending"),
            DemoStatus::Parsing => write!(f, "parsing"),
            DemoStatus::Parsed => write!(f, "parsed"),
            DemoStatus::Error => write!(f, "error"),
        }
    }
}

/// A CS2 demo file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demo {
    pub id: String,
    pub file_path: String,
    pub file_hash: String,
    pub map_name: String,
    pub played_at: Option<DateTime<Utc>>,
    pub duration_ticks: i64,
    pub tickrate: i32,
    pub status: DemoStatus,
    pub parsed_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary view of a demo for list displays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoSummary {
    pub id: String,
    pub map_name: String,
    pub played_at: Option<DateTime<Utc>>,
    pub status: DemoStatus,
    pub player_count: i32,
    pub total_rounds: i32,
}

/// Result of importing a demo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub id: String,
    pub file_path: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Round information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    pub id: i64,
    pub demo_id: String,
    pub round_number: i32,
    pub winner: String,
    pub win_reason: String,
    pub start_tick: i64,
    pub end_tick: i64,
    pub ct_score: i32,
    pub t_score: i32,
}
