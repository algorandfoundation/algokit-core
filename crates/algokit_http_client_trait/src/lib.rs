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
#[cfg_attr(feature = "ffi_uniffi", uniffi::export(with_foreign))]
#[async_trait]
/// This trait must be implemented by any HTTP client that is used by our Rust crates.
/// It is assumed the implementing type will provide the hostname, port, headers, etc. as needed for each request.
///
/// By default, this trait requires the implementing type to be `Send + Sync`.
/// For WASM targets, enable the `ffi_wasm` feature to use a different implementation that is compatible with WASM.
///
/// With the `ffi_uniffi` feature enabled, this is exported as a foreign trait, meaning it is implemented natively in the foreign language.
///
pub trait HttpClient: Send + Sync {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}

#[cfg(feature = "default_client")]
use reqwest;

#[cfg(feature = "default_client")]
pub struct DefaultHttpClient {
    host: String,
}

#[cfg(feature = "default_client")]
impl DefaultHttpClient {
    pub fn new(host: &str) -> Self {
        DefaultHttpClient {
            host: host.to_string(),
        }
    }
}

#[cfg(feature = "default_client")]
#[async_trait]
impl HttpClient for DefaultHttpClient {
    async fn json(&self, path: String) -> Result<String, HttpError> {
        let response = reqwest::get(self.host.clone() + &path)
            .await
            .map_err(|e| HttpError::HttpError(e.to_string()))?
            .text()
            .await
            .map_err(|e| HttpError::HttpError(e.to_string()))?;

        Ok(response)
    }
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
    /// The interface for the JavaScript-based HTTP client that will be used in WASM environments.
    ///
    /// This mirrors the `HttpClient` trait, but wasm-bindgen doesn't support foreign traits so we define it separately.
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
