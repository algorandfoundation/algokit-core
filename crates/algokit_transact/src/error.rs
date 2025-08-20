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
    #[snafu(display("Error ocurred during encoding: {source}"))]
    EncodingError { source: rmp_serde::encode::Error },

    #[snafu(display("Error ocurred during decoding: {source}"))]
    DecodingError { source: rmp_serde::decode::Error },

    #[snafu(display("Error ocurred during msgpack encoding: {source}"))]
    MsgpackEncodingError { source: rmpv::encode::Error },

    #[snafu(display("Error ocurred during msgpack decoding: {source}"))]
    MsgpackDecodingError { source: rmpv::decode::Error },

    #[snafu(display("Unknown transaction type: {message}"))]
    UnknownTransactionType { message: String },

    #[snafu(display("{message}"))]
    InputError { message: String },

    #[snafu(display("{message}"))]
    InvalidAddress { message: String },

    #[snafu(display("Invalid multisig signature: {message}"))]
    InvalidMultisigSignature { message: String },
}

impl From<rmp_serde::encode::Error> for AlgoKitTransactError {
    fn from(source: rmp_serde::encode::Error) -> Self {
        AlgoKitTransactError::EncodingError { source }
    }
}

impl From<rmp_serde::decode::Error> for AlgoKitTransactError {
    fn from(source: rmp_serde::decode::Error) -> Self {
        AlgoKitTransactError::DecodingError { source }
    }
}

impl From<rmpv::encode::Error> for AlgoKitTransactError {
    fn from(source: rmpv::encode::Error) -> Self {
        AlgoKitTransactError::MsgpackEncodingError { source }
    }
}

impl From<rmpv::decode::Error> for AlgoKitTransactError {
    fn from(source: rmpv::decode::Error) -> Self {
        AlgoKitTransactError::MsgpackDecodingError { source }
    }
}
