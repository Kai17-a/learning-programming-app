use anyhow::Result;
use console::style;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;
use tracing::{info, warn};

/// Handles graceful shutdown of the application
#[derive(Clone)]
pub struct ShutdownHandler {
    shutdown_requested: Arc<AtomicBool>,
}

impl ShutdownHandler {
    pub fn new() -> Self {
        Self {
            shutdown_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Wait for shutdown signal (Ctrl+C)
    pub async fn wait_for_shutdown(&self) -> Result<()> {
        tokio::select! {
            _ = signal::ctrl_c() => {
                self.shutdown_requested.store(true, Ordering::SeqCst);
                info!("Received Ctrl+C signal");
                println!("\n{} Received shutdown signal (Ctrl+C)", style("🛑").bold());
                Ok(())
            }
            _ = self.wait_for_sigterm() => {
                self.shutdown_requested.store(true, Ordering::SeqCst);
                info!("Received SIGTERM signal");
                println!("\n{} Received shutdown signal (SIGTERM)", style("🛑").bold());
                Ok(())
            }
        }
    }

    /// Check if shutdown has been requested
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }

    /// Perform cleanup operations
    #[allow(dead_code)]
    pub async fn cleanup(&self) -> Result<()> {
        println!("{} Performing cleanup...", style("🧹").bold());

        // Add any cleanup operations here
        // For example: closing database connections, stopping file watchers, etc.

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        println!("{} Cleanup completed", style("✓").green().bold());
        info!("Application cleanup completed successfully");

        Ok(())
    }

    /// Handle graceful shutdown with cleanup
    #[allow(dead_code)]
    pub async fn handle_shutdown(&self) -> Result<()> {
        self.wait_for_shutdown().await?;
        self.cleanup().await?;

        println!("{} Application terminated gracefully", style("👋").bold());
        info!("Application terminated gracefully");

        Ok(())
    }

    #[cfg(unix)]
    async fn wait_for_sigterm(&self) -> Result<()> {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate())?;
        sigterm.recv().await;
        Ok(())
    }

    #[cfg(not(unix))]
    async fn wait_for_sigterm(&self) -> Result<()> {
        // On Windows, we only handle Ctrl+C
        // This future will never complete, so Ctrl+C will always be used
        std::future::pending::<()>().await;
        Ok(())
    }
}

impl Default for ShutdownHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource cleanup trait for components that need cleanup on shutdown
#[allow(dead_code)]
pub trait Cleanup {
    fn cleanup(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + '_>>;
}

/// Manages multiple resources that need cleanup
#[allow(dead_code)]
pub struct ResourceManager {
    resources: Vec<Box<dyn Cleanup + Send + Sync>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_resource(&mut self, resource: Box<dyn Cleanup + Send + Sync>) {
        self.resources.push(resource);
    }

    #[allow(dead_code)]
    pub async fn cleanup_all(&self) -> Result<()> {
        for (i, resource) in self.resources.iter().enumerate() {
            match resource.cleanup().await {
                Ok(()) => {
                    info!("Resource {} cleaned up successfully", i);
                }
                Err(e) => {
                    warn!("Failed to cleanup resource {}: {}", i, e);
                    // Continue with other resources even if one fails
                }
            }
        }
        Ok(())
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_shutdown_handler_creation() {
        let handler = ShutdownHandler::new();
        assert!(!handler.is_shutdown_requested());
    }

    #[tokio::test]
    async fn test_cleanup() {
        let handler = ShutdownHandler::new();
        let result = handler.cleanup().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resource_manager() {
        struct TestResource {
            cleanup_called: Arc<AtomicBool>,
        }

        impl Cleanup for TestResource {
            fn cleanup(
                &self,
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + '_>>
            {
                let cleanup_called = self.cleanup_called.clone();
                Box::pin(async move {
                    cleanup_called.store(true, Ordering::SeqCst);
                    Ok(())
                })
            }
        }

        let cleanup_called = Arc::new(AtomicBool::new(false));
        let resource = TestResource {
            cleanup_called: cleanup_called.clone(),
        };

        let mut manager = ResourceManager::new();
        manager.add_resource(Box::new(resource));

        manager.cleanup_all().await.unwrap();
        assert!(cleanup_called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_wait_for_shutdown_timeout() {
        let handler = ShutdownHandler::new();

        // This should timeout since we're not sending any signals
        let result = timeout(Duration::from_millis(100), handler.wait_for_shutdown()).await;
        assert!(result.is_err()); // Should timeout
    }
}
