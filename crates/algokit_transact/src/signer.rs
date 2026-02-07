use crate::{AlgoKitTransactError, SignedTransaction, Transaction};

pub trait TransactionSigner {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indexes_to_sign: &[usize],
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError>;
}

pub struct EmptyTransactionSigner;

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
