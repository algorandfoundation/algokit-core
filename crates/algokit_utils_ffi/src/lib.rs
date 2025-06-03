use algokit_transact_ffi::{AlgoKitTransactError, Transaction};
use algokit_utils::Composer as ComposerRs;
use std::sync::Mutex;

use uniffi::{self};

uniffi::setup_scaffolding!();

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
    pub fn new() -> Self {
        Composer {
            composer: Mutex::new(ComposerRs::new()),
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) -> Result<(), ComposerError> {
        self.composer
            .lock()
            .unwrap()
            .add_transaction(transaction.try_into().map_err(|e: AlgoKitTransactError| {
                ComposerError::TransactionsError(e.to_string())
            })?)
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    pub fn encode(&self) -> Result<Vec<Vec<u8>>, ComposerError> {
        self.composer
            .lock()
            .unwrap()
            .encode()
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }
}
