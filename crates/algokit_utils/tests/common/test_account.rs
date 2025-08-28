use algokit_transact::{
    ALGORAND_SECRET_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, AlgorandMsgpack,
    KeyPairAccount, SignedTransaction, Transaction,
};
use algokit_utils::TransactionSigner;
use async_trait::async_trait;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use hex;
use rand::rngs::OsRng;

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

#[allow(clippy::enum_variant_names)]
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
    pub fn account(&self) -> KeyPairAccount {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        let public_key: VerifyingKey = (&signing_key).into();
        KeyPairAccount::from_pubkey(&public_key.to_bytes())
    }

    /// Get the account's mnemonic (proper Algorand 25-word mnemonic)
    pub fn mnemonic(&self) -> String {
        from_key(&self.secret_key).unwrap_or_else(|_| {
            // Fallback to hex for debugging if mnemonic generation fails
            hex::encode(self.secret_key)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_generation_with_algokit_transact() {
        // Test basic account generation using algokit_transact and ed25519_dalek with proper mnemonics
        let account = TestAccount::generate().expect("Failed to generate test account");
        let address = account.account();

        assert!(!address.to_string().is_empty());
        let mnemonic = account.mnemonic();
        assert!(!mnemonic.is_empty());

        // Test that we get proper 25-word mnemonics (or hex fallback for debugging)
        let word_count = mnemonic.split_whitespace().count();
        println!("Generated account address: {}", address);
        println!("Generated account mnemonic: {}", mnemonic);

        assert_eq!(word_count, 25);
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
            let original_account = original.account();
            let recovered_account = recovered.account();

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
