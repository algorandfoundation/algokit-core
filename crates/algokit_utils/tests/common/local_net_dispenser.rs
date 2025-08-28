use std::{process::Command, sync::Arc};

use algod_client::AlgodClient;
use algokit_transact::{
    AlgorandMsgpack, PaymentTransactionBuilder, Transaction, TransactionHeaderBuilder,
};
use algokit_utils::TransactionSigner;
use regex::Regex;

use crate::common::TestAccount;

/// LocalNet dispenser for funding test accounts using AlgoKit CLI
pub struct LocalNetDispenser {
    client: Arc<AlgodClient>,
    dispenser_account: Option<TestAccount>,
}

// TODO: we only use Algokit CLI to fund test account because we don't support KMD yet
// once KMD is implemented, we should remove this
impl LocalNetDispenser {
    /// Create a new LocalNet dispenser
    pub fn new(client: Arc<AlgodClient>) -> Self {
        Self {
            client,
            dispenser_account: None,
        }
    }

    /// Get the LocalNet dispenser account from AlgoKit CLI
    pub async fn get_dispenser_account(
        &mut self,
    ) -> Result<&TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        if self.dispenser_account.is_none() {
            self.dispenser_account = Some(self.fetch_dispenser_from_algokit().await?);
        }

        Ok(self.dispenser_account.as_ref().unwrap())
    }

    /// Fetch the dispenser account using AlgoKit CLI
    async fn fetch_dispenser_from_algokit(
        &self,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        // Get list of accounts to find the one with highest balance
        let output = Command::new("algokit")
            .args(["goal", "account", "list"])
            .output()
            .map_err(|e| format!("Failed to run algokit goal account list: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "algokit goal account list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        let accounts_output = String::from_utf8_lossy(&output.stdout);

        // Parse output to find account with highest balance
        let re = Regex::new(r"([A-Z0-9]{58})\s+(\d+)\s+microAlgos")?;
        let mut highest_balance = 0u64;
        let mut dispenser_address = String::new();

        for cap in re.captures_iter(&accounts_output) {
            let address = cap[1].to_string();
            let balance: u64 = cap[2].parse().unwrap_or(0);

            if balance > highest_balance {
                highest_balance = balance;
                dispenser_address = address;
            }
        }

        if dispenser_address.is_empty() {
            return Err("No funded accounts found in LocalNet".into());
        }

        println!(
            "Found LocalNet dispenser account: {} with {} microALGOs",
            dispenser_address, highest_balance
        );

        // Export the account to get its mnemonic
        let output = Command::new("algokit")
            .args(["goal", "account", "export", "-a", &dispenser_address])
            .output()
            .map_err(|e| format!("Failed to export account {}: {}", dispenser_address, e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to export account {}: {}",
                dispenser_address,
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        let export_output = String::from_utf8_lossy(&output.stdout);

        // Parse mnemonic from output
        let mnemonic = export_output
            .split('"')
            .nth(1)
            .ok_or("Could not extract mnemonic from algokit output")?;

        // Create account from mnemonic using proper Algorand mnemonic parsing
        TestAccount::from_mnemonic(mnemonic)
    }

    /// Fund an account with ALGOs using the dispenser
    pub async fn fund_account(
        &mut self,
        recipient_address: &str,
        amount: u64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Get transaction parameters first (before borrowing self mutably)
        let params = self
            .client
            .transaction_params()
            .await
            .map_err(|e| format!("Failed to get transaction params: {:?}", e))?;

        let dispenser = self.get_dispenser_account().await?;

        // Convert recipient address string to algokit_transact::Address
        let recipient = recipient_address.parse()?;

        // Convert genesis hash Vec<u8> to 32-byte array (already decoded from base64)
        let genesis_hash_bytes: [u8; 32] =
            params.genesis_hash.try_into().map_err(|v: Vec<u8>| {
                format!("Genesis hash must be 32 bytes, got {} bytes", v.len())
            })?;

        // Build funding transaction
        let header = TransactionHeaderBuilder::default()
            .sender(dispenser.account().address())
            .fee(params.min_fee)
            .first_valid(params.last_round)
            .last_valid(params.last_round + 1000)
            .genesis_id(params.genesis_id.clone())
            .genesis_hash(genesis_hash_bytes)
            .note(b"LocalNet test funding".to_vec())
            .build()?;

        let payment_fields = PaymentTransactionBuilder::default()
            .header(header)
            .receiver(recipient)
            .amount(amount)
            .build_fields()?;

        let transaction = Transaction::Payment(payment_fields);
        let signed_transaction = dispenser.sign_transaction(&transaction).await?;
        let signed_bytes = signed_transaction
            .encode()
            .map_err(|e| format!("Failed to encode signed transaction: {:?}", e))?;

        // Submit transaction
        let response = self
            .client
            .raw_transaction(signed_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction: {:?}", e))?;

        println!(
            "✓ Funded account {} with {} microALGOs (txn: {})",
            recipient_address, amount, response.tx_id
        );

        Ok(response.tx_id)
    }
}
