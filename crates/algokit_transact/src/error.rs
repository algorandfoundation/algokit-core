use thiserror::Error;

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
}
