use algokit_msgpack::{
    decode_base64_msgpack_to_json as internal_decode_base64_msgpack_to_json,
    decode_msgpack_to_json as internal_decode_msgpack_to_json,
    encode_json_to_base64_msgpack as internal_encode_json_to_base64_msgpack,
    encode_json_to_msgpack as internal_encode_json_to_msgpack,
    AlgoKitMsgPackError as InternalMsgPackError, ModelType as InternalModelType,
};
use ffi_macros::ffi_func;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitMsgPackError {
    #[error("SerializationError: {0}")]
    SerializationError(String),
    #[error("MsgPackEncodeError: {0}")]
    MsgPackEncodeError(String),
    #[error("MsgPackDecodeError: {0}")]
    MsgPackDecodeError(String),
    #[error("Base64DecodeError: {0}")]
    Base64DecodeError(String),
    #[error("MsgPackWriteError: {0}")]
    MsgPackWriteError(String),
    #[error("UnknownModelError: {0}")]
    UnknownModelError(String),
}

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ffi_wasm")]
impl From<AlgoKitMsgPackError> for JsValue {
    fn from(e: AlgoKitMsgPackError) -> Self {
        JsValue::from(e.to_string())
    }
}

impl From<InternalMsgPackError> for AlgoKitMsgPackError {
    fn from(e: InternalMsgPackError) -> Self {
        match e {
            InternalMsgPackError::SerializationError(e) => {
                AlgoKitMsgPackError::SerializationError(e.to_string())
            }
            InternalMsgPackError::MsgpackEncodingError(e) => {
                AlgoKitMsgPackError::MsgPackEncodeError(e.to_string())
            }
            InternalMsgPackError::MsgpackDecodingError(e) => {
                AlgoKitMsgPackError::MsgPackDecodeError(e.to_string())
            }
            InternalMsgPackError::Base64DecodingError(e) => {
                AlgoKitMsgPackError::Base64DecodeError(e.to_string())
            }
            InternalMsgPackError::MsgpackWriteError(s) => AlgoKitMsgPackError::MsgPackWriteError(s),
            InternalMsgPackError::UnknownModelError(s) => AlgoKitMsgPackError::UnknownModelError(s),
            InternalMsgPackError::IoError(s) => AlgoKitMsgPackError::MsgPackWriteError(s),
            InternalMsgPackError::ValueWriteError(s) => AlgoKitMsgPackError::MsgPackEncodeError(s),
        }
    }
}

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum ModelType {
    SimulateRequest,
    SimulateTransaction200Response,
    Account,
}

impl From<ModelType> for InternalModelType {
    fn from(model_type: ModelType) -> Self {
        match model_type {
            ModelType::SimulateRequest => InternalModelType::SimulateRequest,
            ModelType::SimulateTransaction200Response => {
                InternalModelType::SimulateTransaction200Response
            }
            ModelType::Account => InternalModelType::Account,
        }
    }
}

impl From<InternalModelType> for ModelType {
    fn from(model_type: InternalModelType) -> Self {
        match model_type {
            InternalModelType::SimulateRequest => ModelType::SimulateRequest,
            InternalModelType::SimulateTransaction200Response => {
                ModelType::SimulateTransaction200Response
            }
            InternalModelType::Account => ModelType::Account,
        }
    }
}

#[ffi_func]
pub fn encode_json_to_msgpack(
    model_type: ModelType,
    json_str: &str,
) -> Result<Vec<u8>, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(internal_encode_json_to_msgpack(internal_type, json_str)?)
}

#[ffi_func]
pub fn decode_msgpack_to_json(
    model_type: ModelType,
    msgpack_bytes: &[u8],
) -> Result<String, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(internal_decode_msgpack_to_json(
        internal_type,
        msgpack_bytes,
    )?)
}

#[ffi_func]
pub fn encode_json_to_base64_msgpack(
    model_type: ModelType,
    json_str: &str,
) -> Result<String, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(internal_encode_json_to_base64_msgpack(
        internal_type,
        json_str,
    )?)
}

#[ffi_func]
pub fn decode_base64_msgpack_to_json(
    model_type: ModelType,
    base64_str: &str,
) -> Result<String, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(internal_decode_base64_msgpack_to_json(
        internal_type,
        base64_str,
    )?)
}

#[ffi_func]
pub fn supported_models() -> Vec<ModelType> {
    algokit_msgpack::supported_models()
        .into_iter()
        .map(Into::into)
        .collect()
}
