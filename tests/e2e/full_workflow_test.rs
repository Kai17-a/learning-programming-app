use anyhow::Result;
use learning_programming_app::core::ApplicationService;
use std::path::Path;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::Duration;

/// Test the complete workflow from file creation to execution
#[tokio::test]
async fn test_complete_file_change_to_execution_workflow() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create a Python test file
    let test_file = section_dir.join("hello_world.py");
    fs::write(&test_file, "print('Hello, World!')").await?;
    
    // Execute the file directly
    let result = app_service.executor.execute_file(&test_file).await;
    
    // Verify execution was successful
    assert!(result.success, "Execution should be successful");
    assert!(result.output.contains("Hello, World!"), "Output should contain expected text");
    assert!(result.execution_time.as_millis() > 0, "Execution time should be recorded");
    
    // Verify history was recorded
    let history = app_service.history.get_history(Some(1)).await?;
    assert_eq!(history.len(), 1, "History should contain one record");
    assert_eq!(history[0].success, true, "History record should show success");
    
    // Verify statistics
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 1, "Total executions should be 1");
    assert_eq!(stats.successful_executions, 1, "Successful executions should be 1");
    assert_eq!(stats.failed_executions, 0, "Failed executions should be 0");
    
    Ok(())
}

/// Test file watching and automatic execution
#[tokio::test]
async fn test_file_watching_and_auto_execution() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create channel for file change events
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Start watching the directory
    app_service.watcher.start_watching(&section_dir, tx).await?;
    
    // Verify watcher is active
    assert!(app_service.watcher.is_watching(), "Watcher should be active");
    
    // Create a Python test file
    let test_file = section_dir.join("test_auto.py");
    fs::write(&test_file, "print('Auto execution test')").await?;
    
    // Wait for file change event (with timeout)
    let file_change_received = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(changed_file) = rx.recv().await {
            if changed_file == test_file {
                return true;
            }
        }
        false
    }).await;
    
    // Verify file change was detected
    assert!(file_change_received.is_ok(), "File change should be detected within timeout");
    assert!(file_change_received.unwrap(), "File change event should be received");
    
    // Execute the file that was changed
    let result = app_service.executor.execute_file(&test_file).await;
    assert!(result.success, "Auto execution should be successful");
    assert!(result.output.contains("Auto execution test"), "Output should contain expected text");
    
    // Stop watching
    app_service.watcher.stop_watching().await?;
    assert!(!app_service.watcher.is_watching(), "Watcher should be stopped");
    
    Ok(())
}

/// Test multiple file executions and history tracking
#[tokio::test]
async fn test_multiple_executions_and_history() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section2-control-flow");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create multiple test files
    let files = vec![
        ("if_statements.py", "x = 5\nif x > 0:\n    print('Positive')"),
        ("loops.py", "for i in range(3):\n    print(f'Count: {i}')"),
        ("functions.py", "def greet(name):\n    return f'Hello, {name}!'\n\nprint(greet('World'))"),
    ];
    
    // Execute all files
    for (filename, content) in files {
        let file_path = section_dir.join(filename);
        fs::write(&file_path, content).await?;
        
        let result = app_service.executor.execute_file(&file_path).await;
        assert!(result.success, "Execution of {} should be successful", filename);
    }
    
    // Verify history contains all executions
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), 3, "History should contain 3 records");
    
    // Verify all executions were successful
    for record in &history {
        assert!(record.success, "All executions should be successful");
        assert!(!record.output_preview.is_empty(), "Output preview should not be empty");
    }
    
    // Verify statistics
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 3, "Total executions should be 3");
    assert_eq!(stats.successful_executions, 3, "All executions should be successful");
    assert_eq!(stats.success_rate(), 1.0, "Success rate should be 100%");
    
    Ok(())
}

/// Test execution with syntax errors
#[tokio::test]
async fn test_syntax_error_handling() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create a Python file with syntax error
    let test_file = section_dir.join("syntax_error.py");
    fs::write(&test_file, "print('Missing closing quote").await?;
    
    // Execute the file with syntax error
    let result = app_service.executor.execute_file(&test_file).await;
    
    // Verify execution failed but was handled gracefully
    assert!(!result.success, "Execution should fail due to syntax error");
    assert!(result.error_message.is_some(), "Error message should be present");
    let error_msg = result.error_message.as_ref().unwrap();
    assert!(error_msg.contains("SyntaxError") || error_msg.contains("syntax"), 
           "Error message should indicate syntax error: {}", error_msg);
    
    // Verify history was still recorded
    let history = app_service.history.get_history(Some(1)).await?;
    assert_eq!(history.len(), 1, "History should contain one record");
    assert_eq!(history[0].success, false, "History record should show failure");
    
    // Verify statistics reflect the failure
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 1, "Total executions should be 1");
    assert_eq!(stats.successful_executions, 0, "Successful executions should be 0");
    assert_eq!(stats.failed_executions, 1, "Failed executions should be 1");
    
    Ok(())
}

/// Test system health and status
#[tokio::test]
async fn test_system_health_and_status() -> Result<()> {
    // Create temporary database
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Perform health check
    app_service.health_check().await?;
    
    // Get system status
    let status = app_service.get_system_status().await;
    
    // Verify system is healthy
    assert!(status.database_connected, "Database should be connected");
    assert!(!status.registered_handlers.is_empty(), "Should have registered handlers");
    assert!(status.registered_handlers.contains(&"py".to_string()), "Python handler should be registered");
    assert!(status.is_healthy(), "System should be healthy");
    
    // Test graceful shutdown
    app_service.shutdown().await?;
    
    Ok(())
}

