use learning_programming_app::core::{HistoryManagerService, StatisticsService, ExecutionRecord};
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio_test;
use uuid::Uuid;
use chrono::Utc;

/// Helper function to create a test StatisticsService with a temporary database
async fn create_test_statistics_service() -> StatisticsService {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path();
    let history_manager = Arc::new(HistoryManagerService::new(db_path).await.unwrap());
    StatisticsService::new(history_manager)
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
async fn test_statistics_service_creation() {
    let service = create_test_statistics_service().await;
    
    // Should not panic when displaying empty statistics
    let result = service.display_overview().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_overview_with_data() {
    let service = create_test_statistics_service().await;
    
    // Add some test data
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
        create_test_record("test3.py", "section2", true, 1.5),
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Should display overview without errors
    let result = service.display_overview().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_file_stats_existing_file() {
    let service = create_test_statistics_service().await;
    
    // Add records for a specific file
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test1.py", "section1", false, 2.0),
        create_test_record("test2.py", "section1", true, 1.5), // Different file
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Should display file stats without errors
    let result = service.display_file_stats("test1.py").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_file_stats_nonexistent_file() {
    let service = create_test_statistics_service().await;
    
    // Should handle non-existent file gracefully
    let result = service.display_file_stats("nonexistent.py").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_section_stats_existing_section() {
    let service = create_test_statistics_service().await;
    
    // Add records for a specific section
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
        create_test_record("test3.py", "section2", true, 1.5), // Different section
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Should display section stats without errors
    let result = service.display_section_stats("section1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_section_stats_nonexistent_section() {
    let service = create_test_statistics_service().await;
    
    // Should handle non-existent section gracefully
    let result = service.display_section_stats("nonexistent").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_trends() {
    let service = create_test_statistics_service().await;
    
    // Add some test data
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Should display trends without errors
    let result = service.display_trends(7).await;
    assert!(result.is_ok());
    
    // Test with empty data
    service.clear_statistics().await.unwrap();
    let result = service.display_trends(7).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_top_files() {
    let service = create_test_statistics_service().await;
    
    // Add records with different execution counts
    let records = vec![
        // test1.py: 2 executions
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test1.py", "section1", true, 2.0),
        // test2.py: 1 execution
        create_test_record("test2.py", "section1", true, 1.5),
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Should display top files without errors
    let result = service.display_top_files(5).await;
    assert!(result.is_ok());
    
    // Test with empty data
    service.clear_statistics().await.unwrap();
    let result = service.display_top_files(5).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_display_recent_history() {
    let service = create_test_statistics_service().await;
    
    // Add some test data
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
        create_test_record("test3.py", "section2", true, 1.5),
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Should display recent history without errors
    let result = service.display_recent_history(10).await;
    assert!(result.is_ok());
    
    // Test with limit smaller than available records
    let result = service.display_recent_history(2).await;
    assert!(result.is_ok());
    
    // Test with empty data
    service.clear_statistics().await.unwrap();
    let result = service.display_recent_history(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_clear_statistics() {
    let service = create_test_statistics_service().await;
    
    // Add some test data
    let records = vec![
        create_test_record("test1.py", "section1", true, 1.0),
        create_test_record("test2.py", "section1", false, 2.0),
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Verify data exists
    let stats = service.history_manager.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 2);
    
    // Clear statistics
    let result = service.clear_statistics().await;
    assert!(result.is_ok());
    
    // Verify data is cleared
    let stats = service.history_manager.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 0);
}

#[tokio::test]
async fn test_statistics_service_with_large_dataset() {
    let service = create_test_statistics_service().await;
    
    // Add a larger dataset to test performance and correctness
    for i in 0..100 {
        let record = create_test_record(
            &format!("test{}.py", i % 10), // 10 different files
            &format!("section{}", i % 5),  // 5 different sections
            i % 3 != 0, // ~67% success rate
            (i as f64) * 0.1,
        );
        service.history_manager.record_execution(&record).await.unwrap();
    }
    
    // Test all display functions with larger dataset
    let result = service.display_overview().await;
    assert!(result.is_ok());
    
    let result = service.display_file_stats("test0.py").await;
    assert!(result.is_ok());
    
    let result = service.display_section_stats("section0").await;
    assert!(result.is_ok());
    
    let result = service.display_trends(30).await;
    assert!(result.is_ok());
    
    let result = service.display_top_files(5).await;
    assert!(result.is_ok());
    
    let result = service.display_recent_history(20).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_statistics_service_error_handling() {
    let service = create_test_statistics_service().await;
    
    // Test with various edge cases
    
    // Empty strings
    let result = service.display_file_stats("").await;
    assert!(result.is_ok());
    
    let result = service.display_section_stats("").await;
    assert!(result.is_ok());
    
    // Very long strings
    let long_string = "a".repeat(1000);
    let result = service.display_file_stats(&long_string).await;
    assert!(result.is_ok());
    
    let result = service.display_section_stats(&long_string).await;
    assert!(result.is_ok());
    
    // Zero limits
    let result = service.display_top_files(0).await;
    assert!(result.is_ok());
    
    let result = service.display_recent_history(0).await;
    assert!(result.is_ok());
    
    let result = service.display_trends(0).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_statistics_display_consistency() {
    let service = create_test_statistics_service().await;
    
    // Add known test data
    let records = vec![
        create_test_record("file1.py", "section1", true, 1.0),
        create_test_record("file1.py", "section1", false, 2.0),
        create_test_record("file2.py", "section1", true, 1.5),
        create_test_record("file3.py", "section2", true, 0.5),
    ];
    
    for record in &records {
        service.history_manager.record_execution(record).await.unwrap();
    }
    
    // Multiple calls should be consistent
    for _ in 0..3 {
        let result = service.display_overview().await;
        assert!(result.is_ok());
        
        let result = service.display_file_stats("file1.py").await;
        assert!(result.is_ok());
        
        let result = service.display_section_stats("section1").await;
        assert!(result.is_ok());
    }
    
    // Verify the underlying data hasn't changed
    let stats = service.history_manager.get_stats().await.unwrap();
    assert_eq!(stats.total_executions, 4);
    assert_eq!(stats.successful_executions, 3);
    assert_eq!(stats.failed_executions, 1);
}