use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::path::Path;
use tracing::{debug, info};

use crate::core::models::{ExecutionRecord, ExecutionStats};

/// HistoryManagerService manages execution history using SQLite database
#[derive(Debug, Clone)]
pub struct HistoryManagerService {
    db_pool: SqlitePool,
}

impl HistoryManagerService {
    /// Create a new HistoryManagerService with SQLite database
    pub async fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let path = db_path.as_ref();
        
        // Ensure the parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        
        let db_url = format!("sqlite:{}?mode=rwc", path.display());
        debug!("Connecting to database: {}", db_url);
        
        let db_pool = SqlitePool::connect(&db_url).await?;
        
        let service = Self { db_pool };
        service.init_database().await?;
        
        info!("HistoryManagerService initialized with database: {}", db_url);
        Ok(service)
    }

    /// Record an execution in the database
    pub async fn record_execution(&self, execution: &ExecutionRecord) -> Result<()> {
        debug!("Recording execution: {}", execution.id);
        
        sqlx::query(
            "INSERT INTO execution_history (id, file_path, section, success, execution_time, timestamp, output_preview) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&execution.id)
        .bind(&execution.file_path)
        .bind(&execution.section)
        .bind(execution.success)
        .bind(execution.execution_time)
        .bind(&execution.timestamp)
        .bind(&execution.output_preview)
        .execute(&self.db_pool)
        .await?;

        debug!("Successfully recorded execution: {}", execution.id);
        Ok(())
    }

    /// Get execution history with optional limit
    pub async fn get_history(&self, limit: Option<u32>) -> Result<Vec<ExecutionRecord>> {
        debug!("Fetching execution history with limit: {:?}", limit);
        
        let query_str = if limit.is_some() {
            "SELECT id, file_path, section, success, execution_time, timestamp, output_preview FROM execution_history ORDER BY timestamp DESC LIMIT ?"
        } else {
            "SELECT id, file_path, section, success, execution_time, timestamp, output_preview FROM execution_history ORDER BY timestamp DESC"
        };
        
        let mut query = sqlx::query(query_str);
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        
        let rows = query.fetch_all(&self.db_pool).await?;
        
        let records: Vec<ExecutionRecord> = rows.into_iter().map(|row| {
            ExecutionRecord {
                id: row.get("id"),
                file_path: row.get("file_path"),
                section: row.get("section"),
                success: row.get("success"),
                execution_time: row.get("execution_time"),
                timestamp: row.get("timestamp"),
                output_preview: row.get("output_preview"),
            }
        }).collect();

        debug!("Retrieved {} execution records", records.len());
        Ok(records)
    }

    /// Get execution statistics
    pub async fn get_stats(&self) -> Result<ExecutionStats> {
        debug!("Calculating execution statistics");
        
        // Get basic counts
        let total_row = sqlx::query("SELECT COUNT(*) as total FROM execution_history")
            .fetch_one(&self.db_pool)
            .await?;
        
        let total_executions = total_row.get::<i64, _>("total") as u64;
        
        if total_executions == 0 {
            return Ok(ExecutionStats::default());
        }

        // Get success/failure counts
        let success_row = sqlx::query("SELECT COUNT(*) as successful FROM execution_history WHERE success = true")
            .fetch_one(&self.db_pool)
            .await?;
        
        let successful_executions = success_row.get::<i64, _>("successful") as u64;
        let failed_executions = total_executions - successful_executions;

        // Get average execution time
        let avg_time_row = sqlx::query("SELECT AVG(execution_time) as avg_time FROM execution_history")
            .fetch_one(&self.db_pool)
            .await?;
        
        let average_execution_time = avg_time_row.get::<Option<f64>, _>("avg_time").unwrap_or(0.0);

        // Get most executed file
        let most_executed_row = sqlx::query(
            "SELECT file_path, COUNT(*) as count FROM execution_history GROUP BY file_path ORDER BY count DESC LIMIT 1"
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        let most_executed_file = most_executed_row.map(|row| row.get::<String, _>("file_path"));

        // Get last execution timestamp
        let last_execution_row = sqlx::query("SELECT MAX(timestamp) as last_timestamp FROM execution_history")
            .fetch_one(&self.db_pool)
            .await?;
        
        let last_execution = last_execution_row.get::<Option<DateTime<Utc>>, _>("last_timestamp");

        let stats = ExecutionStats {
            total_executions,
            successful_executions,
            failed_executions,
            most_executed_file,
            average_execution_time,
            last_execution,
        };

        debug!("Calculated stats: {:?}", stats);
        Ok(stats)
    }

    /// Clear all execution history
    pub async fn clear_history(&self) -> Result<()> {
        info!("Clearing all execution history");
        
        sqlx::query("DELETE FROM execution_history")
            .execute(&self.db_pool)
            .await?;
        
        info!("Successfully cleared execution history");
        Ok(())
    }

    /// Get execution history for a specific file
    pub async fn get_file_history(&self, file_path: impl AsRef<str>) -> Result<Vec<ExecutionRecord>> {
        let file_path = file_path.as_ref();
        debug!("Fetching history for file: {}", file_path);
        
        let rows = sqlx::query(
            "SELECT id, file_path, section, success, execution_time, timestamp, output_preview FROM execution_history WHERE file_path = ? ORDER BY timestamp DESC"
        )
        .bind(file_path)
        .fetch_all(&self.db_pool)
        .await?;

        let records: Vec<ExecutionRecord> = rows.into_iter().map(|row| {
            ExecutionRecord {
                id: row.get("id"),
                file_path: row.get("file_path"),
                section: row.get("section"),
                success: row.get("success"),
                execution_time: row.get("execution_time"),
                timestamp: row.get("timestamp"),
                output_preview: row.get("output_preview"),
            }
        }).collect();

        debug!("Retrieved {} records for file: {}", records.len(), file_path);
        Ok(records)
    }

    /// Get execution history for a specific section
    pub async fn get_section_history(&self, section: impl AsRef<str>) -> Result<Vec<ExecutionRecord>> {
        let section = section.as_ref();
        debug!("Fetching history for section: {}", section);
        
        let rows = sqlx::query(
            "SELECT id, file_path, section, success, execution_time, timestamp, output_preview FROM execution_history WHERE section = ? ORDER BY timestamp DESC"
        )
        .bind(section)
        .fetch_all(&self.db_pool)
        .await?;

        let records: Vec<ExecutionRecord> = rows.into_iter().map(|row| {
            ExecutionRecord {
                id: row.get("id"),
                file_path: row.get("file_path"),
                section: row.get("section"),
                success: row.get("success"),
                execution_time: row.get("execution_time"),
                timestamp: row.get("timestamp"),
                output_preview: row.get("output_preview"),
            }
        }).collect();

        debug!("Retrieved {} records for section: {}", records.len(), section);
        Ok(records)
    }

    /// Get statistics for a specific file
    pub async fn get_file_stats(&self, file_path: impl AsRef<str>) -> Result<ExecutionStats> {
        let file_path = file_path.as_ref();
        debug!("Calculating statistics for file: {}", file_path);
        
        let total_row = sqlx::query("SELECT COUNT(*) as total FROM execution_history WHERE file_path = ?")
            .bind(file_path)
            .fetch_one(&self.db_pool)
            .await?;
        
        let total_executions = total_row.get::<i64, _>("total") as u64;
        
        if total_executions == 0 {
            return Ok(ExecutionStats::default());
        }

        let success_row = sqlx::query("SELECT COUNT(*) as successful FROM execution_history WHERE file_path = ? AND success = true")
            .bind(file_path)
            .fetch_one(&self.db_pool)
            .await?;
        
        let successful_executions = success_row.get::<i64, _>("successful") as u64;
        let failed_executions = total_executions - successful_executions;

        let avg_time_row = sqlx::query("SELECT AVG(execution_time) as avg_time FROM execution_history WHERE file_path = ?")
            .bind(file_path)
            .fetch_one(&self.db_pool)
            .await?;
        
        let average_execution_time = avg_time_row.get::<Option<f64>, _>("avg_time").unwrap_or(0.0);

        let last_execution_row = sqlx::query("SELECT MAX(timestamp) as last_timestamp FROM execution_history WHERE file_path = ?")
            .bind(file_path)
            .fetch_one(&self.db_pool)
            .await?;
        
        let last_execution = last_execution_row.get::<Option<DateTime<Utc>>, _>("last_timestamp");

        Ok(ExecutionStats {
            total_executions,
            successful_executions,
            failed_executions,
            most_executed_file: Some(file_path.to_string()),
            average_execution_time,
            last_execution,
        })
    }

    /// Get statistics for a specific section
    pub async fn get_section_stats(&self, section: impl AsRef<str>) -> Result<ExecutionStats> {
        let section = section.as_ref();
        debug!("Calculating statistics for section: {}", section);
        
        let total_row = sqlx::query("SELECT COUNT(*) as total FROM execution_history WHERE section = ?")
            .bind(section)
            .fetch_one(&self.db_pool)
            .await?;
        
        let total_executions = total_row.get::<i64, _>("total") as u64;
        
        if total_executions == 0 {
            return Ok(ExecutionStats::default());
        }

        let success_row = sqlx::query("SELECT COUNT(*) as successful FROM execution_history WHERE section = ? AND success = true")
            .bind(section)
            .fetch_one(&self.db_pool)
            .await?;
        
        let successful_executions = success_row.get::<i64, _>("successful") as u64;
        let failed_executions = total_executions - successful_executions;

        let avg_time_row = sqlx::query("SELECT AVG(execution_time) as avg_time FROM execution_history WHERE section = ?")
            .bind(section)
            .fetch_one(&self.db_pool)
            .await?;
        
        let average_execution_time = avg_time_row.get::<Option<f64>, _>("avg_time").unwrap_or(0.0);

        let most_executed_row = sqlx::query(
            "SELECT file_path, COUNT(*) as count FROM execution_history WHERE section = ? GROUP BY file_path ORDER BY count DESC LIMIT 1"
        )
        .bind(section)
        .fetch_optional(&self.db_pool)
        .await?;
        
        let most_executed_file = most_executed_row.map(|row| row.get::<String, _>("file_path"));

        let last_execution_row = sqlx::query("SELECT MAX(timestamp) as last_timestamp FROM execution_history WHERE section = ?")
            .bind(section)
            .fetch_one(&self.db_pool)
            .await?;
        
        let last_execution = last_execution_row.get::<Option<DateTime<Utc>>, _>("last_timestamp");

        Ok(ExecutionStats {
            total_executions,
            successful_executions,
            failed_executions,
            most_executed_file,
            average_execution_time,
            last_execution,
        })
    }

    /// Get recent execution trends (last N days)
    pub async fn get_execution_trends(&self, days: u32) -> Result<Vec<(String, u64, u64)>> {
        debug!("Calculating execution trends for last {} days", days);
        
        let rows = sqlx::query(
            "SELECT DATE(timestamp) as date, COUNT(*) as total, SUM(CASE WHEN success = true THEN 1 ELSE 0 END) as successful FROM execution_history WHERE timestamp >= datetime('now', '-' || ? || ' days') GROUP BY DATE(timestamp) ORDER BY date DESC"
        )
        .bind(days)
        .fetch_all(&self.db_pool)
        .await?;

        let trends: Vec<(String, u64, u64)> = rows
            .into_iter()
            .map(|row| {
                let date = row.get::<Option<String>, _>("date").unwrap_or_default();
                let total = row.get::<i64, _>("total") as u64;
                let successful = row.get::<Option<i64>, _>("successful").unwrap_or(0) as u64;
                (date, total, successful)
            })
            .collect();

        debug!("Retrieved {} trend data points", trends.len());
        Ok(trends)
    }

    /// Get top executed files with their statistics
    pub async fn get_top_files(&self, limit: u32) -> Result<Vec<(String, u64, f64)>> {
        debug!("Getting top {} executed files", limit);
        
        let rows = sqlx::query(
            "SELECT file_path, COUNT(*) as execution_count, AVG(execution_time) as avg_time FROM execution_history GROUP BY file_path ORDER BY execution_count DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        let top_files: Vec<(String, u64, f64)> = rows
            .into_iter()
            .map(|row| {
                let file_path = row.get::<String, _>("file_path");
                let count = row.get::<i64, _>("execution_count") as u64;
                let avg_time = row.get::<Option<f64>, _>("avg_time").unwrap_or(0.0);
                (file_path, count, avg_time)
            })
            .collect();

        debug!("Retrieved {} top files", top_files.len());
        Ok(top_files)
    }

    /// Check if the database connection is healthy
    pub async fn is_healthy(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_optional(&self.db_pool)
            .await
            .is_ok()
    }

    /// Close database connections
    pub async fn close(&self) -> Result<()> {
        self.db_pool.close().await;
        Ok(())
    }

    /// Initialize the database schema
    async fn init_database(&self) -> Result<()> {
        debug!("Initializing database schema");
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS execution_history (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                section TEXT NOT NULL,
                success BOOLEAN NOT NULL,
                execution_time REAL NOT NULL,
                timestamp TEXT NOT NULL,
                output_preview TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.db_pool)
        .await?;

        // Create indexes for better query performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_execution_history_timestamp ON execution_history(timestamp)")
            .execute(&self.db_pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_execution_history_file_path ON execution_history(file_path)")
            .execute(&self.db_pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_execution_history_section ON execution_history(section)")
            .execute(&self.db_pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_execution_history_success ON execution_history(success)")
            .execute(&self.db_pool)
            .await?;

        debug!("Database schema initialized successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    async fn create_test_service() -> HistoryManagerService {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path();
        HistoryManagerService::new(db_path).await.unwrap()
    }

    #[tokio::test]
    async fn test_new_service_creation() {
        let service = create_test_service().await;
        assert!(!service.db_pool.is_closed());
    }

    #[tokio::test]
    async fn test_record_and_get_execution() {
        let service = create_test_service().await;
        
        let record = ExecutionRecord {
            id: "test-id".to_string(),
            file_path: "test.py".to_string(),
            section: "test-section".to_string(),
            success: true,
            execution_time: 1.5,
            timestamp: Utc::now(),
            output_preview: "Hello, World!".to_string(),
        };

        service.record_execution(&record).await.unwrap();
        
        let history = service.get_history(None).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, "test-id");
        assert_eq!(history[0].file_path, "test.py");
    }

    #[tokio::test]
    async fn test_get_stats() {
        let service = create_test_service().await;
        
        // Initially empty
        let stats = service.get_stats().await.unwrap();
        assert_eq!(stats.total_executions, 0);
        
        // Add some records
        let record1 = ExecutionRecord {
            id: "test-1".to_string(),
            file_path: "test1.py".to_string(),
            section: "section1".to_string(),
            success: true,
            execution_time: 1.0,
            timestamp: Utc::now(),
            output_preview: "Success".to_string(),
        };
        
        let record2 = ExecutionRecord {
            id: "test-2".to_string(),
            file_path: "test2.py".to_string(),
            section: "section1".to_string(),
            success: false,
            execution_time: 2.0,
            timestamp: Utc::now(),
            output_preview: "Error".to_string(),
        };

        service.record_execution(&record1).await.unwrap();
        service.record_execution(&record2).await.unwrap();
        
        let stats = service.get_stats().await.unwrap();
        assert_eq!(stats.total_executions, 2);
        assert_eq!(stats.successful_executions, 1);
        assert_eq!(stats.failed_executions, 1);
        assert_eq!(stats.average_execution_time, 1.5);
    }

    #[tokio::test]
    async fn test_clear_history() {
        let service = create_test_service().await;
        
        let record = ExecutionRecord {
            id: "test-id".to_string(),
            file_path: "test.py".to_string(),
            section: "test-section".to_string(),
            success: true,
            execution_time: 1.0,
            timestamp: Utc::now(),
            output_preview: "Test".to_string(),
        };

        service.record_execution(&record).await.unwrap();
        
        let history = service.get_history(None).await.unwrap();
        assert_eq!(history.len(), 1);
        
        service.clear_history().await.unwrap();
        
        let history = service.get_history(None).await.unwrap();
        assert_eq!(history.len(), 0);
    }
}