#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

// Re-export for use in models
#[cfg(feature = "ffi_wasm")]
pub use wasm_bindgen;

pub mod models;
pub use models::*;

// ===========================
// Error Types
// ===========================

/// Comprehensive error types for serialization operations
#[derive(Error, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_wasm", serde(rename_all = "camelCase"))]
pub enum MsgpackError {
    #[error("Encoding not supported for this model")]
    EncodingNotSupported,

    #[error("Decoding not supported for this model")]
    DecodingNotSupported,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Base64 decoding error: {0}")]
    Base64Error(String),

    #[error("Invalid data structure: {0}")]
    InvalidStructure(String),
}

// Error conversions
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

// ===========================
// Core Traits
// ===========================

/// Trait for types that can be encoded to msgpack
pub trait MsgpackEncodable: Serialize {}

/// Trait for types that can be decoded from msgpack
pub trait MsgpackDecodable: for<'de> Deserialize<'de> {}

/// Trait for types that support JSON serialization with additional utilities
pub trait JsonSerializable: Serialize + for<'de> Deserialize<'de> {
    /// Convert to a dictionary representation
    fn to_dict(
        &self,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>, MsgpackError> {
        let json_value = serde_json::to_value(self)?;
        match json_value {
            serde_json::Value::Object(map) => Ok(map.into_iter().collect()),
            _ => Err(MsgpackError::InvalidStructure(
                "Model did not serialize to a JSON object".to_string(),
            )),
        }
    }

    /// Create from a dictionary representation
    fn from_dict(
        dict: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<Self, MsgpackError> {
        let json_value = serde_json::Value::Object(dict.into_iter().collect());
        serde_json::from_value(json_value).map_err(Into::into)
    }

    /// Convert to JSON string
    fn to_json(&self) -> Result<String, MsgpackError> {
        serde_json::to_string(self).map_err(Into::into)
    }

    /// Create from JSON string with preprocessing
    fn from_json(json_str: &str) -> Result<Self, MsgpackError> {
        let processed_json = json::preprocess_json_string(json_str);
        serde_json::from_str(&processed_json).map_err(Into::into)
    }
}

// ===========================
// Core Modules
// ===========================

/// JSON processing utilities
mod json {
    /// Preprocesses JSON strings to handle literal escape characters
    ///
    /// Converts literal `\n`, `\t`, `\r`, etc. to actual escape sequences
    pub fn preprocess_json_string(json_str: &str) -> String {
        json_str
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\b", "\u{0008}") // backspace
            .replace("\\f", "\u{000C}") // form feed
            .replace("\\\n", "\\n") // handle actual backslash-n sequences
            .replace("\\\t", "\\t") // handle actual backslash-t sequences
            .replace("\\\r", "\\r") // handle actual backslash-r sequences
    }
}

/// Msgpack encoding/decoding implementations
mod msgpack {
    use super::*;
    use base64::{engine::general_purpose, Engine as _};

    /// Sort msgpack values recursively for canonical encoding
    fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
        match value {
            rmpv::Value::Map(m) => {
                let sorted_map: BTreeMap<String, rmpv::Value> = m
                    .into_iter()
                    .filter_map(|(k, v)| {
                        if let rmpv::Value::String(key) = k {
                            key.as_str()
                                .map(|key_str| (key_str.to_string(), sort_msgpack_value(v)))
                        } else {
                            None
                        }
                    })
                    .collect();

                rmpv::Value::Map(
                    sorted_map
                        .into_iter()
                        .map(|(k, v)| (rmpv::Value::String(k.into()), v))
                        .collect(),
                )
            }
            rmpv::Value::Array(arr) => {
                rmpv::Value::Array(arr.into_iter().map(sort_msgpack_value).collect())
            }
            v => v,
        }
    }

    /// Encode a value to canonical msgpack following Algorand's rules
    pub fn encode_canonical<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
        // First encode to msgpack
        let mut temp_buf = Vec::new();
        let mut serializer = rmp_serde::Serializer::new(&mut temp_buf).with_struct_map();
        model.serialize(&mut serializer)?;

        // Parse and sort
        let msgpack_value =
            rmpv::decode::read_value(&mut std::io::Cursor::new(temp_buf)).map_err(|e| {
                MsgpackError::SerializationError(format!("Failed to parse msgpack: {}", e))
            })?;

        let sorted_value = sort_msgpack_value(msgpack_value);

        // Encode sorted value
        let mut final_buf = Vec::new();
        rmpv::encode::write_value(&mut final_buf, &sorted_value).map_err(|e| {
            MsgpackError::SerializationError(format!("Failed to encode msgpack: {}", e))
        })?;

        Ok(final_buf)
    }

