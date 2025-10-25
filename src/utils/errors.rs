use anyhow::Error;
use console::style;
use std::path::Path;
use tracing::{error, warn};

/// ErrorHandler provides centralized error handling and formatting for the application
#[derive(Debug, Default, Clone)]
pub struct ErrorHandler;

impl ErrorHandler {
    /// Creates a new ErrorHandler instance
    pub fn new() -> Self {
        Self
    }
    
    /// Handles execution errors that occur when running code files
    /// Returns a formatted error message for display
    pub fn handle_execution_error(&self, error: &Error, file_path: impl AsRef<Path>) -> String {
        let path_str = file_path.as_ref().display();
        error!("Execution error in {}: {}", path_str, error);
        
        format!(
            "{} Execution failed for {}: {}",
            style("✗").red().bold(),
            style(path_str).cyan(),
            style(error).red()
        )
    }
    
    /// Handles file watching errors from the notify crate
    /// Returns a formatted warning message for display
    pub fn handle_file_watch_error(&self, error: &notify::Error) -> String {
        warn!("File watch error: {}", error);
        
        format!(
            "{} File watching error: {}",
            style("⚠").yellow().bold(),
            style(error).yellow()
        )
    }
    
    /// Handles system-level errors
    /// Returns a formatted error message for display
    pub fn handle_system_error(&self, error: &Error) -> String {
        error!("System error: {}", error);
        
        format!(
            "{} System error: {}",
            style("✗").red().bold(),
            style(error).red()
        )
    }
    
    /// Formats a generic error message with consistent styling
    #[allow(dead_code)]
    pub fn format_error_message(&self, error: &Error) -> String {
        format!("{} {}", style("Error:").red().bold(), error)
    }
    
    /// Handles syntax errors specifically for code execution
    pub fn handle_syntax_error(&self, error: &str, file_path: impl AsRef<Path>) -> String {
        let path_str = file_path.as_ref().display();
        warn!("Syntax error in {}: {}", path_str, error);
        
        format!(
            "{} Syntax error in {}: {}",
            style("⚠").yellow().bold(),
            style(path_str).cyan(),
            style(error).yellow()
        )
    }
    
    /// Handles runtime errors that occur during code execution
    pub fn handle_runtime_error(&self, error: &str, file_path: impl AsRef<Path>) -> String {
        let path_str = file_path.as_ref().display();
        warn!("Runtime error in {}: {}", path_str, error);
        
        format!(
            "{} Runtime error in {}: {}",
            style("✗").red().bold(),
            style(path_str).cyan(),
            style(error).red()
        )
    }
    
    /// Handles timeout errors when code execution takes too long
    #[allow(dead_code)]
    pub fn handle_timeout_error(&self, file_path: impl AsRef<Path>, timeout_secs: u64) -> String {
        let path_str = file_path.as_ref().display();
        warn!("Execution timeout for {}: {} seconds", path_str, timeout_secs);
        
        format!(
            "{} Execution timeout for {} ({}s limit)",
            style("⏱").yellow().bold(),
            style(path_str).cyan(),
            style(timeout_secs).yellow()
        )
    }
    
    /// Handles permission errors when accessing files or directories
    #[allow(dead_code)]
    pub fn handle_permission_error(&self, path: impl AsRef<Path>) -> String {
        let path_str = path.as_ref().display();
        error!("Permission denied: {}", path_str);
        
        format!(
            "{} Permission denied: {}",
            style("🔒").red().bold(),
            style(path_str).cyan()
        )
    }
    
    /// Handles file not found errors
    pub fn handle_file_not_found_error(&self, path: impl AsRef<Path>) -> String {
        let path_str = path.as_ref().display();
        warn!("File not found: {}", path_str);
        
        format!(
            "{} File not found: {}",
            style("📁").yellow().bold(),
            style(path_str).cyan()
        )
    }
}

/// Custom error types for the application
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("File watching error: {0}")]
    FileWatch(#[from] notify::Error),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Syntax error: {0}")]
    Syntax(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("Timeout error: execution exceeded {timeout}s")]
    Timeout { timeout: u64 },
    
    #[error("Permission denied: {path}")]
    Permission { path: String },
    
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("System error: {0}")]
    System(#[from] anyhow::Error),
}

impl AppError {
    /// Creates a new execution error
    pub fn execution(msg: impl Into<String>) -> Self {
        Self::Execution(msg.into())
    }
    
    /// Creates a new syntax error
    #[allow(dead_code)]
    pub fn syntax(msg: impl Into<String>) -> Self {
        Self::Syntax(msg.into())
    }
    
    /// Creates a new runtime error
    #[allow(dead_code)]
    pub fn runtime(msg: impl Into<String>) -> Self {
        Self::Runtime(msg.into())
    }
    
    /// Creates a new timeout error
    #[allow(dead_code)]
    pub fn timeout(timeout: u64) -> Self {
        Self::Timeout { timeout }
    }
    
    /// Creates a new permission error
    #[allow(dead_code)]
    pub fn permission(path: impl AsRef<Path>) -> Self {
        Self::Permission {
            path: path.as_ref().display().to_string(),
        }
    }
    
    /// Creates a new file not found error
    pub fn file_not_found(path: impl AsRef<Path>) -> Self {
        Self::FileNotFound {
            path: path.as_ref().display().to_string(),
        }
    }
}