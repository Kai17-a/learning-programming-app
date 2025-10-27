mod cli;
mod core;
mod generators;
mod handlers;
mod utils;

use anyhow::Result;
use cli::CommandLineInterface;
use tracing::info;

/// Main application entry point that integrates all system components
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing with proper configuration
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "learning_programming_app=info".into()),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    info!("Starting Learning Programming App");

    // Create integrated CLI interface
    let cli = CommandLineInterface::new().await?;

    // Run the application with proper error handling and graceful shutdown
    let result = cli.run().await;

    // Perform graceful shutdown
    if let Err(e) = cli.shutdown().await {
        eprintln!("Shutdown error: {}", e);
    }

    match result {
        Ok(()) => {
            info!("Application completed successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}
