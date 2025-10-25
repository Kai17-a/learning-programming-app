use anyhow::Result;
use learning_programming_app::core::{FileChangeEventHandler, FileWatcherService};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};

/// Integration test for file watcher and event handler working together
#[tokio::test]
async fn test_file_watcher_with_event_handler_integration() -> Result<()> {
    // Create temporary directory
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Set up file watcher service
    let watcher_service = FileWatcherService::new();
    let (tx, rx) = mpsc::unbounded_channel();

    // Set up event handler
    let mut event_handler = FileChangeEventHandler::new();
    event_handler.set_receiver(rx);

    // Counter to track processed files
    let processed_count = Arc::new(AtomicUsize::new(0));
    let counter_clone = processed_count.clone();

    event_handler.set_callback(move |path| {
        println!("Processing file: {}", path.display());
        counter_clone.fetch_add(1, Ordering::SeqCst);
        Ok(())
    });

    // Start watching
    watcher_service.start_watching(temp_path, tx).await?;
    assert!(watcher_service.is_watching());

    // Start event processing in background
    let processing_handle = tokio::spawn(async move {
        event_handler.start_processing().await
    });

    // Give the watcher time to initialize
    sleep(Duration::from_millis(100)).await;

    // Create and modify Python files
    let py_file1 = temp_path.join("test1.py");
    let py_file2 = temp_path.join("test2.py");
    let txt_file = temp_path.join("test.txt");

    // Create files
    fs::write(&py_file1, "print('hello')").await?;
    sleep(Duration::from_millis(100)).await;

    fs::write(&py_file2, "print('world')").await?;
    sleep(Duration::from_millis(100)).await;

    // Create a non-Python file (should be ignored)
    fs::write(&txt_file, "hello world").await?;
    sleep(Duration::from_millis(100)).await;

    // Modify Python files
    fs::write(&py_file1, "print('hello modified')").await?;
    sleep(Duration::from_millis(100)).await;

    fs::write(&py_file2, "print('world modified')").await?;
    sleep(Duration::from_millis(100)).await;

    // Modify text file (should be ignored)
    fs::write(&txt_file, "hello world modified").await?;
    sleep(Duration::from_millis(100)).await;

    // Wait for events to be processed
    sleep(Duration::from_millis(500)).await;

    // Stop watching
    watcher_service.stop_watching().await?;
    assert!(!watcher_service.is_watching());

    // Wait for processing to complete
    let _ = timeout(Duration::from_secs(2), processing_handle).await;

    // Check that only Python files were processed
    // We expect at least 2 events (modifications of the Python files)
    // The exact number may vary depending on the file system events
    let count = processed_count.load(Ordering::SeqCst);
    println!("Total processed files: {}", count);
    assert!(count >= 2, "Expected at least 2 processed files, got {}", count);

    Ok(())
}

/// Test file watcher with subdirectories
#[tokio::test]
async fn test_file_watcher_recursive_monitoring() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Create subdirectory
    let sub_dir = temp_path.join("subdir");
    fs::create_dir(&sub_dir).await?;

    let watcher_service = FileWatcherService::new();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Start watching
    watcher_service.start_watching(temp_path, tx).await?;

    // Give the watcher time to initialize
    sleep(Duration::from_millis(100)).await;

    // Create file in subdirectory
    let sub_file = sub_dir.join("nested.py");
    fs::write(&sub_file, "print('nested')").await?;

    // Wait for the create event to settle
    sleep(Duration::from_millis(100)).await;

    // Modify the file
    fs::write(&sub_file, "print('nested modified')").await?;

    // Wait for change event
    let result = timeout(Duration::from_secs(2), rx.recv()).await;
    assert!(result.is_ok(), "Should receive file change event");

    let changed_path = result.unwrap().unwrap();
    assert_eq!(changed_path, sub_file);

    watcher_service.stop_watching().await?;
    Ok(())
}

/// Test error handling when watching non-existent directory
#[tokio::test]
async fn test_watcher_error_handling() -> Result<()> {
    let watcher_service = FileWatcherService::new();
    let (tx, _rx) = mpsc::unbounded_channel();

    // Try to watch non-existent directory
    let result = watcher_service.start_watching("/non/existent/path", tx).await;
    assert!(result.is_err());
    assert!(!watcher_service.is_watching());

    Ok(())
}

/// Test stopping watcher when not running
#[tokio::test]
async fn test_stop_watcher_when_not_running() -> Result<()> {
    let watcher_service = FileWatcherService::new();
    
    // Should not error when stopping a watcher that's not running
    let result = watcher_service.stop_watching().await;
    assert!(result.is_ok());
    assert!(!watcher_service.is_watching());

    Ok(())
}

/// Test watcher status reporting
#[tokio::test]
async fn test_watcher_status_reporting() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let watcher_service = FileWatcherService::new();
    let (tx, _rx) = mpsc::unbounded_channel();

    // Initially not watching
    let status = watcher_service.get_status().await;
    assert_eq!(status, "Not watching");

    // Start watching
    watcher_service.start_watching(temp_dir.path(), tx).await?;
    
    let status = watcher_service.get_status().await;
    assert!(status.starts_with("Watching:"));
    assert!(status.contains(&temp_dir.path().display().to_string()));

    // Stop watching
    watcher_service.stop_watching().await?;
    
    let status = watcher_service.get_status().await;
    assert_eq!(status, "Not watching");

    Ok(())
}

/// Test event handler file filtering
#[tokio::test]
async fn test_event_handler_file_filtering() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let handler = FileChangeEventHandler::new();

    // Create various file types
    let py_file = temp_dir.path().join("script.py");
    let rs_file = temp_dir.path().join("main.rs");
    let txt_file = temp_dir.path().join("readme.txt");
    let no_ext_file = temp_dir.path().join("makefile");

    fs::write(&py_file, "print('hello')").await?;
    fs::write(&rs_file, "fn main() {}").await?;
    fs::write(&txt_file, "readme content").await?;
    fs::write(&no_ext_file, "makefile content").await?;

    // Test filtering
    assert!(handler.should_process_file(&py_file));
    assert!(!handler.should_process_file(&rs_file));
    assert!(!handler.should_process_file(&txt_file));
    assert!(!handler.should_process_file(&no_ext_file));

    Ok(())
}

/// Test event handler with custom extensions
#[tokio::test]
async fn test_event_handler_custom_extensions() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let mut handler = FileChangeEventHandler::with_extensions(vec![
        "rs".to_string(),
        "js".to_string(),
    ]);

    // Add another extension
    handler.add_extension("ts");

    // Create test files
    let rs_file = temp_dir.path().join("main.rs");
    let js_file = temp_dir.path().join("script.js");
    let ts_file = temp_dir.path().join("app.ts");
    let py_file = temp_dir.path().join("script.py");

    fs::write(&rs_file, "fn main() {}").await?;
    fs::write(&js_file, "console.log('hello')").await?;
    fs::write(&ts_file, "console.log('hello')").await?;
    fs::write(&py_file, "print('hello')").await?;

    // Test filtering with custom extensions
    assert!(handler.should_process_file(&rs_file));
    assert!(handler.should_process_file(&js_file));
    assert!(handler.should_process_file(&ts_file));
    assert!(!handler.should_process_file(&py_file)); // Not in custom list

    Ok(())
}