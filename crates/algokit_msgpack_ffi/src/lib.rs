use algokit_msgpack::{
    decode_base64_msgpack_to_json, decode_msgpack_to_json, encode_json_to_base64_msgpack,
    encode_json_to_msgpack, ModelType as InternalModelType, MsgPackError,
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

// Convert internal errors to FFI errors
impl From<MsgPackError> for AlgoKitMsgPackError {
    fn from(e: MsgPackError) -> Self {
        match e {
            MsgPackError::SerializationError(e) => {
                AlgoKitMsgPackError::SerializationError(e.to_string())
            }
            MsgPackError::MsgPackEncodeError(e) => {
                AlgoKitMsgPackError::MsgPackEncodeError(e.to_string())
            }
            MsgPackError::MsgPackDecodeError(e) => {
                AlgoKitMsgPackError::MsgPackDecodeError(e.to_string())
            }
            MsgPackError::Base64DecodeError(e) => {
                AlgoKitMsgPackError::Base64DecodeError(e.to_string())
            }
            MsgPackError::MsgPackWriteError(s) => AlgoKitMsgPackError::MsgPackWriteError(s),
            MsgPackError::UnknownModelError(s) => AlgoKitMsgPackError::UnknownModelError(s),
            MsgPackError::IoError(s) => AlgoKitMsgPackError::MsgPackWriteError(s),
            MsgPackError::ValueWriteError(s) => AlgoKitMsgPackError::MsgPackEncodeError(s),
        }
    }
}

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

// This enum will be exposed to foreign language bindings
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum ModelType {
    SimulateRequest,
    SimulateRequestTransactionGroup,
    SimulateTraceConfig,
    SimulateTransaction200Response,
    SimulateUnnamedResourcesAccessed,
    SimulationTransactionExecTrace,
    SimulationEvalOverrides,
    SimulateInitialStates,
    SimulateTransactionResult,
    SimulateTransactionGroupResult,
}

// Convert between the FFI enum and the internal enum
impl From<ModelType> for InternalModelType {
    fn from(model_type: ModelType) -> Self {
        match model_type {
            ModelType::SimulateRequest => InternalModelType::SimulateRequest,
            ModelType::SimulateRequestTransactionGroup => {
                InternalModelType::SimulateRequestTransactionGroup
            }
            ModelType::SimulateTraceConfig => InternalModelType::SimulateTraceConfig,
            ModelType::SimulateTransaction200Response => {
                InternalModelType::SimulateTransaction200Response
            }
            ModelType::SimulateUnnamedResourcesAccessed => {
                InternalModelType::SimulateUnnamedResourcesAccessed
            }
            ModelType::SimulationTransactionExecTrace => {
                InternalModelType::SimulationTransactionExecTrace
            }
            ModelType::SimulationEvalOverrides => InternalModelType::SimulationEvalOverrides,
            ModelType::SimulateInitialStates => InternalModelType::SimulateInitialStates,
            ModelType::SimulateTransactionResult => InternalModelType::SimulateTransactionResult,
            ModelType::SimulateTransactionGroupResult => {
                InternalModelType::SimulateTransactionGroupResult
            }
        }
    }
}

impl From<InternalModelType> for ModelType {
    fn from(model_type: InternalModelType) -> Self {
        match model_type {
            InternalModelType::SimulateRequest => ModelType::SimulateRequest,
            InternalModelType::SimulateRequestTransactionGroup => {
                ModelType::SimulateRequestTransactionGroup
            }
            InternalModelType::SimulateTraceConfig => ModelType::SimulateTraceConfig,
            InternalModelType::SimulateTransaction200Response => {
                ModelType::SimulateTransaction200Response
            }
            InternalModelType::SimulateUnnamedResourcesAccessed => {
                ModelType::SimulateUnnamedResourcesAccessed
            }
            InternalModelType::SimulationTransactionExecTrace => {
                ModelType::SimulationTransactionExecTrace
            }
            InternalModelType::SimulationEvalOverrides => ModelType::SimulationEvalOverrides,
            InternalModelType::SimulateInitialStates => ModelType::SimulateInitialStates,
            InternalModelType::SimulateTransactionResult => ModelType::SimulateTransactionResult,
            InternalModelType::SimulateTransactionGroupResult => {
                ModelType::SimulateTransactionGroupResult
            }
        }
    }
}

/// Encode a JSON string to MessagePack format based on the model type
#[ffi_func]
pub fn encode_json_to_msgpack_ffi(
    model_type: ModelType,
    json_str: &str,
) -> Result<Vec<u8>, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(encode_json_to_msgpack(internal_type, json_str)?)
}