    /// Specialized encoder for SimulateRequest
    pub mod specialized {
        use super::*;

        /// Custom encoder for SimulateRequest that handles base64-encoded transactions
        pub fn encode_simulate_request(model: &SimulateRequest) -> Result<Vec<u8>, MsgpackError> {
            let json_value = serde_json::to_value(model)?;

            match &json_value {
                serde_json::Value::Object(map) => {
                    let mut buf = Vec::new();
                    encode_object_canonical(&mut buf, map)?;
                    Ok(buf)
                }
                _ => Err(MsgpackError::InvalidStructure(
                    "Expected JSON object".to_string(),
                )),
            }
        }

        fn encode_object_canonical(
            buf: &mut Vec<u8>,
            map: &serde_json::Map<String, serde_json::Value>,
        ) -> Result<(), MsgpackError> {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            rmp::encode::write_map_len(buf, map.len() as u32)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

            for key in keys {
                let value = &map[key];
                rmp::encode::write_str(buf, key)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                match key.as_str() {
                    "txn-groups" => encode_txn_groups(buf, value)?,
                    _ => encode_value(buf, value)?,
                }
            }
            Ok(())
        }

        fn encode_txn_groups(
            buf: &mut Vec<u8>,
            value: &serde_json::Value,
        ) -> Result<(), MsgpackError> {
            if let serde_json::Value::Array(groups) = value {
                rmp::encode::write_array_len(buf, groups.len() as u32)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                for group in groups {
                    encode_txn_group(buf, group)?;
                }
            } else {
                encode_value(buf, value)?;
            }
            Ok(())
        }

        fn encode_txn_group(
            buf: &mut Vec<u8>,
            group: &serde_json::Value,
        ) -> Result<(), MsgpackError> {
            match group {
                serde_json::Value::Object(group_obj) => {
                    rmp::encode::write_map_len(buf, group_obj.len() as u32)
                        .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                    for (key, val) in group_obj {
                        rmp::encode::write_str(buf, key)
                            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                        if key == "txns" {
                            encode_transactions(buf, val)?;
                        } else {
                            encode_value(buf, val)?;
                        }
                    }
                }
                _ => encode_value(buf, group)?,
            }
            Ok(())
        }

        fn encode_transactions(
            buf: &mut Vec<u8>,
            value: &serde_json::Value,
        ) -> Result<(), MsgpackError> {
            if let serde_json::Value::Array(txns) = value {
                rmp::encode::write_array_len(buf, txns.len() as u32)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                for txn in txns {
                    match txn {
                        serde_json::Value::String(s) => {
                            let decoded = general_purpose::STANDARD
                                .decode(s)
                                .map_err(|e| MsgpackError::Base64Error(e.to_string()))?;
                            buf.extend_from_slice(&decoded);
                        }
                        _ => encode_value(buf, txn)?,
                    }
                }
            } else {
                encode_value(buf, value)?;
            }
            Ok(())
        }

