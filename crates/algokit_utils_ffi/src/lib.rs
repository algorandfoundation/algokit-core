use algokit_transact_ffi::AlgoKitTransactError;
use algokit_transact_ffi::Transaction as FfiTransaction;
use algokit_utils::Composer as ComposerRs;

use js_sys::JSON;
use js_sys::JsString;

use js_sys::Uint8Array;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[cfg(feature = "ffi_wasm")]
use tsify_next::Tsify;
#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::JsValue;
#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "ffi_uniffi")]
impl UniffiCustomTypeConverter for Uint8Array {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Uint8Array::from(val.as_slice()))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec()
    }
}

#[cfg(feature = "ffi_uniffi")]
uniffi::custom_type!(Uint8Array, Vec<u8>);

// thiserror is used to easily create errors than can be propagated to the language bindings
// UniFFI will create classes for errors (i.e. `MsgPackError.EncodingError` in Python)
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum ComposerError {
    #[error("TransactionsError: {0}")]
    TransactionsError(String),
}

// For now, in WASM we just throw the string, hence the error
// type being included in the error string above
// Perhaps in the future we could use a class like in UniFFI
#[cfg(feature = "ffi_wasm")]
impl From<ComposerError> for JsValue {
    fn from(e: ComposerError) -> Self {
        JsValue::from(e.to_string())
    }
}

#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Object))]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
pub struct Composer {
    composer: Mutex<ComposerRs>,
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
impl Composer {
    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(constructor))]
    #[cfg_attr(feature = "ffi_uniffi", uniffi::constructor)]
    pub fn new() -> Self {
        Composer {
            composer: Mutex::new(ComposerRs::new()),
        }
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "addTransaction"))]
    pub fn add_transaction(&self, transaction: FfiTransaction) -> Result<(), ComposerError> {
        self.composer
            .lock()
            .unwrap()
            .add_transaction(transaction.try_into().map_err(|e: AlgoKitTransactError| {
                ComposerError::TransactionsError(e.to_string())
            })?)
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))
    }

    pub fn encode(&self) -> Result<Vec<Uint8Array>, ComposerError> {
        Ok(self
            .composer
            .lock()
            .unwrap()
            .encode()
            .map_err(|e| ComposerError::TransactionsError(e.to_string()))?
            .iter()
            .map(|b| Uint8Array::from(b.as_slice()))
            .collect::<Vec<Uint8Array>>())
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(getter))]
    pub fn transactions(&self) -> Result<Vec<FfiTransaction>, ComposerError> {
        Ok(self
            .composer
            .lock()
            .unwrap()
            .transactions()
            .into_iter()
            .map(|tx| {
                tx.try_into().map_err(|e: AlgoKitTransactError| {
                    ComposerError::TransactionsError(e.to_string())
                })
            })
            .collect::<Result<Vec<FfiTransaction>, ComposerError>>()?)
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "throwRustError"))]
    pub fn throw_rust_error(&self) -> Result<(), ComposerError> {
        Err(ComposerError::TransactionsError(
            "This is a Rust error thrown from the Composer".to_string(),
        ))
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
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
impl Composer {
    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "valueOf"))]
    pub fn value_of(&self) -> Result<JsValue, JsValue> {
        let ser = serde_wasm_bindgen::Serializer::new()
            .serialize_large_number_types_as_bigints(true)
            .serialize_bytes_as_arrays(false);

        // Ok(serde_wasm_bindgen::to_value(&JsComposerValue::from(self))?)
        Ok(JsComposerValue::from(self).serialize(&ser)?)
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "toJSON"))]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JSON::parse(
            &self
                .to_string()?
                .as_string()
                .ok_or("Failed to convert JS string to Rust string")?,
        )
    }

    #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "toString"))]
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
