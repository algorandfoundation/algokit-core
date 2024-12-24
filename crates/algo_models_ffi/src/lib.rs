use algo_models::AlgorandMsgpack;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

// thiserror is used to easily create errors than can be propagated to the language bindings
// UniFFI will create classes for errors (i.e. `MsgPackError.EncodingError` in Python)
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum MsgPackError {
    #[error("EncodingError: {0}")]
    EncodingError(String),
    #[error("DecodingError: {0}")]
    DecodingError(String),
}

// For now, in WASM we just throw the string, hence the error
// type being included in the error string above
// Perhaps in the future we could use a class like in UniFFI
#[cfg(feature = "ffi_wasm")]
impl From<MsgPackError> for JsValue {
    fn from(e: MsgPackError) -> Self {
        JsValue::from(e.to_string())
    }
}

// Convert errors from the Rust crate into the FFI-specific errors
impl From<algo_models::MsgPackError> for MsgPackError {
    fn from(e: algo_models::MsgPackError) -> Self {
        match e {
            algo_models::MsgPackError::DecodeError(_) => MsgPackError::DecodingError(e.to_string()),
            algo_models::MsgPackError::EncodeError(_) => MsgPackError::EncodingError(e.to_string()),
            algo_models::MsgPackError::RmpvDecodeError(_) => {
                MsgPackError::DecodingError(e.to_string())
            }
            algo_models::MsgPackError::RmpvEncodeError(_) => {
                MsgPackError::EncodingError(e.to_string())
            }
            algo_models::MsgPackError::UnknownTransactionType => {
                MsgPackError::DecodingError(e.to_string())
            }
        }
    }
}

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "ffi_wasm")]
use tsify_next::Tsify;
#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

// We need to use ByteBuf directly in the structs to get Uint8Array in TSify
// custom_type! and this impl is used to convert the ByteBuf to a Vec<u8> for the UniFFI bindings
#[cfg(feature = "ffi_uniffi")]
impl UniffiCustomTypeConverter for ByteBuf {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(ByteBuf::from(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec()
    }
}

#[cfg(feature = "ffi_uniffi")]
uniffi::custom_type!(ByteBuf, Vec<u8>);

// This becomes an enum in UniFFI lnaguage bindings and a
// string literal union in TS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum TransactionType {
    Payment,
    AssetTransfer,
    AssetFreeze,
    AssetConfig,
    KeyRegistration,
    ApplicationCall,
}

// Even though these stucts are all defined in the crate, we need to re-define them
// because we have to use different serde attributes for the struct fields.
// In the crate, we need to use the msgpack names for the fields, but in the FFI
// we need to use the camelCase names for the fields for TSify.

