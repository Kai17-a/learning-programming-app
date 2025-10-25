use crate::core::{AutoExecutorService, FileWatcherService, HistoryManagerService};
use crate::handlers::{LanguageHandlerService, PythonHandler, GoHandler};
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

/// Integrated application service that manages all core components
pub struct ApplicationService {
    pub executor: Arc<AutoExecutorService>,
    pub watcher: Arc<FileWatcherService>,
    pub history: Arc<HistoryManagerService>,
    pub language_handlers: Arc<LanguageHandlerService>,
}

impl ApplicationService {
    /// Create a new integrated application service with all components initialized
    pub async fn new(database_path: &str) -> Result<Self> {
        info!("Initializing application services");

        // Initialize history manager with database
        let history = Arc::new(HistoryManagerService::new(database_path).await?);
        info!("History manager initialized");

        // Initialize language handler service
        let language_handlers = Arc::new(LanguageHandlerService::new());
        
        // Register supported language handlers
        Self::register_language_handlers(&language_handlers).await;
        info!("Language handlers registered");

        // Initialize executor with language handlers and history manager
        let executor = Arc::new(AutoExecutorService::new(language_handlers.clone(), history.clone()));
        info!("Auto executor initialized");

        // Initialize file watcher
        let watcher = Arc::new(FileWatcherService::new());
        info!("File watcher initialized");

        Ok(Self {
            executor,
            watcher,
            history,
            language_handlers,
        })
    }

    /// Register all supported language handlers
    async fn register_language_handlers(service: &LanguageHandlerService) {
        // Register Python handler
        service
            .register_handler("py", Arc::new(PythonHandler::new()))
            .await;
        
        info!("Registered Python handler for .py files");
        
        // Register Go handler
        service
            .register_handler("go", Arc::new(GoHandler::new()))
            .await;
        
        info!("Registered Go handler for .go files");
        
        // Future language handlers can be registered here
        // service.register_handler("js", Arc::new(JavaScriptHandler::new())).await;
        // service.register_handler("rs", Arc::new(RustHandler::new())).await;
    }

    /// Get system health status
    pub async fn get_system_status(&self) -> SystemStatus {
        SystemStatus {
            database_connected: self.history.is_healthy().await,
            watcher_active: self.watcher.is_watching(),
            registered_handlers: self.language_handlers.get_supported_extensions().await,
        }
    }

    /// Perform system health check
    pub async fn health_check(&self) -> Result<()> {
        let status = self.get_system_status().await;
        
        if !status.database_connected {
            warn!("Database connection issue detected");
        }
        
        if status.registered_handlers.is_empty() {
            warn!("No language handlers registered");
        }
        
        info!("System health check completed: {} handlers registered", 
              status.registered_handlers.len());
        
        Ok(())
    }

    /// Graceful shutdown of all services
    pub async fn shutdown(&self) -> Result<()> {
        info!("Initiating graceful shutdown");
        
        // Stop file watching
        if self.watcher.is_watching() {
            self.watcher.stop_watching().await?;
            info!("File watcher stopped");
        }
        
        // Close database connections
        self.history.close().await?;
        info!("Database connections closed");
        
        info!("Graceful shutdown completed");
        Ok(())
    }
}

/// System status information
#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub database_connected: bool,
    #[allow(dead_code)]
    pub watcher_active: bool,
    pub registered_handlers: Vec<String>,
}

impl SystemStatus {
    #[allow(dead_code)]
    pub fn is_healthy(&self) -> bool {
        self.database_connected && !self.registered_handlers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_application_service_initialization() {
        let temp_db = NamedTempFile::new().unwrap();
        let db_path = temp_db.path().to_str().unwrap();
        
        let app_service = ApplicationService::new(db_path).await.unwrap();
        
        // Verify all components are initialized
        assert!(!app_service.watcher.is_watching());
        
        let status = app_service.get_system_status().await;
        assert!(status.database_connected);
        assert!(!status.registered_handlers.is_empty());
        assert!(status.registered_handlers.contains(&"py".to_string()));
        assert!(status.registered_handlers.contains(&"go".to_string()));
    }

    #[tokio::test]
    async fn test_health_check() {
        let temp_db = NamedTempFile::new().unwrap();
        let db_path = temp_db.path().to_str().unwrap();
        
        let app_service = ApplicationService::new(db_path).await.unwrap();
        let result = app_service.health_check().await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let temp_db = NamedTempFile::new().unwrap();
        let db_path = temp_db.path().to_str().unwrap();
        
        let app_service = ApplicationService::new(db_path).await.unwrap();
        let result = app_service.shutdown().await;
        
        assert!(result.is_ok());
    }
}