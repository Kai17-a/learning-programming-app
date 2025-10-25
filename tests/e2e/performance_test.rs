use anyhow::Result;
use learning_programming_app::core::ApplicationService;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::fs;

/// Test performance with large number of files
#[tokio::test]
async fn test_large_file_monitoring_performance() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("performance-test");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("perf_test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create a large number of Python files
    let num_files = 100;
    let mut file_paths = Vec::new();
    
    let start_time = Instant::now();
    
    for i in 0..num_files {
        let file_path = section_dir.join(format!("perf_test_{}.py", i));
        let content = format!("# Performance test file {}\nprint('Performance test {}')", i, i);
        fs::write(&file_path, content).await?;
        file_paths.push(file_path);
    }
    
    let file_creation_time = start_time.elapsed();
    println!("Created {} files in {:?}", num_files, file_creation_time);
    
    // Test file watcher performance
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    let watch_start = Instant::now();
    app_service.watcher.start_watching(&section_dir, tx).await?;
    let watch_setup_time = watch_start.elapsed();
    
    println!("File watcher setup time: {:?}", watch_setup_time);
    assert!(watch_setup_time < Duration::from_secs(5), "File watcher setup should be fast");
    
    // Modify files and measure detection time
    let modify_start = Instant::now();
    let test_file = &file_paths[0];
    fs::write(test_file, "print('Modified for performance test')").await?;
    
    // Wait for change detection
    let change_detected = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(changed_file) = rx.recv().await {
            if changed_file == *test_file {
                return true;
            }
        }
        false
    }).await;
    
    let detection_time = modify_start.elapsed();
    println!("File change detection time: {:?}", detection_time);
    
    assert!(change_detected.is_ok() && change_detected.unwrap(), 
           "File change should be detected within timeout");
    assert!(detection_time < Duration::from_secs(2), 
           "File change detection should be fast");
    
    // Stop watching
    app_service.watcher.stop_watching().await?;
    
    Ok(())
}

/// Test execution performance with multiple files
#[tokio::test]
async fn test_execution_performance() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("execution-perf");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("exec_perf_test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create test files with varying complexity
    let test_files = vec![
        ("simple.py", "print('Hello, World!')"),
        ("loop.py", "for i in range(100):\n    print(f'Count: {i}')"),
        ("calculation.py", "result = sum(range(1000))\nprint(f'Sum: {result}')"),
        ("string_ops.py", "text = 'Performance test ' * 100\nprint(len(text))"),
        ("list_ops.py", "data = list(range(500))\nprint(f'Length: {len(data)}, Sum: {sum(data)}')"),
    ];
    
    let mut execution_times = Vec::new();
    
    for (filename, content) in test_files {
        let file_path = section_dir.join(filename);
        fs::write(&file_path, content).await?;
        
        // Measure execution time
        let exec_start = Instant::now();
        let result = app_service.executor.execute_file(&file_path).await;
        let exec_time = exec_start.elapsed();
        
        execution_times.push((filename, exec_time, result.success));
        
        println!("Executed {} in {:?} (success: {})", filename, exec_time, result.success);
        
        // Individual execution should complete within reasonable time
        assert!(exec_time < Duration::from_secs(10), 
               "Execution of {} should complete within 10 seconds", filename);
    }
    
    // Calculate average execution time
    let total_time: Duration = execution_times.iter().map(|(_, time, _)| *time).sum();
    let avg_time = total_time / execution_times.len() as u32;
    
    println!("Average execution time: {:?}", avg_time);
    assert!(avg_time < Duration::from_secs(3), "Average execution time should be reasonable");
    
    // Verify all executions were recorded in history
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), execution_times.len(), 
              "All executions should be recorded in history");
    
    Ok(())
}

/// Test concurrent execution performance
#[tokio::test]
async fn test_concurrent_execution_performance() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("concurrent-perf");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("concurrent_perf_test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create multiple test files
    let num_concurrent = 20;
    let mut tasks = Vec::new();
    
    let start_time = Instant::now();
    
    for i in 0..num_concurrent {
        let file_path = section_dir.join(format!("concurrent_{}.py", i));
        let content = format!("import time\ntime.sleep(0.01)\nprint('Concurrent execution {}')", i);
        fs::write(&file_path, content).await?;
        
        // Clone the executor for concurrent access
        let executor = app_service.executor.clone();
        let task = tokio::spawn(async move {
            let task_start = Instant::now();
            let result = executor.execute_file(&file_path).await;
            let task_time = task_start.elapsed();
            (result.success, task_time)
        });
        
        tasks.push(task);
    }
    
    // Wait for all concurrent executions
    let results = futures::future::join_all(tasks).await;
    let total_time = start_time.elapsed();
    
    println!("Completed {} concurrent executions in {:?}", num_concurrent, total_time);
    
    // Verify all executions completed successfully
    let mut successful_count = 0;
    let mut max_individual_time = Duration::default();
    
    for (i, result) in results.into_iter().enumerate() {
        let (success, individual_time) = result.expect("Task should complete");
        if success {
            successful_count += 1;
        }
        if individual_time > max_individual_time {
            max_individual_time = individual_time;
        }
        println!("Concurrent task {} completed in {:?} (success: {})", i, individual_time, success);
    }
    
    assert_eq!(successful_count, num_concurrent, "All concurrent executions should succeed");
    
    // Concurrent execution should be faster than sequential
    let estimated_sequential_time = max_individual_time * num_concurrent as u32;
    println!("Estimated sequential time: {:?}, Actual concurrent time: {:?}", 
             estimated_sequential_time, total_time);
    
    // Concurrent should be significantly faster (at least 50% improvement)
    assert!(total_time < estimated_sequential_time / 2, 
           "Concurrent execution should be significantly faster than sequential");
    
    // Verify all executions were recorded
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), num_concurrent, "All concurrent executions should be recorded");
    
    Ok(())
}

