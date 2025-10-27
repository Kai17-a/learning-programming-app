use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub file_path: PathBuf,
    pub success: bool,
    pub output: String,
    pub error_message: Option<String>,
    pub execution_time: Duration,
    pub timestamp: DateTime<Utc>,
    pub exit_code: Option<i32>,
}

impl ExecutionResult {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            success: false,
            output: String::new(),
            error_message: None,
            execution_time: Duration::default(),
            timestamp: Utc::now(),
            exit_code: None,
        }
    }

    pub fn with_success(
        mut self,
        output: String,
        execution_time: Duration,
        exit_code: i32,
    ) -> Self {
        self.success = true;
        self.output = output;
        self.execution_time = execution_time;
        self.exit_code = Some(exit_code);
        self
    }

    pub fn with_error(
        mut self,
        error_message: String,
        execution_time: Duration,
        exit_code: Option<i32>,
    ) -> Self {
        self.success = false;
        self.error_message = Some(error_message);
        self.execution_time = execution_time;
        self.exit_code = exit_code;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExecutionRecord {
    pub id: String,
    pub file_path: String,
    pub section: String,
    pub success: bool,
    pub execution_time: f64, // seconds
    pub timestamp: DateTime<Utc>,
    pub output_preview: String, // First 100 chars of output
}

impl ExecutionRecord {
    pub fn from_result(result: &ExecutionResult, section: impl Into<String>) -> Self {
        let output_preview = if result.success {
            result.output.chars().take(100).collect()
        } else {
            result
                .error_message
                .as_ref()
                .unwrap_or(&String::new())
                .chars()
                .take(100)
                .collect()
        };

        Self {
            id: Uuid::new_v4().to_string(),
            file_path: result.file_path.to_string_lossy().to_string(),
            section: section.into(),
            success: result.success,
            execution_time: result.execution_time.as_secs_f64(),
            timestamp: result.timestamp,
            output_preview,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub most_executed_file: Option<String>,
    pub average_execution_time: f64,
    pub last_execution: Option<DateTime<Utc>>,
}

impl ExecutionStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }

    #[allow(dead_code)]
    pub fn failure_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.failed_executions as f64 / self.total_executions as f64
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            error_message: None,
            warnings: Vec::new(),
        }
    }

    pub fn invalid(error_message: impl Into<String>) -> Self {
        Self {
            is_valid: false,
            error_message: Some(error_message.into()),
            warnings: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
}
