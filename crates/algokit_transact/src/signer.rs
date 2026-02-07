use crate::{
    Address, AlgoKitTransactError, SignedTransaction, Transaction, address::Addressable,
    traits::AlgorandMsgpack,
};
use algokit_crypto::ed25519::Ed25519KeyAndSigner;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TransactionSigner {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indexes_to_sign: &[usize],
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError>;
}

pub struct EmptyTransactionSigner;

#[async_trait]
impl TransactionSigner for EmptyTransactionSigner {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indexes_to_sign: &[usize],
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        for index in indexes_to_sign {
            if *index >= transactions.len() {
                return Err(AlgoKitTransactError::SigningError {
                    err_msg: format!(
                        "Index {} is out of bounds for transactions of length {}",
                        index,
                        transactions.len()
                    ),
                });
            }
        }

        Ok(transactions
            .iter()
            .enumerate()
            .map(|(index, txn)| {
                if indexes_to_sign.contains(&index) {
                    SignedTransaction {
                        transaction: txn.clone(),
                        auth_address: None,
                        signature: None,
                        multisignature: None,
                    }
                } else {
                    SignedTransaction {
                        transaction: txn.clone(),
                        auth_address: None,
                        signature: Some([0u8; 64]),
                        multisignature: None,
                    }
                }
            })
            .collect())
    }
}

pub struct AddressWithSigners {
    pub addr: Address,
    pub signer: Arc<dyn TransactionSigner + Send + Sync>,
}

impl Addressable for AddressWithSigners {
    fn address(&self) -> Address {
        self.addr.clone()
    }
}

#[async_trait]
impl TransactionSigner for AddressWithSigners {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indexes_to_sign: &[usize],
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        self.signer
            .sign_transactions(transactions, indexes_to_sign)
            .await
    }
}

// Used to implement a TransactionSigner from an algokit_crypto::Signer and Keypair, but we want to avoid exposing algokit_crypto in the public

struct AlgorandSigner {
    auth_addr: Option<Address>,
    key_and_signer: Arc<dyn Ed25519KeyAndSigner + Send + Sync>,
}

impl AlgorandSigner {
    fn new(
        key_and_signer: impl Ed25519KeyAndSigner + Send + Sync + 'static,
        auth_addr: Option<Address>,
    ) -> Self {
        Self {
            auth_addr,
            key_and_signer: Arc::new(key_and_signer),
        }
    }
}

#[async_trait]
impl TransactionSigner for AlgorandSigner {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indexes_to_sign: &[usize],
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        for index in indexes_to_sign {
            if *index >= transactions.len() {
                return Err(AlgoKitTransactError::SigningError {
                    err_msg: format!(
                        "Index {} is out of bounds for transactions of length {}",
                        index,
                        transactions.len()
                    ),
                });
            }
        }

        let mut signed_transactions = Vec::with_capacity(transactions.len());

        for (index, txn) in transactions.iter().enumerate() {
            if indexes_to_sign.contains(&index) {
                // Encode the transaction for signing
                let txn_bytes = txn.encode()?;

                // Sign the transaction bytes
                let signature = self.key_and_signer.try_sign(&txn_bytes).map_err(|e| {
                    AlgoKitTransactError::SigningError {
                        err_msg: format!("Failed to sign transaction {}: {}", index, e),
                    }
                })?;

                signed_transactions.push(SignedTransaction {
                    transaction: txn.clone(),
                    auth_address: self.auth_addr.clone(),
                    signature: Some(signature),
                    multisignature: None,
                });
            } else {
                // Transaction not meant to be signed by this signer
                signed_transactions.push(SignedTransaction {
                    transaction: txn.clone(),
                    auth_address: None,
                    signature: None,
                    multisignature: None,
                });
            }
        }

        Ok(signed_transactions)
    }
}

pub fn generate_address_with_signers(
    key_and_signer: impl Ed25519KeyAndSigner + Send + Sync + 'static,
) -> AddressWithSigners {
    let address = Address(key_and_signer.verifying_key());
    let algorand_signer = AlgorandSigner::new(key_and_signer, None);
    let signer = Arc::new(algorand_signer);
    AddressWithSigners {
        addr: address,
        signer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TransactionMother;
    use algokit_crypto::Keypair;
    use algokit_crypto::ed25519::{CryptoixdeEd25519Keypair, Ed25519Generator};

    #[tokio::test]
    async fn test_generate_address_with_signers() {
        // Generate a keypair with a known seed
        let seed = [1u8; 32];
        let keypair =
            CryptoixdeEd25519Keypair::try_generate(Some(seed)).expect("Failed to generate keypair");

        // Get the verifying key (public key) for later verification
        let verifying_key = keypair.verifying_key();

        // Generate address with signers
        let address_with_signers = generate_address_with_signers(keypair);

        // Verify the address matches the verifying key
        assert_eq!(address_with_signers.addr.0, verifying_key);

        // Create a test transaction
        let transaction = TransactionMother::simple_payment().build().unwrap();

        // Sign the transaction
        let signed_transactions = address_with_signers
            .sign_transactions(&[transaction.clone()], &[0])
            .await
            .expect("Failed to sign transaction");

        // Verify the transaction was signed
        assert_eq!(signed_transactions.len(), 1);
        assert!(signed_transactions[0].signature.is_some());
        assert_eq!(signed_transactions[0].transaction, transaction);
        assert!(signed_transactions[0].auth_address.is_none());
        assert!(signed_transactions[0].multisignature.is_none());

        // Verify signature is 64 bytes (Ed25519 signature length)
        let signature = signed_transactions[0].signature.unwrap();
        assert_eq!(signature.len(), 64);
    }
}
