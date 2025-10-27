use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

use learning_programming_app::core::models::ExecutionResult;
use learning_programming_app::core::{AutoExecutorService, HistoryManagerService};
use learning_programming_app::handlers::{LanguageHandlerService, PythonHandler};
use learning_programming_app::utils::display::DisplayService;

#[tokio::test]
async fn test_auto_executor_service_creation() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service.clone(), history_manager);

    // Test that the service is created successfully
    assert_eq!(executor.get_supported_extensions().await.len(), 0);
}

#[tokio::test]
async fn test_execute_file_with_python_handler() {
    let handler_service = Arc::new(LanguageHandlerService::new());

    // Register Python handler
    let python_handler = Arc::new(PythonHandler::new());
    handler_service.register_handler("py", python_handler).await;

    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    // Create a temporary Python file
    let temp_dir = TempDir::new().unwrap();
    let python_file = temp_dir.path().join("test.py");
    fs::write(&python_file, "print('Hello, World!')").unwrap();

    // Execute the file
    let result = executor.execute_file(&python_file).await;

    // Verify the result structure (success depends on Python being installed)
    assert_eq!(result.file_path, python_file);
    assert!(result.execution_time >= std::time::Duration::default());
}

#[tokio::test]
async fn test_execute_nonexistent_file() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let nonexistent_file = PathBuf::from("nonexistent.py");
    let result = executor.execute_file(&nonexistent_file).await;

    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result
        .error_message
        .unwrap()
        .contains("File not found"));
}

#[tokio::test]
async fn test_execute_file_without_extension() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let temp_dir = TempDir::new().unwrap();
    let file_without_ext = temp_dir.path().join("test");
    fs::write(&file_without_ext, "print('Hello')").unwrap();

    let result = executor.execute_file(&file_without_ext).await;

    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result.error_message.unwrap().contains("No file extension"));
}

#[tokio::test]
async fn test_execute_unsupported_file_type() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let temp_dir = TempDir::new().unwrap();
    let unsupported_file = temp_dir.path().join("test.txt");
    fs::write(&unsupported_file, "Hello, World!").unwrap();

    let result = executor.execute_file(&unsupported_file).await;

    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result.error_message.unwrap().contains("Unsupported file type"));
}

#[tokio::test]
async fn test_is_supported_file() {
    let handler_service = Arc::new(LanguageHandlerService::new());

    // Register Python handler
    let python_handler = Arc::new(PythonHandler::new());
    handler_service.register_handler("py", python_handler).await;

    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let python_file = PathBuf::from("test.py");
    let text_file = PathBuf::from("test.txt");
    let no_ext_file = PathBuf::from("test");

    assert!(executor.is_supported_file(&python_file).await);
    assert!(!executor.is_supported_file(&text_file).await);
    assert!(!executor.is_supported_file(&no_ext_file).await);
}

#[tokio::test]
async fn test_extract_section() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let temp_dir = TempDir::new().unwrap();
    let section_dir = temp_dir.path().join("section1-basics");
    fs::create_dir(&section_dir).unwrap();
    let file_path = section_dir.join("test.py");

    let section = executor.extract_section(&file_path);
    assert_eq!(section, "section1-basics");

    // Test with file in root directory
    let root_file = temp_dir.path().join("test.py");
    let root_section = executor.extract_section(&root_file);
    // Should return the temp directory name or "unknown"
    assert!(!root_section.is_empty());
}

#[tokio::test]
async fn test_format_output_success() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let result = ExecutionResult::new(PathBuf::from("test.py")).with_success(
        "Hello, World!".to_string(),
        std::time::Duration::from_millis(100),
        0,
    );

    let output = executor.format_output(&result);

    // Check that the output contains expected elements
    assert!(output.contains("✓"));
    assert!(output.contains("Executed"));
    assert!(output.contains("test.py"));
    assert!(output.contains("Hello, World!"));
    assert!(output.contains("100ms"));
}

#[tokio::test]
async fn test_format_output_failure() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let result = ExecutionResult::new(PathBuf::from("test.py")).with_error(
        "Syntax error".to_string(),
        std::time::Duration::from_millis(50),
        Some(1),
    );

    let output = executor.format_output(&result);

    // Check that the output contains expected elements
    assert!(output.contains("✗"));
    assert!(output.contains("Failed"));
    assert!(output.contains("test.py"));
    assert!(output.contains("Syntax error"));
    assert!(output.contains("50ms"));
}

#[tokio::test]
async fn test_format_output_empty_output() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let result = ExecutionResult::new(PathBuf::from("test.py")).with_success(
        "".to_string(),
        std::time::Duration::from_millis(25),
        0,
    );

    let output = executor.format_output(&result);

    // Check that it handles empty output gracefully
    assert!(output.contains("✓"));
    assert!(output.contains("(no output)"));
}

#[tokio::test]
async fn test_get_supported_extensions() {
    let handler_service = Arc::new(LanguageHandlerService::new());

    // Register Python handler
    let python_handler = Arc::new(PythonHandler::new());
    handler_service.register_handler("py", python_handler).await;

    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let extensions = executor.get_supported_extensions().await;
    assert_eq!(extensions.len(), 1);
    assert!(extensions.contains(&"py".to_string()));
}

#[tokio::test]
async fn test_get_language_handler() {
    let handler_service = Arc::new(LanguageHandlerService::new());

    // Register Python handler
    let python_handler = Arc::new(PythonHandler::new());
    handler_service.register_handler("py", python_handler).await;

    let history_manager = Arc::new(HistoryManagerService::new("test.db").await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);

    let handler = executor.get_language_handler("py").await;
    assert!(handler.is_some());

    let no_handler = executor.get_language_handler("js").await;
    assert!(no_handler.is_none());
}

#[test]
fn test_display_service_creation() {
    let _display = DisplayService::new();
    // Test that the service is created successfully

    let _display_custom = DisplayService::with_settings(false, true);
    // Test custom settings creation
}

#[test]
fn test_display_service_default() {
    let _display = DisplayService::default();
    // Test that default creation works
}

#[test]
fn test_display_methods_dont_panic() {
    let display = DisplayService::new();

    // Test that these methods can be called without panicking
    let result = ExecutionResult::new(PathBuf::from("test.py")).with_success(
        "Hello".to_string(),
        std::time::Duration::from_millis(50),
        0,
    );

    // These might fail due to terminal issues in test environment, but shouldn't panic
    let _ = display.display_execution_result(&result);
    let _ = display.display_file_change(&PathBuf::from("test.py"));
    let _ = display.display_error("Test error");
    let _ = display.display_warning("Test warning");
    let _ = display.display_info("Test info");

    let _ = display.clear_screen();
    let _ = display.move_cursor_to_start();
}
