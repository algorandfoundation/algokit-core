#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

use algokit_http_client_trait::HTTPClient;
use algokit_transact_ffi::{AlgoKitTransactError, Transaction};
use algokit_utils::Composer as ComposerRs;
use std::sync::Arc;

use algokit_transact_ffi::Transaction as FfiTransaction;

///////////////////////
// WASM specific code
////////////////////////

#[cfg(feature = "ffi_wasm")]
use js_sys::JSON;

#[cfg(feature = "ffi_wasm")]
use js_sys::JsString;

#[cfg(feature = "ffi_wasm")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "ffi_wasm")]
use tsify_next::Tsify;

#[cfg(feature = "ffi_wasm")]
use async_trait::async_trait;

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ffi_wasm")]
use js_sys::Uint8Array;

#[cfg(feature = "ffi_wasm")]
use js_sys::wasm_bindgen::JsValue;

#[cfg(feature = "ffi_uniffi")]
pub type InnerMutex<T> = tokio::sync::Mutex<T>;

#[cfg(feature = "ffi_wasm")]
pub type InnerMutex<T> = std::cell::RefCell<T>;

#[cfg(feature = "ffi_wasm")]
use algokit_http_client_trait::HttpError;

// Create a wrapper that provides a unified interface
pub struct UnifiedMutex<T>(InnerMutex<T>);

impl<T> UnifiedMutex<T> {
    pub fn new(value: T) -> Self {
        #[cfg(feature = "ffi_uniffi")]
        return Self(tokio::sync::Mutex::new(value));

        #[cfg(feature = "ffi_wasm")]
        return Self(std::cell::RefCell::new(value));
    }

    #[cfg(feature = "ffi_uniffi")]
    pub fn blocking_lock(&self) -> tokio::sync::MutexGuard<'_, T> {
        self.0.blocking_lock()
    }

    #[cfg(feature = "ffi_wasm")]
    pub fn blocking_lock(&self) -> std::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }

    #[cfg(feature = "ffi_uniffi")]
    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, T> {
        self.0.lock().await
    }

    #[cfg(feature = "ffi_wasm")]
    pub async fn lock(&self) -> std::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

#[cfg(feature = "ffi_wasm")]
#[wasm_bindgen]
extern "C" {
    pub type WasmHTTPClient;

    #[wasm_bindgen(method, catch)]
    async fn json(this: &WasmHTTPClient, path: &str) -> Result<JsValue, JsValue>;
}

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
impl HTTPClient for WasmHTTPClient {
    async fn json(&self, path: String) -> Result<String, HttpError> {
        let result = self.json(&path).await.unwrap();

        let result = result.as_string().ok_or_else(|| {
            HttpError::HttpError("Failed to convert JS string to Rust string".to_string())
        })?;

        Ok(result)
    }
}

/////////////////
// General Code
////////////////

#[cfg(feature = "ffi_uniffi")]
type Uint8Array = Vec<u8>;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum ComposerError {
    #[error("TransactionsError: {0}")]
    TransactionsError(String),
}

#[cfg(feature = "ffi_wasm")]
impl From<ComposerError> for JsValue {
    fn from(e: ComposerError) -> Self {
        JsValue::from(e.to_string())
    }
}

#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Object))]
pub struct Composer {
    composer: UnifiedMutex<ComposerRs>,
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
    pub fn new(algod_client: Arc<dyn HTTPClient>) -> Self {
        let algod_client = algokit_utils::AlgodClient::new(algod_client);
        Composer {
            composer: UnifiedMutex::new(ComposerRs::new(algod_client)),
        }
    }
}

#[cfg(feature = "ffi_wasm")]
#[derive(Serialize, Deserialize)]
struct JsComposerValue {
    transactions: Vec<FfiTransaction>,
}

#[cfg(feature = "ffi_wasm")]
impl From<&Composer> for JsComposerValue {
    fn from(composer: &Composer) -> Self {
        JsComposerValue {
            transactions: composer.transactions().unwrap_or_else(|_| vec![]),
        }
    }
}

#[cfg(feature = "ffi_wasm")]
#[wasm_bindgen]
impl Composer {
    #[wasm_bindgen(constructor)]
    pub fn new(algod_client: WasmHTTPClient) -> Self {
        let algod_client = algokit_utils::AlgodClient::new(Arc::new(algod_client));
        Composer {
            composer: UnifiedMutex::new(ComposerRs::new(algod_client)),
        }
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> Result<JsValue, JsValue> {
        let ser = serde_wasm_bindgen::Serializer::new()
            .serialize_large_number_types_as_bigints(true)
            .serialize_bytes_as_arrays(false);

        // Ok(serde_wasm_bindgen::to_value(&JsComposerValue::from(self))?)
        Ok(JsComposerValue::from(self).serialize(&ser)?)
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JSON::parse(
            &self
                .to_string()?
                .as_string()
                .ok_or("Failed to convert JS string to Rust string")?,
        )
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<JsString, JsValue> {
        let replacer = js_sys::Function::new_with_args(
            "key, value",
            r#"
            if (typeof value === 'bigint') {
                return value.toString() + 'n';
            }
            return value;
            "#,
        );
        JSON::stringify_with_replacer(&self.value_of().unwrap(), &replacer)
    }
}
