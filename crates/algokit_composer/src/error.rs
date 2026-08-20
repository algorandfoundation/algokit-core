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

    /// The algod call failed: transport error, non-2xx status, or an undecodable body.
    /// A simulation that ran and reported a failing group is data on `SimulateResult`,
    /// not an error.
    #[snafu(display("{source}"))]
    Algod { source: algod_client::apis::Error },

    #[snafu(display("msgpack error: {message}"))]
    Msgpack { message: String },

    /// The response does not line up with the request that produced it.
    #[snafu(display("unexpected simulate response: {message}"))]
    SimulateResponseShape { message: String },

    #[snafu(display("invalid simulate options: {message}"))]
    InvalidSimulateOptions { message: String },

    /// Produced only by `SimulateResult::into_result`; no simulate entry point returns
    /// it. Carries the response so the failure diagnostics survive.
    #[snafu(display(
        "Transaction failed at transaction(s) {failed_at} in the group. {failure_message}"
    ))]
    SimulationFailed {
        /// `failed-at` joined with ", ", or "unknown" when absent or empty.
        failed_at: String,
        failure_message: String,
        response: Box<algod_client::models::SimulateResponse>,
    },
}

impl From<algod_client::apis::Error> for ComposerError {
    fn from(source: algod_client::apis::Error) -> Self {
        ComposerError::Algod { source }
    }
}

impl From<algokit_transact::AlgoKitTransactError> for ComposerError {
    fn from(source: algokit_transact::AlgoKitTransactError) -> Self {
        ComposerError::Transact { source }
    }
}
