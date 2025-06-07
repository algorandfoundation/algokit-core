use async_trait::async_trait;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum HttpError {
    #[error("HttpError: {0}")]
    HttpError(String),
}

#[cfg_attr(feature = "uniffi", uniffi::export(with_foreign))]
#[async_trait]
pub trait HTTPClient: Send + Sync {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}
