use algokit_http_client_trait::WasmHttpClient;
use js_sys::JSON;
use js_sys::JsString;
use js_sys::Uint8Array;
use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
impl Composer {
    #[wasm_bindgen(constructor)]
    pub fn new(algod_client: WasmHttpClient) -> Self {
        let algod_client = algokit_utils::AlgodClient::new(Arc::new(algod_client));
        Composer {
            composer: FfiMutex::new(ComposerRs::new(algod_client)),
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

#[cfg(feature = "ffi_wasm")]
impl From<ComposerError> for JsValue {
    fn from(e: ComposerError) -> Self {
        JsValue::from(e.to_string())
    }
}
