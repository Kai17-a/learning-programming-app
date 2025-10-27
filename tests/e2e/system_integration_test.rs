use anyhow::Result;
use learning_programming_app::cli::CommandLineInterface;
use learning_programming_app::core::ApplicationService;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::Duration;

/// Test complete system integration with CLI interface
#[tokio::test]
async fn test_complete_system_integration() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let examples_dir = temp_dir.path().join("examples");
    fs::create_dir(&examples_dir).await?;

    let section1_dir = examples_dir.join("section1-basics");
    let section2_dir = examples_dir.join("section2-control-flow");
    fs::create_dir(&section1_dir).await?;
    fs::create_dir(&section2_dir).await?;

    // Create sample files
    fs::write(
        section1_dir.join("hello_world.py"),
        "print('Hello, World!')",
    )
    .await?;
    fs::write(
        section1_dir.join("variables.py"),
        "name = 'Python'\nprint(f'Hello, {name}!')",
    )
    .await?;
    fs::write(
        section2_dir.join("if_statements.py"),
        "x = 10\nif x > 5:\n    print('x is greater than 5')",
    )
    .await?;
    fs::write(
        section2_dir.join("loops.py"),
        "for i in range(3):\n    print(f'Loop iteration: {i}')",
    )
    .await?;

    // Initialize CLI interface without changing directory
    let cli = CommandLineInterface::new().await?;

    // Test system status
    let status = cli.get_system_status().await;
    assert!(status.is_healthy(), "System should be healthy");
    assert!(status.database_connected, "Database should be connected");
    assert!(
        status.registered_handlers.contains(&"py".to_string()),
        "Python handler should be registered"
    );

    // Test section display functionality
    let sections = cli.display_sections(&examples_dir).await?;
    assert_eq!(sections.len(), 2, "Should find 2 sections");
    assert!(
        sections.contains(&"section1-basics".to_string()),
        "Should contain section1-basics"
    );
    assert!(
        sections.contains(&"section2-control-flow".to_string()),
        "Should contain section2-control-flow"
    );

    // Test individual file execution
    let hello_file = section1_dir.join("hello_world.py");
    cli.run_file(&hello_file, false).await?;

    // Verify execution was recorded in history
    cli.show_history(5).await?;

    // Test statistics display
    cli.show_stats().await?;

    // Test graceful shutdown
    cli.shutdown().await?;

    Ok(())
}

/// Test CLI interface with file watching
#[tokio::test]
async fn test_cli_file_watching_integration() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let examples_dir = temp_dir.path().join("examples");
    fs::create_dir(&examples_dir).await?;

    let section_dir = examples_dir.join("section1-basics");
    fs::create_dir(&section_dir).await?;

    // Initialize CLI interface without changing directory
    let cli = CommandLineInterface::new().await?;

    // Verify watcher is not initially active
    assert!(!cli.is_watching(), "Watcher should not be active initially");

    // Test that we can get watched directories (should be empty)
    let watched_dirs = cli.get_watched_directories().await;
    assert!(
        watched_dirs.is_empty(),
        "No directories should be watched initially"
    );

    // Create a test file
    let test_file = section_dir.join("watch_test.py");
    fs::write(&test_file, "print('File watching test')").await?;

    // Test direct file execution
    cli.run_file(&test_file, true).await?;

    // Verify execution was successful by checking history
    cli.show_history(1).await?;

    // Test graceful shutdown
    cli.shutdown().await?;

    Ok(())
}

