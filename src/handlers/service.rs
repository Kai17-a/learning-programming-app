use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::base::LanguageHandler;

/// Service for managing language handlers
pub struct LanguageHandlerService {
    handlers: RwLock<HashMap<String, Arc<dyn LanguageHandler + Send + Sync>>>,
}

impl LanguageHandlerService {
    /// Create a new LanguageHandlerService
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    /// Register a handler for a specific file extension
    pub async fn register_handler(
        &self,
        extension: impl Into<String>,
        handler: Arc<dyn LanguageHandler + Send + Sync>,
    ) {
        let ext = extension.into().to_lowercase();
        let mut handlers = self.handlers.write().await;
        handlers.insert(ext, handler);
    }

    /// Get a handler for a specific file extension
    pub async fn get_handler(
        &self,
        extension: impl AsRef<str>,
    ) -> Option<Arc<dyn LanguageHandler + Send + Sync>> {
        let ext = extension.as_ref().to_lowercase();
        let handlers = self.handlers.read().await;
        handlers.get(&ext).cloned()
    }

    /// Check if a file extension is supported
    #[allow(dead_code)]
    pub async fn is_supported(&self, extension: impl AsRef<str>) -> bool {
        let ext = extension.as_ref().to_lowercase();
        let handlers = self.handlers.read().await;
        handlers.contains_key(&ext)
    }

    /// Get all supported extensions
    pub async fn get_supported_extensions(&self) -> Vec<String> {
        let handlers = self.handlers.read().await;
        handlers.keys().cloned().collect()
    }

    /// Get the number of registered handlers
    #[allow(dead_code)]
    pub async fn handler_count(&self) -> usize {
        let handlers = self.handlers.read().await;
        handlers.len()
    }
}

impl Default for LanguageHandlerService {
    fn default() -> Self {
        Self::new()
    }
}
