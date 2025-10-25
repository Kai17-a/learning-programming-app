use crate::cli::commands::{Cli, Commands};
use crate::cli::shutdown::ShutdownHandler;
use crate::core::ApplicationService;
use anyhow::Result;
use console::{style, Term};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn};

#[derive(Clone)]
pub struct CommandLineInterface {
    app_service: Arc<ApplicationService>,
    term: Term,
}

impl CommandLineInterface {
    pub async fn new() -> Result<Self> {
        info!("Initializing CLI interface");
        
        // Initialize integrated application service
        let app_service = Arc::new(ApplicationService::new("./learning_app.db").await?);
        
        // Perform health check
        app_service.health_check().await?;
        
        Ok(Self {
            app_service,
            term: Term::stdout(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let cli = Cli::parse_args();
        
        match cli.command {
            Some(command) => self.handle_command(command).await,
            None => {
                self.display_help();
                Ok(())
            }
        }
    }

    async fn handle_command(&self, command: Commands) -> Result<()> {
        match command {
            Commands::Watch { directory, verbose } => {
                self.start_watching(&directory, verbose).await
            }
            Commands::Sections { directory } => {
                self.display_sections(&directory).await?;
                Ok(())
            }
            Commands::History { limit } => {
                self.show_history(limit).await
            }
            Commands::Stats => {
                self.show_stats().await
            }
            Commands::Clear { force } => {
                self.clear_history(force).await
            }
            Commands::Run { file, verbose } => {
                self.run_file(&file, verbose).await
            }
        }
    }

    pub fn display_help(&self) {
        println!("{}", style("Learning Programming App").bold().cyan());
        println!("{}", style("A file-watching automatic code execution CLI for programming learning").dim());
        println!();
        println!("Available commands:");
        println!("  {} - Start watching a directory for file changes", style("watch").green());
        println!("  {} - List available sections", style("sections").green());
        println!("  {} - Show execution history", style("history").green());
        println!("  {} - Show execution statistics", style("stats").green());
        println!("  {} - Clear execution history", style("clear").green());
        println!("  {} - Execute a specific file once", style("run").green());
        println!();
        println!("Use {} for more information about a specific command.", style("--help").yellow());
    }

    pub async fn display_sections(&self, directory: &Path) -> Result<Vec<String>> {
        let mut sections = Vec::new();
        
        if !directory.exists() {
            println!("{} Directory {} does not exist", 
                style("✗").red().bold(), 
                style(directory.display()).cyan()
            );
            return Ok(sections);
        }

        println!("{} Available sections in {}:", 
            style("📚").bold(), 
            style(directory.display()).cyan()
        );
        
        let mut entries = tokio::fs::read_dir(directory).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let section_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                sections.push(section_name.to_string());
                
                // Count files in section
                let mut file_count = 0;
                if let Ok(mut section_entries) = tokio::fs::read_dir(&path).await {
                    while let Some(file_entry) = section_entries.next_entry().await? {
                        if let Some(ext) = file_entry.path().extension().and_then(|ext| ext.to_str()) {
                            if ext == "py" || ext == "go" {
                                file_count += 1;
                            }
                        }
                    }
                }
                
                println!("  {} {} ({} files)", 
                    style("•").blue(), 
                    style(section_name).bold(),
                    style(file_count).dim()
                );
            }
        }
        
        if sections.is_empty() {
            println!("  {}", style("No sections found").dim());
        }
        
        Ok(sections)
    }

    pub async fn start_watching(&self, directory: &Path, verbose: bool) -> Result<()> {
        if !directory.exists() {
            println!("{} Directory {} does not exist", 
                style("✗").red().bold(), 
                style(directory.display()).cyan()
            );
            return Ok(());
        }

        println!("{} Starting to watch directory: {}", 
            style("👀").bold(), 
            style(directory.display()).cyan()
        );
        
        if verbose {
            println!("{} Verbose mode enabled", style("ℹ").blue());
        }

        // Create channel for file change events
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // Start watching
        self.app_service.watcher.start_watching(directory, tx).await?;
        
        println!("{} Watching for changes... Press {} to stop", 
            style("✓").green().bold(),
            style("Ctrl+C").yellow()
        );

        // Create shutdown handler
        let shutdown_handler = ShutdownHandler::new();
        
        // Handle file change events and shutdown signals
        tokio::select! {
            _ = shutdown_handler.wait_for_shutdown() => {
                // Shutdown signal received, cleanup will be handled by the shutdown handler
            }
            _ = async {
                while let Some(file_path) = rx.recv().await {
                    // Check for shutdown during file processing
                    if shutdown_handler.is_shutdown_requested() {
                        break;
                    }
                    
                    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
                        if extension == "py" || extension == "go" {
                            if verbose {
                                println!("{} File changed: {}", 
                                    style("📝").bold(), 
                                    style(file_path.display()).cyan()
                                );
                            }
                            
                            let result = self.app_service.executor.execute_file(&file_path).await;
                            let output = self.app_service.executor.format_output(&result);
                            println!("{}", output);
                        }
                    }
                }
            } => {}
        }

        // Stop watching
        self.app_service.watcher.stop_watching().await?;
        println!("{} Stopped watching", style("✓").green().bold());
        
        Ok(())
    }

