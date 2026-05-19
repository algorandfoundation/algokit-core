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
}
