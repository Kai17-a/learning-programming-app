use anyhow::Result;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use crate::utils::errors::{ErrorHandler, AppError};

/// Service for watching file system changes
pub struct FileWatcherService {
    watcher: RwLock<Option<RecommendedWatcher>>,
    is_watching: AtomicBool,
    watched_dirs: RwLock<Vec<PathBuf>>,
    event_sender: RwLock<Option<mpsc::UnboundedSender<PathBuf>>>,
    error_handler: ErrorHandler,
}

impl FileWatcherService {
    /// Create a new FileWatcherService instance
    pub fn new() -> Self {
        Self {
            watcher: RwLock::new(None),
            is_watching: AtomicBool::new(false),
            watched_dirs: RwLock::new(Vec::new()),
            event_sender: RwLock::new(None),
            error_handler: ErrorHandler::new(),
        }
    }

    /// Start watching a directory for file changes
    /// Handles errors gracefully and ensures the application continues running
    pub async fn start_watching(
        &self,
        directory: impl AsRef<Path>,
        callback: mpsc::UnboundedSender<PathBuf>,
    ) -> Result<()> {
        let dir_path = directory.as_ref().to_path_buf();
        
        // Check if directory exists
        if !dir_path.exists() {
            let error_msg = self.error_handler.handle_file_not_found_error(&dir_path);
            println!("{}", error_msg);
            return Err(AppError::file_not_found(&dir_path).into());
        }

        if !dir_path.is_dir() {
            let app_error = AppError::execution(format!("Path is not a directory: {}", dir_path.display()));
            let anyhow_error: anyhow::Error = app_error.into();
            let error_msg = self.error_handler.handle_system_error(&anyhow_error);
            println!("{}", error_msg);
            return Err(anyhow_error);
        }

        // Stop any existing watcher (ignore errors to ensure continuation)
        if let Err(e) = self.stop_watching().await {
            warn!("Error stopping previous watcher: {}", e);
        }

        info!("Starting file watcher for directory: {}", dir_path.display());

        // Store the callback sender
        {
            let mut sender_guard = self.event_sender.write().await;
            *sender_guard = Some(callback.clone());
        }

        // Create a new watcher with error handling
        let callback_clone = callback.clone();
        let error_handler = self.error_handler.clone();
        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    debug!("File system event: {:?}", event);
                    
                    // Filter for modify events on files
                    if matches!(event.kind, EventKind::Modify(_)) {
                        for path in event.paths {
                            if path.is_file() {
                                debug!("File modified: {}", path.display());
                                if let Err(e) = callback_clone.send(path) {
                                    error!("Failed to send file change event: {}", e);
                                    // Continue operation even if sending fails
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    // Handle file watcher errors but continue operation
                    let error_msg = error_handler.handle_file_watch_error(&e);
                    println!("{}", error_msg);
                    // Don't stop watching - continue monitoring
                }
            }
        });

        let watcher = match watcher {
            Ok(w) => w,
            Err(e) => {
                let anyhow_error: anyhow::Error = e.into();
                let error_msg = self.error_handler.handle_system_error(&anyhow_error);
                println!("{}", error_msg);
                return Err(anyhow_error);
            }
        };

        // Store the watcher
        {
            let mut watcher_guard = self.watcher.write().await;
            *watcher_guard = Some(watcher);
        }

        // Start watching the directory with error handling
        {
            let mut watcher_guard = self.watcher.write().await;
            if let Some(ref mut watcher) = *watcher_guard {
                if let Err(e) = watcher.watch(&dir_path, RecursiveMode::Recursive) {
                    let error_msg = self.error_handler.handle_file_watch_error(&e);
                    println!("{}", error_msg);
                    return Err(e.into());
                }
            }
        }

        // Update state
        {
            let mut dirs_guard = self.watched_dirs.write().await;
            dirs_guard.clear();
            dirs_guard.push(dir_path.clone());
        }
        
        self.is_watching.store(true, Ordering::SeqCst);
        
        info!("File watcher started successfully for: {}", dir_path.display());
        Ok(())
    }

    /// Stop watching for file changes
    pub async fn stop_watching(&self) -> Result<()> {
        if !self.is_watching() {
            debug!("File watcher is not currently running");
            return Ok(());
        }

        info!("Stopping file watcher");

        // Clear the watcher
        {
            let mut watcher_guard = self.watcher.write().await;
            *watcher_guard = None;
        }

        // Clear the event sender
        {
            let mut sender_guard = self.event_sender.write().await;
            *sender_guard = None;
        }

        // Clear watched directories
        {
            let mut dirs_guard = self.watched_dirs.write().await;
            dirs_guard.clear();
        }

        self.is_watching.store(false, Ordering::SeqCst);
        
        info!("File watcher stopped successfully");
        Ok(())
    }

