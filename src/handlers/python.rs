use async_trait::async_trait;
use anyhow::{anyhow, Result};
use std::path::Path;
use std::time::Instant;
use tokio::process::Command;

use crate::core::models::{ExecutionResult, ValidationResult};
use super::base::LanguageHandler;

/// Handler for Python files
#[derive(Debug, Clone)]
pub struct PythonHandler {
    python_command: String,
}

impl PythonHandler {
    /// Create a new PythonHandler with default python command
    pub fn new() -> Self {
        Self {
            python_command: "python".to_string(),
        }
    }
    
    /// Create a new PythonHandler with custom python command
    pub fn with_command(command: impl Into<String>) -> Self {
        Self {
            python_command: command.into(),
        }
    }
    
    /// Execute a Python file and capture output
    async fn execute_python_file(&self, file_path: impl AsRef<Path>) -> Result<(String, String, Option<i32>, std::time::Duration)> {
        let start_time = Instant::now();
        let file_path = file_path.as_ref();
        
        let output = Command::new(&self.python_command)
            .arg(file_path)
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute python command: {}", e))?;
        
        let execution_time = start_time.elapsed();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code();
        
        Ok((stdout, stderr, exit_code, execution_time))
    }
    
    /// Check Python syntax using python -m py_compile
    async fn check_syntax(&self, file_path: impl AsRef<Path>) -> Result<ValidationResult> {
        let file_path = file_path.as_ref();
        
        let output = Command::new(&self.python_command)
            .args(["-m", "py_compile"])
            .arg(file_path)
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute syntax check: {}", e))?;
        
        if output.status.success() {
            Ok(ValidationResult::valid())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(ValidationResult::invalid(error_message))
        }
    }
}

impl Default for PythonHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LanguageHandler for PythonHandler {
    async fn execute(&self, file_path: &Path) -> Result<ExecutionResult> {
        let file_path_buf = file_path.to_path_buf();
        let mut result = ExecutionResult::new(file_path_buf);
        
        match self.execute_python_file(file_path).await {
            Ok((stdout, stderr, exit_code, execution_time)) => {
                result.execution_time = execution_time;
                result.exit_code = exit_code;
                
                if exit_code == Some(0) {
                    // Successful execution
                    result = result.with_success(stdout, execution_time, 0);
                } else {
                    // Execution failed
                    let error_message = if stderr.is_empty() {
                        stdout
                    } else {
                        stderr
                    };
                    result = result.with_error(error_message, execution_time, exit_code);
                }
            }
            Err(e) => {
                result = result.with_error(e.to_string(), std::time::Duration::default(), None);
            }
        }
        
        Ok(result)
    }
    
    async fn validate_syntax(&self, file_path: &Path) -> Result<ValidationResult> {
        self.check_syntax(file_path).await
    }
    
    fn get_command(&self, file_path: &Path) -> Vec<String> {
        vec![
            self.python_command.clone(),
            file_path.to_string_lossy().to_string(),
        ]
    }
    
    fn get_extension(&self) -> &'static str {
        "py"
    }
    
    fn get_name(&self) -> &'static str {
        "Python"
    }
}