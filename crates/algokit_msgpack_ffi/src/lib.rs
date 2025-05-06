use ffi_macros::{ffi_func, ffi_record};
use serde_bytes::ByteBuf;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum MsgPackError {
    #[error("EncodingError: {0}")]
    EncodingError(String),
    #[error("DecodingError: {0}")]
    DecodingError(String),
    #[error("JsonConversionError: {0}")]
    JsonConversionError(String),
}

// Convert errors from the Rust crate into the FFI-specific errors
impl From<algokit_msgpack::MsgPackError> for MsgPackError {
    fn from(e: algokit_msgpack::MsgPackError) -> Self {
        match e {
            algokit_msgpack::MsgPackError::EncodingError(msg) => MsgPackError::EncodingError(msg),
            algokit_msgpack::MsgPackError::DecodingError(msg) => MsgPackError::DecodingError(msg),
            algokit_msgpack::MsgPackError::JsonConversionError(msg) => {
                MsgPackError::JsonConversionError(msg)
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

/// Convert JSON string to MessagePack bytes
#[ffi_func]
pub fn json_to_msgpack(json_str: &str) -> Result<Vec<u8>, MsgPackError> {
    algokit_msgpack::MsgPack::json_to_msgpack(json_str).map_err(|e| e.into())
}

/// Convert MessagePack bytes to JSON string
#[ffi_func]
pub fn msgpack_to_json(bytes: &[u8]) -> Result<String, MsgPackError> {
    algokit_msgpack::MsgPack::msgpack_to_json(bytes).map_err(|e| e.into())
}

/// Check if a content type is for MessagePack
#[ffi_func]
pub fn is_msgpack_content_type(content_type: &str) -> bool {
    algokit_msgpack::MsgPack::is_msgpack_content_type(content_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_msgpack_and_back() {
        let json_str = r#"{"name":"Test","age":30,"data":[1,2,3,4]}"#;

        let msgpack = json_to_msgpack(json_str).unwrap();
        let json = msgpack_to_json(&msgpack).unwrap();

        // Parse both as Value to handle formatting differences
        let original_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let converted_value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(original_value, converted_value);
    }

    #[test]
    fn test_is_msgpack_content_type() {
        assert!(is_msgpack_content_type("application/msgpack"));
        assert!(is_msgpack_content_type("application/x-msgpack"));
        assert!(!is_msgpack_content_type("application/json"));
    }
}
