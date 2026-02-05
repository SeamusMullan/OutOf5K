//! Settings service

use crate::db::Database;
use crate::error::AppResult;
use crate::models::Settings;
use std::sync::Arc;

/// Service for managing user settings
pub struct SettingsService {
    db: Arc<Database>,
}

impl SettingsService {
    /// Create a new SettingsService
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Get current settings
    pub async fn get_settings(&self) -> AppResult<Settings> {
        let settings = self.db.get_settings().await?;
        Ok(settings.unwrap_or_default())
    }

    /// Update settings
    pub async fn update_settings(&self, settings: &Settings) -> AppResult<()> {
        self.db.update_settings(settings).await
    }
}