        fn encode_value(buf: &mut Vec<u8>, value: &serde_json::Value) -> Result<(), MsgpackError> {
            use serde_json::Value;

            match value {
                Value::Null => rmp::encode::write_nil(buf)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?,
                Value::Bool(b) => rmp::encode::write_bool(buf, *b)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?,
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        rmp::encode::write_sint(buf, i)
                            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
                    } else if let Some(u) = n.as_u64() {
                        rmp::encode::write_uint(buf, u)
                            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
                    } else if let Some(f) = n.as_f64() {
                        rmp::encode::write_f64(buf, f)
                            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
                    } else {
                        return Err(MsgpackError::InvalidStructure("Invalid number".to_string()));
                    }
                }
                Value::String(s) => rmp::encode::write_str(buf, s)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?,
                Value::Array(arr) => {
                    rmp::encode::write_array_len(buf, arr.len() as u32)
                        .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
                    for item in arr {
                        encode_value(buf, item)?;
                    }
                }
                Value::Object(obj) => {
                    let mut keys: Vec<&String> = obj.keys().collect();
                    keys.sort();
                    rmp::encode::write_map_len(buf, obj.len() as u32)
                        .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
                    for key in keys {
                        rmp::encode::write_str(buf, key)
                            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
                        if let Some(val) = obj.get(key) {
                            encode_value(buf, val)?;
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

// ===========================
// Public API
// ===========================

/// Encode a value to msgpack using canonical encoding
pub fn encode_msgpack<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
    msgpack::encode_canonical(model)
}

/// Decode a value from msgpack
pub fn decode_msgpack<T: MsgpackDecodable>(data: &[u8]) -> Result<T, MsgpackError> {
    rmp_serde::from_slice(data).map_err(Into::into)
}

/// Serialize to JSON
pub fn serialize_to_json<T: JsonSerializable>(model: &T) -> Result<String, MsgpackError> {
    model.to_json()
}

/// Deserialize from JSON
pub fn deserialize_from_json<T: JsonSerializable>(json: &str) -> Result<T, MsgpackError> {
    T::from_json(json)
}

// ===========================
// WASM-specific utilities
// ===========================

#[cfg(feature = "ffi_wasm")]
mod wasm_utils {
    use super::*;

    /// Convert JSON string to JsValue
    pub fn json_string_to_js_value(json: &str) -> Result<wasm_bindgen::JsValue, MsgpackError> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        serde_wasm_bindgen::to_value(&json_value)
            .map_err(|e| MsgpackError::SerializationError(e.to_string()))
    }

    /// Convert JsValue to JSON string
    pub fn js_value_to_json_string(value: wasm_bindgen::JsValue) -> Result<String, MsgpackError> {
        let json_value: serde_json::Value = serde_wasm_bindgen::from_value(value)
            .map_err(|e| MsgpackError::DeserializationError(e.to_string()))?;
        serde_json::to_string(&json_value).map_err(Into::into)
    }
}

#[cfg(feature = "ffi_wasm")]
pub use wasm_utils::*;

// ===========================
// FFI Macros
// ===========================

/// Macro for msgpack encoding/decoding FFI functions
#[macro_export]
macro_rules! impl_msgpack_ffi {
    ($type:ty, $encode_fn:ident, $decode_fn:ident) => {
        /// Encode to msgpack
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
        pub fn $encode_fn(model: $type) -> Result<Vec<u8>, $crate::MsgpackError> {
            $crate::encode_msgpack(&model)
        }

        /// Decode from msgpack
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
        pub fn $decode_fn(data: Vec<u8>) -> Result<$type, $crate::MsgpackError> {
            $crate::decode_msgpack(&data)
        }
    };
}

/// Macro for JSON serialization/deserialization FFI functions
#[macro_export]
macro_rules! impl_json_ffi {
    ($type:ty, $to_json_fn:ident, $from_json_fn:ident) => {
        /// Convert to JSON
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        pub fn $to_json_fn(model: $type) -> Result<String, $crate::MsgpackError> {
            <$type as $crate::JsonSerializable>::to_json(&model)
        }

        /// Create from JSON
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        pub fn $from_json_fn(json: String) -> Result<$type, $crate::MsgpackError> {
            <$type as $crate::JsonSerializable>::from_json(&json)
        }
    };
}

/// Macro for WASM-specific JsValue conversions
#[cfg(feature = "ffi_wasm")]
#[macro_export]
macro_rules! impl_wasm_js_value {
    ($type:ty, $to_js_fn:ident, $from_js_fn:ident) => {
        ::paste::paste! {
            /// Convert to JsValue
            #[wasm_bindgen(js_name = [<$to_js_fn:camel>])]
            pub fn $to_js_fn(model: $type) -> Result<wasm_bindgen::JsValue, $crate::MsgpackError> {
                serde_wasm_bindgen::to_value(&model)
                    .map_err(|e| $crate::MsgpackError::SerializationError(e.to_string()))
            }

            /// Create from JsValue
            #[wasm_bindgen(js_name = [<$from_js_fn:camel>])]
            pub fn $from_js_fn(value: wasm_bindgen::JsValue) -> Result<$type, $crate::MsgpackError> {
                serde_wasm_bindgen::from_value(value)
                    .map_err(|e| $crate::MsgpackError::DeserializationError(e.to_string()))
            }
        }
    };
}

/// Combined macro for all JSON FFI functions
#[macro_export]
macro_rules! impl_all_json_ffi {
    ($type:ty, $base_name_snake:ident, $base_name_camel:ident) => {
        ::paste::paste! {
            // Python/UniFFI: snake_case
            $crate::impl_json_ffi!($type, [<$base_name_snake _to_json>], [<$base_name_snake _from_json>]);

            // WASM/TypeScript: camelCase
            #[cfg(feature = "ffi_wasm")]
            $crate::impl_wasm_js_value!($type, [<$base_name_camel ToJsValue>], [<$base_name_camel FromJsValue>]);
        }
    };
}

// ===========================
// FFI Exports Module
// ===========================

/// Module containing all FFI exports
pub mod ffi {
    use super::*;
    #[cfg(feature = "ffi_wasm")]
    use crate::impl_wasm_js_value;

    // Models supporting both encoding and decoding
    impl_msgpack_ffi!(Account, encode_account, decode_account);

    // Request models (encode only)

    /// Encode DryrunRequest to msgpack
    #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
    #[cfg(not(feature = "ffi_wasm"))]
    pub fn encode_dryrun_request(model: DryrunRequest) -> Result<Vec<u8>, MsgpackError> {
        encode_msgpack(&model)
    }

    /// Encode SimulateRequest using custom canonical encoding
    #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
    #[cfg(not(feature = "ffi_wasm"))]
    pub fn encode_simulate_request(model: SimulateRequest) -> Result<Vec<u8>, MsgpackError> {
        msgpack::specialized::encode_simulate_request(&model)
    }

    // Response models (decode only)

    /// Decode PendingTransactionResponse from msgpack
    #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
    #[cfg(not(feature = "ffi_wasm"))]
    pub fn decode_pending_transaction_response(
        data: Vec<u8>,
    ) -> Result<PendingTransactionResponse, MsgpackError> {
        decode_msgpack(&data)
    }

    /// Decode ErrorResponse from msgpack
    #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
    #[cfg(not(feature = "ffi_wasm"))]
    pub fn decode_error_response(data: Vec<u8>) -> Result<ErrorResponse, MsgpackError> {
        decode_msgpack(&data)
    }

    /// Decode LedgerStateDeltaForTransactionGroup from msgpack
    #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
    #[cfg(not(feature = "ffi_wasm"))]
    pub fn decode_ledger_state_delta_for_transaction_group(
        data: Vec<u8>,
    ) -> Result<LedgerStateDeltaForTransactionGroup, MsgpackError> {
        decode_msgpack(&data)
    }
}

// UniFFI scaffolding
#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

// ===========================
// Tests
// ===========================

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose, Engine as _};

