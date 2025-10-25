use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;
use tokio_test;

use learning_programming_app::core::{AutoExecutorService, ExecutionResult};
use learning_programming_app::handlers::{LanguageHandlerService, PythonHandler};
use learning_programming_app::utils::DisplayService;

#[tokio::test]
async fn test_auto_executor_service_creation() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let executor = AutoExecutorService::new(handler_service.clone());
    
    // Test that the service is created successfully
    assert_eq!(executor.get_supported_extensions().await.len(), 0);
}

#[tokio::test]
async fn test_execute_file_with_python_handler() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    
    // Register Python handler
    let python_handler = Arc::new(PythonHandler::new());
    handler_service.register_handler("py", python_handler).await;
    
    let executor = AutoExecutorService::new(handler_service);
    
    // Create a temporary Python file
    let temp_dir = TempDir::new().unwrap();
    let python_file = temp_dir.path().join("test.py");
    fs::write(&python_file, "print('Hello, World!')").unwrap();
    
    // Execute the file
    let result = executor.execute_file(&python_file).await.unwrap();
    
    // Verify the result
    assert_eq!(result.file_path, python_file);
    // Note: We can't guarantee success without Python installed, so we just check the structure
    assert!(result.execution_time >= std::time::Duration::default());
}

#[tokio::test]
async fn test_execute_nonexistent_file() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let executor = AutoExecutorService::new(handler_service);
    
    let nonexistent_file = PathBuf::from("nonexistent.py");
    let result = executor.execute_file(&nonexistent_file).await.unwrap();
    
    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result.error_message.unwrap().contains("File does not exist"));
}

#[tokio::test]
async fn test_execute_file_without_extension() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let executor = AutoExecutorService::new(handler_service);
    
    let temp_dir = TempDir::new().unwrap();
    let file_without_ext = temp_dir.path().join("test");
    fs::write(&file_without_ext, "print('Hello')").unwrap();
    
    let result = executor.execute_file(&file_without_ext).await.unwrap();
    
    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result.error_message.unwrap().contains("no extension"));
}

#[tokio::test]
async fn test_execute_unsupported_file_type() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    let executor = AutoExecutorService::new(handler_service);
    
    let temp_dir = TempDir::new().unwrap();
    let unsupported_file = temp_dir.path().join("test.txt");
    fs::write(&unsupported_file, "Hello, World!").unwrap();
    
    let result = executor.execute_file(&unsupported_file).await.unwrap();
    
    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result.error_message.unwrap().contains("No handler found"));
}

#[tokio::test]
async fn test_is_supported_file() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    
    // Register Python handler
    let python_handler = Arc::new(PythonHandler::new());
    handler_service.register_handler("py", python_handler).await;
    
    let executor = AutoExecutorService::new(handler_service);
    
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
    let executor = AutoExecutorService::new(handler_service);
    
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
    let executor = AutoExecutorService::new(handler_service);
    
    let result = ExecutionResult::new(PathBuf::from("test.py"))
        .with_success("Hello, World!".to_string(), std::time::Duration::from_millis(100), 0);
        
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
    let executor = AutoExecutorService::new(handler_service);
    
    let result = ExecutionResult::new(PathBuf::from("test.py"))
        .with_error("Syntax error".to_string(), std::time::Duration::from_millis(50), Some(1));
        
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
    let executor = AutoExecutorService::new(handler_service);
    
    let result = ExecutionResult::new(PathBuf::from("test.py"))
        .with_success("".to_string(), std::time::Duration::from_millis(25), 0);
        
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
    
    let executor = AutoExecutorService::new(handler_service);
    
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
    
    let executor = AutoExecutorService::new(handler_service);
    
    let handler = executor.get_language_handler("py").await;
    assert!(handler.is_some());
    
    let no_handler = executor.get_language_handler("js").await;
    assert!(no_handler.is_none());
}