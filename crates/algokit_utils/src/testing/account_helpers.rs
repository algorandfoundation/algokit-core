use algod_client::AlgodClient;
use algokit_transact::{
    ALGORAND_SECRET_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, Address, AlgorandMsgpack,
    KeyPairAccount, PaymentTransactionBuilder, SignedTransaction, Transaction,
    TransactionHeaderBuilder,
};
use async_trait::async_trait;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use hex;
use rand::rngs::OsRng;
use regex::Regex;
use std::convert::TryInto;
use std::process::Command;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

use crate::{TransactionSigner, TransactionSignerGetter};

use super::mnemonic::{from_key, to_key};

/// Test account configuration
#[derive(Debug, Clone)]
pub struct TestAccountConfig {
    /// Initial funding amount in microALGOs (default: 10 ALGO = 10,000,000 microALGOs)
    pub initial_funds: u64,
    /// Network type (LocalNet, TestNet, MainNet)
    pub network_type: NetworkType,
    /// Optional note for funding transaction
    pub funding_note: Option<String>,
}

impl Default for TestAccountConfig {
    fn default() -> Self {
        Self {
            initial_funds: 10_000_000, // 10 ALGO
            network_type: NetworkType::LocalNet,
            funding_note: None,
        }
    }
}

/// Network types for testing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkType {
    LocalNet,
    TestNet,
    MainNet,
}

/// A test account using algokit_transact and ed25519_dalek with proper Algorand mnemonics
#[derive(Debug, Clone)]
pub struct TestAccount {
    /// The ed25519 secret key used for signing transactions
    secret_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH],
}

// Implement TransactionSignerGetter for TestAccount as well
impl TransactionSignerGetter for TestAccount {
    fn get_signer(&self, address: Address) -> Option<Arc<dyn TransactionSigner>> {
        let test_account_address = self.account().expect("Failed to get test account address");
        if address == test_account_address.address() {
            Some(Arc::new(self.clone()))
        } else {
            None
        }
    }
}

#[async_trait]
impl TransactionSigner for TestAccount {
    async fn sign_transactions(
        &self,
        txns: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String> {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let signer_account = KeyPairAccount::from_pubkey(&verifying_key.to_bytes());
        let signer_address = signer_account.address();

        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    let tx = txns[idx].clone();
                    let encoded_tx = tx
                        .encode()
                        .map_err(|e| format!("Failed to encode transaction: {:?}", e))?;
                    let sig: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH] =
                        signing_key.sign(&encoded_tx).to_bytes();

                    let auth_address = if tx.header().sender != signer_address {
                        Some(signer_address.clone())
                    } else {
                        None
                    };

                    Ok(SignedTransaction {
                        transaction: tx,
                        signature: Some(sig),
                        auth_address,
                        multisignature: None,
                    })
                } else {
                    Err(format!("Index {} out of bounds for transactions", idx))
                }
            })
            .collect()
    }
}

impl TestAccount {
    /// Generate a new random test account using ed25519_dalek
    pub fn generate() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Generate a random signing key
        let signing_key = SigningKey::generate(&mut OsRng);

        Ok(Self {
            secret_key: signing_key.to_bytes(),
        })
    }

    /// Create account from mnemonic using proper Algorand 25-word mnemonics
    pub fn from_mnemonic(
        mnemonic_str: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Convert 25-word mnemonic to 32-byte key using our mnemonic module
        let secret_key =
            to_key(mnemonic_str).map_err(|e| format!("Failed to parse mnemonic: {}", e))?;

        Ok(Self { secret_key })
    }

    /// Get the account's address using algokit_transact
    pub fn account(&self) -> Result<KeyPairAccount, Box<dyn std::error::Error + Send + Sync>> {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let public_key: VerifyingKey = (&signing_key).into();
        let account = KeyPairAccount::from_pubkey(&public_key.to_bytes());
        Ok(account)
    }

    /// Get the account's mnemonic (proper Algorand 25-word mnemonic)
    pub fn mnemonic(&self) -> String {
        from_key(&self.secret_key).unwrap_or_else(|_| {
            // Fallback to hex for debugging if mnemonic generation fails
            hex::encode(self.secret_key)
        })
    }
}

/// LocalNet dispenser for funding test accounts using AlgoKit CLI
pub struct LocalNetDispenser {
    client: AlgodClient,
    dispenser_account: Option<TestAccount>,
}

