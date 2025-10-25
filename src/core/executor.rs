use std::path::Path;
use std::sync::Arc;
use tracing::{info, debug, warn};
use console::style;

use crate::core::models::{ExecutionResult, ExecutionRecord};
use crate::core::history::HistoryManagerService;
use crate::handlers::service::LanguageHandlerService;
use crate::utils::errors::{ErrorHandler, AppError};

/// Service for automatically executing files when they change
pub struct AutoExecutorService {
    language_handlers: Arc<LanguageHandlerService>,
    history_manager: Arc<HistoryManagerService>,
    error_handler: ErrorHandler,
}

impl AutoExecutorService {
    /// Create a new AutoExecutorService
    pub fn new(
        language_handlers: Arc<LanguageHandlerService>,
        history_manager: Arc<HistoryManagerService>
    ) -> Self {
        Self {
            language_handlers,
            history_manager,
            error_handler: ErrorHandler::new(),
        }
    }
    
    /// Execute a file and return the execution result
    /// This method ensures the application continues running even if execution fails
    pub async fn execute_file(&self, file_path: impl AsRef<Path>) -> ExecutionResult {
        let path = file_path.as_ref();
        let path_str = path.display().to_string();
        
        debug!("Attempting to execute file: {}", path_str);
        
        // Check if file exists
        if !path.exists() {
            let error_msg = self.error_handler.handle_file_not_found_error(path);
            println!("{}", error_msg);
            return ExecutionResult::new(path.to_path_buf())
                .with_error("File not found".to_string(), std::time::Duration::default(), None);
        }
        
        // Get file extension
        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => {
                let app_error = AppError::execution("File has no extension");
                let error_msg = self.error_handler.handle_execution_error(&app_error.into(), path);
                println!("{}", error_msg);
                return ExecutionResult::new(path.to_path_buf())
                    .with_error("No file extension".to_string(), std::time::Duration::default(), None);
            }
        };
        
        // Get language handler
        let handler = match self.language_handlers.get_handler(extension).await {
            Some(handler) => handler,
            None => {
                let app_error = AppError::execution(format!("No handler found for extension: .{}", extension));
                let error_msg = self.error_handler.handle_execution_error(&app_error.into(), path);
                println!("{}", error_msg);
                return ExecutionResult::new(path.to_path_buf())
                    .with_error(format!("Unsupported file type: .{}", extension), std::time::Duration::default(), None);
            }
        };
        
        info!("Executing {} file: {}", handler.get_name(), path_str);
        
        // Execute the file with error handling
        let result = match handler.execute(path).await {
            Ok(result) => {
                if result.success {
                    info!("Successfully executed: {} ({}ms)", 
                          path_str, result.execution_time.as_millis());
                } else {
                    // Handle execution errors but continue operation
                    let error_msg = result.error_message.as_deref().unwrap_or("Unknown error");
                    if error_msg.contains("SyntaxError") || error_msg.contains("syntax") {
                        let formatted_error = self.error_handler.handle_syntax_error(error_msg, path);
                        println!("{}", formatted_error);
                    } else {
                        let formatted_error = self.error_handler.handle_runtime_error(error_msg, path);
                        println!("{}", formatted_error);
                    }
                }
                result
            }
            Err(e) => {
                // Handle handler errors and continue operation
                let error_msg = self.error_handler.handle_execution_error(&e, path);
                println!("{}", error_msg);
                ExecutionResult::new(path.to_path_buf())
                    .with_error(format!("Handler error: {}", e), std::time::Duration::default(), None)
            }
        };

        // Record execution in history
        let section = self.extract_section(path);
        let record = ExecutionRecord::from_result(&result, section);
        
        if let Err(e) = self.history_manager.record_execution(&record).await {
            warn!("Failed to record execution in history: {}", e);
            // Continue execution even if history recording fails
        }

