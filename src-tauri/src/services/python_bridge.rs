//! Python subprocess bridge for demo parsing

use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

/// Request message sent to Python service
#[derive(Debug, Serialize)]
struct Request {
    id: String,
    cmd: String,
    params: serde_json::Value,
}

/// Response message from Python service
#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub status: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorInfo>,
    pub progress: Option<u8>,
    pub message: Option<String>,
}

/// Error information from Python service
#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Bridge to the Python demo parsing service
pub struct PythonBridge {
    process: Child,
}

impl PythonBridge {
    /// Create a new Python bridge and start the subprocess
    pub fn new() -> AppResult<Self> {
        let process = Command::new("python")
            .args(["-m", "outof5k_parser"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                AppError::PythonService(format!("Failed to spawn Python process: {}", e))
            })?;

        let mut bridge = Self { process };

        // Wait for ready signal
        bridge.wait_for_ready()?;

        Ok(bridge)
    }

    /// Wait for the Python service to signal it's ready
    fn wait_for_ready(&mut self) -> AppResult<()> {
        let stdout = self
            .process
            .stdout
            .as_mut()
            .ok_or_else(|| AppError::PythonService("No stdout".to_string()))?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        reader
            .read_line(&mut line)
            .map_err(|e| AppError::PythonService(format!("Failed to read ready signal: {}", e)))?;

        let response: Response = serde_json::from_str(&line)
            .map_err(|e| AppError::PythonService(format!("Invalid ready signal: {}", e)))?;

        if response.status != "ready" {
            return Err(AppError::PythonService(
                "Python service not ready".to_string(),
            ));
        }

        log::info!("Python service is ready");
        Ok(())
    }

    /// Send a command to the Python service
    pub fn send_command(&mut self, cmd: &str, params: serde_json::Value) -> AppResult<Response> {
        let request = Request {
            id: uuid::Uuid::new_v4().to_string(),
            cmd: cmd.to_string(),
            params,
        };

        // Write request
        let stdin = self
            .process
            .stdin
            .as_mut()
            .ok_or_else(|| AppError::PythonService("No stdin".to_string()))?;

        let request_json = serde_json::to_string(&request)?;
        writeln!(stdin, "{}", request_json)
            .map_err(|e| AppError::PythonService(format!("Failed to write request: {}", e)))?;
        stdin
            .flush()
            .map_err(|e| AppError::PythonService(format!("Failed to flush: {}", e)))?;

        // Read response(s)
        let stdout = self
            .process
            .stdout
            .as_mut()
            .ok_or_else(|| AppError::PythonService("No stdout".to_string()))?;
        let mut reader = BufReader::new(stdout);

        loop {
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .map_err(|e| AppError::PythonService(format!("Failed to read response: {}", e)))?;

            let response: Response = serde_json::from_str(&line)
                .map_err(|e| AppError::PythonService(format!("Invalid response: {}", e)))?;

            match response.status.as_str() {
                "progress" => {
                    // TODO: Emit progress event to frontend
                    log::debug!("Parse progress: {}%", response.progress.unwrap_or(0));
                }
                "success" | "error" => {
                    return Ok(response);
                }
                _ => {
                    return Err(AppError::PythonService(format!(
                        "Unknown status: {}",
                        response.status
                    )));
                }
            }
        }
    }

    /// Parse a demo file
    pub fn parse_demo(&mut self, file_path: &str) -> AppResult<serde_json::Value> {
        let params = serde_json::json!({
            "file_path": file_path,
            "options": {
                "extract_positions": true,
                "position_interval": 16
            }
        });

        let response = self.send_command("parse", params)?;

        if response.status == "error" {
            let error = response.error.unwrap_or(ErrorInfo {
                code: "UNKNOWN".to_string(),
                message: "Unknown error".to_string(),
                details: None,
            });
            return Err(AppError::ParseError(format!(
                "{}: {}",
                error.code, error.message
            )));
        }

        response
            .data
            .ok_or_else(|| AppError::ParseError("No data in response".to_string()))
    }

    /// Shutdown the Python service gracefully
    pub fn shutdown(&mut self) -> AppResult<()> {
        let _ = self.send_command("shutdown", serde_json::json!({}));
        let _ = self.process.wait();
        Ok(())
    }
}

impl Drop for PythonBridge {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
