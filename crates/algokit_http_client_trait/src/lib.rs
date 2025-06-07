use async_trait::async_trait;

uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum HttpError {
    #[error("HttpError: {0}")]
    HttpError(String),
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait HTTPClient: Send + Sync {
    async fn json(&self, path: String) -> Result<String, HttpError>;
}
