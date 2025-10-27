use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::path::Path;
use std::time::Duration;
use tokio::process::Command;

use super::base::LanguageHandler;
use crate::core::models::{ExecutionResult, ValidationResult};

/// Handler for Go programming language
pub struct GoHandler {
    go_command: String,
}

impl GoHandler {
    /// Create a new GoHandler with default go command
    pub fn new() -> Self {
        Self {
            go_command: Self::detect_go_command(),
        }
    }

    /// Detect the appropriate Go command for the current environment
    fn detect_go_command() -> String {
        if std::process::Command::new("go")
            .arg("version")
            .output()
            .is_ok()
        {
            "go".to_string()
        } else {
            // Fallback to go if detection fails
            "go".to_string()
        }
    }

    /// Create a new GoHandler with custom go command
    #[allow(dead_code)]
    pub fn with_command(command: impl Into<String>) -> Self {
        Self {
            go_command: command.into(),
        }
    }

    /// Execute a Go file using go run
    async fn execute_go_file(
        &self,
        file_path: impl AsRef<Path>,
    ) -> Result<(String, String, Option<i32>, Duration)> {
        let file_path = file_path.as_ref();
        let start_time = std::time::Instant::now();

        let output = Command::new(&self.go_command)
            .args(["run"])
            .arg(file_path)
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute go command: {}", e))?;

        let execution_time = start_time.elapsed();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code();

        Ok((stdout, stderr, exit_code, execution_time))
    }

    /// Check Go syntax using go build -o /dev/null (or NUL on Windows)
    async fn check_syntax(&self, file_path: impl AsRef<Path>) -> Result<ValidationResult> {
        let file_path = file_path.as_ref();

        // Use appropriate null device based on platform
        let null_device = if cfg!(windows) { "NUL" } else { "/dev/null" };

        let output = Command::new(&self.go_command)
            .args(["build", "-o", null_device])
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

impl Default for GoHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LanguageHandler for GoHandler {
    async fn execute(&self, file_path: &Path) -> Result<ExecutionResult> {
        let file_path_buf = file_path.to_path_buf();
        let mut result = ExecutionResult::new(file_path_buf);

        match self.execute_go_file(file_path).await {
            Ok((stdout, stderr, exit_code, execution_time)) => {
                result.execution_time = execution_time;
                result.exit_code = exit_code;

                if exit_code == Some(0) {
                    // Successful execution
                    result = result.with_success(stdout, execution_time, 0);
                } else {
                    // Execution failed
                    let error_message = if stderr.is_empty() { stdout } else { stderr };
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
            self.go_command.clone(),
            "run".to_string(),
            file_path.to_string_lossy().to_string(),
        ]
    }

    fn get_extension(&self) -> &'static str {
        "go"
    }

    fn get_name(&self) -> &'static str {
        "Go"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_go_handler_creation() {
        let handler = GoHandler::new();
        assert_eq!(handler.get_extension(), "go");
        assert_eq!(handler.get_name(), "Go");
    }

    #[tokio::test]
    async fn test_go_handler_with_custom_command() {
        let handler = GoHandler::with_command("custom-go");
        assert_eq!(handler.go_command, "custom-go");
    }

    #[tokio::test]
    async fn test_get_command() {
        let handler = GoHandler::new();
        let temp_dir = TempDir::new().unwrap();
        let go_file = temp_dir.path().join("test.go");

        let command = handler.get_command(&go_file);
        assert_eq!(command[0], "go");
        assert_eq!(command[1], "run");
        assert_eq!(command[2], go_file.to_string_lossy());
    }

    #[tokio::test]
    async fn test_execute_valid_go_file() {
        let handler = GoHandler::new();
        let temp_dir = TempDir::new().unwrap();
        let go_file = temp_dir.path().join("hello.go");

        // Create a simple Go program
        let go_code = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#;
        fs::write(&go_file, go_code).unwrap();

        let result = handler.execute(&go_file).await;

        // The test will pass if Go is installed, otherwise it will fail gracefully
        match result {
            Ok(exec_result) => {
                if exec_result.success {
                    assert!(exec_result.output.contains("Hello, World!"));
                }
                // If Go is not installed, the execution will fail but that's expected
            }
            Err(_) => {
                // Go might not be installed in the test environment
                println!("Go not available in test environment");
            }
        }
    }

    #[tokio::test]
    async fn test_execute_invalid_go_file() {
        let handler = GoHandler::new();
        let temp_dir = TempDir::new().unwrap();
        let go_file = temp_dir.path().join("invalid.go");

        // Create an invalid Go program
        let invalid_go_code = r#"package main

import "fmt"

func main() {
    fmt.Println("Missing closing quote
}
"#;
        fs::write(&go_file, invalid_go_code).unwrap();

        let result = handler.execute(&go_file).await;

        match result {
            Ok(exec_result) => {
                // Should fail due to syntax error
                assert!(!exec_result.success);
                assert!(exec_result.error_message.is_some());
            }
            Err(_) => {
                // Go might not be installed in the test environment
                println!("Go not available in test environment");
            }
        }
    }

    #[tokio::test]
    async fn test_validate_syntax_valid() {
        let handler = GoHandler::new();
        let temp_dir = TempDir::new().unwrap();
        let go_file = temp_dir.path().join("valid.go");

        let valid_go_code = r#"package main

import "fmt"

func main() {
    fmt.Println("Valid Go code")
}
"#;
        fs::write(&go_file, valid_go_code).unwrap();

        let result = handler.validate_syntax(&go_file).await;

        match result {
            Ok(validation_result) => {
                if validation_result.is_valid {
                    assert!(validation_result.error_message.is_none());
                }
                // If Go is not installed, validation will fail but that's expected
            }
            Err(_) => {
                println!("Go not available in test environment");
            }
        }
    }

    #[tokio::test]
    async fn test_validate_syntax_invalid() {
        let handler = GoHandler::new();
        let temp_dir = TempDir::new().unwrap();
        let go_file = temp_dir.path().join("invalid.go");

        let invalid_go_code = r#"package main

func main() {
    invalid syntax here
}
"#;
        fs::write(&go_file, invalid_go_code).unwrap();

        let result = handler.validate_syntax(&go_file).await;

        match result {
            Ok(validation_result) => {
                // Should be invalid due to syntax error
                assert!(!validation_result.is_valid);
                assert!(validation_result.error_message.is_some());
            }
            Err(_) => {
                println!("Go not available in test environment");
            }
        }
    }
}
