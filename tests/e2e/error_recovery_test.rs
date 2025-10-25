use anyhow::Result;
use learning_programming_app::core::ApplicationService;
use std::path::Path;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::{sleep, Duration};

/// Test error recovery and continuation after various types of errors
#[tokio::test]
async fn test_error_recovery_and_continuation() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Test sequence: success -> error -> success to verify continuation
    let test_sequence = vec![
        ("success1.py", "print('First success')", true),
        ("syntax_error.py", "print('Missing quote", false),
        ("runtime_error.py", "x = 1 / 0  # Division by zero", false),
        ("success2.py", "print('Second success')", true),
        ("import_error.py", "import nonexistent_module", false),
        ("success3.py", "print('Third success')", true),
    ];
    
    let mut execution_results = Vec::new();
    
    // Execute files in sequence
    for (filename, content, expected_success) in test_sequence {
        let file_path = section_dir.join(filename);
        fs::write(&file_path, content).await?;
        
        let result = app_service.executor.execute_file(&file_path).await;
        execution_results.push((filename, result.success, expected_success));
        
        // Verify the execution result matches expectation
        assert_eq!(result.success, expected_success, 
                  "Execution of {} should have success={}", filename, expected_success);
        
        // Small delay to ensure proper sequencing
        sleep(Duration::from_millis(10)).await;
    }
    
    // Verify all executions were attempted (no early termination)
    assert_eq!(execution_results.len(), 6, "All files should have been executed");
    
    // Verify history contains all attempts
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), 6, "History should contain all execution attempts");
    
    // Verify statistics reflect mixed results
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 6, "Total executions should be 6");
    assert_eq!(stats.successful_executions, 3, "Should have 3 successful executions");
    assert_eq!(stats.failed_executions, 3, "Should have 3 failed executions");
    assert_eq!(stats.success_rate(), 0.5, "Success rate should be 50%");
    
    Ok(())
}

/// Test file watcher continuation after errors
#[tokio::test]
async fn test_file_watcher_error_continuation() -> Result<()> {
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
    
    // Create files that will cause errors and successes
    let test_files = vec![
        ("good1.py", "print('Good 1')", true),
        ("bad1.py", "print('Bad syntax", false),
        ("good2.py", "print('Good 2')", true),
        ("bad2.py", "undefined_variable", false),
        ("good3.py", "print('Good 3')", true),
    ];
    
    let mut detected_changes = Vec::new();
    
    // Create files and collect change events
    for (filename, content, _) in &test_files {
        let file_path = section_dir.join(filename);
        fs::write(&file_path, content).await?;
        
        // Wait for file change detection
        let change_detected = tokio::time::timeout(Duration::from_secs(1), async {
            while let Some(changed_file) = rx.recv().await {
                if changed_file.file_name().and_then(|n| n.to_str()) == Some(filename) {
                    return true;
                }
            }
            false
        }).await;
        
        if change_detected.is_ok() && change_detected.unwrap() {
            detected_changes.push(filename);
        }
        
        sleep(Duration::from_millis(100)).await;
    }
    
    // Verify file watcher continued to detect changes even after errors
    assert!(detected_changes.len() >= 3, "File watcher should continue detecting changes after errors");
    
    // Execute all files to verify system continues working
    for (filename, content, expected_success) in test_files {
        let file_path = section_dir.join(filename);
        let result = app_service.executor.execute_file(&file_path).await;
        
        assert_eq!(result.success, expected_success, 
                  "Execution of {} should match expected result", filename);
    }
    
    // Verify watcher is still active
    assert!(app_service.watcher.is_watching(), "File watcher should still be active");
    
    // Stop watching
    app_service.watcher.stop_watching().await?;
    
    Ok(())
}

/// Test database error recovery
#[tokio::test]
async fn test_database_error_recovery() -> Result<()> {
    // Create temporary directory
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create a test file
    let test_file = section_dir.join("test.py");
    fs::write(&test_file, "print('Database test')").await?;
    
    // Execute file to ensure database is working
    let result1 = app_service.executor.execute_file(&test_file).await;
    assert!(result1.success, "First execution should be successful");
    
    // Verify database health
    assert!(app_service.history.is_healthy().await, "Database should be healthy");
    
    // Execute another file to verify continued operation
    fs::write(&test_file, "print('Database test 2')").await?;
    let result2 = app_service.executor.execute_file(&test_file).await;
    assert!(result2.success, "Second execution should be successful");
    
    // Verify history contains both executions
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), 2, "History should contain both executions");
    
    Ok(())
}

