use chrono::{DateTime, Utc};
use learning_programming_app::core::{HistoryManagerService, ExecutionRecord, ExecutionStats};
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio_test;
use uuid::Uuid;

/// Helper function to create a test HistoryManagerService with a temporary database
async fn create_test_history_service() -> HistoryManagerService {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path();
    HistoryManagerService::new(db_path).await.unwrap()
}

/// Helper function to create a test ExecutionRecord
fn create_test_record(
    file_path: &str,
    section: &str,
    success: bool,
    execution_time: f64,
) -> ExecutionRecord {
    ExecutionRecord {
        id: Uuid::new_v4().to_string(),
        file_path: file_path.to_string(),
        section: section.to_string(),
        success,
        execution_time,
        timestamp: Utc::now(),
        output_preview: if success {
            "Success output".to_string()
        } else {
            "Error message".to_string()
        },
    }
}

#[tokio::test]
async fn test_history_service_creation() {
    let service = create_test_history_service().await;
    
    // Verify the service was created successfully
    let stats = service.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 0);
    assert_eq!(stats.successful_executions, 0);
    assert_eq!(stats.failed_executions, 0);
}

#[tokio::test]
async fn test_record_single_execution() {
    let service = create_test_history_service().await;
    
    let record = create_test_record("test.py", "section1", true, 1.5);
    
    // Record the execution
    service.record_execution(&record).await.unwrap();
    
    // Verify it was recorded
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].file_path, "test.py");
    assert_eq!(history[0].section, "section1");
    assert!(history[0].success);
    assert_eq!(history[0].execution_time, 1.5);
}

#[tokio::test]
async fn test_record_multiple_executions() {
    let service = create_test_history_service().await;
    
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
        create_test_record("test3.py", "section2", true, 1.5),
    ];
    
    // Record all executions
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    // Verify all were recorded
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 3);
    
    // Verify order (should be newest first)
    assert_eq!(history[0].file_path, "test3.py");
    assert_eq!(history[1].file_path, "test2.py");
    assert_eq!(history[2].file_path, "test1.py");
}

#[tokio::test]
async fn test_get_history_with_limit() {
    let service = create_test_history_service().await;
    
    // Add 5 records
    for i in 1..=5 {
        let record = create_test_record(
            &format!("test{}.py", i),
            "section1",
            true,
            1.0,
        );
        service.record_execution(&record).await.unwrap();
    }
    
    // Test with limit
    let history = service.get_history(Some(3)).await.unwrap();
    assert_eq!(history.len(), 3);
    
    // Test without limit
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 5);
}

#[tokio::test]
async fn test_get_stats_calculation() {
    let service = create_test_history_service().await;
    
    // Add test records with known values
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),   // success
        create_test_record("test2.py", "section1", false, 2.0),  // failure
        create_test_record("test1.py", "section1", true, 3.0),   // success (same file)
        create_test_record("test3.py", "section2", false, 4.0),  // failure
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    let stats = service.get_stats().await.unwrap();
    
    // Verify basic counts
    assert_eq!(stats.total_executions, 4);
    assert_eq!(stats.successful_executions, 2);
    assert_eq!(stats.failed_executions, 2);
    
    // Verify success rate calculation
    assert_eq!(stats.success_rate(), 0.5);
    assert_eq!(stats.failure_rate(), 0.5);
    
    // Verify average execution time (1+2+3+4)/4 = 2.5
    assert_eq!(stats.average_execution_time, 2.5);
    
    // Verify most executed file (test1.py appears twice)
    assert_eq!(stats.most_executed_file, Some("test1.py".to_string()));
}

#[tokio::test]
async fn test_get_file_history() {
    let service = create_test_history_service().await;
    
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
        create_test_record("test1.py", "section1", true, 3.0),  // Same file
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    // Get history for specific file
    let file_history = service.get_file_history("test1.py").await.unwrap();
    assert_eq!(file_history.len(), 2);
    
    // Verify all records are for the correct file
    for record in &file_history {
        assert_eq!(record.file_path, "test1.py");
    }
    
    // Test non-existent file
    let empty_history = service.get_file_history("nonexistent.py").await.unwrap();
    assert_eq!(empty_history.len(), 0);
}

#[tokio::test]
async fn test_get_section_history() {
    let service = create_test_history_service().await;
    
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section2", false, 2.0),
        create_test_record("test3.py", "section1", true, 3.0),  // Same section
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    // Get history for specific section
    let section_history = service.get_section_history("section1").await.unwrap();
    assert_eq!(section_history.len(), 2);
    
    // Verify all records are for the correct section
    for record in &section_history {
        assert_eq!(record.section, "section1");
    }
    
    // Test non-existent section
    let empty_history = service.get_section_history("nonexistent").await.unwrap();
    assert_eq!(empty_history.len(), 0);
}