    /// Check if the watcher is currently active
    pub fn is_watching(&self) -> bool {
        self.is_watching.load(Ordering::SeqCst)
    }

    /// Get the list of currently watched directories
    pub async fn get_watched_directories(&self) -> Vec<PathBuf> {
        let dirs_guard = self.watched_dirs.read().await;
        dirs_guard.clone()
    }

    /// Get the current status as a formatted string
    pub async fn get_status(&self) -> String {
        if self.is_watching() {
            let dirs = self.get_watched_directories().await;
            if dirs.is_empty() {
                "Watching: No directories".to_string()
            } else {
                format!(
                    "Watching: {}",
                    dirs.iter()
                        .map(|d| d.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        } else {
            "Not watching".to_string()
        }
    }
    
    /// Attempt to recover from watcher errors by restarting
    pub async fn recover_from_error(&self) -> Result<()> {
        warn!("Attempting to recover from file watcher error");
        
        // Get current watched directories
        let dirs = self.get_watched_directories().await;
        let sender = {
            let sender_guard = self.event_sender.read().await;
            sender_guard.clone()
        };
        
        if let Some(sender) = sender {
            // Try to restart watching for each directory
            for dir in dirs {
                if let Err(e) = self.start_watching(&dir, sender.clone()).await {
                    let error_msg = self.error_handler.handle_system_error(&e);
                    println!("{}", error_msg);
                    return Err(e);
                }
            }
            info!("Successfully recovered from file watcher error");
            Ok(())
        } else {
            warn!("Cannot recover: no event sender available");
            Err(anyhow::anyhow!("No event sender available for recovery"))
        }
    }
    
    /// Handle permission errors gracefully
    pub fn handle_permission_error(&self, path: impl AsRef<Path>) -> String {
        self.error_handler.handle_permission_error(path)
    }
}

impl Default for FileWatcherService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::TempDir;
    use tokio::fs;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_new_watcher_service() {
        let service = FileWatcherService::new();
        assert!(!service.is_watching());
        assert!(service.get_watched_directories().await.is_empty());
    }

    #[tokio::test]
    async fn test_start_watching_nonexistent_directory() {
        let service = FileWatcherService::new();
        let (tx, _rx) = mpsc::unbounded_channel();
        
        let result = service.start_watching("/nonexistent/path", tx).await;
        assert!(result.is_err());
        assert!(!service.is_watching());
    }

    #[tokio::test]
    async fn test_start_watching_valid_directory() {
        let temp_dir = TempDir::new().unwrap();
        let service = FileWatcherService::new();
        let (tx, _rx) = mpsc::unbounded_channel();
        
        let result = service.start_watching(temp_dir.path(), tx).await;
        assert!(result.is_ok());
        assert!(service.is_watching());
        
        let watched_dirs = service.get_watched_directories().await;
        assert_eq!(watched_dirs.len(), 1);
        assert_eq!(watched_dirs[0], temp_dir.path());
    }

    #[tokio::test]
    async fn test_stop_watching() {
        let temp_dir = TempDir::new().unwrap();
        let service = FileWatcherService::new();
        let (tx, _rx) = mpsc::unbounded_channel();
        
        // Start watching
        service.start_watching(temp_dir.path(), tx).await.unwrap();
        assert!(service.is_watching());
        
        // Stop watching
        let result = service.stop_watching().await;
        assert!(result.is_ok());
        assert!(!service.is_watching());
        assert!(service.get_watched_directories().await.is_empty());
    }

    #[tokio::test]
    async fn test_file_change_detection() {
        let temp_dir = TempDir::new().unwrap();
        let service = FileWatcherService::new();
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // Start watching
        service.start_watching(temp_dir.path(), tx).await.unwrap();
        
        // Create a test file
        let test_file = temp_dir.path().join("test.py");
        fs::write(&test_file, "print('hello')").await.unwrap();
        
        // Wait a bit for the initial create event to settle
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Modify the file
        fs::write(&test_file, "print('hello world')").await.unwrap();
        
        // Wait for the change event
        let result = timeout(Duration::from_secs(2), rx.recv()).await;
        assert!(result.is_ok());
        
        let changed_path = result.unwrap().unwrap();
        assert_eq!(changed_path, test_file);
    }

    #[tokio::test]
    async fn test_get_status() {
        let service = FileWatcherService::new();
        
        // Initially not watching
        let status = service.get_status().await;
        assert_eq!(status, "Not watching");
        
        // Start watching
        let temp_dir = TempDir::new().unwrap();
        let (tx, _rx) = mpsc::unbounded_channel();
        service.start_watching(temp_dir.path(), tx).await.unwrap();
        
        let status = service.get_status().await;
        assert!(status.starts_with("Watching:"));
        assert!(status.contains(&temp_dir.path().display().to_string()));
    }
}