    pub async fn show_history(&self, limit: u32) -> Result<()> {
        println!("{} Recent executions (last {}):", 
            style("📋").bold(), 
            style(limit).cyan()
        );
        
        let history = self.app_service.history.get_history(Some(limit)).await?;
        
        if history.is_empty() {
            println!("  {}", style("No execution history found").dim());
            return Ok(());
        }

        for record in history {
            let status_icon = if record.success {
                style("✓").green()
            } else {
                style("✗").red()
            };
            
            let time_str = record.timestamp.format("%Y-%m-%d %H:%M:%S");
            
            println!("  {} {} {} ({}s) - {}", 
                status_icon,
                style(time_str).dim(),
                style(&record.file_path).cyan(),
                style(format!("{:.3}", record.execution_time)).yellow(),
                style(&record.section).blue()
            );
            
            if !record.output_preview.is_empty() {
                println!("    {}", style(&record.output_preview).dim());
            }
        }
        
        Ok(())
    }

    pub async fn show_stats(&self) -> Result<()> {
        println!("{} Execution Statistics:", style("📊").bold());
        
        let stats = self.app_service.history.get_stats().await?;
        
        println!("  Total executions: {}", style(stats.total_executions).cyan());
        println!("  Successful: {} ({}%)", 
            style(stats.successful_executions).green(),
            style(format!("{:.1}", stats.success_rate() * 100.0)).green()
        );
        println!("  Failed: {}", style(stats.failed_executions).red());
        
        if let Some(avg_time) = (stats.average_execution_time > 0.0).then_some(stats.average_execution_time) {
            println!("  Average execution time: {}s", 
                style(format!("{:.3}", avg_time)).yellow()
            );
        }
        
        if let Some(most_executed) = &stats.most_executed_file {
            println!("  Most executed file: {}", style(most_executed).cyan());
        }
        
        if let Some(last_execution) = stats.last_execution {
            println!("  Last execution: {}", 
                style(last_execution.format("%Y-%m-%d %H:%M:%S")).dim()
            );
        }
        
        Ok(())
    }

    pub async fn clear_history(&self, force: bool) -> Result<()> {
        if !force {
            print!("Are you sure you want to clear all execution history? [y/N]: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if !input.trim().to_lowercase().starts_with('y') {
                println!("{} Operation cancelled", style("ℹ").blue());
                return Ok(());
            }
        }
        
        self.app_service.history.clear_history().await?;
        println!("{} Execution history cleared", style("✓").green().bold());
        
        Ok(())
    }

    pub async fn run_file(&self, file_path: &Path, verbose: bool) -> Result<()> {
        if !file_path.exists() {
            println!("{} File {} does not exist", 
                style("✗").red().bold(), 
                style(file_path.display()).cyan()
            );
            return Ok(());
        }

        if verbose {
            println!("{} Executing file: {}", 
                style("🚀").bold(), 
                style(file_path.display()).cyan()
            );
        }

        let result = self.app_service.executor.execute_file(file_path).await;
        let output = self.app_service.executor.format_output(&result);
        println!("{}", output);
        
        Ok(())
    }

    pub fn is_watching(&self) -> bool {
        self.app_service.watcher.is_watching()
    }

    pub async fn get_watched_directories(&self) -> Vec<PathBuf> {
        self.app_service.watcher.get_watched_directories().await
    }

    /// Get system status information
    pub async fn get_system_status(&self) -> crate::core::SystemStatus {
        self.app_service.get_system_status().await
    }

    /// Perform graceful shutdown
    pub async fn shutdown(&self) -> Result<()> {
        info!("CLI shutting down gracefully");
        self.app_service.shutdown().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

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
        let section_path = temp_dir.path().join("section1");
        fs::create_dir(&section_path).await.unwrap();
        fs::write(section_path.join("test.py"), "print('hello')").await.unwrap();
        
        let cli = CommandLineInterface::new().await.unwrap();
        let sections = cli.display_sections(temp_dir.path()).await.unwrap();
        
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0], "section1");
    }

    #[tokio::test]
    async fn test_run_file_nonexistent() {
        let cli = CommandLineInterface::new().await.unwrap();
        let result = cli.run_file(Path::new("nonexistent.py"), false).await;
        assert!(result.is_ok());
    }
}