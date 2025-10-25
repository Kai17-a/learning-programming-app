use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tracing::{debug, info, warn, error};
use crate::utils::errors::ErrorHandler;

/// Handler for processing file change events with filtering capabilities
pub struct FileChangeEventHandler {
    supported_extensions: HashSet<String>,
    event_receiver: Option<mpsc::UnboundedReceiver<PathBuf>>,
    callback: Option<Box<dyn Fn(PathBuf) -> Result<()> + Send + Sync>>,
    error_handler: ErrorHandler,
}

impl FileChangeEventHandler {
    /// Create a new event handler with default supported extensions
    pub fn new() -> Self {
        let mut supported_extensions = HashSet::new();
        supported_extensions.insert("py".to_string());
        
        Self {
            supported_extensions,
            event_receiver: None,
            callback: None,
            error_handler: ErrorHandler::new(),
        }
    }

    /// Create a new event handler with custom supported extensions
    pub fn with_extensions(extensions: Vec<String>) -> Self {
        let supported_extensions = extensions.into_iter().collect();
        
        Self {
            supported_extensions,
            event_receiver: None,
            callback: None,
            error_handler: ErrorHandler::new(),
        }
    }

    /// Add a supported file extension
    pub fn add_extension(&mut self, extension: impl Into<String>) {
        let ext = extension.into();
        debug!("Adding supported extension: .{}", ext);
        self.supported_extensions.insert(ext);
    }

    /// Remove a supported file extension
    pub fn remove_extension(&mut self, extension: impl AsRef<str>) {
        let ext = extension.as_ref();
        debug!("Removing supported extension: .{}", ext);
        self.supported_extensions.remove(ext);
    }

    /// Get the list of supported extensions
    pub fn get_supported_extensions(&self) -> Vec<String> {
        self.supported_extensions.iter().cloned().collect()
    }

    /// Check if a file extension is supported
    pub fn is_extension_supported(&self, extension: impl AsRef<str>) -> bool {
        self.supported_extensions.contains(extension.as_ref())
    }

    /// Check if a file path should be processed based on its extension
    pub fn should_process_file(&self, file_path: impl AsRef<Path>) -> bool {
        let path = file_path.as_ref();
        
        // Check if it's a file
        if !path.is_file() {
            debug!("Skipping non-file path: {}", path.display());
            return false;
        }

        // Check extension
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let is_supported = self.is_extension_supported(ext_str);
                debug!(
                    "File {} has extension '{}', supported: {}",
                    path.display(),
                    ext_str,
                    is_supported
                );
                return is_supported;
            }
        }

        debug!("File {} has no extension or invalid extension", path.display());
        false
    }

    /// Set the event receiver channel
    pub fn set_receiver(&mut self, receiver: mpsc::UnboundedReceiver<PathBuf>) {
        debug!("Setting event receiver");
        self.event_receiver = Some(receiver);
    }

    /// Set the callback function to be called when a valid file change is detected
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(PathBuf) -> Result<()> + Send + Sync + 'static,
    {
        debug!("Setting file change callback");
        self.callback = Some(Box::new(callback));
    }

    /// Start processing file change events with robust error handling
    /// Ensures the event loop continues even when individual file processing fails
    pub async fn start_processing(&mut self) -> Result<()> {
        info!("Starting file change event processing with error recovery");
        
        let mut receiver = self.event_receiver.take()
            .ok_or_else(|| anyhow::anyhow!("No event receiver set"))?;

        let callback = self.callback.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No callback function set"))?;

        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: usize = 10;

        while let Some(file_path) = receiver.recv().await {
            debug!("Received file change event: {}", file_path.display());
            
            // Check if file still exists (it might have been deleted)
            if !file_path.exists() {
                debug!("File no longer exists, skipping: {}", file_path.display());
                continue;
            }
            
            if self.should_process_file(&file_path) {
                info!("Processing file change: {}", file_path.display());
                
                match callback(file_path.clone()) {
                    Ok(()) => {
                        debug!("Successfully processed file: {}", file_path.display());
                        consecutive_errors = 0; // Reset error counter on success
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        
                        // Handle different types of errors appropriately
                        let error_msg = if e.to_string().contains("permission") {
                            self.error_handler.handle_permission_error(&file_path)
                        } else if e.to_string().contains("not found") {
                            self.error_handler.handle_file_not_found_error(&file_path)
                        } else {
                            self.error_handler.handle_execution_error(&e, &file_path)
                        };
                        
                        println!("{}", error_msg);
                        
                        // If too many consecutive errors, log warning but continue
                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                            error!(
                                "Too many consecutive errors ({}), but continuing to process events",
                                consecutive_errors
                            );
                            consecutive_errors = 0; // Reset to prevent spam
                        }
                        
                        // Always continue processing other files
                    }
                }
            } else {
                debug!("Skipping file (not supported): {}", file_path.display());
            }
        }

        info!("File change event processing stopped");
        Ok(())
    }
    
    /// Handle system errors during event processing
    pub fn handle_processing_error(&self, error: &anyhow::Error, file_path: &Path) -> String {
        self.error_handler.handle_execution_error(error, file_path)
    }
    
    /// Recover from event processing errors
    pub async fn recover_from_processing_error(&mut self) -> Result<()> {
        warn!("Attempting to recover from event processing error");
        
        // Clear any pending events and restart processing
        if let Some(mut receiver) = self.event_receiver.take() {
            // Drain any pending events
            while receiver.try_recv().is_ok() {
                // Discard pending events
            }
            
            // Restore the receiver
            self.event_receiver = Some(receiver);
        }
        
        info!("Event processing recovery completed");
        Ok(())
    }

    /// Filter a list of file paths based on supported extensions
    pub fn filter_supported_files(&self, files: Vec<PathBuf>) -> Vec<PathBuf> {
        files
            .into_iter()
            .filter(|path| self.should_process_file(path))
            .collect()
    }

    /// Get statistics about supported vs unsupported files in a directory
    pub async fn get_file_stats(&self, directory: impl AsRef<Path>) -> Result<FileStats> {
        self.get_file_stats_recursive(directory.as_ref()).await
    }

    /// Internal recursive function for getting file stats
    fn get_file_stats_recursive<'a>(&'a self, dir_path: &'a Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<FileStats>> + Send + 'a>> {
        Box::pin(async move {
            let mut stats = FileStats::default();

            if !dir_path.exists() || !dir_path.is_dir() {
                return Err(anyhow::anyhow!(
                    "Directory does not exist or is not a directory: {}",
                    dir_path.display()
                ));
            }

            let mut entries = tokio::fs::read_dir(dir_path).await?;
            
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                
                if path.is_file() {
                    stats.total_files += 1;
                    
                    if self.should_process_file(&path) {
                        stats.supported_files += 1;
                    } else {
                        stats.unsupported_files += 1;
                    }
                } else if path.is_dir() {
                    // Recursively check subdirectories
                    let sub_stats = self.get_file_stats_recursive(&path).await?;
                    stats.total_files += sub_stats.total_files;
                    stats.supported_files += sub_stats.supported_files;
                    stats.unsupported_files += sub_stats.unsupported_files;
                }
            }

            Ok(stats)
        })
    }
}

