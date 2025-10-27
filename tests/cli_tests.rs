use clap::Parser;
use learning_programming_app::cli::commands::{Cli, Commands};
use learning_programming_app::cli::shutdown::ShutdownHandler;
use learning_programming_app::cli::CommandLineInterface;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_cli_command_parsing() {
    // Test watch command
    let cli = Cli::try_parse_from(&[
        "learning-programming-app",
        "watch",
        "--directory",
        "./test",
        "--verbose",
    ])
    .unwrap();
    if let Some(Commands::Watch { directory, verbose }) = cli.command {
        assert_eq!(directory, PathBuf::from("./test"));
        assert!(verbose);
    } else {
        panic!("Expected Watch command");
    }

    // Test sections command
    let cli = Cli::try_parse_from(&[
        "learning-programming-app",
        "sections",
        "--directory",
        "./examples",
    ])
    .unwrap();
    if let Some(Commands::Sections { directory }) = cli.command {
        assert_eq!(directory, PathBuf::from("./examples"));
    } else {
        panic!("Expected Sections command");
    }

    // Test history command
    let cli =
        Cli::try_parse_from(&["learning-programming-app", "history", "--limit", "20"]).unwrap();
    if let Some(Commands::History { limit }) = cli.command {
        assert_eq!(limit, 20);
    } else {
        panic!("Expected History command");
    }

    // Test stats command
    let cli = Cli::try_parse_from(&["learning-programming-app", "stats"]).unwrap();
    if let Some(Commands::Stats) = cli.command {
        // Success
    } else {
        panic!("Expected Stats command");
    }

    // Test clear command
    let cli = Cli::try_parse_from(&["learning-programming-app", "clear", "--force"]).unwrap();
    if let Some(Commands::Clear { force }) = cli.command {
        assert!(force);
    } else {
        panic!("Expected Clear command");
    }

    // Test run command
    let cli =
        Cli::try_parse_from(&["learning-programming-app", "run", "test.py", "--verbose"]).unwrap();
    if let Some(Commands::Run { file, verbose }) = cli.command {
        assert_eq!(file, PathBuf::from("test.py"));
        assert!(verbose);
    } else {
        panic!("Expected Run command");
    }
}

#[tokio::test]
async fn test_command_line_interface_creation() {
    let result = CommandLineInterface::new().await;
    assert!(result.is_ok());

    let cli = result.unwrap();
    assert!(!cli.is_watching());

    let watched_dirs = cli.get_watched_directories().await;
    assert!(watched_dirs.is_empty());
}

#[tokio::test]
async fn test_display_sections_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let cli = CommandLineInterface::new().await.unwrap();

    let sections = cli.display_sections(temp_dir.path()).await.unwrap();
    assert!(sections.is_empty());
}

#[tokio::test]
async fn test_display_sections_with_content() {
    let temp_dir = TempDir::new().unwrap();

    // Create section directories with Python files
    let section1_path = temp_dir.path().join("section1-basics");
    let section2_path = temp_dir.path().join("section2-advanced");

    fs::create_dir(&section1_path).await.unwrap();
    fs::create_dir(&section2_path).await.unwrap();

    // Add Python files to sections
    fs::write(section1_path.join("hello.py"), "print('Hello, World!')")
        .await
        .unwrap();
    fs::write(section1_path.join("variables.py"), "x = 42\nprint(x)")
        .await
        .unwrap();
    fs::write(section2_path.join("classes.py"), "class Test:\n    pass")
        .await
        .unwrap();

    let cli = CommandLineInterface::new().await.unwrap();
    let sections = cli.display_sections(temp_dir.path()).await.unwrap();

    assert_eq!(sections.len(), 2);
    assert!(sections.contains(&"section1-basics".to_string()));
    assert!(sections.contains(&"section2-advanced".to_string()));
}

#[tokio::test]
async fn test_display_sections_nonexistent_directory() {
    let cli = CommandLineInterface::new().await.unwrap();
    let nonexistent_path = PathBuf::from("/nonexistent/directory");

    let sections = cli.display_sections(&nonexistent_path).await.unwrap();
    assert!(sections.is_empty());
}

#[tokio::test]
async fn test_run_file_nonexistent() {
    let cli = CommandLineInterface::new().await.unwrap();
    let result = cli.run_file(&PathBuf::from("nonexistent.py"), false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_run_file_existing() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.py");

    fs::write(&test_file, "print('Test execution')")
        .await
        .unwrap();

    let cli = CommandLineInterface::new().await.unwrap();
    let result = cli.run_file(&test_file, true).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_history_empty() {
    let cli = CommandLineInterface::new().await.unwrap();
    let result = cli.show_history(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_stats() {
    let cli = CommandLineInterface::new().await.unwrap();
    let result = cli.show_stats().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_clear_history_with_force() {
    let cli = CommandLineInterface::new().await.unwrap();
    let result = cli.clear_history(true).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_shutdown_handler_creation() {
    let handler = ShutdownHandler::new();
    assert!(!handler.is_shutdown_requested());
}

#[tokio::test]
async fn test_shutdown_handler_cleanup() {
    let handler = ShutdownHandler::new();
    let result = handler.cleanup().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_shutdown_handler_timeout() {
    let handler = ShutdownHandler::new();

    // This should timeout since we're not sending any signals
    let result = timeout(Duration::from_millis(100), handler.wait_for_shutdown()).await;
    assert!(result.is_err()); // Should timeout
}

#[tokio::test]
async fn test_start_watching_nonexistent_directory() {
    let cli = CommandLineInterface::new().await.unwrap();
    let nonexistent_path = PathBuf::from("/nonexistent/directory");

    let result = cli.start_watching(&nonexistent_path, false).await;
    assert!(result.is_ok()); // Should handle gracefully
}

#[tokio::test]
async fn test_cli_help_display() {
    let cli = CommandLineInterface::new().await.unwrap();
    // This should not panic
    cli.display_help();
}

#[tokio::test]
async fn test_cli_default_command() {
    // Test CLI with no subcommand (should show help)
    let cli = Cli::try_parse_from(&["learning-programming-app"]).unwrap();
    assert!(cli.command.is_none());
}

#[tokio::test]
async fn test_cli_invalid_command() {
    // Test invalid command
    let result = Cli::try_parse_from(&["learning-programming-app", "invalid"]);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_watch_command_defaults() {
    let cli = Cli::try_parse_from(&["learning-programming-app", "watch"]).unwrap();

    if let Some(Commands::Watch { directory, verbose }) = cli.command {
        assert_eq!(directory, PathBuf::from("./examples"));
        assert!(!verbose);
    } else {
        panic!("Expected Watch command");
    }
}

#[tokio::test]
async fn test_history_command_defaults() {
    let cli = Cli::try_parse_from(&["learning-programming-app", "history"]).unwrap();

    if let Some(Commands::History { limit }) = cli.command {
        assert_eq!(limit, 10);
    } else {
        panic!("Expected History command");
    }
}

#[tokio::test]
async fn test_sections_command_defaults() {
    let cli = Cli::try_parse_from(&["learning-programming-app", "sections"]).unwrap();

    if let Some(Commands::Sections { directory }) = cli.command {
        assert_eq!(directory, PathBuf::from("./examples"));
    } else {
        panic!("Expected Sections command");
    }
}
