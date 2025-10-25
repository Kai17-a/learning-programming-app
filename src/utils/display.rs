use console::{style, Term};
use tracing::{info, warn, error};
use chrono::Utc;

use crate::core::models::{ExecutionResult, ExecutionStats};

/// Service for displaying execution results with beautiful formatting
pub struct DisplayService {
    term: Term,
    show_timestamps: bool,
    show_execution_time: bool,
}

impl DisplayService {
    /// Create a new DisplayService
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
            show_timestamps: true,
            show_execution_time: true,
        }
    }
    
    /// Create a DisplayService with custom settings
    pub fn with_settings(show_timestamps: bool, show_execution_time: bool) -> Self {
        Self {
            term: Term::stdout(),
            show_timestamps,
            show_execution_time,
        }
    }
    
    /// Display execution result with beautiful formatting
    pub fn display_execution_result(&self, result: &ExecutionResult) -> std::io::Result<()> {
        let file_name = result.file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
            
        // Create header line
        let mut header_parts = Vec::new();
        
        // Status icon and text
        if result.success {
            header_parts.push(format!("{} {}", 
                style("✓").green().bold(), 
                style("SUCCESS").green().bold()
            ));
        } else {
            header_parts.push(format!("{} {}", 
                style("✗").red().bold(), 
                style("FAILED").red().bold()
            ));
        }
        
        // File name
        header_parts.push(style(file_name).cyan().bold().to_string());
        
        // Timestamp
        if self.show_timestamps {
            let timestamp = result.timestamp.format("%H:%M:%S");
            header_parts.push(format!("at {}", style(timestamp).dim()));
        }
        
        // Execution time
        if self.show_execution_time {
            let execution_time = if result.execution_time.as_millis() > 0 {
                format!("{}ms", result.execution_time.as_millis())
            } else {
                format!("{}μs", result.execution_time.as_micros())
            };
            header_parts.push(format!("({})", style(execution_time).dim()));
        }
        
        // Print header
        let header = header_parts.join(" ");
        self.term.write_line(&header)?;
        
        // Print separator
        let separator = "─".repeat(60);
        self.term.write_line(&style(separator).dim().to_string())?;
        
        // Print output or error
        if result.success {
            if !result.output.is_empty() {
                // Print output with proper formatting
                for line in result.output.lines() {
                    self.term.write_line(&format!("  {}", line))?;
                }
            } else {
                self.term.write_line(&format!("  {}", style("(no output)").dim()))?;
            }
        } else {
            // Print error message
            if let Some(error_msg) = &result.error_message {
                for line in error_msg.lines() {
                    self.term.write_line(&format!("  {}", style(line).red()))?;
                }
            } else {
                self.term.write_line(&format!("  {}", style("Unknown error").red()))?;
            }
            
            // Print exit code if available
            if let Some(exit_code) = result.exit_code {
                self.term.write_line(&format!("  {} {}", 
                    style("Exit code:").dim(), 
                    style(exit_code).red().bold()
                ))?;
            }
        }
        
        // Print empty line for spacing
        self.term.write_line("")?;
        
        // Log the execution result
        self.log_execution_result(result);
        
        Ok(())
    }
    
    /// Display a file change notification
    pub fn display_file_change(&self, file_path: &std::path::Path) -> std::io::Result<()> {
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
            
        let timestamp = Utc::now().format("%H:%M:%S");
        
        let message = format!(
            "{} {} {} {} {}",
            style("📝").bold(),
            style(timestamp).dim(),
            style("File changed:").blue(),
            style(file_name).cyan().bold(),
            style("- executing...").dim()
        );
        
        self.term.write_line(&message)?;
        
        info!("File changed: {}", file_path.display());
        
        Ok(())
    }
    
    /// Display application startup message
    pub fn display_startup(&self, watch_dir: &std::path::Path, supported_extensions: &[String]) -> std::io::Result<()> {
        // Clear screen
        self.term.clear_screen()?;
        
        // Display banner
        let banner = vec![
            "╔══════════════════════════════════════════════════════════════╗",
            "║                Learning Programming App                      ║",
            "║              File Watcher & Auto Executor                   ║",
            "╚══════════════════════════════════════════════════════════════╝",
        ];
        
        for line in banner {
            self.term.write_line(&style(line).cyan().bold().to_string())?;
        }
        
        self.term.write_line("")?;
        
        // Display configuration
        self.term.write_line(&format!("📁 Watching directory: {}", 
            style(watch_dir.display()).green().bold()))?;
            
        self.term.write_line(&format!("📄 Supported files: {}", 
            style(format!(".{}", supported_extensions.join(", ."))).yellow()))?;
            
        self.term.write_line("")?;
        
        // Display instructions
        let instructions = vec![
            "💡 Instructions:",
            "   • Save any supported file to execute it automatically",
            "   • Press Ctrl+C to stop watching and exit",
            "   • Check the output below for execution results",
        ];
        
        for instruction in instructions {
            self.term.write_line(&style(instruction).dim().to_string())?;
        }
        
        self.term.write_line("")?;
        self.term.write_line(&style("Waiting for file changes...").green().to_string())?;
        self.term.write_line(&style("═".repeat(60)).dim().to_string())?;
        self.term.write_line("")?;
        
        info!("Application started, watching directory: {}", watch_dir.display());
        
        Ok(())
    }
    
    /// Display statistics
    pub fn display_stats(&self, stats: &ExecutionStats) -> std::io::Result<()> {
        self.term.write_line(&style("📊 Execution Statistics").blue().bold().to_string())?;
        self.term.write_line(&style("─".repeat(30)).dim().to_string())?;
        
        self.term.write_line(&format!("Total executions: {}", 
            style(stats.total_executions).bold()))?;
            
        self.term.write_line(&format!("Successful: {} ({}%)", 
            style(stats.successful_executions).green().bold(),
            style(format!("{:.1}", stats.success_rate() * 100.0)).green()))?;
            
        self.term.write_line(&format!("Failed: {} ({}%)", 
            style(stats.failed_executions).red().bold(),
            style(format!("{:.1}", stats.failure_rate() * 100.0)).red()))?;
            
        if let Some(most_executed) = &stats.most_executed_file {
            self.term.write_line(&format!("Most executed: {}", 
                style(most_executed).cyan()))?;
        }
        
        if stats.average_execution_time > 0.0 {
            self.term.write_line(&format!("Average time: {}ms", 
                style(format!("{:.1}", stats.average_execution_time * 1000.0)).dim()))?;
        }
        
        if let Some(last_execution) = &stats.last_execution {
            self.term.write_line(&format!("Last execution: {}", 
                style(last_execution.format("%Y-%m-%d %H:%M:%S")).dim()))?;
        }
        
        self.term.write_line("")?;
        
        Ok(())
    }
    
    /// Display error message
    pub fn display_error(&self, message: &str) -> std::io::Result<()> {
        let formatted_message = format!("{} {}", 
            style("❌ Error:").red().bold(), 
            style(message).red()
        );
        
        self.term.write_line(&formatted_message)?;
        error!("{}", message);
        
        Ok(())
    }
    
    /// Display warning message
    pub fn display_warning(&self, message: &str) -> std::io::Result<()> {
        let formatted_message = format!("{} {}", 
            style("⚠️  Warning:").yellow().bold(), 
            style(message).yellow()
        );
        
        self.term.write_line(&formatted_message)?;
        warn!("{}", message);
        
        Ok(())
    }
    
    /// Display info message
    pub fn display_info(&self, message: &str) -> std::io::Result<()> {
        let formatted_message = format!("{} {}", 
            style("ℹ️  Info:").blue().bold(), 
            message
        );
        
        self.term.write_line(&formatted_message)?;
        info!("{}", message);
        
        Ok(())
    }
    
    /// Log execution result using tracing
    fn log_execution_result(&self, result: &ExecutionResult) {
        let file_path = result.file_path.display().to_string();
        let execution_time_ms = result.execution_time.as_millis();
        
        if result.success {
            info!(
                file_path = %file_path,
                execution_time_ms = execution_time_ms,
                output_length = result.output.len(),
                "File executed successfully"
            );
        } else {
            error!(
                file_path = %file_path,
                execution_time_ms = execution_time_ms,
                error_message = %result.error_message.as_deref().unwrap_or("Unknown error"),
                exit_code = result.exit_code,
                "File execution failed"
            );
        }
    }
    
    /// Clear the terminal screen
    pub fn clear_screen(&self) -> std::io::Result<()> {
        self.term.clear_screen()
    }
    
    /// Move cursor to the beginning of the line
    pub fn move_cursor_to_start(&self) -> std::io::Result<()> {
        self.term.move_cursor_to(0, self.term.size().0 as usize)
    }
}

impl Default for DisplayService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Duration;
    
    #[test]
    fn test_display_service_creation() {
        let display = DisplayService::new();
        assert!(display.show_timestamps);
        assert!(display.show_execution_time);
        
        let display_custom = DisplayService::with_settings(false, true);
        assert!(!display_custom.show_timestamps);
        assert!(display_custom.show_execution_time);
    }
    
    #[test]
    fn test_log_execution_result() {
        let display = DisplayService::new();
        
        let success_result = ExecutionResult::new(PathBuf::from("test.py"))
            .with_success("Hello".to_string(), Duration::from_millis(100), 0);
            
        // This should not panic
        display.log_execution_result(&success_result);
        
        let error_result = ExecutionResult::new(PathBuf::from("test.py"))
            .with_error("Syntax error".to_string(), Duration::from_millis(50), Some(1));
            
        // This should not panic
        display.log_execution_result(&error_result);
    }
}