impl Default for FileChangeEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about files in a directory
#[derive(Debug, Default, Clone)]
pub struct FileStats {
    pub total_files: usize,
    pub supported_files: usize,
    pub unsupported_files: usize,
}

impl FileStats {
    /// Get the percentage of supported files
    pub fn support_percentage(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.supported_files as f64 / self.total_files as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio::fs;

    #[tokio::test]
    async fn test_new_event_handler() {
        let handler = FileChangeEventHandler::new();
        assert!(handler.is_extension_supported("py"));
        assert!(!handler.is_extension_supported("txt"));
    }

    #[tokio::test]
    async fn test_with_extensions() {
        let handler = FileChangeEventHandler::with_extensions(vec![
            "rs".to_string(),
            "js".to_string(),
        ]);
        
        assert!(handler.is_extension_supported("rs"));
        assert!(handler.is_extension_supported("js"));
        assert!(!handler.is_extension_supported("py"));
    }

    #[tokio::test]
    async fn test_add_remove_extension() {
        let mut handler = FileChangeEventHandler::new();
        
        // Add extension
        handler.add_extension("rs");
        assert!(handler.is_extension_supported("rs"));
        
        // Remove extension
        handler.remove_extension("py");
        assert!(!handler.is_extension_supported("py"));
    }

    #[tokio::test]
    async fn test_should_process_file() {
        let temp_dir = TempDir::new().unwrap();
        let handler = FileChangeEventHandler::new();
        
        // Create test files
        let py_file = temp_dir.path().join("test.py");
        let txt_file = temp_dir.path().join("test.txt");
        
        fs::write(&py_file, "print('hello')").await.unwrap();
        fs::write(&txt_file, "hello").await.unwrap();
        
        assert!(handler.should_process_file(&py_file));
        assert!(!handler.should_process_file(&txt_file));
    }

    #[tokio::test]
    async fn test_filter_supported_files() {
        let temp_dir = TempDir::new().unwrap();
        let handler = FileChangeEventHandler::new();
        
        // Create test files
        let py_file = temp_dir.path().join("test.py");
        let txt_file = temp_dir.path().join("test.txt");
        let rs_file = temp_dir.path().join("test.rs");
        
        fs::write(&py_file, "print('hello')").await.unwrap();
        fs::write(&txt_file, "hello").await.unwrap();
        fs::write(&rs_file, "fn main() {}").await.unwrap();
        
        let files = vec![py_file.clone(), txt_file, rs_file];
        let filtered = handler.filter_supported_files(files);
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0], py_file);
    }

    #[tokio::test]
    async fn test_get_file_stats() {
        let temp_dir = TempDir::new().unwrap();
        let handler = FileChangeEventHandler::new();
        
        // Create test files
        fs::write(temp_dir.path().join("test1.py"), "print('hello')").await.unwrap();
        fs::write(temp_dir.path().join("test2.py"), "print('world')").await.unwrap();
        fs::write(temp_dir.path().join("readme.txt"), "readme").await.unwrap();
        
        let stats = handler.get_file_stats(temp_dir.path()).await.unwrap();
        
        assert_eq!(stats.total_files, 3);
        assert_eq!(stats.supported_files, 2);
        assert_eq!(stats.unsupported_files, 1);
        assert!((stats.support_percentage() - 66.67).abs() < 0.1);
    }

    #[tokio::test]
    async fn test_event_processing() {
        let temp_dir = TempDir::new().unwrap();
        let mut handler = FileChangeEventHandler::new();
        
        // Create test file
        let py_file = temp_dir.path().join("test.py");
        fs::write(&py_file, "print('hello')").await.unwrap();
        
        // Set up channels
        let (tx, rx) = mpsc::unbounded_channel();
        handler.set_receiver(rx);
        
        // Set up callback to count processed files
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        handler.set_callback(move |_path| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });
        
        // Send events
        tx.send(py_file.clone()).unwrap();
        tx.send(temp_dir.path().join("test.txt")).unwrap(); // Should be filtered out
        drop(tx); // Close channel to stop processing
        
        // Process events
        handler.start_processing().await.unwrap();
        
        // Check that only the .py file was processed
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}