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
pub trait HTTPClient: Send + Sync {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
pub trait HTTPClient {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}
