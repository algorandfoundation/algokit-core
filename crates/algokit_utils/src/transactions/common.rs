use algokit_transact::{Address, EMPTY_SIGNATURE, SignedTransaction, Transaction};
use async_trait::async_trait;
use derive_more::Debug;
use std::sync::Arc;

#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String>;

    async fn sign_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<SignedTransaction, String> {
        let result = self.sign_transactions(&[transaction.clone()], &[0]).await?;
        Ok(result[0].clone())
    }
}

pub trait TransactionSignerGetter {
    fn get_signer(&self, address: Address) -> Option<Arc<dyn TransactionSigner>>;
}

#[derive(Clone)]
pub struct EmptySigner {}

#[async_trait]
impl TransactionSigner for EmptySigner {
    async fn sign_transactions(
        &self,
        txns: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String> {
        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    Ok(SignedTransaction {
                        transaction: txns[idx].clone(),
                        signature: Some(EMPTY_SIGNATURE),
                        auth_address: None,
                        multisignature: None,
                    })
                } else {
                    Err(format!("Index {} out of bounds for transactions", idx))
                }
            })
            .collect()
    }
}

impl TransactionSignerGetter for EmptySigner {
    fn get_signer(&self, _address: Address) -> Option<Arc<dyn TransactionSigner>> {
        Some(Arc::new(self.clone()))
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommonParams {
    pub sender: Address,
    #[debug(skip)]
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TransactionWithSigner {
    pub transaction: Transaction,
    #[debug(skip)]
    pub signer: Arc<dyn TransactionSigner>,
}
