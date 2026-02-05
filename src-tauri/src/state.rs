//! Application state management

use crate::db::Database;
use crate::error::{AppError, AppResult};
use crate::services::{DemoService, PythonBridge, SettingsService, StatsService};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::RwLock;

/// Global application state managed by Tauri
pub struct AppState {
    /// Database connection pool
    pub db: Arc<Database>,
    /// Demo management service
    pub demo_service: Arc<DemoService>,
    /// Statistics service
    pub stats_service: Arc<StatsService>,
    /// Settings service
    pub settings_service: Arc<SettingsService>,
    /// Python subprocess bridge
    pub python_bridge: Arc<RwLock<Option<PythonBridge>>>,
}

impl AppState {
    /// Create a new AppState with initialized services
    pub fn new(app_handle: &AppHandle) -> AppResult<Self> {
        // Get the app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::Config(format!("Failed to get app data dir: {}", e)))?;

        // Ensure directory exists
        std::fs::create_dir_all(&app_data_dir)?;

        // Initialize database
        let db_path = app_data_dir.join("outof5k.db");
        let db = Arc::new(Database::new(&db_path)?);

        // Initialize services
        let demo_service = Arc::new(DemoService::new(db.clone()));
        let stats_service = Arc::new(StatsService::new(db.clone()));
        let settings_service = Arc::new(SettingsService::new(db.clone()));

        Ok(Self {
            db,
            demo_service,
            stats_service,
            settings_service,
            python_bridge: Arc::new(RwLock::new(None)),
        })
    }

    /// Get or create the Python bridge
    pub async fn get_python_bridge(&self) -> AppResult<tokio::sync::RwLockReadGuard<'_, Option<PythonBridge>>> {
        let bridge = self.python_bridge.read().await;
        if bridge.is_none() {
            drop(bridge);
            let mut bridge_write = self.python_bridge.write().await;
            if bridge_write.is_none() {
                *bridge_write = Some(PythonBridge::new()?);
            }
            drop(bridge_write);
            return Ok(self.python_bridge.read().await);
        }
        Ok(bridge)
    }
}