/// Test database performance under load
#[tokio::test]
async fn test_database_performance_under_load() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("db-perf");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("db_perf_test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Execute many files to generate database load
    let num_executions = 200;
    let file_path = section_dir.join("db_load_test.py");
    fs::write(&file_path, "print('Database load test')").await?;
    
    let db_start = Instant::now();
    
    // Execute the same file multiple times to test database performance
    for i in 0..num_executions {
        let result = app_service.executor.execute_file(&file_path).await;
        assert!(result.success, "Execution {} should be successful", i);
        
        // Periodically check database health
        if i % 50 == 0 {
            assert!(app_service.history.is_healthy().await, 
                   "Database should remain healthy during load test");
        }
    }
    
    let db_total_time = db_start.elapsed();
    println!("Completed {} database operations in {:?}", num_executions, db_total_time);
    
    // Calculate operations per second
    let ops_per_sec = num_executions as f64 / db_total_time.as_secs_f64();
    println!("Database operations per second: {:.2}", ops_per_sec);
    
    // Should handle at least 10 operations per second
    assert!(ops_per_sec >= 10.0, "Database should handle at least 10 operations per second");
    
    // Test database query performance
    let query_start = Instant::now();
    let history = app_service.history.get_history(None).await?;
    let query_time = query_start.elapsed();
    
    println!("Retrieved {} history records in {:?}", history.len(), query_time);
    assert_eq!(history.len(), num_executions, "All executions should be in history");
    assert!(query_time < Duration::from_secs(2), "History query should be fast");
    
    // Test statistics calculation performance
    let stats_start = Instant::now();
    let stats = app_service.history.get_stats().await?;
    let stats_time = stats_start.elapsed();
    
    println!("Calculated statistics in {:?}", stats_time);
    assert_eq!(stats.total_executions, num_executions as u64, "Statistics should be accurate");
    assert!(stats_time < Duration::from_secs(1), "Statistics calculation should be fast");
    
    Ok(())
}

/// Test memory usage and resource management
#[tokio::test]
async fn test_memory_and_resource_management() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("memory-test");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("memory_test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create files with varying output sizes
    let test_cases = vec![
        ("small_output.py", "print('Small')"),
        ("medium_output.py", "print('Medium output: ' + 'x' * 1000)"),
        ("large_output.py", "print('Large output: ' + 'x' * 10000)"),
        ("multiple_lines.py", "for i in range(100):\n    print(f'Line {i}: ' + 'data' * 10)"),
    ];
    
    let resource_start = Instant::now();
    
    // Execute files multiple times to test memory management
    for round in 0..10 {
        for (filename, content) in &test_cases {
            let file_path = section_dir.join(format!("{}_{}.py", round, filename));
            fs::write(&file_path, content).await?;
            
            let result = app_service.executor.execute_file(&file_path).await;
            assert!(result.success, "Execution should be successful");
            
            // Verify output is captured but not causing memory issues
            assert!(!result.output.is_empty() || result.error_message.is_some(), 
                   "Should have output or error message");
        }
    }
    
    let resource_time = resource_start.elapsed();
    println!("Completed resource management test in {:?}", resource_time);
    
    // Verify system is still responsive
    let final_test_file = section_dir.join("final_test.py");
    fs::write(&final_test_file, "print('System still responsive')").await?;
    
    let final_start = Instant::now();
    let final_result = app_service.executor.execute_file(&final_test_file).await;
    let final_time = final_start.elapsed();
    
    assert!(final_result.success, "System should still be responsive");
    assert!(final_time < Duration::from_secs(5), "Final execution should be fast");
    
    // Verify database is still healthy
    assert!(app_service.history.is_healthy().await, "Database should remain healthy");
    
    // Check that history contains all executions
    let history = app_service.history.get_history(None).await?;
    let expected_count = test_cases.len() * 10 + 1; // 4 test cases * 10 rounds + 1 final test
    assert_eq!(history.len(), expected_count, "All executions should be recorded");
    
    Ok(())
}

