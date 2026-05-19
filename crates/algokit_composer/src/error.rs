use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ComposerError {
    #[snafu(display("invalid address {address}: {source}"))]
    InvalidAddress {
        address: String,
        source: algokit_transact::AlgoKitTransactError,
    },

    #[snafu(display("invalid genesis hash length: expected 32 bytes, got {found}"))]
    InvalidGenesisHashLength { found: usize },

    #[snafu(display("invalid lease length: expected 32 bytes, got {found}"))]
    InvalidLeaseLength { found: usize },

    #[snafu(display("transaction type not yet supported by compose()"))]
    UnsupportedTxnType,
}
