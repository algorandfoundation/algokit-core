#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

pub mod models;

// Re-export all models
pub use models::*;

// TODO: Re-export FFI types from algokit_transact_ffi
// pub use algokit_transact_ffi::{SignedTransaction, Transaction, PaymentTransactionFields, Address};

// Create FFI-compatible types for auto-generated models
// These are simpler than the full algokit_transact_ffi types and focused on msgpack serialization

/// FFI-compatible SignedTransaction for msgpack operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct SignedTransaction {
    /// The transaction that has been signed.
    #[serde(rename = "txn")]
    pub transaction: Transaction,

    /// The Ed25519 signature authorizing the transaction (64 bytes).
    #[serde(rename = "sig")]
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
}

impl SignedTransaction {
    /// Create a new SignedTransaction
    pub fn new(transaction: Transaction, signature: Vec<u8>) -> Self {
        Self {
            transaction,
            signature,
        }
    }
}

impl MsgpackEncodable for SignedTransaction {}

/// FFI-compatible Transaction for msgpack operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
#[serde(tag = "type")]
pub enum Transaction {
    #[serde(rename = "pay")]
    Payment {
        #[serde(flatten)]
        fields: PaymentTransactionFields,
    },
}

/// FFI-compatible PaymentTransactionFields for msgpack operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct PaymentTransactionFields {
    /// Amount in microAlgos
    #[serde(rename = "amt")]
    pub amount: u64,

    /// Transaction fee in microAlgos
    #[serde(rename = "fee")]
    pub fee: u64,

    /// First valid round
    #[serde(rename = "fv")]
    pub first_valid: u64,

    /// Genesis ID
    #[serde(rename = "gen")]
    pub genesis_id: String,

    /// Genesis hash
    #[serde(rename = "gh", skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes")]
    pub genesis_hash: Option<Vec<u8>>,

    /// Last valid round
    #[serde(rename = "lv")]
    pub last_valid: u64,

    /// Note field
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes")]
    pub note: Option<Vec<u8>>,

    /// Receiver address
    #[serde(rename = "rcv")]
    #[serde(with = "serde_bytes")]
    pub receiver: Vec<u8>,

    /// Sender address
    #[serde(rename = "snd")]
    #[serde(with = "serde_bytes")]
    pub sender: Vec<u8>,

    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: String,
}

// Conversion functions between algokit_transact types and msgpack FFI types
impl From<algokit_transact::SignedTransaction> for SignedTransaction {
    fn from(value: algokit_transact::SignedTransaction) -> Self {
        Self {
            transaction: value.transaction.into(),
            signature: value.signature.to_vec(),
        }
    }
}

impl From<algokit_transact::Transaction> for Transaction {
    fn from(value: algokit_transact::Transaction) -> Self {
        match value {
            algokit_transact::Transaction::Payment(payment) => Transaction::Payment {
                fields: PaymentTransactionFields {
                    sender: payment.header.sender.pub_key.to_vec(),
                    receiver: payment.receiver.pub_key.to_vec(),
                    amount: payment.amount,
                    fee: payment.header.fee,
                    first_valid: payment.header.first_valid,
                    last_valid: payment.header.last_valid,
                    genesis_id: payment.header.genesis_id.unwrap_or_default(),
                    genesis_hash: payment.header.genesis_hash.map(|h| h.to_vec()),
                    note: payment.header.note,
                    transaction_type: "pay".to_string(),
                },
            },
            // Add other transaction types as needed
            _ => panic!("Transaction type not supported in msgpack FFI wrapper yet"),
        }
    }
}

// Import algokit_transact types for the encoding functions
use algokit_transact::{AlgorandMsgpack, EstimateTransactionSize, TransactionId};

/// Wrapper for a single transaction that matches the original JS structure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct TransactionWrapper {
    /// The transaction that matches the original JS structure.
    #[serde(rename = "txn")]
    pub transaction: Transaction,
}

impl TransactionWrapper {
    /// Create a new TransactionWrapper
    pub fn new(transaction: Transaction) -> Self {
        Self { transaction }
    }
}

impl MsgpackEncodable for TransactionWrapper {}

/// Error types for msgpack operations
#[derive(Error, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub enum MsgpackError {
    #[error("Encoding not supported for this model")]
    EncodingNotSupported,
    #[error("Decoding not supported for this model")]
    DecodingNotSupported,
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

impl From<rmp_serde::encode::Error> for MsgpackError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        MsgpackError::SerializationError(err.to_string())
    }
}

impl From<rmp_serde::decode::Error> for MsgpackError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        MsgpackError::DeserializationError(err.to_string())
    }
}

impl From<serde_json::Error> for MsgpackError {
    fn from(err: serde_json::Error) -> Self {
        MsgpackError::SerializationError(err.to_string())
    }
}

/// Trait for models that support msgpack encoding
pub trait MsgpackEncodable: Serialize {}

/// Trait for models that support msgpack decoding
pub trait MsgpackDecodable: for<'de> Deserialize<'de> {}

