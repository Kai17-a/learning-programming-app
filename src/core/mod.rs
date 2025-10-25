pub mod config;
pub mod event_handler;
pub mod executor;
pub mod history;
pub mod integration;
pub mod models;
pub mod statistics;
pub mod watcher;

pub use config::ApplicationConfig;
pub use event_handler::{FileChangeEventHandler, FileStats};
pub use executor::AutoExecutorService;
pub use history::HistoryManagerService;
pub use integration::{ApplicationService, SystemStatus};
pub use models::{ExecutionResult, ExecutionRecord, ExecutionStats, ValidationResult};
pub use statistics::StatisticsService;
pub use watcher::FileWatcherService;