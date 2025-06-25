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
    async fn get(&self, path: String) -> Result<Vec<u8>, HttpError>;
}

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
#[cfg_attr(feature = "ffi_wasm", async_trait(?Send))]
#[cfg_attr(not(feature = "ffi_wasm"), async_trait)]
impl HttpClient for DefaultHttpClient {
    async fn get(&self, path: String) -> Result<Vec<u8>, HttpError> {
        let response = reqwest::get(self.host.clone() + &path)
            .await
            .map_err(|e| HttpError::HttpError(e.to_string()))?
            .bytes()
            .await
            .map_err(|e| HttpError::HttpError(e.to_string()))?
            .to_vec();

        Ok(response)
    }
}

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ffi_wasm")]
use js_sys::Uint8Array;

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
pub trait HttpClient {
    async fn get(&self, path: String) -> Result<Vec<u8>, HttpError>;
}

#[wasm_bindgen]
#[cfg(feature = "ffi_wasm")]
extern "C" {
    /// The interface for the JavaScript-based HTTP client that will be used in WASM environments.
    ///
    /// This mirrors the `HttpClient` trait, but wasm-bindgen doesn't support foreign traits so we define it separately.
    pub type WasmHttpClient;

    #[wasm_bindgen(method, catch)]
    async fn get(this: &WasmHttpClient, path: &str) -> Result<Uint8Array, JsValue>;
}

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
impl HttpClient for WasmHttpClient {
    async fn get(&self, path: String) -> Result<Vec<u8>, HttpError> {
        let result = self.get(&path).await.map_err(|e| {
            HttpError::HttpError(
                e.as_string().unwrap_or(
                    "A HTTP error ocurred in JavaScript, but it cannot be converted to a string"
                        .to_string(),
                ),
            )
        })?;

        Ok(result.to_vec())
    }
}
