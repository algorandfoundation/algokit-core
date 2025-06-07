use async_trait::async_trait;

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum HttpError {
    #[error("HttpError: {0}")]
    HttpError(String),
}

#[cfg(not(feature = "ffi_wasm"))]
#[uniffi::export(with_foreign)]
#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
pub trait HttpClient {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}

#[wasm_bindgen]
#[cfg(feature = "ffi_wasm")]
extern "C" {
    pub type WasmHttpClient;

    #[wasm_bindgen(method, catch)]
    async fn json(this: &WasmHttpClient, path: &str) -> Result<JsValue, JsValue>;
}

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
impl HttpClient for WasmHttpClient {
    async fn json(&self, path: String) -> Result<String, HttpError> {
        let result = self.json(&path).await.unwrap();

        let result = result.as_string().ok_or_else(|| {
            HttpError::HttpError("Failed to convert JS string to Rust string".to_string())
        })?;

        Ok(result)
    }
}
