use crate::transactions::common::UtilsError;
use regex::Regex;
use std::process::Command;

/// Fetch the dispenser mnemonic from LocalNet
pub async fn get_dispenser_mnemonic() -> Result<String, UtilsError> {
    // Check LocalNet status and start if needed
    ensure_localnet_running().await?;

    // Get account with highest balance
    let dispenser_address = find_dispenser_account().await?;

    // Export and return mnemonic
    export_account_mnemonic(&dispenser_address).await
}

/// Ensure LocalNet is running, start it if not
async fn ensure_localnet_running() -> Result<(), UtilsError> {
    // Check LocalNet status
    let status_output = Command::new("algokit")
        .args(["localnet", "status"])
        .output()
        .map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to check LocalNet status: {}", e),
        })?;

    let status_str = String::from_utf8_lossy(&status_output.stdout);

    if !status_str.to_lowercase().contains("running") {
        // Try to start LocalNet
        let start_output = Command::new("algokit")
            .args(["localnet", "start"])
            .output()
            .map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to start LocalNet: {}", e),
            })?;

        if !start_output.status.success() {
            return Err(UtilsError::UtilsError {
                message: format!(
                    "Failed to start LocalNet: {}",
                    String::from_utf8_lossy(&start_output.stderr)
                ),
            });
        }

        // Wait a bit for LocalNet to fully start
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    Ok(())
}

/// Find the LocalNet account with the highest balance (dispenser)
async fn find_dispenser_account() -> Result<String, UtilsError> {
    let accounts_output = Command::new("algokit")
        .args([
            "goal",
            "account",
            "list",
            "-w",
            "unencrypted-default-wallet",
        ])
        .output()
        .map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to list accounts: {}", e),
        })?;

    if !accounts_output.status.success() {
        return Err(UtilsError::UtilsError {
            message: format!(
                "Failed to list accounts: {}",
                String::from_utf8_lossy(&accounts_output.stderr)
            ),
        });
    }

    let output_str = String::from_utf8_lossy(&accounts_output.stdout);

    // Create regex pattern for parsing account lines
    let re =
        Regex::new(r"([A-Z0-9]{58})\s+(\d+)\s+microAlgos").map_err(|e| UtilsError::UtilsError {
            message: format!("Regex error: {}", e),
        })?;

    let mut highest_balance = 0u64;
    let mut dispenser_address = String::new();

    // Find account with highest balance
    for cap in re.captures_iter(&output_str) {
        let address = cap[1].to_string();
        let balance: u64 = cap[2].parse().unwrap_or(0);

        if balance > highest_balance {
            highest_balance = balance;
            dispenser_address = address;
        }
    }

    if dispenser_address.is_empty() {
        return Err(UtilsError::UtilsError {
            message: "No funded accounts found in LocalNet".to_string(),
        });
    }

    Ok(dispenser_address)
}

/// Export the mnemonic for a given account address
async fn export_account_mnemonic(address: &str) -> Result<String, UtilsError> {
    let export_output = Command::new("algokit")
        .args([
            "goal",
            "account",
            "export",
            "-a",
            address,
            "-w",
            "unencrypted-default-wallet",
        ])
        .output()
        .map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to export account {}: {}", address, e),
        })?;

    if !export_output.status.success() {
        return Err(UtilsError::UtilsError {
            message: format!(
                "Failed to export account {}: {}",
                address,
                String::from_utf8_lossy(&export_output.stderr)
            ),
        });
    }

    let export_str = String::from_utf8_lossy(&export_output.stdout);

    // Extract mnemonic from output
    // The output format is typically: 'Exported key for account <address>: "mnemonic words"'
    let mnemonic = if let Some(start_quote) = export_str.find('"') {
        if let Some(end_quote) = export_str[start_quote + 1..].find('"') {
            export_str[start_quote + 1..start_quote + 1 + end_quote].to_string()
        } else {
            // Fallback: take everything after the last colon
            export_str
                .rfind(": ")
                .map(|idx| export_str[idx + 2..].trim().to_string())
                .unwrap_or_else(|| export_str.trim().to_string())
        }
    } else {
        // Fallback: take everything after the last colon or the whole string
        export_str
            .rfind(": ")
            .map(|idx| export_str[idx + 2..].trim().to_string())
            .unwrap_or_else(|| export_str.trim().to_string())
    };

    Ok(mnemonic.trim().to_string())
}