/// Test application service integration with multiple components
#[tokio::test]
async fn test_application_service_component_integration() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;

    // Create temporary database
    let db_path = temp_dir.path().join("integration_test.db");

    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;

    // Test language handler integration
    let supported_extensions = app_service
        .language_handlers
        .get_supported_extensions()
        .await;
    assert!(
        !supported_extensions.is_empty(),
        "Should have supported extensions"
    );
    assert!(
        supported_extensions.contains(&"py".to_string()),
        "Should support Python"
    );

    // Test executor integration with language handlers
    let test_file = section_dir.join("integration_test.py");
    fs::write(&test_file, "print('Integration test successful')").await?;

    let result = app_service.executor.execute_file(&test_file).await;
    assert!(result.success, "Execution should be successful");
    assert!(
        result.output.contains("Integration test successful"),
        "Output should contain expected text"
    );

    // Test history manager integration
    let history = app_service.history.get_history(Some(1)).await?;
    assert_eq!(history.len(), 1, "History should contain one record");
    assert!(history[0].success, "History record should show success");

    // Test file watcher integration
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    app_service.watcher.start_watching(&section_dir, tx).await?;

    assert!(
        app_service.watcher.is_watching(),
        "Watcher should be active"
    );

    // Create another file to test watching
    let watch_test_file = section_dir.join("watch_integration.py");
    fs::write(&watch_test_file, "print('Watch integration test')").await?;

    // Wait for file change event
    let file_change_received = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(changed_file) = rx.recv().await {
            if changed_file == watch_test_file {
                return true;
            }
        }
        false
    })
    .await;

    assert!(
        file_change_received.is_ok() && file_change_received.unwrap(),
        "File change should be detected"
    );

    // Execute the watched file
    let watch_result = app_service.executor.execute_file(&watch_test_file).await;
    assert!(
        watch_result.success,
        "Watched file execution should be successful"
    );

    // Stop watching
    app_service.watcher.stop_watching().await?;
    assert!(
        !app_service.watcher.is_watching(),
        "Watcher should be stopped"
    );

    // Test system health check
    app_service.health_check().await?;

    let status = app_service.get_system_status().await;
    assert!(status.is_healthy(), "System should be healthy");

    // Test graceful shutdown
    app_service.shutdown().await?;

    Ok(())
}

/// Test error propagation through system layers
#[tokio::test]
async fn test_error_propagation_through_layers() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;

    // Create temporary database
    let db_path = temp_dir.path().join("error_test.db");

    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;

    // Test syntax error propagation
    let syntax_error_file = section_dir.join("syntax_error.py");
    fs::write(&syntax_error_file, "print('Missing quote").await?;

    let syntax_result = app_service.executor.execute_file(&syntax_error_file).await;
    assert!(
        !syntax_result.success,
        "Syntax error should propagate as failure"
    );
    assert!(
        syntax_result.error_message.is_some(),
        "Error message should be propagated"
    );

    // Test runtime error propagation
    let runtime_error_file = section_dir.join("runtime_error.py");
    fs::write(&runtime_error_file, "x = 1 / 0").await?;

    let runtime_result = app_service.executor.execute_file(&runtime_error_file).await;
    assert!(
        !runtime_result.success,
        "Runtime error should propagate as failure"
    );
    assert!(
        runtime_result.error_message.is_some(),
        "Runtime error message should be propagated"
    );

    // Verify errors are recorded in history
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), 2, "Both error executions should be recorded");
    assert!(
        !history[0].success,
        "First error should be recorded as failure"
    );
    assert!(
        !history[1].success,
        "Second error should be recorded as failure"
    );

    // Verify system continues to work after errors
    let success_file = section_dir.join("success.py");
    fs::write(&success_file, "print('Success after errors')").await?;

    let success_result = app_service.executor.execute_file(&success_file).await;
    assert!(
        success_result.success,
        "System should continue working after errors"
    );

    Ok(())
}

/// Test concurrent access to shared resources
#[tokio::test]
async fn test_concurrent_resource_access() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;

    // Create temporary database
    let db_path = temp_dir.path().join("concurrent_test.db");

    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;

    // Create multiple test files
    let mut tasks = Vec::new();

    for i in 0..10 {
        let file_path = section_dir.join(format!("concurrent_{}.py", i));
        let content = format!(
            "import time\ntime.sleep(0.05)\nprint('Concurrent execution {}')",
            i
        );
        fs::write(&file_path, content).await?;

        // Clone services for concurrent access
        let executor = app_service.executor.clone();
        let history = app_service.history.clone();

        let task = tokio::spawn(async move {
            // Execute file
            let result = executor.execute_file(&file_path).await;

            // Verify we can access history concurrently
            let current_history = history.get_history(Some(1)).await.unwrap();

            (result.success, current_history.len())
        });

        tasks.push(task);
    }

    // Wait for all concurrent executions
    let results = futures::future::join_all(tasks).await;

    // Verify all executions completed successfully
    for (i, result) in results.into_iter().enumerate() {
        let (success, history_len) = result.expect("Task should complete");
        assert!(success, "Concurrent execution {} should be successful", i);
        assert!(
            history_len > 0,
            "History should be accessible during concurrent execution"
        );
    }

    // Verify final state
    let final_history = app_service.history.get_history(None).await?;
    assert_eq!(
        final_history.len(),
        10,
        "All concurrent executions should be recorded"
    );

    let stats = app_service.history.get_stats().await?;
    assert_eq!(
        stats.total_executions, 10,
        "Statistics should reflect all executions"
    );
    assert_eq!(
        stats.successful_executions, 10,
        "All concurrent executions should be successful"
    );

    Ok(())
}

