//! Error types for the AlgoKit Core transact module.
//!
//! This module defines the various error types that can occur during Algorand
//! transaction processing, including encoding/decoding errors, validation errors,
//! and other transaction-related failures.

use snafu::Snafu;

/// Represents errors that can occur during Algorand transaction operations.
///
/// This enum encompasses various failure scenarios that may arise when creating,
/// manipulating, serializing, or deserializing Algorand transactions.
#[derive(Debug, Snafu)]
pub enum AlgoKitTransactError {
    #[snafu(display("Error occurred during encoding: {source}"))]
    EncodingError { source: rmp_serde::encode::Error },

    #[snafu(display("Error occurred during msgpack encoding: {source}"))]
    MsgpackEncodingError { source: rmpv::encode::Error },

    #[snafu(display("Error occurred during decoding at path {path}: {source}"))]
    DecodingError {
        path: String,
        source: rmp_serde::decode::Error,
    },

    #[snafu(display("Unknown transaction type: {err_msg}"))]
    UnknownTransactionType { err_msg: String },

    #[snafu(display("{err_msg}"))]
    InputError { err_msg: String },

    #[snafu(display("{err_msg}"))]
    InvalidAddress { err_msg: String },

    #[snafu(display("Invalid multisig signature: {err_msg}"))]
    InvalidMultisigSignature { err_msg: String },
}

impl From<rmp_serde::encode::Error> for AlgoKitTransactError {
    fn from(source: rmp_serde::encode::Error) -> Self {
        AlgoKitTransactError::EncodingError { source }
    }
}

impl From<rmpv::encode::Error> for AlgoKitTransactError {
    fn from(source: rmpv::encode::Error) -> Self {
        AlgoKitTransactError::MsgpackEncodingError { source }
    }
}

impl From<serde_path_to_error::Error<rmp_serde::decode::Error>> for AlgoKitTransactError {
    fn from(err: serde_path_to_error::Error<rmp_serde::decode::Error>) -> Self {
        AlgoKitTransactError::DecodingError {
            path: err.path().to_string(),
            source: err.into_inner(),
        }
    }
}
