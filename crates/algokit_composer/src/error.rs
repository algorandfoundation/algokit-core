use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ComposerError {
    #[snafu(display("invalid address {address}: {source}"))]
    InvalidAddress {
        address: String,
        source: algokit_transact::AlgoKitTransactError,
    },

    #[snafu(display("invalid {field} byte length: expected {expected} bytes, got {found}"))]
    InvalidByteLength {
        field: &'static str,
        expected: usize,
        found: usize,
    },

    #[snafu(display("{source}"))]
    Transact {
        source: algokit_transact::AlgoKitTransactError,
    },

    #[snafu(display("transaction count {txns} does not match secret key count {keys}"))]
    SignerCountMismatch { txns: usize, keys: usize },

    #[snafu(display("signing failed: {message}"))]
    Signing { message: String },
}

impl From<algokit_transact::AlgoKitTransactError> for ComposerError {
    fn from(source: algokit_transact::AlgoKitTransactError) -> Self {
        ComposerError::Transact { source }
    }
}
