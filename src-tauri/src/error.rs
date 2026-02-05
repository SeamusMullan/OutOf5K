//! Error types for OutOf5K

use thiserror::Error;

/// Application-wide error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Python service error: {0}")]
    PythonService(String),

    #[error("Demo file not found: {0}")]
    DemoNotFound(String),

    #[error("Demo parsing failed: {0}")]
    ParseError(String),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for AppError
pub type AppResult<T> = Result<T, AppError>;

/// Implement conversion to Tauri's invoke error
impl From<AppError> for tauri::ipc::InvokeError {
    fn from(error: AppError) -> Self {
        tauri::ipc::InvokeError::from(error.to_string())
    }
}

/// Command-specific error that can be returned from Tauri commands
#[derive(Debug, serde::Serialize)]
pub struct CommandError {
    pub code: String,
    pub message: String,
}

impl From<AppError> for CommandError {
    fn from(error: AppError) -> Self {
        let code = match &error {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Io(_) => "IO_ERROR",
            AppError::Json(_) => "JSON_ERROR",
            AppError::PythonService(_) => "PYTHON_SERVICE_ERROR",
            AppError::DemoNotFound(_) => "DEMO_NOT_FOUND",
            AppError::ParseError(_) => "PARSE_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Internal(_) => "INTERNAL_ERROR",
        };

        CommandError {
            code: code.to_string(),
            message: error.to_string(),
        }
    }
}
