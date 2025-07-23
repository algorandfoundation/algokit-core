use thiserror::Error;

/// Represents an error that can occur during ABI operations.
#[derive(Debug, Error)]
pub enum ABIError {
    /// An error that occurs during ABI type validation.
    #[error("ABI validation failed: {0}")]
    ValidationError(String),

    /// An error that occurs during ABI encoding.
    #[error("ABI encoding failed: {0}")]
    EncodingError(String),

    /// An error that occurs during ABI decoding.
    #[error("ABI decoding failed: {0}")]
    DecodingError(String),
}