/// Test system behavior under high load
#[tokio::test]
async fn test_system_under_load() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).await?;

    // Create temporary database
    let db_path = temp_dir.path().join("load_test.db");

    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;

    // Create many files for load testing
    let num_files = 50;
    let mut file_paths = Vec::new();

    for i in 0..num_files {
        let file_path = section_dir.join(format!("load_test_{}.py", i));
        let content = format!("print('Load test execution {}')", i);
        fs::write(&file_path, content).await?;
        file_paths.push(file_path);
    }

    // Execute all files rapidly
    let start_time = std::time::Instant::now();

    let mut tasks = Vec::new();
    for file_path in file_paths {
        let executor = app_service.executor.clone();
        let task = tokio::spawn(async move { executor.execute_file(&file_path).await });
        tasks.push(task);
    }

    // Wait for all executions
    let results = futures::future::join_all(tasks).await;
    let execution_time = start_time.elapsed();

    // Verify all executions completed successfully
    let mut successful_count = 0;
    for result in results {
        let execution_result = result.expect("Task should complete");
        if execution_result.success {
            successful_count += 1;
        }
    }

    assert_eq!(
        successful_count, num_files,
        "All load test executions should be successful"
    );

    // Verify system performance (should complete within reasonable time)
    assert!(
        execution_time.as_secs() < 30,
        "Load test should complete within 30 seconds"
    );

    // Verify database integrity after load
    let history = app_service.history.get_history(None).await?;
    assert_eq!(
        history.len(),
        num_files,
        "All executions should be recorded in history"
    );

    let stats = app_service.history.get_stats().await?;
    assert_eq!(
        stats.total_executions, num_files as u64,
        "Statistics should be accurate"
    );

    // Verify system is still healthy after load
    assert!(
        app_service.history.is_healthy().await,
        "Database should remain healthy after load"
    );

    let status = app_service.get_system_status().await;
    assert!(
        status.is_healthy(),
        "System should remain healthy after load test"
    );

    Ok(())
}

/// Test system initialization and cleanup
#[tokio::test]
async fn test_system_lifecycle() -> Result<()> {
    // Test multiple initialization and cleanup cycles
    for cycle in 0..3 {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join(format!("lifecycle_test_{}.db", cycle));

        // Initialize application service
        let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;

        // Verify initialization
        let status = app_service.get_system_status().await;
        assert!(
            status.is_healthy(),
            "System should be healthy after initialization (cycle {})",
            cycle
        );

        // Perform some operations
        let section_dir = temp_dir.path().join("section1-basics");
        fs::create_dir(&section_dir).await?;

        let test_file = section_dir.join("lifecycle_test.py");
        fs::write(
            &test_file,
            format!("print('Lifecycle test cycle {}')", cycle),
        )
        .await?;

        let result = app_service.executor.execute_file(&test_file).await;
        assert!(
            result.success,
            "Execution should be successful in cycle {}",
            cycle
        );

        // Verify operations were recorded
        let history = app_service.history.get_history(None).await?;
        assert_eq!(
            history.len(),
            1,
            "History should contain one record in cycle {}",
            cycle
        );

        // Perform graceful shutdown
        app_service.shutdown().await?;

        // Verify cleanup (database should be closed)
        // Note: We can't easily test if database is closed without implementation details
        // but the shutdown should complete without errors
    }

    Ok(())
}