/// Test system stability under continuous load
#[tokio::test]
async fn test_system_stability_under_load() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("stability-test");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("stability_test.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Create test files with different characteristics
    let test_files = vec![
        ("success.py", "print('Success')", true),
        ("syntax_error.py", "print('Missing quote", false),
        ("runtime_error.py", "undefined_variable", false),
        ("slow_execution.py", "import time\ntime.sleep(0.1)\nprint('Slow')", true),
    ];
    
    // Create all test files
    for (filename, content, _) in &test_files {
        let file_path = section_dir.join(filename);
        fs::write(&file_path, content).await?;
    }
    
    let stability_start = Instant::now();
    let test_duration = Duration::from_secs(10); // Run for 10 seconds
    let mut execution_count = 0;
    let mut success_count = 0;
    
    // Continuously execute files for the test duration
    while stability_start.elapsed() < test_duration {
        for (filename, _, expected_success) in &test_files {
            let file_path = section_dir.join(filename);
            let result = app_service.executor.execute_file(&file_path).await;
            
            execution_count += 1;
            if result.success {
                success_count += 1;
            }
            
            // Verify result matches expectation
            assert_eq!(result.success, *expected_success, 
                      "Execution result should be consistent for {}", filename);
            
            // Small delay to prevent overwhelming the system
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        // Periodically check system health
        if execution_count % 20 == 0 {
            assert!(app_service.history.is_healthy().await, 
                   "Database should remain healthy during stability test");
            
            let status = app_service.get_system_status().await;
            assert!(status.is_healthy(), "System should remain healthy");
        }
    }
    
    let total_time = stability_start.elapsed();
    println!("Stability test completed: {} executions in {:?}", execution_count, total_time);
    println!("Success rate: {:.2}%", (success_count as f64 / execution_count as f64) * 100.0);
    
    // Verify system maintained reasonable performance
    let avg_execution_time = total_time / execution_count as u32;
    println!("Average execution time during stability test: {:?}", avg_execution_time);
    
    assert!(avg_execution_time < Duration::from_millis(500), 
           "Average execution time should remain reasonable under load");
    
    // Verify all executions were recorded
    let history = app_service.history.get_history(None).await?;
    assert_eq!(history.len(), execution_count, "All executions should be recorded");
    
    // Verify statistics are accurate
    let stats = app_service.history.get_stats().await?;
    assert_eq!(stats.total_executions, execution_count as u64, "Statistics should be accurate");
    assert_eq!(stats.successful_executions, success_count as u64, "Success count should be accurate");
    
    Ok(())
}

/// Benchmark file watcher responsiveness
#[tokio::test]
async fn test_file_watcher_responsiveness() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let section_dir = temp_dir.path().join("watcher-benchmark");
    fs::create_dir(&section_dir).await?;
    
    // Create temporary database
    let db_path = temp_dir.path().join("watcher_bench.db");
    
    // Initialize application service
    let app_service = ApplicationService::new(db_path.to_str().unwrap()).await?;
    
    // Set up file watcher
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    app_service.watcher.start_watching(&section_dir, tx).await?;
    
    // Test multiple rapid file changes
    let num_changes = 50;
    let mut detection_times = Vec::new();
    
    for i in 0..num_changes {
        let file_path = section_dir.join(format!("rapid_change_{}.py", i));
        
        let change_start = Instant::now();
        fs::write(&file_path, format!("print('Rapid change {}')", i)).await?;
        
        // Wait for change detection
        let detected = tokio::time::timeout(Duration::from_secs(2), async {
            while let Some(changed_file) = rx.recv().await {
                if changed_file == file_path {
                    return true;
                }
            }
            false
        }).await;
        
        let detection_time = change_start.elapsed();
        
        assert!(detected.is_ok() && detected.unwrap(), 
               "File change {} should be detected", i);
        
        detection_times.push(detection_time);
        
        // Small delay between changes
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    
    // Calculate detection statistics
    let total_detection_time: Duration = detection_times.iter().sum();
    let avg_detection_time = total_detection_time / detection_times.len() as u32;
    let max_detection_time = detection_times.iter().max().unwrap();
    let min_detection_time = detection_times.iter().min().unwrap();
    
    println!("File watcher responsiveness results:");
    println!("  Average detection time: {:?}", avg_detection_time);
    println!("  Max detection time: {:?}", max_detection_time);
    println!("  Min detection time: {:?}", min_detection_time);
    
    // Performance assertions
    assert!(avg_detection_time < Duration::from_millis(500), 
           "Average detection time should be under 500ms");
    assert!(*max_detection_time < Duration::from_secs(2), 
           "Max detection time should be under 2 seconds");
    
    // Stop watching
    app_service.watcher.stop_watching().await?;
    
    Ok(())
}