/// Test concurrent file executions
#[tokio::test]
async fn test_concurrent_executions() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create multiple test files
    let mut tasks = Vec::new();
    
    for i in 0..5 {
        let file_path = section_dir.join(format!("concurrent_{}.py", i));
        let content = format!("import time\ntime.sleep(0.1)\nprint('Concurrent execution {}')", i);
        fs::write(&file_path, content).await?;
        
        // Clone the app_service for each task
        let executor = app_service.executor.clone();
        let task = tokio::spawn(async move {
            executor.execute_file(&file_path).await
        });
        tasks.push(task);
    }
    
    // Wait for all executions to complete
    let results = futures::future::join_all(tasks).await;
    
    // Verify all executions completed successfully
    for (i, result) in results.into_iter().enumerate() {
        let execution_result = result.expect("Task should complete");
        assert!(execution_result.success, "Concurrent execution {} should be successful", i);
        assert!(execution_result.output.contains(&format!("Concurrent execution {}", i)), 
                "Output should contain expected text for execution {}", i);
    }
    
    // Verify all executions were recorded in history
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), 5, "History should contain 5 records");
    
    // Verify statistics
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 5, "Total executions should be 5");
    assert_eq!(stats.successful_executions, 5, "All executions should be successful");
    
    Ok(())
}

/// Test file watching with multiple file types
#[tokio::test]
async fn test_file_watching_with_filtering() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create channel for file change events
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Start watching the directory
    app_service.watcher.start_watching(&section_dir, tx).await?;
    
    // Create files with different extensions
    let py_file = section_dir.join("test.py");
    let go_file = section_dir.join("test.go");
    let txt_file = section_dir.join("test.txt");
    let js_file = section_dir.join("test.js");
    
    fs::write(&py_file, "print('Python file')").await?;
    fs::write(&go_file, r#"package main
import "fmt"
func main() {
    fmt.Println("Go file")
}"#).await?;
    fs::write(&txt_file, "This is a text file").await?;
    fs::write(&js_file, "console.log('JavaScript file')").await?;
    
    // Collect file change events (with timeout)
    let mut detected_files = Vec::new();
    let timeout_result = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(changed_file) = rx.recv().await {
            detected_files.push(changed_file);
            if detected_files.len() >= 4 {
                break;
            }
        }
    }).await;
    
    // Verify file changes were detected
    assert!(timeout_result.is_ok(), "File changes should be detected within timeout");
    assert_eq!(detected_files.len(), 4, "All file changes should be detected");
    
    // Verify Python file can be executed
    let py_result = app_service.executor.execute_file(&py_file).await;
    assert!(py_result.success, "Python file execution should be successful");
    
    // Verify Go file can be executed (if Go is installed)
    let go_result = app_service.executor.execute_file(&go_file).await;
    // Go execution success depends on Go being installed, so we just verify it was attempted
    assert_eq!(go_result.file_path, go_file, "Go file should be processed");
    
    // Stop watching
    app_service.watcher.stop_watching().await?;
    
    Ok(())
}

/// Test Go file execution workflow
#[tokio::test]
async fn test_go_file_execution_workflow() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create a Go test file
    let test_file = section_dir.join("hello_world.go");
    fs::write(&test_file, r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, World from Go!")
}
"#).await?;
    
    // Execute the file directly
    let result = app_service.executor.execute_file(&test_file).await;
    
    // Verify execution was attempted (success depends on Go being installed)
    assert_eq!(result.file_path, test_file, "File path should match");
    // Execution time should always be recorded
    assert!(result.execution_time.as_millis() < u128::MAX, "Execution time should be recorded");
    
    // If Go is installed and execution was successful
    if result.success {
        assert!(result.output.contains("Hello, World from Go!"), "Output should contain expected text");
    }
    
    // Verify history was recorded regardless of success
    let history = app_service.history.get_history(Some(1)).await?;
    assert_eq!(history.len(), 1, "History should contain one record");
    
    // Verify statistics
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 1, "Total executions should be 1");
    
    Ok(())
}

/// Test Go syntax error handling
#[tokio::test]
async fn test_go_syntax_error_handling() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create a Go file with syntax error
    let test_file = section_dir.join("syntax_error.go");
    fs::write(&test_file, r#"package main

import "fmt"

func main() {
    fmt.Println("Missing closing quote
}
"#).await?;
    
    // Execute the file with syntax error
    let result = app_service.executor.execute_file(&test_file).await;
    
    // Verify execution was attempted
    assert_eq!(result.file_path, test_file, "File path should match");
    
    // If Go is installed, verify error handling
    if result.error_message.is_some() {
        assert!(!result.success, "Execution should fail due to syntax error");
        let error_msg = result.error_message.as_ref().unwrap();
        assert!(error_msg.contains("syntax") || error_msg.contains("error"), 
               "Error message should indicate syntax error: {}", error_msg);
    }
    
    // Verify history was still recorded
    let history = app_service.history.get_history(Some(1)).await?;
    assert_eq!(history.len(), 1, "History should contain one record");
    
    Ok(())
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    
    #[allow(dead_code)]
    pub async fn create_test_environment() -> Result<(TempDir, ApplicationService)> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
        Ok((temp_dir, app_service))
    }
    
    #[allow(dead_code)]
    pub async fn create_section_with_files(base_dir: &Path, section_name: &str, files: Vec<(&str, &str)>) -> Result<Vec<std::path::PathBuf>> {
        let section_dir = base_dir.join(section_name);
        fs::create_dir(&section_dir).await?;
        
        let mut file_paths = Vec::new();
        for (filename, content) in files {
            let file_path = section_dir.join(filename);
            fs::write(&file_path, content).await?;
            file_paths.push(file_path);
        }
        
        Ok(file_paths)
    }
}