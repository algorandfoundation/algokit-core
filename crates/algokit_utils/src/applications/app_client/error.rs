use crate::ComposerError;
use crate::applications::app_client::types::LogicError;
use crate::clients::app_manager::AppManagerError;
use crate::clients::client_manager::ClientManagerError;
use crate::transactions::TransactionSenderError;
use algokit_abi::error::ABIError;
use algokit_transact::AlgoKitTransactError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AppClientError {
    #[snafu(display(
        "No app ID found for network {network_names:?}. Available keys in spec: {available:?}"
    ))]
    AppIdNotFound {
        network_names: Vec<String>,
        available: Vec<String>,
    },
    #[snafu(display("Network error: {message}"))]
    Network { message: String },
    #[snafu(display("Lookup error: {message}"))]
    Lookup { message: String },
    #[snafu(display("Method not found: {message}"))]
    MethodNotFound { message: String },
    #[snafu(display("ABI error: {source}"))]
    ABIError { source: ABIError },
    #[snafu(display("Transaction sender error: {source}"))]
    TransactionSenderError { source: TransactionSenderError },
    #[snafu(display("App manager error: {source}"))]
    AppManagerError { source: AppManagerError },
    #[snafu(display("Compilation error: {message}"))]
    CompilationError { message: String },
    #[snafu(display("Validation error: {message}"))]
    ValidationError { message: String },
    #[snafu(display("{message}"))]
    LogicError {
        message: String,
        logic: Box<LogicError>,
    },
    #[snafu(display("Transact error: {source}"))]
    TransactError { source: AlgoKitTransactError },
    #[snafu(display("Params builder error: {message}"))]
    ParamsBuilderError { message: String },
    #[snafu(display("Composer error: {source}"))]
    ComposerError { source: ComposerError },
    #[snafu(display("App state error: {message}"))]
    AppStateError { message: String },
    #[snafu(display("Decode error: {message}"))]
    DecodeError { message: String },
    #[snafu(display("Client manager error: {source}"))]
    ClientManagerError { source: ClientManagerError },
}