    #[test]
    fn test_simulate_request_encoding() {
        let encoded_signed_txn = "gqNzaWfEQOeqkYx2i+fq1t7p5y7Epr3BRDZ7yfAcGY0B8QgdfmOWN2TYWkverOJYdf+h+2Xp/R7tQqPmjI/2vzWYbhZbSQajdHhuiaNhbXTOAA9CQKNmZWXOAA9CQKJmds0D6KNnZW6sdGVzdG5ldC12MS4womx2zQfQpG5vdGXED0hlbGxvIEFsZ29yYW5kIaNyY3bEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPo3NuZMQgi1khV+YlkqSD1HzaHjPDm1ubtJRN2ItSK/SE513BSc+kdHlwZaNwYXk=";

        let txns = vec![encoded_signed_txn.to_string()];
        let group = SimulateRequestTransactionGroup::new(txns);
        let exec_trace_config =
            SimulateTraceConfig::new(Some(true), Some(true), Some(true), Some(true));
        let simulate_request = SimulateRequest::new(
            vec![group],
            Some(1_000_000),
            Some(true),
            Some(true),
            Some(true),
            Some(1_000_000),
            Some(exec_trace_config),
            None,
        );

        let encoded = ffi::encode_simulate_request(simulate_request).unwrap();
        let base64 = general_purpose::STANDARD.encode(&encoded);

        let expected_base64 = "h7ZhbGxvdy1lbXB0eS1zaWduYXR1cmVzw7JhbGxvdy1tb3JlLWxvZ2dpbmfDt2FsbG93LXVubmFtZWQtcmVzb3VyY2Vzw7FleGVjLXRyYWNlLWNvbmZpZ4SmZW5hYmxlw65zY3JhdGNoLWNoYW5nZcOsc3RhY2stY2hhbmdlw6xzdGF0ZS1jaGFuZ2XDs2V4dHJhLW9wY29kZS1idWRnZXTOAA9CQKVyb3VuZM4AD0JAqnR4bi1ncm91cHORgaR0eG5zkYKjc2lnxEDnqpGMdovn6tbe6ecuxKa9wUQ2e8nwHBmNAfEIHX5jljdk2FpL3qziWHX/oftl6f0e7UKj5oyP9r81mG4WW0kGo3R4bomjYW10zgAPQkCjZmVlzgAPQkCiZnbNA+ijZ2VurHRlc3RuZXQtdjEuMKJsds0H0KRub3RlxA9IZWxsbyBBbGdvcmFuZCGjcmN2xCCLWSFX5iWSpIPUfNoeM8ObW5u0lE3Yi1Ir9ITnXcFJz6NzbmTEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPpHR5cGWjcGF5";

        assert_eq!(base64, expected_base64);
    }

