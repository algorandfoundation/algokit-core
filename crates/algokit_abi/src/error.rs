use snafu::Snafu;

/// Represents an error that can occur during ABI operations.
#[derive(Debug, Snafu)]
pub enum ABIError {
    /// An error that occurs during ABI type validation.
    #[snafu(display("ABI validation failed: {message}"))]
    ValidationError { message: String },

    /// An error that occurs during ABI encoding.
    #[snafu(display("ABI encoding failed: {message}"))]
    EncodingError { message: String },

    /// An error that occurs during ABI decoding.
    #[snafu(display("ABI decoding failed: {message}"))]
    DecodingError { message: String },
}
