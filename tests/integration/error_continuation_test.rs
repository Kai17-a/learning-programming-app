use learning_programming_app::core::executor::AutoExecutorService;
use learning_programming_app::core::watcher::FileWatcherService;
use learning_programming_app::core::event_handler::FileChangeEventHandler;
use learning_programming_app::handlers::service::LanguageHandlerService;
use learning_programming_app::handlers::python::PythonHandler;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::sync::mpsc;
use tokio::time::timeout;

#[tokio::test]
async fn test_executor_continues_after_syntax_error() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    handler_service.register_handler("py", Arc::new(PythonHandler::new())).await;
    
    let executor = AutoExecutorService::new(handler_service);
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file with syntax error
    let syntax_error_file = temp_dir.path().join("syntax_error.py");
    tokio::fs::write(&syntax_error_file, "print('hello'").await.unwrap(); // Missing closing parenthesis
    
    // Execute the file with syntax error
    let result1 = executor.execute_file(&syntax_error_file).await;
    assert!(!result1.success);
    
    // Create a valid file
    let valid_file = temp_dir.path().join("valid.py");
    tokio::fs::write(&valid_file, "print('hello world')").await.unwrap();
    
    // Execute the valid file - should work despite previous error
    let result2 = executor.execute_file(&valid_file).await;
    assert!(result2.success);
}

#[tokio::test]
async fn test_executor_continues_after_runtime_error() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    handler_service.register_handler("py", Arc::new(PythonHandler::new())).await;
    
    let executor = AutoExecutorService::new(handler_service);
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file with runtime error
    let runtime_error_file = temp_dir.path().join("runtime_error.py");
    tokio::fs::write(&runtime_error_file, "print(undefined_variable)").await.unwrap();
    
    // Execute the file with runtime error
    let result1 = executor.execute_file(&runtime_error_file).await;
    assert!(!result1.success);
    
    // Create a valid file
    let valid_file = temp_dir.path().join("valid.py");
    tokio::fs::write(&valid_file, "print('success')").await.unwrap();
    
    // Execute the valid file - should work despite previous error
    let result2 = executor.execute_file(&valid_file).await;
    assert!(result2.success);
}

#[tokio::test]
async fn test_executor_handles_missing_file() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    handler_service.register_handler("py", Arc::new(PythonHandler::new())).await;
    
    let executor = AutoExecutorService::new(handler_service);
    let temp_dir = TempDir::new().unwrap();
    
    // Try to execute non-existent file
    let missing_file = temp_dir.path().join("missing.py");
    let result1 = executor.execute_file(&missing_file).await;
    assert!(!result1.success);
    
    // Create and execute a valid file - should work despite previous error
    let valid_file = temp_dir.path().join("valid.py");
    tokio::fs::write(&valid_file, "print('file exists')").await.unwrap();
    
    let result2 = executor.execute_file(&valid_file).await;
    assert!(result2.success);
}

#[tokio::test]
async fn test_executor_handles_unsupported_extension() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    handler_service.register_handler("py", Arc::new(PythonHandler::new())).await;
    
    let executor = AutoExecutorService::new(handler_service);
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file with unsupported extension
    let unsupported_file = temp_dir.path().join("test.txt");
    tokio::fs::write(&unsupported_file, "Hello world").await.unwrap();
    
    // Try to execute unsupported file
    let result1 = executor.execute_file(&unsupported_file).await;
    assert!(!result1.success);
    
    // Create and execute a supported file - should work despite previous error
    let supported_file = temp_dir.path().join("test.py");
    tokio::fs::write(&supported_file, "print('supported')").await.unwrap();
    
    let result2 = executor.execute_file(&supported_file).await;
    assert!(result2.success);
}

#[tokio::test]
async fn test_watcher_continues_after_error() {
    let watcher = FileWatcherService::new();
    let temp_dir = TempDir::new().unwrap();
    
    // Create a valid directory
    let valid_dir = temp_dir.path().join("valid");
    tokio::fs::create_dir(&valid_dir).await.unwrap();
    
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // Start watching valid directory
    let result1 = watcher.start_watching(&valid_dir, tx.clone()).await;
    assert!(result1.is_ok());
    assert!(watcher.is_watching());
    
    // Try to watch non-existent directory (should fail but not crash)
    let invalid_dir = temp_dir.path().join("nonexistent");
    let result2 = watcher.start_watching(&invalid_dir, tx.clone()).await;
    assert!(result2.is_err());
    
    // Should still be able to watch valid directories
    let another_valid_dir = temp_dir.path().join("another_valid");
    tokio::fs::create_dir(&another_valid_dir).await.unwrap();
    
    let result3 = watcher.start_watching(&another_valid_dir, tx).await;
    assert!(result3.is_ok());
    assert!(watcher.is_watching());
}

