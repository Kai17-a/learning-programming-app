use std::sync::Arc;
use std::fs;
use tempfile::TempDir;

use learning_programming_app::core::{AutoExecutorService, HistoryManagerService};
use learning_programming_app::handlers::{LanguageHandlerService, GoHandler};
use learning_programming_app::handlers::base::LanguageHandler;

#[tokio::test]
async fn test_go_handler_registration() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    
    // Register Go handler
    let go_handler = Arc::new(GoHandler::new());
    handler_service.register_handler("go", go_handler).await;
    
    // Test that Go handler is registered
    assert!(handler_service.is_supported("go").await);
    
    let supported_extensions = handler_service.get_supported_extensions().await;
    assert!(supported_extensions.contains(&"go".to_string()));
}

#[tokio::test]
async fn test_go_file_execution() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    
    // Register Go handler
    let go_handler = Arc::new(GoHandler::new());
    handler_service.register_handler("go", go_handler).await;
    
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);
    
    // Create a temporary Go file
    let temp_dir = TempDir::new().unwrap();
    let go_file = temp_dir.path().join("test.go");
    let go_code = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello from Go!")
}
"#;
    fs::write(&go_file, go_code).unwrap();
    
    // Execute the file
    let result = executor.execute_file(&go_file).await;
    
    // Verify the result structure (success depends on Go being installed)
    assert_eq!(result.file_path, go_file);
    assert!(result.execution_time >= std::time::Duration::default());
    
    // If Go is installed, check the output
    if result.success {
        assert!(result.output.contains("Hello from Go!"));
    }
}

#[tokio::test]
async fn test_go_syntax_error_handling() {
    let handler_service = Arc::new(LanguageHandlerService::new());
    
    // Register Go handler
    let go_handler = Arc::new(GoHandler::new());
    handler_service.register_handler("go", go_handler).await;
    
    let temp_db = tempfile::NamedTempFile::new().unwrap();
    let history_manager = Arc::new(HistoryManagerService::new(temp_db.path()).await.unwrap());
    let executor = AutoExecutorService::new(handler_service, history_manager);
    
    // Create a Go file with syntax error
    let temp_dir = TempDir::new().unwrap();
    let go_file = temp_dir.path().join("syntax_error.go");
    let invalid_go_code = r#"package main

import "fmt"

func main() {
    fmt.Println("Missing closing quote
}
"#;
    fs::write(&go_file, invalid_go_code).unwrap();
    
    // Execute the file with syntax error
    let result = executor.execute_file(&go_file).await;
    
    // If Go is installed, verify error handling
    if result.error_message.is_some() {
        assert!(!result.success);
        let error_msg = result.error_message.as_ref().unwrap();
        // Go compiler errors typically contain "syntax error" or similar
        assert!(error_msg.contains("syntax") || error_msg.contains("error"));
    }
}

#[tokio::test]
async fn test_go_handler_get_command() {
    let handler = GoHandler::new();
    let temp_dir = TempDir::new().unwrap();
    let go_file = temp_dir.path().join("test.go");
    
    let command = handler.get_command(&go_file);
    assert_eq!(command[0], "go");
    assert_eq!(command[1], "run");
    assert_eq!(command[2], go_file.to_string_lossy());
}

#[tokio::test]
async fn test_go_handler_properties() {
    let handler = GoHandler::new();
    
    assert_eq!(handler.get_extension(), "go");
    assert_eq!(handler.get_name(), "Go");
}

#[tokio::test]
async fn test_go_handler_with_custom_command() {
    let handler = GoHandler::with_command("custom-go");
    let temp_dir = TempDir::new().unwrap();
    let go_file = temp_dir.path().join("test.go");
    
    let command = handler.get_command(&go_file);
    assert_eq!(command[0], "custom-go");
    assert_eq!(command[1], "run");
}