use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

/// Utility functions for managing localnet for tests
pub struct LocalnetManager;

impl LocalnetManager {
    /// Check if localnet is running by trying to connect to the algod endpoint
    pub async fn is_running() -> bool {
        let client = reqwest::Client::new();
        let url =
            std::env::var("ALGORAND_HOST").unwrap_or_else(|_| "http://localhost:4001".to_string());

        match client.get(format!("{}/health", url)).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    /// Start localnet using algokit
    pub fn start() -> Result<(), String> {
        println!("Starting localnet...");

        let output = Command::new("algokit")
            .args(["localnet", "start"])
            .output()
            .map_err(|e| format!("Failed to execute algokit: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to start localnet: {}", stderr));
        }

        println!("Localnet started successfully");
        Ok(())
    }

    /// Stop localnet using algokit
    pub fn stop() -> Result<(), String> {
        println!("Stopping localnet...");

        let output = Command::new("algokit")
            .args(["localnet", "stop"])
            .output()
            .map_err(|e| format!("Failed to execute algokit: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to stop localnet: {}", stderr));
        }

        println!("Localnet stopped successfully");
        Ok(())
    }

    /// Reset localnet to a clean state
    pub fn reset() -> Result<(), String> {
        println!("Resetting localnet...");

        let output = Command::new("algokit")
            .args(["localnet", "reset"])
            .output()
            .map_err(|e| format!("Failed to execute algokit: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to reset localnet: {}", stderr));
        }

        println!("Localnet reset successfully");
        Ok(())
    }

    /// Wait for localnet to be ready
    pub async fn wait_for_ready(timeout_seconds: u64) -> Result<(), String> {
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_seconds);

        while start.elapsed() < timeout {
            if Self::is_running().await {
                println!("Localnet is ready!");
                return Ok(());
            }

            sleep(Duration::from_millis(500)).await;
        }

        Err(format!(
            "Timeout waiting for localnet to be ready after {} seconds",
            timeout_seconds
        ))
    }

    /// Ensure localnet is running, start if needed
    pub async fn ensure_running() -> Result<(), String> {
        if !Self::is_running().await {
            Self::start()?;
            Self::wait_for_ready(30).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Ignore by default, run with --ignored for integration tests
    async fn test_localnet_management() {
        // Test that we can check if localnet is running
        let is_running = LocalnetManager::is_running().await;
        println!("Localnet running: {}", is_running);

        // If not running, try to start it
        if !is_running {
            LocalnetManager::ensure_running()
                .await
                .expect("Failed to start localnet");
            assert!(
                LocalnetManager::is_running().await,
                "Localnet should be running after start"
            );
        }
    }
}