/// Decode MessagePack bytes to a JSON string based on the model type
#[ffi_func]
pub fn decode_msgpack_to_json_ffi(
    model_type: ModelType,
    msgpack_bytes: &[u8],
) -> Result<String, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(decode_msgpack_to_json(internal_type, msgpack_bytes)?)
}

/// Encode a JSON string to base64-encoded MessagePack based on the model type
#[ffi_func]
pub fn encode_json_to_base64_msgpack_ffi(
    model_type: ModelType,
    json_str: &str,
) -> Result<String, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(encode_json_to_base64_msgpack(internal_type, json_str)?)
}

/// Decode base64-encoded MessagePack to a JSON string based on the model type
#[ffi_func]
pub fn decode_base64_msgpack_to_json_ffi(
    model_type: ModelType,
    base64_str: &str,
) -> Result<String, AlgoKitMsgPackError> {
    let internal_type: InternalModelType = model_type.into();
    Ok(decode_base64_msgpack_to_json(internal_type, base64_str)?)
}

/// List all supported model types
#[ffi_func]
pub fn list_supported_models_ffi() -> Vec<ModelType> {
    algokit_msgpack::list_supported_models()
        .into_iter()
        .map(Into::into)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simulate_request() {
        // Sample SimulateRequest with base64 encoded transactions
        let simulate_request_json = r#"{"txn-groups": [{"txns": ["gqNzaWfEQGpkwpZwdiKgrD6dltReq30ENJoNoAAPSDPCs0TuI98NNi+Eu0nOmNu/v3lUXN+inep37jniiXciQB5s05kAUQWjdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnYlo2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQNo3JjdsQgx1kn38qirfWaWFS6DiVqMAY76Jm/s3hYGnKHxvDG1lejc25kxCDHWSffyqKt9ZpYVLoOJWowBjvomb+zeFgacofG8MbWV6R0eXBlo3BheQ=="]}], "allow-empty-signatures": true, "allow-more-logging": true, "allow-unnamed-resources": true, "exec-trace-config": {"enable": true, "stack-change": true, "scratch-change": true, "state-change": true}}"#;

        // Encode to MessagePack
        let msgpack_bytes =
            encode_json_to_msgpack_ffi(ModelType::SimulateRequest, simulate_request_json)
                .expect("Failed to encode");

        // Verify the encoding is compact and efficient

        // 1. Check the first byte is a map marker
        assert!(
            msgpack_bytes[0] >= 0x80 && msgpack_bytes[0] <= 0x8f,
            "First byte should indicate a fixmap"
        );

        // 2. Verify no excessive type markers (0xCC)
        let cc_count = msgpack_bytes.iter().filter(|&&b| b == 0xCC).count();
        let cc_ratio = cc_count as f64 / msgpack_bytes.len() as f64;

        assert!(
            cc_ratio < 0.1,
            "Too many 0xCC type markers ({}%), suggesting verbose encoding",
            cc_ratio * 100.0
        );

        // 3. Verify encoded size is reasonable for the input
        let encoded_size = msgpack_bytes.len() as f64;
        let json_size = simulate_request_json.len() as f64;

        assert!(
            encoded_size / json_size < 1.5, // Expected to be less than 1.5x the JSON size
            "Encoded MessagePack is larger than expected: {} bytes vs {} JSON bytes",
            msgpack_bytes.len(),
            simulate_request_json.len()
        );
    }
}