#[tokio::test]
async fn test_event_handler_continues_after_callback_error() {
    let mut event_handler = FileChangeEventHandler::new();
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    let file1 = temp_dir.path().join("test1.py");
    let file2 = temp_dir.path().join("test2.py");
    tokio::fs::write(&file1, "print('test1')").await.unwrap();
    tokio::fs::write(&file2, "print('test2')").await.unwrap();
    
    let (tx, rx) = mpsc::unbounded_channel();
    event_handler.set_receiver(rx);
    
    // Set up a callback that fails on the first file but succeeds on others
    let mut call_count = 0;
    event_handler.set_callback(move |path| {
        call_count += 1;
        if call_count == 1 {
            // Fail on first call
            Err(anyhow::anyhow!("Simulated callback error"))
        } else {
            // Succeed on subsequent calls
            Ok(())
        }
    });
    
    // Send file change events
    tx.send(file1.clone()).unwrap();
    tx.send(file2.clone()).unwrap();
    drop(tx); // Close channel to stop processing
    
    // Process events - should handle the error and continue
    let result = event_handler.start_processing().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_timeout_handling() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    handler_service.register_handler("py", Arc::new(PythonHandler::new())).await;
    
    let executor = AutoExecutorService::new(handler_service);
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file that would run for a long time (simulated by short timeout)
    let long_running_file = temp_dir.path().join("long_running.py");
    tokio::fs::write(&long_running_file, "import time; time.sleep(10)").await.unwrap();
    
    // Execute with very short timeout
    let result = executor.execute_file_with_timeout(&long_running_file, 1).await;
    assert!(!result.success);
    assert!(result.error_message.as_ref().unwrap().contains("timeout"));
    
    // Should still be able to execute other files
    let quick_file = temp_dir.path().join("quick.py");
    tokio::fs::write(&quick_file, "print('quick')").await.unwrap();
    
    let result2 = executor.execute_file(&quick_file).await;
    assert!(result2.success);
}

#[tokio::test]
async fn test_multiple_consecutive_errors() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    handler_service.register_handler("py", Arc::new(PythonHandler::new())).await;
    
    let executor = AutoExecutorService::new(handler_service);
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple files with different types of errors
    let syntax_error_file = temp_dir.path().join("syntax.py");
    tokio::fs::write(&syntax_error_file, "print('hello'").await.unwrap();
    
    let runtime_error_file = temp_dir.path().join("runtime.py");
    tokio::fs::write(&runtime_error_file, "print(undefined)").await.unwrap();
    
    let missing_file = temp_dir.path().join("missing.py");
    
    // Execute all error files
    let result1 = executor.execute_file(&syntax_error_file).await;
    assert!(!result1.success);
    
    let result2 = executor.execute_file(&runtime_error_file).await;
    assert!(!result2.success);
    
    let result3 = executor.execute_file(&missing_file).await;
    assert!(!result3.success);
    
    // Should still be able to execute a valid file
    let valid_file = temp_dir.path().join("valid.py");
    tokio::fs::write(&valid_file, "print('still working')").await.unwrap();
    
    let result4 = executor.execute_file(&valid_file).await;
    assert!(result4.success);
}

#[tokio::test]
async fn test_watcher_recovery() {
    let watcher = FileWatcherService::new();
    let temp_dir = TempDir::new().unwrap();
    
    // Create a valid directory
    let valid_dir = temp_dir.path().join("valid");
    tokio::fs::create_dir(&valid_dir).await.unwrap();
    
    let (tx, _rx) = mpsc::unbounded_channel();
    
    // Start watching
    watcher.start_watching(&valid_dir, tx.clone()).await.unwrap();
    assert!(watcher.is_watching());
    
    // Simulate an error by stopping the watcher
    watcher.stop_watching().await.unwrap();
    assert!(!watcher.is_watching());
    
    // Should be able to recover by starting again
    let result = watcher.start_watching(&valid_dir, tx).await;
    assert!(result.is_ok());
    assert!(watcher.is_watching());
}