/// Trait for models that support JSON serialization
pub trait JsonSerializable: Serialize + for<'de> Deserialize<'de> {
    /// Convert the model to a JSON dictionary (HashMap)
    fn to_dict(
        &self,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>, MsgpackError> {
        let json_str = serde_json::to_string(self)?;
        let json_value: serde_json::Value = serde_json::from_str(&json_str)?;
        if let serde_json::Value::Object(map) = json_value {
            Ok(map.into_iter().collect())
        } else {
            Err(MsgpackError::SerializationError(
                "Model did not serialize to a JSON object".to_string(),
            ))
        }
    }

    /// Create the model from a JSON dictionary (HashMap)
    fn from_dict(
        dict: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<Self, MsgpackError> {
        let json_value = serde_json::Value::Object(dict.into_iter().collect());
        let result = serde_json::from_value(json_value)?;
        Ok(result)
    }

    /// Convert the model to a JSON string
    fn to_json(&self) -> Result<String, MsgpackError> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    /// Create the model from a JSON string
    fn from_json(json_str: &str) -> Result<Self, MsgpackError> {
        serde_json::from_str(json_str).map_err(|e| e.into())
    }
}

/// Canonical msgpack encoding following Algorand's rules:
/// 1. Every integer must be encoded to the smallest type possible (0-255->8bit, 256-65535->16bit, etc)
/// 2. All field names must be sorted alphabetically
/// 3. All empty and 0 fields should be omitted (handled by serde skip_serializing_if)
/// 4. Every positive number must be encoded as uint (handled by rmp-serde)
/// 5. Binary blob should be used for binary data and string for strings (handled by rmp-serde)
pub fn encode_msgpack_canonical<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
    // Create a custom serializer with struct_map to ensure keys are sorted alphabetically
    let mut buf = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut buf).with_struct_map();

    // Serialize with canonical settings
    model.serialize(&mut serializer)?;
    Ok(buf)
}

/// Generic function to encode a model to msgpack using canonical rules
pub fn encode_msgpack<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
    encode_msgpack_canonical(model)
}

/// Generic function to decode msgpack to a model
pub fn decode_msgpack<T: MsgpackDecodable>(data: &[u8]) -> Result<T, MsgpackError> {
    rmp_serde::from_slice(data).map_err(|e| e.into())
}

// =============================================================================
// ENDPOINT-LEVEL MODELS - Models directly used in API request/response schemas
// =============================================================================

/// Encode Account to msgpack bytes
/// Used in: GET /v2/accounts/{address} response
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn encode_account(model: Account) -> Result<Vec<u8>, MsgpackError> {
    encode_msgpack(&model)
}

/// Decode msgpack bytes to Account
/// Used in: GET /v2/accounts/{address} response
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_account(data: Vec<u8>) -> Result<Account, MsgpackError> {
    decode_msgpack(&data)
}

/// Encode SimulateRequest to msgpack bytes
/// Used in: POST /v2/transactions/simulate request body
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn encode_simulate_request(model: SimulateRequest) -> Result<Vec<u8>, MsgpackError> {
    encode_msgpack(&model)
}

/// Encode DryrunRequest to msgpack bytes
/// Used in: POST /v2/teal/dryrun request body
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn encode_dryrun_request(model: DryrunRequest) -> Result<Vec<u8>, MsgpackError> {
    encode_msgpack(&model)
}

/// Decode msgpack bytes to ErrorResponse
/// Used in: Various error responses across endpoints
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_error_response(data: Vec<u8>) -> Result<ErrorResponse, MsgpackError> {
    decode_msgpack(&data)
}

/// Decode msgpack bytes to PendingTransactionResponse
/// Used in: GET /v2/transactions/pending/{txid} response
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_pending_transaction_response(
    data: Vec<u8>,
) -> Result<PendingTransactionResponse, MsgpackError> {
    decode_msgpack(&data)
}

/// Decode msgpack bytes to LedgerStateDeltaForTransactionGroup
/// Used in: GET /v2/deltas/{round}/txn/group response (as array items)
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_ledger_state_delta_for_transaction_group(
    data: Vec<u8>,
) -> Result<LedgerStateDeltaForTransactionGroup, MsgpackError> {
    decode_msgpack(&data)
}

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

// Test SimulateRequest encoding
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_request_encoding() {
        let txns = vec![SignedTransaction::new(
            Transaction::Payment {
                fields: PaymentTransactionFields {
                    amount: 1000000,
                    fee: 1000,
                    first_valid: 1000000,
                    last_valid: 1000000,
                    genesis_id: "wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=".to_string(),
                    genesis_hash: None,
                    note: None,
                    receiver: vec![0; 32],
                    sender: vec![0; 32],
                    transaction_type: "pay".to_string(),
                },
            },
            vec![0; 64],
        )];
        let group = SimulateRequestTransactionGroup::new(txns);
        let simulate_request = SimulateRequest::new(vec![group]);
        let encoded = encode_simulate_request(simulate_request).unwrap();
        let base64 = base64::encode(encoded);
        println!("encoded: {:?}", base64);
    }
}