#[tokio::test]
async fn test_get_file_stats() {
    let service = create_test_history_service().await;
    
    // Add records for specific file
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),   // success
        create_test_record("test1.py", "section1", false, 2.0),  // failure
        create_test_record("test1.py", "section1", true, 3.0),   // success
        create_test_record("test2.py", "section1", true, 4.0),   // different file
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    let file_stats = service.get_file_stats("test1.py").await.unwrap();
    
    // Verify file-specific statistics
    assert_eq!(file_stats.total_executions, 3);
    assert_eq!(file_stats.successful_executions, 2);
    assert_eq!(file_stats.failed_executions, 1);
    assert_eq!(file_stats.average_execution_time, 2.0); // (1+2+3)/3 = 2.0
    assert_eq!(file_stats.most_executed_file, Some("test1.py".to_string()));
    
    // Test non-existent file
    let empty_stats = service.get_file_stats("nonexistent.py").await.unwrap();
    assert_eq!(empty_stats.total_executions, 0);
}

#[tokio::test]
async fn test_get_section_stats() {
    let service = create_test_history_service().await;
    
    // Add records for specific section
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),   // success
        create_test_record("test2.py", "section1", false, 2.0),  // failure
        create_test_record("test3.py", "section1", true, 3.0),   // success
        create_test_record("test4.py", "section2", true, 4.0),   // different section
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    let section_stats = service.get_section_stats("section1").await.unwrap();
    
    // Verify section-specific statistics
    assert_eq!(section_stats.total_executions, 3);
    assert_eq!(section_stats.successful_executions, 2);
    assert_eq!(section_stats.failed_executions, 1);
    assert_eq!(section_stats.average_execution_time, 2.0); // (1+2+3)/3 = 2.0
    
    // Test non-existent section
    let empty_stats = service.get_section_stats("nonexistent").await.unwrap();
    assert_eq!(empty_stats.total_executions, 0);
}

#[tokio::test]
async fn test_get_top_files() {
    let service = create_test_history_service().await;
    
    // Add records with different execution counts
    let records = vec![
        // test1.py: 3 executions
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test1.py", "section1", true, 2.0),
        create_test_record("test1.py", "section1", true, 3.0),
        // test2.py: 2 executions
        create_test_record("test2.py", "section1", true, 1.5),
        create_test_record("test2.py", "section1", true, 2.5),
        // test3.py: 1 execution
        create_test_record("test3.py", "section1", true, 2.0),
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    let top_files = service.get_top_files(2).await.unwrap();
    
    // Verify top files are returned in correct order
    assert_eq!(top_files.len(), 2);
    assert_eq!(top_files[0].0, "test1.py");
    assert_eq!(top_files[0].1, 3); // 3 executions
    assert_eq!(top_files[0].2, 2.0); // average time (1+2+3)/3 = 2.0
    
    assert_eq!(top_files[1].0, "test2.py");
    assert_eq!(top_files[1].1, 2); // 2 executions
    assert_eq!(top_files[1].2, 2.0); // average time (1.5+2.5)/2 = 2.0
}

#[tokio::test]
async fn test_get_execution_trends() {
    let service = create_test_history_service().await;
    
    // Add some test records (note: this test may be limited by the fact that
    // we're using current timestamps, so trends might not show multiple days)
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
        create_test_record("test3.py", "section1", true, 3.0),
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    let trends = service.get_execution_trends(7).await.unwrap();
    
    // Should have at least one day of data (today)
    assert!(!trends.is_empty());
    
    // Verify the structure of trend data
    if let Some((date, total, successful)) = trends.first() {
        assert!(!date.is_empty());
        assert_eq!(*total, 3);
        assert_eq!(*successful, 2);
    }
}

#[tokio::test]
async fn test_clear_history() {
    let service = create_test_history_service().await;
    
    // Add some records
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
    ];
    
    for record in &records {
        service.record_execution(record).await.unwrap();
    }
    
    // Verify records exist
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 2);
    
    let stats = service.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 2);
    
    // Clear history
    service.clear_history().await.unwrap();
    
    // Verify history is cleared
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 0);
    
    let stats = service.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 0);
}

#[tokio::test]
async fn test_empty_database_operations() {
    let service = create_test_history_service().await;
    
    // Test operations on empty database
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 0);
    
    let stats = service.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 0);
    assert_eq!(stats.success_rate(), 0.0);
    assert_eq!(stats.failure_rate(), 0.0);
    
    let file_history = service.get_file_history("test.py").await.unwrap();
    assert_eq!(file_history.len(), 0);
    
    let section_history = service.get_section_history("section1").await.unwrap();
    assert_eq!(section_history.len(), 0);
    
    let top_files = service.get_top_files(5).await.unwrap();
    assert_eq!(top_files.len(), 0);
    
    let trends = service.get_execution_trends(7).await.unwrap();
    assert_eq!(trends.len(), 0);
}

#[tokio::test]
async fn test_concurrent_operations() {
    let service = Arc::new(create_test_history_service().await);
    
    // Test concurrent record operations
    let mut handles = vec![];
    
    for i in 0..10 {
        let service_clone = Arc::clone(&service);
        let handle = tokio::spawn(async move {
            let record = create_test_record(
                &format!("test{}.py", i),
                "section1",
                i % 2 == 0, // alternate success/failure
                i as f64,
            );
            service_clone.record_execution(&record).await.unwrap();
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Verify all records were inserted
    let history = service.get_history(None).await.unwrap();
    assert_eq!(history.len(), 10);
    
    let stats = service.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 10);
    assert_eq!(stats.successful_executions, 5); // Even numbers (0,2,4,6,8)
    assert_eq!(stats.failed_executions, 5);     // Odd numbers (1,3,5,7,9)
}