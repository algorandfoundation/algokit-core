use crate::transactions::common::UtilsError;
use algokit_transact::{ALGORAND_SECRET_KEY_BYTE_LENGTH, KeyPairAccount};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

use super::mnemonic::{from_key, to_key};

#[derive(uniffi::Record, Clone)]
pub struct TestAccount {
    pub address: String,
    pub private_key: Vec<u8>,
    pub mnemonic: String,
}

impl TestAccount {
    pub fn generate() -> Result<Self, UtilsError> {
        // Generate a random signing key using ed25519_dalek
        let signing_key = SigningKey::generate(&mut OsRng);
        let private_key_bytes = signing_key.to_bytes();

        // Get the public key and address
        let verifying_key: VerifyingKey = (&signing_key).into();
        let account = KeyPairAccount::from_pubkey(&verifying_key.to_bytes());
        let address = account.address().to_string();

        // Generate mnemonic from private key
        let mnemonic = from_key(&private_key_bytes).map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to generate mnemonic: {}", e),
        })?;

        Ok(TestAccount {
            address,
            private_key: private_key_bytes.to_vec(),
            mnemonic,
        })
    }

    pub fn from_mnemonic(mnemonic: String) -> Result<Self, UtilsError> {
        // Convert 25-word mnemonic to 32-byte key using our mnemonic module
        let private_key_bytes = to_key(&mnemonic).map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to parse mnemonic: {}", e),
        })?;

        // Create signing key from bytes
        let signing_key = SigningKey::from_bytes(&private_key_bytes);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let account = KeyPairAccount::from_pubkey(&verifying_key.to_bytes());
        let address = account.address().to_string();

        Ok(TestAccount {
            address,
            private_key: private_key_bytes.to_vec(),
            mnemonic,
        })
    }

    pub fn account(&self) -> Result<KeyPairAccount, UtilsError> {
        if self.private_key.len() != ALGORAND_SECRET_KEY_BYTE_LENGTH {
            return Err(UtilsError::UtilsError {
                message: "Invalid private key length".to_string(),
            });
        }

        let mut key_bytes = [0u8; ALGORAND_SECRET_KEY_BYTE_LENGTH];
        key_bytes.copy_from_slice(&self.private_key);

        let signing_key = SigningKey::from_bytes(&key_bytes);
        let verifying_key: VerifyingKey = (&signing_key).into();
        Ok(KeyPairAccount::from_pubkey(&verifying_key.to_bytes()))
    }
}