    #[test]
    fn test_json_preprocessing() {
        let json_with_escapes = r#"{\n  "confirmed-round": 1096,\n  "pool-error": ""\n}"#;
        let processed = json::preprocess_json_string(json_with_escapes);

        // Should now be valid JSON
        assert!(serde_json::from_str::<serde_json::Value>(&processed).is_ok());
    }

    #[test]
    fn test_pending_transaction_response_json() {
        let json = r#"{
            "confirmed-round": 1096,
            "pool-error": "",
            "txn": "gqNzaWfEQOeqkYx2i+fq1t7p5y7Epr3BRDZ7yfAcGY0B8QgdfmOWN2TYWkverOJYdf+h+2Xp/R7tQqPmjI/2vzWYbhZbSQajdHhuiaNhbXTOAA9CQKNmZWXOAA9CQKJmds0D6KNnZW6sdGVzdG5ldC12MS4womx2zQfQpG5vdGXED0hlbGxvIEFsZ29yYW5kIaNyY3bEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPo3NuZMQgi1khV+YlkqSD1HzaHjPDm1ubtJRN2ItSK/SE513BSc+kdHlwZaNwYXk="
        }"#;

        let response = PendingTransactionResponse::from_json(json).unwrap();
        assert_eq!(response.confirmed_round, Some(1096));
        assert_eq!(response.pool_error, "");
    }
}
