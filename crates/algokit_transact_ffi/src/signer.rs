use crate::{AlgoKitTransactError, SignedTransaction, Transaction};
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

/// FFI-compatible trait for transaction signing operations
///
/// This trait is exported with `with_foreign` to allow foreign languages (Python, Swift, Kotlin, etc.)
/// to implement it and provide custom signing logic.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export(with_foreign))]
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    /// Sign a collection of transactions at the specified indices.
    ///
    /// # Parameters
    /// * `transactions` - The transactions to sign
    /// * `indexes_to_sign` - The indices of the transactions to sign (as u8 for FFI compatibility)
    ///
    /// # Returns
    /// A vector of signed transactions or an error if signing fails.
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indexes_to_sign: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError>;
}

/// Wrapper struct to convert from FFI TransactionSigner to Rust TransactionSigner
pub struct RustTransactionSignerFromFfi {
    pub ffi_signer: Arc<dyn TransactionSigner>,
}

#[async_trait]
impl algokit_transact::signer::TransactionSigner for RustTransactionSignerFromFfi {
    async fn sign_transactions(
        &self,
        transactions: &[algokit_transact::Transaction],
        indexes_to_sign: &[usize],
    ) -> Result<Vec<algokit_transact::SignedTransaction>, algokit_transact::AlgoKitTransactError>
    {
        // Convert Rust transactions to FFI transactions
        let ffi_transactions: Vec<Transaction> =
            transactions.iter().cloned().map(|t| t.into()).collect();

        // Convert usize indices to u8 for FFI
        let ffi_indexes: Vec<u8> = indexes_to_sign.iter().map(|&i| i as u8).collect();

        // Call the FFI signer
        let ffi_signed_transactions = self
            .ffi_signer
            .sign_transactions(ffi_transactions, ffi_indexes)
            .await
            .map_err(|e| algokit_transact::AlgoKitTransactError::SigningError {
                err_msg: e.to_string(),
            })?;

        // Convert FFI signed transactions back to Rust signed transactions
        let rust_signed_transactions: Result<Vec<_>, _> = ffi_signed_transactions
            .into_iter()
            .map(|st| st.try_into())
            .collect();

        rust_signed_transactions.map_err(|e| algokit_transact::AlgoKitTransactError::SigningError {
            err_msg: format!("Failed to convert signed transaction: {}", e),
        })
    }
}

/// Wrapper struct to convert from Rust TransactionSigner to FFI TransactionSigner
pub struct FfiTransactionSignerFromRust {
    pub rust_signer: Arc<dyn algokit_transact::signer::TransactionSigner + Send + Sync>,
}

#[async_trait]
impl TransactionSigner for FfiTransactionSignerFromRust {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indexes_to_sign: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        // Convert FFI transactions to Rust transactions
        let rust_transactions: Result<Vec<_>, _> =
            transactions.into_iter().map(|t| t.try_into()).collect();
        let rust_transactions =
            rust_transactions.map_err(|e| AlgoKitTransactError::DecodingError {
                error_msg: format!("Failed to convert transactions: {}", e),
            })?;

        // Convert u8 indices to usize for Rust
        let rust_indexes: Vec<usize> = indexes_to_sign.iter().map(|&i| i as usize).collect();

        // Call the Rust signer
        let rust_signed_transactions = self
            .rust_signer
            .sign_transactions(&rust_transactions, &rust_indexes)
            .await
            .map_err(|e| AlgoKitTransactError::SigningError {
                error_msg: e.to_string(),
            })?;

        // Convert Rust signed transactions to FFI signed transactions
        Ok(rust_signed_transactions
            .into_iter()
            .map(|st| st.into())
            .collect())
    }
}

/// A transaction signer that returns placeholder signatures.
///
/// This is useful for testing and fee estimation where actual signatures
/// are not needed.
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Object))]
pub struct EmptyTransactionSigner;

#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
impl EmptyTransactionSigner {
    /// Create a new EmptyTransactionSigner
    #[cfg_attr(feature = "ffi_uniffi", uniffi::constructor)]
    pub fn new() -> Self {
        Self
    }
}

impl Default for EmptyTransactionSigner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TransactionSigner for EmptyTransactionSigner {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indexes_to_sign: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        let indexes: Vec<usize> = indexes_to_sign.iter().map(|&i| i as usize).collect();

        // Check bounds
        for index in &indexes {
            if *index >= transactions.len() {
                return Err(AlgoKitTransactError::SigningError {
                    error_msg: format!(
                        "Index {} is out of bounds for transactions of length {}",
                        index,
                        transactions.len()
                    ),
                });
            }
        }

        Ok(transactions
            .into_iter()
            .enumerate()
            .map(|(index, txn)| {
                if indexes.contains(&index) {
                    // Return placeholder signature for transactions we were asked to sign
                    SignedTransaction {
                        transaction: txn,
                        signature: Some(vec![0u8; 64]),
                        auth_address: None,
                        multisignature: None,
                    }
                } else {
                    // Return completely unsigned for transactions we weren't asked to sign
                    SignedTransaction {
                        transaction: txn,
                        signature: None,
                        auth_address: None,
                        multisignature: None,
                    }
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TransactionType;

    fn create_test_transaction() -> Transaction {
        Transaction {
            transaction_type: TransactionType::Payment,
            sender: "7LQ7G6P7RRKBPBJD4B7W5R5FBJV5J6RJK3QY7N4K7KJB3Y5Q5R5G5P7X3U".to_string(),
            fee: Some(1000),
            first_valid: 1000,
            last_valid: 2000,
            genesis_hash: None,
            genesis_id: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
            payment: None,
            asset_transfer: None,
            asset_config: None,
            app_call: None,
            key_registration: None,
            asset_freeze: None,
            heartbeat: None,
            state_proof: None,
        }
    }

    #[tokio::test]
    async fn test_empty_transaction_signer() {
        let signer = EmptyTransactionSigner::new();
        let transaction = create_test_transaction();
        let transactions = vec![transaction.clone()];

        // Sign with index 0
        let signed = signer
            .sign_transactions(transactions.clone(), vec![0])
            .await
            .expect("Should succeed");

        assert_eq!(signed.len(), 1);
        assert!(signed[0].signature.is_some()); // Empty placeholder signature
        assert_eq!(signed[0].signature.as_ref().unwrap(), &vec![0u8; 64]);

        // Don't sign with any index
        let unsigned = signer
            .sign_transactions(transactions.clone(), vec![])
            .await
            .expect("Should succeed");

        assert_eq!(unsigned.len(), 1);
        assert!(unsigned[0].signature.is_none()); // Completely unsigned
    }

    #[tokio::test]
    async fn test_out_of_bounds_index() {
        let signer = EmptyTransactionSigner::new();
        let transaction = create_test_transaction();
        let transactions = vec![transaction];

        let result = signer.sign_transactions(transactions, vec![5]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of bounds"));
    }

    #[tokio::test]
    async fn test_default_empty_transaction_signer() {
        let signer: EmptyTransactionSigner = Default::default();
        let transaction = create_test_transaction();
        let transactions = vec![transaction];

        // Just verify it was created successfully with default
        let result = signer.sign_transactions(transactions, vec![]).await;
        assert!(result.is_ok());
    }
}
