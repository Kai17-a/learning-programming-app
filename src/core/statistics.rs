use anyhow::Result;
use console::{style, Term};
use std::sync::Arc;
use tracing::info;

use crate::core::history::HistoryManagerService;
use crate::core::models::ExecutionStats;

/// StatisticsService provides comprehensive statistics display and analysis
pub struct StatisticsService {
    pub history_manager: Arc<HistoryManagerService>,
}

impl StatisticsService {
    /// Create a new StatisticsService
    pub fn new(history_manager: Arc<HistoryManagerService>) -> Self {
        Self {
            history_manager,
        }
    }

    /// Display comprehensive statistics overview
    pub async fn display_overview(&self) -> Result<()> {
        info!("Displaying statistics overview");
        
        let stats = self.history_manager.get_stats().await?;
        
        let term = Term::stdout();
        term.write_line(&style("📊 Execution Statistics").blue().bold().to_string())?;
        term.write_line(&style("─".repeat(30)).dim().to_string())?;
        
        term.write_line(&format!("Total executions: {}", 
            style(stats.total_executions).bold()))?;
            
        term.write_line(&format!("Successful: {} ({}%)", 
            style(stats.successful_executions).green().bold(),
            style(format!("{:.1}", stats.success_rate() * 100.0)).green()))?;
            
        term.write_line(&format!("Failed: {} ({}%)", 
            style(stats.failed_executions).red().bold(),
            style(format!("{:.1}", stats.failure_rate() * 100.0)).red()))?;
            
        if let Some(most_executed) = &stats.most_executed_file {
            term.write_line(&format!("Most executed: {}", 
                style(most_executed).cyan()))?;
        }
        
        if stats.average_execution_time > 0.0 {
            term.write_line(&format!("Average time: {}ms", 
                style(format!("{:.1}", stats.average_execution_time * 1000.0)).dim()))?;
        }
        
        if let Some(last_execution) = &stats.last_execution {
            term.write_line(&format!("Last execution: {}", 
                style(last_execution.format("%Y-%m-%d %H:%M:%S")).dim()))?;
        }
        
        term.write_line("")?;
        
        Ok(())
    }

    /// Display statistics for a specific file
    pub async fn display_file_stats(&self, file_path: impl AsRef<str>) -> Result<()> {
        let file_path = file_path.as_ref();
        info!("Displaying statistics for file: {}", file_path);
        
        let stats = self.history_manager.get_file_stats(file_path).await?;
        
        let term = Term::stdout();
        term.write_line(&format!("📄 {} Statistics", 
            style(file_path).cyan().bold()))?;
        term.write_line(&style("─".repeat(50)).dim().to_string())?;
        
        if stats.total_executions == 0 {
            term.write_line(&style("No execution history found for this file.").dim().to_string())?;
            return Ok(());
        }
        
        term.write_line(&format!("Total executions: {}", 
            style(stats.total_executions).bold()))?;
            
        term.write_line(&format!("Successful: {} ({}%)", 
            style(stats.successful_executions).green().bold(),
            style(format!("{:.1}", stats.success_rate() * 100.0)).green()))?;
            
        term.write_line(&format!("Failed: {} ({}%)", 
            style(stats.failed_executions).red().bold(),
            style(format!("{:.1}", stats.failure_rate() * 100.0)).red()))?;
            
        if stats.average_execution_time > 0.0 {
            term.write_line(&format!("Average execution time: {}ms", 
                style(format!("{:.1}", stats.average_execution_time * 1000.0)).dim()))?;
        }
        
        if let Some(last_execution) = &stats.last_execution {
            term.write_line(&format!("Last execution: {}", 
                style(last_execution.format("%Y-%m-%d %H:%M:%S")).dim()))?;
        }
        
        term.write_line("")?;
        
        Ok(())
    }

    /// Display statistics for a specific section
    pub async fn display_section_stats(&self, section: impl AsRef<str>) -> Result<()> {
        let section = section.as_ref();
        info!("Displaying statistics for section: {}", section);
        
        let stats = self.history_manager.get_section_stats(section).await?;
        
        let term = Term::stdout();
        term.write_line(&format!("📁 {} Section Statistics", 
            style(section).cyan().bold()))?;
        term.write_line(&style("─".repeat(50)).dim().to_string())?;
        
        if stats.total_executions == 0 {
            term.write_line(&style("No execution history found for this section.").dim().to_string())?;
            return Ok(());
        }
        
        term.write_line(&format!("Total executions: {}", 
            style(stats.total_executions).bold()))?;
            
        term.write_line(&format!("Successful: {} ({}%)", 
            style(stats.successful_executions).green().bold(),
            style(format!("{:.1}", stats.success_rate() * 100.0)).green()))?;
            
        term.write_line(&format!("Failed: {} ({}%)", 
            style(stats.failed_executions).red().bold(),
            style(format!("{:.1}", stats.failure_rate() * 100.0)).red()))?;
            
        if let Some(most_executed) = &stats.most_executed_file {
            term.write_line(&format!("Most executed file: {}", 
                style(most_executed).cyan()))?;
        }
        
        if stats.average_execution_time > 0.0 {
            term.write_line(&format!("Average execution time: {}ms", 
                style(format!("{:.1}", stats.average_execution_time * 1000.0)).dim()))?;
        }
        
        if let Some(last_execution) = &stats.last_execution {
            term.write_line(&format!("Last execution: {}", 
                style(last_execution.format("%Y-%m-%d %H:%M:%S")).dim()))?;
        }
        
        term.write_line("")?;
        
        Ok(())
    }

    /// Clear statistics (clear history)
    pub async fn clear_statistics(&self) -> Result<()> {
        info!("Clearing all statistics");
        
        self.history_manager.clear_history().await?;
        
        let term = Term::stdout();
        term.write_line(&format!("{} All statistics have been cleared.", 
            style("✓").green().bold()))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::sync::Arc;

    async fn create_test_service() -> StatisticsService {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path();
        let history_manager = Arc::new(HistoryManagerService::new(db_path).await.unwrap());
        StatisticsService::new(history_manager)
    }

    #[tokio::test]
    async fn test_statistics_service_creation() {
        let service = create_test_service().await;
        
        // Should not panic when displaying empty statistics
        let result = service.display_overview().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clear_statistics() {
        let service = create_test_service().await;
        
        // Clear statistics should work even when empty
        let result = service.clear_statistics().await;
        assert!(result.is_ok());
        
        // Verify data is cleared
        let stats = service.history_manager.get_stats().await.unwrap();
        assert_eq!(stats.total_executions, 0);
    }
}