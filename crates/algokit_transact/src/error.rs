//! Error types for the AlgoKit Core transact module.
//!
//! This module defines the various error types that can occur during Algorand
//! transaction processing, including encoding/decoding errors, validation errors,
//! and other transaction-related failures.

use thiserror::Error;

/// Represents errors that can occur during Algorand transaction operations.
///
/// This enum encompasses various failure scenarios that may arise when creating,
/// manipulating, serializing, or deserializing Algorand transactions.
#[derive(Debug, Error)]
pub enum AlgoKitTransactError {
    #[error("Error ocurred during encoding: {0}")]
    EncodingError(#[from] rmp_serde::encode::Error),

    #[error("Error ocurred during decoding: {0}")]
    DecodingError(#[from] rmp_serde::decode::Error),

    #[error("Error ocurred during msgpack encoding: {0}")]
    MsgpackEncodingError(#[from] rmpv::encode::Error),

    #[error("Error ocurred during msgpack decoding: {0}")]
    MsgpackDecodingError(#[from] rmpv::decode::Error),

    #[error("Unknown transaction type: {0}")]
    UnknownTransactionType(String),

    #[error("{0}")]
    InputError(String),

    #[error("{0}")]
    InvalidAddress(String),

    #[error("Invalid multisig signature: {0}")]
    InvalidMultisigSignature(String),
}
