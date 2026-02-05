//! Demo management service

use crate::db::Database;
use crate::error::{AppError, AppResult};
use crate::models::{Demo, DemoStatus, DemoSummary, ImportResult};
use std::path::Path;
use std::sync::Arc;

/// Service for managing demo files
pub struct DemoService {
    db: Arc<Database>,
}

impl DemoService {
    /// Create a new DemoService
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Import multiple demo files
    pub async fn import_demos(&self, paths: &[String]) -> AppResult<Vec<ImportResult>> {
        let mut results = Vec::new();

        for path in paths {
            let result = self.import_single_demo(path).await;
            results.push(match result {
                Ok(demo) => ImportResult {
                    id: demo.id,
                    file_path: path.clone(),
                    success: true,
                    error: None,
                },
                Err(e) => ImportResult {
                    id: String::new(),
                    file_path: path.clone(),
                    success: false,
                    error: Some(e.to_string()),
                },
            });
        }

        Ok(results)
    }

    /// Import a single demo file
    async fn import_single_demo(&self, path: &str) -> AppResult<Demo> {
        // Validate file exists and has .dem extension
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(AppError::DemoNotFound(path.to_string()));
        }

        if path_obj.extension().map(|e| e != "dem").unwrap_or(true) {
            return Err(AppError::ParseError("File must have .dem extension".to_string()));
        }

        // Calculate file hash for deduplication
        let file_hash = self.calculate_file_hash(path)?;

        // Check if demo already exists
        if let Some(existing) = self.db.get_demo_by_hash(&file_hash).await? {
            return Ok(existing);
        }

        // Create demo record
        let id = uuid::Uuid::new_v4().to_string();
        let demo = Demo {
            id: id.clone(),
            file_path: path.to_string(),
            file_hash,
            map_name: "unknown".to_string(), // Will be updated after parsing
            played_at: None,
            duration_ticks: 0,
            tickrate: 64,
            status: DemoStatus::Pending,
            parsed_at: None,
            metadata: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.db.insert_demo(&demo).await?;

        // TODO: Queue for parsing via Python service
        
        Ok(demo)
    }

    /// Calculate SHA256 hash of file (first 1MB for speed)
    fn calculate_file_hash(&self, path: &str) -> AppResult<String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut buffer = vec![0u8; 1024 * 1024]; // 1MB
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);

        // Simple hash for now - in production use SHA256
        let hash = format!("{:x}", md5::compute(&buffer));
        Ok(hash)
    }

    /// List demos with pagination
    pub async fn list_demos(&self, limit: i32, offset: i32) -> AppResult<Vec<DemoSummary>> {
        self.db.list_demos(limit, offset).await
    }

    /// Get a single demo by ID
    pub async fn get_demo(&self, id: &str) -> AppResult<Demo> {
        self.db.get_demo(id).await?.ok_or_else(|| AppError::NotFound(format!("Demo not found: {}", id)))
    }

    /// Delete a demo
    pub async fn delete_demo(&self, id: &str) -> AppResult<()> {
        self.db.delete_demo(id).await
    }
}
