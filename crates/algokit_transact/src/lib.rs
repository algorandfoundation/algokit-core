mod address;
pub mod constants;
mod error;
pub mod msgpack;
mod traits;
mod transactions;
mod utils;

// Re-export all the public items
pub use address::Address;
pub use constants::*;
pub use error::AlgoKitTransactError;
pub use traits::{AlgorandMsgpack, EstimateTransactionSize, TransactionId, Transactions, Validate};
pub use transactions::{
    ApplicationCallTransactionBuilder, ApplicationCallTransactionFields,
    AssetConfigTransactionBuilder, AssetConfigTransactionFields, AssetTransferTransactionBuilder,
    AssetTransferTransactionFields, BoxReference, FeeParams, OnApplicationComplete,
    PaymentTransactionBuilder, PaymentTransactionFields, SignedTransaction, StateSchema,
    Transaction, TransactionHeader, TransactionHeaderBuilder,
};

// Re-export msgpack functionality
pub use msgpack::{
    decode_base64_msgpack_to_json, decode_msgpack_to_json, encode_json_to_base64_msgpack,
    encode_json_to_msgpack, sort_and_filter_json, supported_models, AlgoKitMsgPackError,
    ModelRegistry, ModelType, ToMsgPack,
};

#[cfg(test)]
mod tests;

#[cfg(feature = "test_utils")]
pub mod test_utils;
