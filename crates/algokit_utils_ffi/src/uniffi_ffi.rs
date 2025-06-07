use algokit_http_client_trait::HTTPClient;
use algokit_transact_ffi::{AlgoKitTransactError, Transaction};
use algokit_utils::Composer as ComposerRs;

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ComposerError {
    #[error("TransactionsError: {0}")]
    TransactionsError(String),
}

#[derive(uniffi::Object)]
pub struct Composer {
    pub composer: Mutex<ComposerRs>,
}

#[uniffi::export]
impl Composer {
    #[uniffi::constructor]
    pub fn new(algod_client: Arc<dyn HTTPClient>) -> Self {
        let algod_client = algokit_utils::AlgodClient::new(algod_client);
        Composer {
            composer: Mutex::new(ComposerRs::new(algod_client)),
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) -> Result<(), ComposerError> {
        self.composer
            .blocking_lock()
            .add_transaction(transaction.try_into().map_err(|e: AlgoKitTransactError| {
                ComposerError::TransactionsError(e.to_string())
            })?)
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    pub fn encode(&self) -> Result<Vec<Vec<u8>>, ComposerError> {
        self.composer
            .blocking_lock()
            .encode()
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    pub async fn get_suggested_params(&self) -> Result<String, ComposerError> {
        self.composer
            .lock()
            .await
            .get_suggested_params()
            .await
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }
}
