use std::path::Path;
use std::sync::Arc;
use tempfile::NamedTempFile;
use std::io::Write;

use learning_programming_app::handlers::{LanguageHandler, LanguageHandlerService, PythonHandler};

// Python Handler Tests
#[tokio::test]
async fn test_python_handler_successful_execution() {
    let handler = PythonHandler::new();
    
    // Create a temporary Python file with valid code
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "print('Hello, World!')").unwrap();
    let file_path = temp_file.path();
    
    let result = handler.execute(file_path).await.unwrap();
    
    assert!(result.success);
    assert!(result.output.contains("Hello, World!"));
    assert_eq!(result.exit_code, Some(0));
    assert!(result.error_message.is_none());
}

#[tokio::test]
async fn test_python_handler_syntax_error() {
    let handler = PythonHandler::new();
    
    // Create a temporary Python file with syntax error
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "print('Hello, World'").unwrap(); // Missing closing parenthesis
    let file_path = temp_file.path();
    
    let result = handler.execute(file_path).await.unwrap();
    
    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert_ne!(result.exit_code, Some(0));
}

#[tokio::test]
async fn test_python_handler_runtime_error() {
    let handler = PythonHandler::new();
    
    // Create a temporary Python file with runtime error
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "x = 1 / 0").unwrap(); // Division by zero
    let file_path = temp_file.path();
    
    let result = handler.execute(file_path).await.unwrap();
    
    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert_ne!(result.exit_code, Some(0));
}

#[tokio::test]
async fn test_python_handler_validate_syntax_valid() {
    let handler = PythonHandler::new();
    
    // Create a temporary Python file with valid syntax
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "print('Hello, World!')").unwrap();
    let file_path = temp_file.path();
    
    let result = handler.validate_syntax(file_path).await.unwrap();
    
    assert!(result.is_valid);
    assert!(result.error_message.is_none());
}

#[tokio::test]
async fn test_python_handler_validate_syntax_invalid() {
    let handler = PythonHandler::new();
    
    // Create a temporary Python file with invalid syntax
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "print('Hello, World'").unwrap(); // Missing closing parenthesis
    let file_path = temp_file.path();
    
    let result = handler.validate_syntax(file_path).await.unwrap();
    
    assert!(!result.is_valid);
    assert!(result.error_message.is_some());
}

#[test]
fn test_python_handler_get_command() {
    let handler = PythonHandler::new();
    let file_path = Path::new("test.py");
    
    let command = handler.get_command(file_path);
    
    assert_eq!(command.len(), 2);
    assert_eq!(command[0], "python");
    assert_eq!(command[1], "test.py");
}

#[test]
fn test_python_handler_with_custom_command() {
    let handler = PythonHandler::with_command("python3");
    let file_path = Path::new("test.py");
    
    let command = handler.get_command(file_path);
    
    assert_eq!(command.len(), 2);
    assert_eq!(command[0], "python3");
    assert_eq!(command[1], "test.py");
}

#[test]
fn test_python_handler_metadata() {
    let handler = PythonHandler::new();
    
    assert_eq!(handler.get_extension(), "py");
    assert_eq!(handler.get_name(), "Python");
}

// Language Handler Service Tests
#[tokio::test]
async fn test_language_handler_service_register_and_get() {
    let service = LanguageHandlerService::new();
    let python_handler = Arc::new(PythonHandler::new());
    
    // Register the Python handler
    service.register_handler("py", python_handler.clone()).await;
    
    // Retrieve the handler
    let retrieved_handler = service.get_handler("py").await;
    
    assert!(retrieved_handler.is_some());
    let handler = retrieved_handler.unwrap();
    assert_eq!(handler.get_extension(), "py");
    assert_eq!(handler.get_name(), "Python");
}

#[tokio::test]
async fn test_language_handler_service_case_insensitive() {
    let service = LanguageHandlerService::new();
    let python_handler = Arc::new(PythonHandler::new());
    
    // Register with lowercase
    service.register_handler("py", python_handler.clone()).await;
    
    // Retrieve with uppercase
    let retrieved_handler = service.get_handler("PY").await;
    
    assert!(retrieved_handler.is_some());
}

#[tokio::test]
async fn test_language_handler_service_is_supported() {
    let service = LanguageHandlerService::new();
    let python_handler = Arc::new(PythonHandler::new());
    
    // Initially not supported
    assert!(!service.is_supported("py").await);
    
    // Register the handler
    service.register_handler("py", python_handler).await;
    
    // Now supported
    assert!(service.is_supported("py").await);
    assert!(service.is_supported("PY").await); // Case insensitive
    assert!(!service.is_supported("js").await); // Not registered
}

#[tokio::test]
async fn test_language_handler_service_get_supported_extensions() {
    let service = LanguageHandlerService::new();
    let python_handler = Arc::new(PythonHandler::new());
    
    // Initially empty
    let extensions = service.get_supported_extensions().await;
    assert!(extensions.is_empty());
    
    // Register handlers
    service.register_handler("py", python_handler).await;
    
    let extensions = service.get_supported_extensions().await;
    assert_eq!(extensions.len(), 1);
    assert!(extensions.contains(&"py".to_string()));
}

#[tokio::test]
async fn test_language_handler_service_handler_count() {
    let service = LanguageHandlerService::new();
    let python_handler = Arc::new(PythonHandler::new());
    
    // Initially zero
    assert_eq!(service.handler_count().await, 0);
    
    // Register a handler
    service.register_handler("py", python_handler).await;
    
    // Now one
    assert_eq!(service.handler_count().await, 1);
}

#[tokio::test]
async fn test_language_handler_service_get_nonexistent_handler() {
    let service = LanguageHandlerService::new();
    
    let handler = service.get_handler("nonexistent").await;
    
    assert!(handler.is_none());
}