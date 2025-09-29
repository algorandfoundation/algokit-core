use crate::applications::app_client::LogicError;
use crate::applications::app_deployer::AppDeployError;
use crate::{AppClientError, ComposerError, TransactionSenderError};
use algokit_abi::ABIError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AppFactoryError {
    #[snafu(display("Method not found: {message}"))]
    CompilationError { message: String },
    #[snafu(display("Validation error: {message}"))]
    ValidationError { message: String },
    #[snafu(display("Params builder error: {message}"))]
    ParamsBuilderError { message: String },
    #[snafu(display("ABI error: {source}"))]
    ABIError { source: ABIError },
    #[snafu(display("App client error: {source}"))]
    AppClientError { source: AppClientError },
    #[snafu(display("Transaction sender error: {source}"))]
    AppDeployerError { source: AppDeployError },
    #[snafu(display("Composer error: {source}"))]
    ComposerError { source: ComposerError },
    #[snafu(display("{message}"))]
    LogicError {
        message: String,
        logic: Box<LogicError>,
    },
    #[snafu(display("Transaction sender error: {source}"))]
    TransactionSenderError { source: TransactionSenderError },
}