        result
    }
    
    /// Get the language handler for a specific file extension
    #[allow(dead_code)]
    pub async fn get_language_handler(
        &self, 
        file_extension: impl AsRef<str>
    ) -> Option<Arc<dyn crate::handlers::base::LanguageHandler + Send + Sync>> {
        self.language_handlers.get_handler(file_extension).await
    }
    
    /// Format execution output for display
    pub fn format_output(&self, result: &ExecutionResult) -> String {
        let file_name = result.file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
            
        let timestamp = result.timestamp.format("%H:%M:%S");
        let execution_time = format!("{}ms", result.execution_time.as_millis());
        
        if result.success {
            format!(
                "{} {} {} {} ({})\n{}",
                style("✓").green().bold(),
                style(timestamp).dim(),
                style("Executed").green(),
                style(file_name).cyan().bold(),
                style(execution_time).dim(),
                if result.output.is_empty() {
                    style("(no output)").dim().to_string()
                } else {
                    result.output.clone()
                }
            )
        } else {
            format!(
                "{} {} {} {} ({})\n{}",
                style("✗").red().bold(),
                style(timestamp).dim(),
                style("Failed").red(),
                style(file_name).cyan().bold(),
                style(execution_time).dim(),
                style(result.error_message.as_deref().unwrap_or("Unknown error")).red()
            )
        }
    }
    
    /// Check if a file extension is supported
    #[allow(dead_code)]
    pub async fn is_supported_file(&self, file_path: impl AsRef<Path>) -> bool {
        let path = file_path.as_ref();
        
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            self.language_handlers.is_supported(extension).await
        } else {
            false
        }
    }
    
    /// Get all supported file extensions
    #[allow(dead_code)]
    pub async fn get_supported_extensions(&self) -> Vec<String> {
        self.language_handlers.get_supported_extensions().await
    }
    
    /// Execute a file with timeout to prevent hanging
    #[allow(dead_code)]
    pub async fn execute_file_with_timeout(&self, file_path: impl AsRef<Path>, timeout_secs: u64) -> ExecutionResult {
        let path = file_path.as_ref().to_path_buf();
        
        match tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            self.execute_file(&path)
        ).await {
            Ok(result) => result,
            Err(_) => {
                let error_msg = self.error_handler.handle_timeout_error(&path, timeout_secs);
                println!("{}", error_msg);
                ExecutionResult::new(path)
                    .with_error(format!("Execution timeout ({}s)", timeout_secs), std::time::Duration::from_secs(timeout_secs), None)
            }
        }
    }
    
    /// Handle system errors and ensure continuation
    #[allow(dead_code)]
    pub fn handle_system_error(&self, error: &anyhow::Error) -> String {
        self.error_handler.handle_system_error(error)
    }
    
    /// Extract section name from file path
    pub fn extract_section(&self, file_path: impl AsRef<Path>) -> String {
        let path = file_path.as_ref();
        
        // Try to get the parent directory name as section
        if let Some(parent) = path.parent() {
            if let Some(section_name) = parent.file_name().and_then(|name| name.to_str()) {
                return section_name.to_string();
            }
        }
        
        // Fallback to "unknown" if we can't determine the section
        "unknown".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::TempDir;
    use std::fs;
    
    #[tokio::test]
    async fn test_extract_section() {
        let handler_service = Arc::new(LanguageHandlerService::new());
        let temp_db = tempfile::NamedTempFile::new().unwrap();
        let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
        let executor = AutoExecutorService::new(handler_service, history_manager);
        
        let temp_dir = TempDir::new().unwrap();
        let section_dir = temp_dir.path().join("section1-basics");
        fs::create_dir(&section_dir).unwrap();
        let file_path = section_dir.join("test.py");
        
        let section = executor.extract_section(&file_path);
        assert_eq!(section, "section1-basics");
    }
    
    #[tokio::test]
    async fn test_format_output_success() {
        let handler_service = Arc::new(LanguageHandlerService::new());
        let temp_db = tempfile::NamedTempFile::new().unwrap();
        let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
        let executor = AutoExecutorService::new(handler_service, history_manager);
        
        let result = ExecutionResult::new(std::path::PathBuf::from("test.py"))
            .with_success("Hello, World!".to_string(), std::time::Duration::from_millis(100), 0);
            
        let output = executor.format_output(&result);
        assert!(output.contains("✓"));
        assert!(output.contains("Executed"));
        assert!(output.contains("test.py"));
        assert!(output.contains("Hello, World!"));
    }
    
    #[tokio::test]
    async fn test_format_output_failure() {
        let handler_service = Arc::new(LanguageHandlerService::new());
        let temp_db = tempfile::NamedTempFile::new().unwrap();
        let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
        let executor = AutoExecutorService::new(handler_service, history_manager);
        
        let result = ExecutionResult::new(std::path::PathBuf::from("test.py"))
            .with_error("Syntax error".to_string(), std::time::Duration::from_millis(50), Some(1));
            
        let output = executor.format_output(&result);
        assert!(output.contains("✗"));
        assert!(output.contains("Failed"));
        assert!(output.contains("test.py"));
        assert!(output.contains("Syntax error"));
    }
}