use algo_models::AlgorandMsgpack;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum MsgPackError {
    #[error("Algorand encoding failed")]
    EncodingError,
    #[error("Algorand decoding failed")]
    DecodingError,
}

impl From<algo_models::MsgPackError> for MsgPackError {
    fn from(e: algo_models::MsgPackError) -> Self {
        match e {
            algo_models::MsgPackError::DecodeError => MsgPackError::DecodingError,
            algo_models::MsgPackError::EncodeError => MsgPackError::EncodingError,
            algo_models::MsgPackError::RmpvDecodeError => MsgPackError::DecodingError,
            algo_models::MsgPackError::RmpvEncodeError => MsgPackError::EncodingError,
            algo_models::MsgPackError::RmpvConvertError => MsgPackError::EncodingError,
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

#[cfg(feature = "ffi_wasm")]
impl From<MsgPackError> for JsValue {
    fn from(e: MsgPackError) -> Self {
        JsValue::from(e.to_string())
    }
}

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

// Go from wasm to rust
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

// Go from rust to wasm
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

#[cfg_attr(
    feature = "ffi_wasm",
    wasm_bindgen(js_name = "getEncodedTransactionType")
)]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[allow(dead_code)]

/// Get the transaction type from the encoded transaction.
/// This is particularly useful when decoding a transaction that has a unknow type
pub fn get_encoded_transaction_type(bytes: &[u8]) -> Result<TransactionType, MsgPackError> {
    let header: TransactionHeader =
        rmp_serde::from_slice(bytes).map_err(|_| MsgPackError::DecodingError)?;
    Ok(header.transaction_type)
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "encodePayment"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[allow(dead_code)]
pub fn encode_payment(tx: PayTransactionFields) -> Result<Vec<u8>, MsgPackError> {
    let ctx: algo_models::PayTransactionFields = tx.into();
    ctx.encode().map_err(|_| MsgPackError::EncodingError)
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "decodePayment"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[allow(dead_code)]
pub fn decode_payment(bytes: &[u8]) -> Result<PayTransactionFields, MsgPackError> {
    let tx = algo_models::PayTransactionFields::decode(bytes)?;
    Ok(tx.into())
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "encodeAssetTransfer"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[allow(dead_code)]
pub fn encode_asset_transfer(tx: AssetTransferTransactionFields) -> Result<Vec<u8>, MsgPackError> {
    let ctx: algo_models::AssetTransferTransactionFields = tx.into();
    ctx.encode().map_err(|_| MsgPackError::EncodingError)
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "decodeAssetTransfer"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[allow(dead_code)]
pub fn decode_asset_transfer(bytes: &[u8]) -> Result<AssetTransferTransactionFields, MsgPackError> {
    let tx = algo_models::AssetTransferTransactionFields::decode(bytes)?;
    Ok(tx.into())
}

#[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = "attachSignature"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[allow(dead_code)]
pub fn attach_signature(encoded_tx: &[u8], signature: &[u8]) -> Result<Vec<u8>, MsgPackError> {
    let encoded_tx = algo_models::Transaction::decode(encoded_tx)?;
    let signed_tx = algo_models::SignedTransaction {
        transaction: encoded_tx,
        signature: ByteBuf::from(signature.to_vec()),
    };
    signed_tx.encode().map_err(|_| MsgPackError::EncodingError)
}
