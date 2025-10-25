use std::path::PathBuf;
use std::time::Duration;

use learning_programming_app::core::{ExecutionResult, ExecutionStats};
use learning_programming_app::utils::DisplayService;

#[test]
fn test_display_service_creation() {
    let display = DisplayService::new();
    // Test that the service is created successfully
    // We can't easily test the internal state, but we can test that it doesn't panic
    
    let display_custom = DisplayService::with_settings(false, true);
    // Test custom settings creation
}

#[test]
fn test_display_service_default() {
    let display = DisplayService::default();
    // Test that default creation works
}

#[test]
fn test_log_execution_result_success() {
    let display = DisplayService::new();
    
    let success_result = ExecutionResult::new(PathBuf::from("test.py"))
        .with_success("Hello, World!".to_string(), Duration::from_millis(100), 0);
        
    // This should not panic - we're testing the logging functionality
    // The actual logging output would go to the tracing subscriber
    display.log_execution_result(&success_result);
}

#[test]
fn test_log_execution_result_failure() {
    let display = DisplayService::new();
    
    let error_result = ExecutionResult::new(PathBuf::from("test.py"))
        .with_error("Syntax error".to_string(), Duration::from_millis(50), Some(1));
        
    // This should not panic
    display.log_execution_result(&error_result);
}

#[test]
fn test_log_execution_result_with_long_path() {
    let display = DisplayService::new();
    
    let long_path = PathBuf::from("very/long/path/to/some/deeply/nested/directory/test.py");
    let result = ExecutionResult::new(long_path)
        .with_success("Output".to_string(), Duration::from_millis(200), 0);
        
    // This should handle long paths without panicking
    display.log_execution_result(&result);
}

#[test]
fn test_log_execution_result_with_special_characters() {
    let display = DisplayService::new();
    
    let result = ExecutionResult::new(PathBuf::from("test_файл.py"))
        .with_error("Error with special chars: ñáéíóú".to_string(), Duration::from_millis(30), Some(2));
        
    // This should handle special characters without panicking
    display.log_execution_result(&result);
}

#[test]
fn test_log_execution_result_zero_duration() {
    let display = DisplayService::new();
    
    let result = ExecutionResult::new(PathBuf::from("fast.py"))
        .with_success("Quick execution".to_string(), Duration::default(), 0);
        
    // This should handle zero duration without panicking
    display.log_execution_result(&result);
}

#[test]
fn test_log_execution_result_no_error_message() {
    let display = DisplayService::new();
    
    let mut result = ExecutionResult::new(PathBuf::from("test.py"));
    result.success = false;
    result.execution_time = Duration::from_millis(100);
    result.exit_code = Some(1);
    // error_message remains None
    
    // This should handle missing error message gracefully
    display.log_execution_result(&result);
}

// Note: We can't easily test the actual terminal output without mocking the Term,
// but we can test that the methods don't panic and handle edge cases properly.
// The visual output would need to be tested manually or with integration tests.

#[test]
fn test_display_methods_dont_panic() {
    let display = DisplayService::new();
    
    // Test that these methods can be called without panicking
    // The actual output would go to stdout/stderr
    
    let result = ExecutionResult::new(PathBuf::from("test.py"))
        .with_success("Hello".to_string(), Duration::from_millis(50), 0);
    
    // These might fail due to terminal issues in test environment, but shouldn't panic
    let _ = display.display_execution_result(&result);
    let _ = display.display_file_change(&PathBuf::from("test.py"));
    let _ = display.display_error("Test error");
    let _ = display.display_warning("Test warning");
    let _ = display.display_info("Test info");
    
    let stats = ExecutionStats::default();
    let _ = display.display_stats(&stats);
    
    let _ = display.clear_screen();
    let _ = display.move_cursor_to_start();
}

#[test]
fn test_display_startup_method() {
    let display = DisplayService::new();
    
    let watch_dir = PathBuf::from("./examples");
    let extensions = vec!["py".to_string(), "js".to_string()];
    
    // This might fail in test environment but shouldn't panic
    let _ = display.display_startup(&watch_dir, &extensions);
}

#[test]
fn test_display_with_empty_stats() {
    let display = DisplayService::new();
    let empty_stats = ExecutionStats::default();
    
    // Should handle empty stats gracefully
    let _ = display.display_stats(&empty_stats);
}

#[test]
fn test_display_with_populated_stats() {
    let display = DisplayService::new();
    let mut stats = ExecutionStats::default();
    stats.total_executions = 10;
    stats.successful_executions = 8;
    stats.failed_executions = 2;
    stats.most_executed_file = Some("test.py".to_string());
    stats.average_execution_time = 0.15; // 150ms
    stats.last_execution = Some(chrono::Utc::now());
    
    // Should handle populated stats gracefully
    let _ = display.display_stats(&stats);
}