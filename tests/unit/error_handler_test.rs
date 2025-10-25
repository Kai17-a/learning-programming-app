use learning_programming_app::utils::errors::{ErrorHandler, AppError};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_error_handler_creation() {
    let handler = ErrorHandler::new();
    // Test that handler can be created successfully
    assert!(format!("{:?}", handler).contains("ErrorHandler"));
}

#[test]
fn test_handle_execution_error() {
    let handler = ErrorHandler::new();
    let test_path = PathBuf::from("test.py");
    let error = anyhow::anyhow!("Test execution error");
    
    let result = handler.handle_execution_error(&error, &test_path);
    
    assert!(result.contains("✗"));
    assert!(result.contains("Execution failed"));
    assert!(result.contains("test.py"));
    assert!(result.contains("Test execution error"));
}

#[test]
fn test_handle_syntax_error() {
    let handler = ErrorHandler::new();
    let test_path = PathBuf::from("syntax_error.py");
    let error_msg = "SyntaxError: invalid syntax";
    
    let result = handler.handle_syntax_error(error_msg, &test_path);
    
    assert!(result.contains("⚠"));
    assert!(result.contains("Syntax error"));
    assert!(result.contains("syntax_error.py"));
    assert!(result.contains("invalid syntax"));
}

#[test]
fn test_handle_runtime_error() {
    let handler = ErrorHandler::new();
    let test_path = PathBuf::from("runtime_error.py");
    let error_msg = "NameError: name 'undefined_var' is not defined";
    
    let result = handler.handle_runtime_error(error_msg, &test_path);
    
    assert!(result.contains("✗"));
    assert!(result.contains("Runtime error"));
    assert!(result.contains("runtime_error.py"));
    assert!(result.contains("undefined_var"));
}

#[test]
fn test_handle_timeout_error() {
    let handler = ErrorHandler::new();
    let test_path = PathBuf::from("slow_script.py");
    let timeout_secs = 30;
    
    let result = handler.handle_timeout_error(&test_path, timeout_secs);
    
    assert!(result.contains("⏱"));
    assert!(result.contains("Execution timeout"));
    assert!(result.contains("slow_script.py"));
    assert!(result.contains("30s"));
}

#[test]
fn test_handle_permission_error() {
    let handler = ErrorHandler::new();
    let test_path = PathBuf::from("/restricted/file.py");
    
    let result = handler.handle_permission_error(&test_path);
    
    assert!(result.contains("🔒"));
    assert!(result.contains("Permission denied"));
    assert!(result.contains("restricted"));
}

#[test]
fn test_handle_file_not_found_error() {
    let handler = ErrorHandler::new();
    let test_path = PathBuf::from("nonexistent.py");
    
    let result = handler.handle_file_not_found_error(&test_path);
    
    assert!(result.contains("📁"));
    assert!(result.contains("File not found"));
    assert!(result.contains("nonexistent.py"));
}

#[test]
fn test_handle_file_watch_error() {
    let handler = ErrorHandler::new();
    let watch_error = notify::Error::generic("Test watch error");
    
    let result = handler.handle_file_watch_error(&watch_error);
    
    assert!(result.contains("⚠"));
    assert!(result.contains("File watching error"));
    assert!(result.contains("Test watch error"));
}

#[test]
fn test_handle_system_error() {
    let handler = ErrorHandler::new();
    let system_error = anyhow::anyhow!("System failure");
    
    let result = handler.handle_system_error(&system_error);
    
    assert!(result.contains("✗"));
    assert!(result.contains("System error"));
    assert!(result.contains("System failure"));
}

#[test]
fn test_format_error_message() {
    let handler = ErrorHandler::new();
    let error = anyhow::anyhow!("Generic error message");
    
    let result = handler.format_error_message(&error);
    
    assert!(result.contains("Error:"));
    assert!(result.contains("Generic error message"));
}

#[test]
fn test_app_error_creation() {
    // Test execution error
    let exec_error = AppError::execution("Test execution error");
    assert!(exec_error.to_string().contains("Execution error"));
    assert!(exec_error.to_string().contains("Test execution error"));
    
    // Test syntax error
    let syntax_error = AppError::syntax("Invalid syntax");
    assert!(syntax_error.to_string().contains("Syntax error"));
    assert!(syntax_error.to_string().contains("Invalid syntax"));
    
    // Test runtime error
    let runtime_error = AppError::runtime("Runtime failure");
    assert!(runtime_error.to_string().contains("Runtime error"));
    assert!(runtime_error.to_string().contains("Runtime failure"));
    
    // Test timeout error
    let timeout_error = AppError::timeout(30);
    assert!(timeout_error.to_string().contains("Timeout error"));
    assert!(timeout_error.to_string().contains("30s"));
    
    // Test permission error
    let perm_error = AppError::permission("/restricted/path");
    assert!(perm_error.to_string().contains("Permission denied"));
    assert!(perm_error.to_string().contains("restricted"));
    
    // Test file not found error
    let not_found_error = AppError::file_not_found("/missing/file");
    assert!(not_found_error.to_string().contains("File not found"));
    assert!(not_found_error.to_string().contains("missing"));
}

#[test]
fn test_error_handler_clone() {
    let handler1 = ErrorHandler::new();
    let handler2 = handler1.clone();
    
    // Both handlers should work identically
    let test_path = PathBuf::from("test.py");
    let error = anyhow::anyhow!("Test error");
    
    let result1 = handler1.handle_execution_error(&error, &test_path);
    let result2 = handler2.handle_execution_error(&error, &test_path);
    
    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_error_handling_with_real_files() {
    let handler = ErrorHandler::new();
    let temp_dir = TempDir::new().unwrap();
    
    // Test with existing file
    let existing_file = temp_dir.path().join("existing.py");
    tokio::fs::write(&existing_file, "print('hello')").await.unwrap();
    
    // Test with non-existent file
    let missing_file = temp_dir.path().join("missing.py");
    
    // Test file not found error
    let error_msg = handler.handle_file_not_found_error(&missing_file);
    assert!(error_msg.contains("File not found"));
    assert!(error_msg.contains("missing.py"));
    
    // Test that existing file doesn't trigger file not found
    assert!(existing_file.exists());
}

#[test]
fn test_error_conversion() {
    // Test that AppError can be converted to anyhow::Error
    let app_error = AppError::execution("Test error");
    let anyhow_error: anyhow::Error = app_error.into();
    
    assert!(anyhow_error.to_string().contains("Execution error"));
    assert!(anyhow_error.to_string().contains("Test error"));
}

#[test]
fn test_error_chain() {
    // Test error chaining with system errors
    let source_error = anyhow::anyhow!("Source error");
    let app_error = AppError::System(source_error);
    
    assert!(app_error.to_string().contains("System error"));
    assert!(app_error.to_string().contains("Source error"));
}