/// Test system recovery after multiple concurrent errors
#[tokio::test]
async fn test_concurrent_error_recovery() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create multiple files with errors
    let error_files = vec![
        ("error1.py", "print('Missing quote"),
        ("error2.py", "undefined_variable"),
        ("error3.py", "import nonexistent"),
        ("error4.py", "1 / 0"),
        ("error5.py", "syntax error here"),
    ];
    
    let mut tasks = Vec::new();
    
    // Execute all error files concurrently
    for (filename, content) in error_files {
        let file_path = section_dir.join(filename);
        fs::write(&file_path, content).await?;
        
        let executor = app_service.executor.clone();
        let task = tokio::spawn(async move {
            executor.execute_file(&file_path).await
        });
        tasks.push(task);
    }
    
    // Wait for all executions to complete
    let results = futures::future::join_all(tasks).await;
    
    // Verify all executions completed (even though they failed)
    for (i, result) in results.into_iter().enumerate() {
        let execution_result = result.expect("Task should complete");
        assert!(!execution_result.success, "Error execution {} should fail", i);
        assert!(execution_result.error_message.is_some(), "Error message should be present for execution {}", i);
    }
    
    // Verify system is still functional after concurrent errors
    let test_file = section_dir.join("recovery_test.py");
    fs::write(&test_file, "print('System recovered')").await?;
    
    let recovery_result = app_service.executor.execute_file(&test_file).await;
    assert!(recovery_result.success, "System should recover and execute successfully");
    
    // Verify all executions were recorded
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), 6, "History should contain all 6 executions (5 errors + 1 success)");
    
    // Verify system health
    let status = app_service.get_system_status().await;
    assert!(status.is_healthy(), "System should be healthy after error recovery");
    
    Ok(())
}

/// Test graceful handling of file system errors
#[tokio::test]
async fn test_file_system_error_handling() -> Result<()> {
    // Create temporary directory
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Test execution of non-existent file
    let nonexistent_file = temp_dir.path().join("nonexistent.py");
    let result = app_service.executor.execute_file(&nonexistent_file).await;
    
    // Verify error is handled gracefully
    assert!(!result.success, "Execution of non-existent file should fail");
    assert!(result.error_message.is_some(), "Error message should be present");
    
    // Verify system continues to work after file system error
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    let valid_file = section_dir.join("valid.py");
    fs::write(&valid_file, "print('File system recovery')").await?;
    
    let recovery_result = app_service.executor.execute_file(&valid_file).await;
    assert!(recovery_result.success, "System should recover from file system errors");
    
    Ok(())
}

/// Test memory and resource management under error conditions
#[tokio::test]
async fn test_resource_management_under_errors() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Execute many files with errors to test resource management
    for i in 0..20 {
        let file_path = section_dir.join(format!("error_{}.py", i));
        let content = format!("undefined_variable_{}", i);
        fs::write(&file_path, content).await?;
        
        let result = app_service.executor.execute_file(&file_path).await;
        assert!(!result.success, "Error execution {} should fail", i);
    }
    
    // Verify system is still responsive
    let test_file = section_dir.join("final_test.py");
    fs::write(&test_file, "print('Resource management test')").await?;
    
    let final_result = app_service.executor.execute_file(&test_file).await;
    assert!(final_result.success, "System should remain responsive after many errors");
    
    // Verify database is still healthy
    assert!(app_service.history.is_healthy().await, "Database should remain healthy");
    
    // Verify statistics are accurate
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, 21, "Should have 21 total executions");
    assert_eq!(stats.successful_executions, 1, "Should have 1 successful execution");
    assert_eq!(stats.failed_executions, 20, "Should have 20 failed executions");
    
    Ok(())
}

/// Test error handling during file watching startup
#[tokio::test]
async fn test_file_watcher_startup_error_handling() -> Result<()> {
    // Create temporary database
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Try to watch a non-existent directory
    let nonexistent_dir = temp_dir.path().join("nonexistent");
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    
    let watch_result = app_service.watcher.start_watching(&nonexistent_dir, tx).await;
    
    // Verify error is handled gracefully
    assert!(watch_result.is_err(), "Watching non-existent directory should fail");
    
    // Verify watcher is not in an inconsistent state
    assert!(!app_service.watcher.is_watching(), "Watcher should not be active after error");
    
    // Verify system can still start watching a valid directory
    let valid_dir = temp_dir.path().join("valid_section");
    fs::create_dir(&valid_dir).await?;
    
    let (tx2, _rx2) = tokio::sync::mpsc::unbounded_channel();
    let valid_watch_result = app_service.watcher.start_watching(&valid_dir, tx2).await;
    
    assert!(valid_watch_result.is_ok(), "Should be able to watch valid directory after error");
    assert!(app_service.watcher.is_watching(), "Watcher should be active for valid directory");
    
    // Clean up
    app_service.watcher.stop_watching().await?;
    
    Ok(())
}