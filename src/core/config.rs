use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub watch_directory: PathBuf,
    pub supported_extensions: Vec<String>,
    pub max_history_records: u32,
    pub auto_clear_output: bool,
    pub show_execution_time: bool,
    pub execution_timeout: Duration,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            watch_directory: PathBuf::from("./examples"),
            supported_extensions: vec!["py".to_string()],
            max_history_records: 1000,
            auto_clear_output: false,
            show_execution_time: true,
            execution_timeout: Duration::from_secs(30),
        }
    }
}

impl ApplicationConfig {
    /// Load configuration from file, falling back to defaults if file doesn't exist
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: ApplicationConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Check if a file extension is supported
    pub fn is_supported_extension(&self, extension: &str) -> bool {
        self.supported_extensions.contains(&extension.to_string())
    }
}