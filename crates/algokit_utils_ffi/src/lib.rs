#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "ffi_wasm")]
include!("wasm.rs");

use ffi_mutex::FfiMutex;

#[cfg(feature = "ffi_uniffi")]
use algokit_http_client_trait::HttpClient;

use algokit_transact_ffi::{AlgoKitTransactError, Transaction};
use algokit_utils::Composer as ComposerRs;
use std::sync::Arc;

use algokit_transact_ffi::Transaction as FfiTransaction;

#[cfg(feature = "ffi_uniffi")]
type Uint8Array = Vec<u8>;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum ComposerError {
    #[error("TransactionsError: {0}")]
    TransactionsError(String),
}

#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Object))]
pub struct Composer {
    composer: FfiMutex<ComposerRs>,
}

#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
impl Composer {
    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "addTransaction"))]
    pub fn add_transaction(&self, transaction: Transaction) -> Result<(), ComposerError> {
        self.composer
            .blocking_lock()
            .add_transaction(transaction.try_into().map_err(|e: AlgoKitTransactError| {
                ComposerError::TransactionsError(e.to_string())
            })?)
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    pub fn encode(&self) -> Result<Vec<Uint8Array>, ComposerError> {
        Ok(self
            .composer
            .blocking_lock()
            .encode()
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))?
            .iter()
            .map(|b| Uint8Array::from(b.as_slice()))
            .collect::<Vec<Uint8Array>>())
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "getSuggestedParams"))]
    pub async fn get_suggested_params(&self) -> Result<String, ComposerError> {
        self.composer
            .lock()
            .await
            .get_suggested_params()
            .await
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(getter))]
    pub fn transactions(&self) -> Result<Vec<FfiTransaction>, ComposerError> {
        Ok(self
            .composer
            .blocking_lock()
            .transactions()
            .into_iter()
            .map(|tx| {
                tx.try_into().map_err(|e: AlgoKitTransactError| {
                    ComposerError::TransactionsError(e.to_string())
                })
            })
            .collect::<Result<Vec<FfiTransaction>, ComposerError>>()?)
    }
}

#[cfg(feature = "ffi_uniffi")]
#[uniffi::export]
impl Composer {
    #[cfg_attr(feature = "ffi_uniffi", uniffi::constructor)]
    pub fn new(algod_client: Arc<dyn HttpClient>) -> Self {
        let algod_client = algokit_utils::AlgodClient::new(algod_client);
        Composer {
            composer: FfiMutex::new(ComposerRs::new(algod_client)),
        }
    }
}