// A Record in UniFFI becomes a native struct in the language bindings
// and an interface in TS. Using `large_number_types_as_bigints` is essential
// for tsify to correctly use bigint for uint64s
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(
    feature = "ffi_wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
#[cfg_attr(feature = "ffi_wasm", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
/// The transaction header contains the fields that can be present in any transaction.
/// "Header" only indicates that these are common fields, NOT that they are the first fields in the transaction.
pub struct TransactionHeader {
    /// The type of transaction
    transaction_type: TransactionType,

    /// The sender of the transaction
    sender: ByteBuf,

    fee: u64,

    first_valid: u64,

    last_valid: u64,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    genesis_hash: Option<ByteBuf>,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    genesis_id: Option<String>,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    note: Option<ByteBuf>,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    rekey_to: Option<ByteBuf>,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    lease: Option<ByteBuf>,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    group: Option<ByteBuf>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(
    feature = "ffi_wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
#[cfg_attr(feature = "ffi_wasm", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct PayTransactionFields {
    header: TransactionHeader,

    receiver: ByteBuf,

    amount: u64,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    close_remainder_to: Option<ByteBuf>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(
    feature = "ffi_wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
#[cfg_attr(feature = "ffi_wasm", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct AssetTransferTransactionFields {
    header: TransactionHeader,

    asset_id: u64,

    amount: u64,

    receiver: ByteBuf,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    asset_sender: Option<ByteBuf>,

    #[cfg_attr(feature = "ffi_wasm", tsify(optional))]
    close_remainder_to: Option<ByteBuf>,
}

// Various from impls to convert between the FFI types and the crate types

impl From<TransactionType> for algo_models::TransactionType {
    fn from(tx: TransactionType) -> Self {
        match tx {
            TransactionType::Payment => algo_models::TransactionType::Payment,
            TransactionType::AssetTransfer => algo_models::TransactionType::AssetTransfer,
            TransactionType::AssetFreeze => algo_models::TransactionType::AssetFreeze,
            TransactionType::AssetConfig => algo_models::TransactionType::AssetConfig,
            TransactionType::KeyRegistration => algo_models::TransactionType::KeyRegistration,
            TransactionType::ApplicationCall => algo_models::TransactionType::ApplicationCall,
        }
    }
}

impl From<TransactionHeader> for algo_models::TransactionHeader {
    fn from(tx: TransactionHeader) -> Self {
        Self {
            transaction_type: tx.transaction_type.into(),
            sender: tx.sender,
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx.genesis_hash,
            note: tx.note,
            rekey_to: tx.rekey_to,
            lease: tx.lease,
            group: tx.group,
        }
    }
}

impl From<PayTransactionFields> for algo_models::PayTransactionFields {
    fn from(tx: PayTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            receiver: tx.receiver,
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<AssetTransferTransactionFields> for algo_models::AssetTransferTransactionFields {
    fn from(tx: AssetTransferTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver,
            asset_sender: tx.asset_sender,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<algo_models::TransactionHeader> for TransactionHeader {
    fn from(tx: algo_models::TransactionHeader) -> Self {
        Self {
            transaction_type: tx.transaction_type.into(),
            sender: ByteBuf::from(tx.sender.to_vec()),
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx.genesis_hash,
            note: tx.note,
            rekey_to: tx.rekey_to,
            lease: tx.lease,
            group: tx.group,
        }
    }
}

impl From<algo_models::PayTransactionFields> for PayTransactionFields {
    fn from(tx: algo_models::PayTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            receiver: ByteBuf::from(tx.receiver.to_vec()),
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<algo_models::AssetTransferTransactionFields> for AssetTransferTransactionFields {
    fn from(tx: algo_models::AssetTransferTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: ByteBuf::from(tx.receiver.to_vec()),
            asset_sender: tx.asset_sender,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<algo_models::TransactionType> for TransactionType {
    fn from(tx: algo_models::TransactionType) -> Self {
        match tx {
            algo_models::TransactionType::Payment => TransactionType::Payment,
            algo_models::TransactionType::AssetTransfer => TransactionType::AssetTransfer,
            algo_models::TransactionType::AssetFreeze => TransactionType::AssetFreeze,
            algo_models::TransactionType::AssetConfig => TransactionType::AssetConfig,
            algo_models::TransactionType::KeyRegistration => TransactionType::KeyRegistration,
            algo_models::TransactionType::ApplicationCall => TransactionType::ApplicationCall,
        }
    }
}

// Each function need to be explicitly renamed for WASM
// and exported for UniFFI

#[cfg_attr(
    feature = "ffi_wasm",
    wasm_bindgen(js_name = "getEncodedTransactionType")
)]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
/// Get the transaction type from the encoded transaction.
/// This is particularly useful when decoding a transaction that has a unknow type
pub fn get_encoded_transaction_type(bytes: &[u8]) -> Result<TransactionType, MsgPackError> {
    let header: TransactionHeader =
        rmp_serde::from_slice(bytes).map_err(|w| MsgPackError::DecodingError(w.to_string()))?;
    Ok(header.transaction_type)
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "encodePayment"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn encode_payment(tx: PayTransactionFields) -> Result<Vec<u8>, MsgPackError> {
    let ctx: algo_models::PayTransactionFields = tx.into();
    Ok(ctx.encode()?)
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "decodePayment"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn decode_payment(bytes: &[u8]) -> Result<PayTransactionFields, MsgPackError> {
    let tx = algo_models::PayTransactionFields::decode(bytes)?;
    Ok(tx.into())
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "encodeAssetTransfer"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn encode_asset_transfer(tx: AssetTransferTransactionFields) -> Result<Vec<u8>, MsgPackError> {
    let ctx: algo_models::AssetTransferTransactionFields = tx.into();
    Ok(ctx.encode()?)
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "decodeAssetTransfer"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn decode_asset_transfer(bytes: &[u8]) -> Result<AssetTransferTransactionFields, MsgPackError> {
    let tx = algo_models::AssetTransferTransactionFields::decode(bytes)?;
    Ok(tx.into())
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "attachSignature"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn attach_signature(encoded_tx: &[u8], signature: &[u8]) -> Result<Vec<u8>, MsgPackError> {
    let encoded_tx = algo_models::Transaction::decode(encoded_tx)?;
    let signed_tx = algo_models::SignedTransaction {
        transaction: encoded_tx,
        signature: ByteBuf::from(signature.to_vec()),
    };
    Ok(signed_tx.encode()?)
}