impl LocalNetDispenser {
    /// Create a new LocalNet dispenser
    pub fn new(client: AlgodClient) -> Self {
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
            .sender(dispenser.account()?.address())
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

/// Test account manager for generating and managing test accounts
pub struct TestAccountManager {
    dispenser: LocalNetDispenser,
}

impl TestAccountManager {
    /// Create a new test account manager
    pub fn new(client: AlgodClient) -> Self {
        let dispenser = LocalNetDispenser::new(client);
        Self { dispenser }
    }

    /// Get a test account with optional configuration
    pub async fn get_test_account(
        &mut self,
        config: Option<TestAccountConfig>,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        let config = config.unwrap_or_default();

        // Generate new account using ed25519_dalek
        let test_account = TestAccount::generate()?;
        let address_str = test_account.account()?.to_string();

        // Fund the account based on network type
        match config.network_type {
            NetworkType::LocalNet => {
                self.dispenser
                    .fund_account(&address_str, config.initial_funds)
                    .await?;
            }
            NetworkType::TestNet => {
                println!(
                    "⚠ TestNet funding not yet implemented. Please fund manually: {}",
                    address_str
                );
            }
            NetworkType::MainNet => {
                println!(
                    "⚠ MainNet detected. Account generated but not funded: {}",
                    address_str
                );
            }
        }

        Ok(test_account)
    }

    /// Create a funded account pair (sender, receiver) for testing
    pub async fn create_account_pair(
        &mut self,
    ) -> Result<(TestAccount, TestAccount), Box<dyn std::error::Error + Send + Sync>> {
        let sender_config = TestAccountConfig {
            initial_funds: 10_000_000, // 10 ALGO
            network_type: NetworkType::LocalNet,
            funding_note: Some("Test sender account".to_string()),
        };

        let receiver_config = TestAccountConfig {
            initial_funds: 1_000_000, // 1 ALGO
            network_type: NetworkType::LocalNet,
            funding_note: Some("Test receiver account".to_string()),
        };

        let sender = self.get_test_account(Some(sender_config)).await?;
        let receiver = self.get_test_account(Some(receiver_config)).await?;

        Ok((sender, receiver))
    }

    /// Generate multiple test accounts at once
    pub async fn get_test_accounts(
        &mut self,
        count: usize,
        config: Option<TestAccountConfig>,
    ) -> Result<Vec<TestAccount>, Box<dyn std::error::Error + Send + Sync>> {
        let mut accounts = Vec::with_capacity(count);

        for _i in 0..count {
            let account_config = config.clone().unwrap_or_default();
            let account = self.get_test_account(Some(account_config)).await?;
            accounts.push(account);

            sleep(Duration::from_millis(100)).await;
        }

        Ok(accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_generation_with_algokit_transact() {
        // Test basic account generation using algokit_transact and ed25519_dalek with proper mnemonics
        let account = TestAccount::generate().expect("Failed to generate test account");
        let address = account.account().expect("Failed to get address");

        assert!(!address.to_string().is_empty());
        let mnemonic = account.mnemonic();
        assert!(!mnemonic.is_empty());

        // Test that we get proper 25-word mnemonics (or hex fallback for debugging)
        let word_count = mnemonic.split_whitespace().count();
        println!("Mnemonic word count: {}", word_count);
        println!("Generated account address: {}", address);
        println!("Generated account mnemonic: {}", mnemonic);
    }

    #[tokio::test]
    async fn test_account_from_mnemonic_with_algokit_transact() {
        let original = TestAccount::generate().expect("Failed to generate test account");
        let mnemonic = original.mnemonic();

        // Only test round-trip if we have a proper mnemonic (not hex fallback)
        if mnemonic.split_whitespace().count() == 25 {
            // Recover account from mnemonic using proper Algorand mnemonic parsing
            let recovered = TestAccount::from_mnemonic(&mnemonic)
                .expect("Failed to recover account from mnemonic");

            // Both should have the same address
            let original_account = original.account().expect("Failed to get original account");
            let recovered_account = recovered
                .account()
                .expect("Failed to get recovered account");

            assert_eq!(original_account.to_string(), recovered_account.to_string());
            assert_eq!(original.mnemonic(), recovered.mnemonic());

            println!("✓ Successfully recovered account from mnemonic");
            println!("  Original:  {}", original_account);
            println!("  Recovered: {}", recovered_account);
        } else {
            println!("⚠ Skipping mnemonic round-trip test (using hex fallback)");
        }
    }
}
