use algokit_transact_ffi::AlgoKitTransactError;
use algokit_transact_ffi::Transaction as FfiTransaction;
use algokit_utils::Composer as ComposerRs;
use algokit_utils::HTTPClient;
use async_trait::async_trait;
use js_sys::JSON;
use js_sys::JsString;

use js_sys::Uint8Array;
use serde::Deserialize;
use serde::Serialize;

use tsify_next::Tsify;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

// thiserror is used to easily create errors than can be propagated to the language bindings
// UniFFI will create classes for errors (i.e. `MsgPackError.EncodingError` in Python)
#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error("TransactionsError: {0}")]
    TransactionsError(String),
}

// For now, in WASM we just throw the string, hence the error
// type being included in the error string above
// Perhaps in the future we could use a class like in UniFFI
impl From<ComposerError> for JsValue {
    fn from(e: ComposerError) -> Self {
        JsValue::from(e.to_string())
    }
}

#[derive(Tsify)]
#[wasm_bindgen]
pub struct Composer {
    composer: ComposerRs,
}

#[wasm_bindgen]
extern "C" {
    pub type WasmHTTPClient;

    #[wasm_bindgen(method, catch)]
    async fn json(this: &WasmHTTPClient, path: &str) -> Result<JsValue, JsValue>;
}

#[async_trait(?Send)]
impl HTTPClient for WasmHTTPClient {
    async fn json(&self, path: &str) -> Result<String, String> {
        let result = self.json(path).await.unwrap();

        let result = result
            .as_string()
            .ok_or_else(|| "Failed to convert JS string to Rust string".to_string())?;

        Ok(result)
    }
}

#[wasm_bindgen]
impl Composer {
    #[wasm_bindgen(constructor)]
    pub fn new(algod_client: WasmHTTPClient) -> Self {
        let algod_client = algokit_utils::AlgodClient::new(Box::new(algod_client));
        Composer {
            composer: ComposerRs::new(algod_client),
        }
    }

    pub async fn get_suggested_params(&self) -> Result<String, ComposerError> {
        self.composer
            .get_suggested_params()
            .await
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    #[wasm_bindgen(js_name = "addTransaction")]
    pub fn add_transaction(&mut self, transaction: FfiTransaction) -> Result<(), ComposerError> {
        self.composer
            .add_transaction(transaction.try_into().map_err(|e: AlgoKitTransactError| {
                ComposerError::TransactionsError(e.to_string())
            })?)
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    pub fn encode(&self) -> Result<Vec<Uint8Array>, ComposerError> {
        Ok(self
            .composer
            .encode()
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))?
            .iter()
            .map(|b| Uint8Array::from(b.as_slice()))
            .collect::<Vec<Uint8Array>>())
    }

    #[wasm_bindgen(getter)]
    pub fn transactions(&self) -> Result<Vec<FfiTransaction>, ComposerError> {
        Ok(self
            .composer
            .transactions()
            .into_iter()
            .map(|tx| {
                tx.try_into().map_err(|e: AlgoKitTransactError| {
                    ComposerError::TransactionsError(e.to_string())
                })
            })
            .collect::<Result<Vec<FfiTransaction>, ComposerError>>()?)
    }

    #[wasm_bindgen(js_name = "throwRustError")]
    pub fn throw_rust_error(&self) -> Result<(), ComposerError> {
        Err(ComposerError::TransactionsError(
            "This is a Rust error thrown from the Composer".to_string(),
        ))
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

#[derive(Serialize, Deserialize)]
struct JsComposerValue {
    transactions: Vec<FfiTransaction>,
}

impl From<&Composer> for JsComposerValue {
    fn from(composer: &Composer) -> Self {
        JsComposerValue {
            transactions: composer.transactions().unwrap_or_else(|_| vec![]),
        }
    }
}
