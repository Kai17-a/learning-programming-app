use async_trait::async_trait;
use anyhow::Result;
use std::path::Path;

use crate::core::models::{ExecutionResult, ValidationResult};

/// Trait for handling different programming languages
#[async_trait]
pub trait LanguageHandler: Send + Sync {
    /// Execute a file and return the execution result
    async fn execute(&self, file_path: &Path) -> Result<ExecutionResult>;
    
    /// Validate the syntax of a file
    #[allow(dead_code)]
    async fn validate_syntax(&self, file_path: &Path) -> Result<ValidationResult>;
    
    /// Get the command to execute a file
    #[allow(dead_code)]
    fn get_command(&self, file_path: &Path) -> Vec<String>;
    
    /// Get the file extension this handler supports
    #[allow(dead_code)]
    fn get_extension(&self) -> &'static str;
    
    /// Get the display name of the language
    fn get_name(&self) -> &'static str;
}