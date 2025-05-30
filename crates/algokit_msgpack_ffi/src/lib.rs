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

pub trait MsgpackEncodable: Serialize {}
pub trait MsgpackDecodable: for<'de> Deserialize<'de> {}

pub trait JsonSerializable: Serialize + for<'de> Deserialize<'de> {
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

    fn from_dict(
        dict: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<Self, MsgpackError> {
        let json_value = serde_json::Value::Object(dict.into_iter().collect());
        let result = serde_json::from_value(json_value)?;
        Ok(result)
    }

    fn to_json(&self) -> Result<String, MsgpackError> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    fn from_json(json_str: &str) -> Result<Self, MsgpackError> {
        let processed_json = preprocess_json_string(json_str);
        serde_json::from_str(&processed_json).map_err(|e| e.into())
    }
}

/// Preprocesses JSON strings to handle literal escape characters
/// Converts literal \n, \t, \r, etc. to actual escape sequences
fn preprocess_json_string(json_str: &str) -> String {
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

fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
    match value {
        rmpv::Value::Map(m) => {
            let mut sorted_map: BTreeMap<String, rmpv::Value> = BTreeMap::new();

            for (k, v) in m {
                if let rmpv::Value::String(key) = k {
                    let key_str = key.into_str().unwrap_or_default();
                    sorted_map.insert(key_str, sort_msgpack_value(v));
                }
            }

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

/// Canonical msgpack encoding following Algorand's rules
pub fn encode_msgpack_canonical<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
    let mut temp_buf = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut temp_buf).with_struct_map();
    model.serialize(&mut serializer)?;

    let msgpack_value: rmpv::Value = rmpv::decode::read_value(&mut std::io::Cursor::new(temp_buf))
        .map_err(|e| {
            MsgpackError::SerializationError(format!("Failed to parse msgpack for sorting: {}", e))
        })?;

    let sorted_value = sort_msgpack_value(msgpack_value);

    let mut final_buf = Vec::new();
    rmpv::encode::write_value(&mut final_buf, &sorted_value).map_err(|e| {
        MsgpackError::SerializationError(format!("Failed to encode sorted msgpack: {}", e))
    })?;

    Ok(final_buf)
}

/// Custom msgpack encoding for SimulateRequest that handles base64-encoded signed transactions
pub fn encode_simulate_request_canonical(model: &SimulateRequest) -> Result<Vec<u8>, MsgpackError> {
    use base64::{engine::general_purpose, Engine as _};

    let json_str = serde_json::to_string(model)?;
    let json_value: serde_json::Value = serde_json::from_str(&json_str)?;

    let mut buf = Vec::new();
    match &json_value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            rmp::encode::write_map_len(&mut buf, map.len() as u32)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

            for key in keys {
                let value = map.get(key).expect("key just taken exists");
                rmp::encode::write_str(&mut buf, key)
                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                match key.as_str() {
                    "txn-groups" => encode_txn_groups(&mut buf, value)?,
                    _ => encode_value_to_msgpack(value, &mut buf)?,
                }
            }
        }
        _ => {
            return Err(MsgpackError::SerializationError(
                "Expected JSON object".to_string(),
            ))
        }
    }
    Ok(buf)
}

fn encode_txn_groups(buf: &mut Vec<u8>, value: &serde_json::Value) -> Result<(), MsgpackError> {
    use base64::{engine::general_purpose, Engine as _};

    if let serde_json::Value::Array(groups) = value {
        rmp::encode::write_array_len(buf, groups.len() as u32)
            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

        for group in groups {
            match group {
                serde_json::Value::Object(group_obj) => {
                    rmp::encode::write_map_len(buf, group_obj.len() as u32)
                        .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                    for (key, val) in group_obj {
                        rmp::encode::write_str(buf, key)
                            .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                        if key == "txns" {
                            if let serde_json::Value::Array(txns) = val {
                                rmp::encode::write_array_len(buf, txns.len() as u32)
                                    .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;

                                for txn in txns {
                                    match txn {
                                        serde_json::Value::String(s) => {
                                            let decoded_bytes = general_purpose::STANDARD
                                                .decode(s)
                                                .map_err(|e| {
                                                    MsgpackError::SerializationError(format!(
                                                        "Failed to decode base64 transaction: {}",
                                                        e
                                                    ))
                                                })?;
                                            buf.extend_from_slice(&decoded_bytes);
                                        }
                                        _ => encode_value_to_msgpack(txn, buf)?,
                                    }
                                }
                            } else {
                                encode_value_to_msgpack(val, buf)?;
                            }
                        } else {
                            encode_value_to_msgpack(val, buf)?;
                        }
                    }
                }
                _ => encode_value_to_msgpack(group, buf)?,
            }
        }
    } else {
        encode_value_to_msgpack(value, buf)?;
    }
    Ok(())
}

fn encode_value_to_msgpack(
    value: &serde_json::Value,
    buf: &mut Vec<u8>,
) -> Result<(), MsgpackError> {
    use serde_json::Value;

    match value {
        Value::Null => {
            rmp::encode::write_nil(buf)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
        }
        Value::Bool(b) => {
            rmp::encode::write_bool(buf, *b)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
        }
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
                return Err(MsgpackError::SerializationError(
                    "Invalid number".to_string(),
                ));
            }
        }
        Value::String(s) => {
            rmp::encode::write_str(buf, s)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
        }
        Value::Array(arr) => {
            rmp::encode::write_array_len(buf, arr.len() as u32)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
            for item in arr {
                encode_value_to_msgpack(item, buf)?;
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
                    encode_value_to_msgpack(val, buf)?;
                }
            }
        }
    }
    Ok(())
}

pub fn encode_msgpack<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
    encode_msgpack_canonical(model)
}

pub fn decode_msgpack<T: MsgpackDecodable>(data: &[u8]) -> Result<T, MsgpackError> {
    rmp_serde::from_slice(data).map_err(|e| e.into())
}

pub fn serialize_to_json<T: JsonSerializable>(model: &T) -> Result<String, MsgpackError> {
    model.to_json()
}

pub fn deserialize_from_json<T: JsonSerializable>(json: &str) -> Result<T, MsgpackError> {
    T::from_json(json)
}

#[cfg(feature = "ffi_wasm")]
pub fn json_string_to_js_value(json: &str) -> Result<wasm_bindgen::JsValue, MsgpackError> {
    let json_value: serde_json::Value = serde_json::from_str(json)?;
    serde_wasm_bindgen::to_value(&json_value)
        .map_err(|e| MsgpackError::SerializationError(e.to_string()))
}

#[cfg(feature = "ffi_wasm")]
pub fn js_value_to_json_string(value: wasm_bindgen::JsValue) -> Result<String, MsgpackError> {
    let json_value: serde_json::Value = serde_wasm_bindgen::from_value(value)
        .map_err(|e| MsgpackError::DeserializationError(e.to_string()))?;
    serde_json::to_string(&json_value).map_err(|e| e.into())
}

// Macro for msgpack encoding/decoding functions (for models that support msgpack)
#[macro_export]
macro_rules! impl_msgpack_ffi {
    ($type:ty, $encode_fn:ident, $decode_fn:ident) => {
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
        pub fn $encode_fn(model: $type) -> Result<Vec<u8>, $crate::MsgpackError> {
            $crate::encode_msgpack(&model)
        }

        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
        pub fn $decode_fn(data: Vec<u8>) -> Result<$type, $crate::MsgpackError> {
            $crate::decode_msgpack(&data)
        }
    };
}

// Macro for JSON serialization/deserialization functions (uniffi only, wasm has direct support)
#[macro_export]
macro_rules! impl_json_ffi {
    ($type:ty, $to_json_fn:ident, $from_json_fn:ident) => {
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        pub fn $to_json_fn(model: $type) -> Result<String, $crate::MsgpackError> {
            <$type as $crate::JsonSerializable>::to_json(&model)
        }

        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        pub fn $from_json_fn(json: String) -> Result<$type, $crate::MsgpackError> {
            <$type as $crate::JsonSerializable>::from_json(&json)
        }
    };
}

// Macro for WASM-specific JsValue conversions
#[cfg(feature = "ffi_wasm")]
#[macro_export]
macro_rules! impl_wasm_js_value {
    ($type:ty, $to_js_fn:ident, $from_js_fn:ident) => {
        #[wasm_bindgen(js_name = $to_js_fn)]
        pub fn $to_js_fn(model: $type) -> Result<wasm_bindgen::JsValue, $crate::MsgpackError> {
            serde_wasm_bindgen::to_value(&model)
                .map_err(|e| $crate::MsgpackError::SerializationError(e.to_string()))
        }

        #[wasm_bindgen(js_name = $from_js_fn)]
        pub fn $from_js_fn(value: wasm_bindgen::JsValue) -> Result<$type, $crate::MsgpackError> {
            serde_wasm_bindgen::from_value(value)
                .map_err(|e| $crate::MsgpackError::DeserializationError(e.to_string()))
        }
    };
}

// Macro that applies both JSON and WASM conversions to a model type
#[macro_export]
macro_rules! impl_all_json_ffi {
    ($type:ty, $base_name:ident) => {
        ::paste::paste! {
            $crate::impl_json_ffi!($type, [<$base_name _to_json>], [<$base_name _from_json>]);

            #[cfg(feature = "ffi_wasm")]
            $crate::impl_wasm_js_value!($type, [<$base_name ToJsValue>], [<$base_name FromJsValue>]);
        }
    };
}

// Auto-implementation for template-generated JSON functions
#[macro_export]
macro_rules! auto_impl_json_ffi {
    ($type:ty, $base_name:ident) => {
        $crate::impl_all_json_ffi!($type, $base_name);
    };
}

// =============================
// MANUAL MSGPACK IMPLEMENTATIONS
// =============================
// Only models that specifically need msgpack support are implemented here.
// All models get JSON support automatically via the template.

// Models that support both encoding and decoding
impl_msgpack_ffi!(Account, encode_account, decode_account);

// Models that only support encoding (request models)
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn encode_dryrun_request(model: DryrunRequest) -> Result<Vec<u8>, MsgpackError> {
    encode_msgpack(&model)
}

// Special case: SimulateRequest with custom canonical encoding
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn encode_simulate_request(model: SimulateRequest) -> Result<Vec<u8>, MsgpackError> {
    encode_simulate_request_canonical(&model)
}

// Models that only support decoding (response models)
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_pending_transaction_response(
    data: Vec<u8>,
) -> Result<PendingTransactionResponse, MsgpackError> {
    decode_msgpack(&data)
}

#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_error_response(data: Vec<u8>) -> Result<ErrorResponse, MsgpackError> {
    decode_msgpack(&data)
}

#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
#[cfg_attr(feature = "ffi_wasm", wasm_bindgen)]
pub fn decode_ledger_state_delta_for_transaction_group(
    data: Vec<u8>,
) -> Result<LedgerStateDeltaForTransactionGroup, MsgpackError> {
    decode_msgpack(&data)
}

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

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
            Option::None,
        );

        let encoded = encode_simulate_request(simulate_request).unwrap();
        let base64 = general_purpose::STANDARD.encode(&encoded);
        let expected_base64 = "h7ZhbGxvdy1lbXB0eS1zaWduYXR1cmVzw7JhbGxvdy1tb3JlLWxvZ2dpbmfDt2FsbG93LXVubmFtZWQtcmVzb3VyY2Vzw7FleGVjLXRyYWNlLWNvbmZpZ4SmZW5hYmxlw65zY3JhdGNoLWNoYW5nZcOsc3RhY2stY2hhbmdlw6xzdGF0ZS1jaGFuZ2XDs2V4dHJhLW9wY29kZS1idWRnZXTOAA9CQKVyb3VuZM4AD0JAqnR4bi1ncm91cHORgaR0eG5zkYKjc2lnxEDnqpGMdovn6tbe6ecuxKa9wUQ2e8nwHBmNAfEIHX5jljdk2FpL3qziWHX/oftl6f0e7UKj5oyP9r81mG4WW0kGo3R4bomjYW10zgAPQkCjZmVlzgAPQkCiZnbNA+ijZ2VurHRlc3RuZXQtdjEuMKJsds0H0KRub3RlxA9IZWxsbyBBbGdvcmFuZCGjcmN2xCCLWSFX5iWSpIPUfNoeM8ObW5u0lE3Yi1Ir9ITnXcFJz6NzbmTEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPpHR5cGWjcGF5";
        assert_eq!(base64, expected_base64);
    }

    #[test]
    fn test_pending_transaction_response_json_decoding() {
        let json = r#"{
  "confirmed-round": 1096,
  "pool-error": "",
  "txn": "gqNzaWfEQOeqkYx2i+fq1t7p5y7Epr3BRDZ7yfAcGY0B8QgdfmOWN2TYWkverOJYdf+h+2Xp/R7tQqPmjI/2vzWYbhZbSQajdHhuiaNhbXTOAA9CQKNmZWXOAA9CQKJmds0D6KNnZW6sdGVzdG5ldC12MS4womx2zQfQpG5vdGXED0hlbGxvIEFsZ29yYW5kIaNyY3bEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPo3NuZMQgi1khV+YlkqSD1HzaHjPDm1ubtJRN2ItSK/SE513BSc+kdHlwZaNwYXk="
}"#;
        let pending_transaction_response = PendingTransactionResponse::from_json(json).unwrap();
        assert_eq!(pending_transaction_response.confirmed_round, Some(1096));
    }

    #[test]
    fn test_json_preprocessing_with_literal_escapes() {
        // Test with original problematic format containing literal \n characters
        let json_with_literal_escapes = r#"{\n  "confirmed-round": 1096,\n  "pool-error": "",\n  "txn": "gqNzaWfEQOeqkYx2i+fq1t7p5y7Epr3BRDZ7yfAcGY0B8QgdfmOWN2TYWkverOJYdf+h+2Xp/R7tQqPmjI/2vzWYbhZbSQajdHhuiaNhbXTOAA9CQKNmZWXOAA9CQKJmds0D6KNnZW6sdGVzdG5ldC12MS4womx2zQfQpG5vdGXED0hlbGxvIEFsZ29yYW5kIaNyY3bEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPo3NuZMQgi1khV+YlkqSD1HzaHjPDm1ubtJRN2ItSK/SE513BSc+kdHlwZaNwYXk="\n}"#;

        // This should now work with our preprocessing
        let result = PendingTransactionResponse::from_json(json_with_literal_escapes);
        assert!(result.is_ok());
        let pending_transaction_response = result.unwrap();
        assert_eq!(pending_transaction_response.confirmed_round, Some(1096));
        assert_eq!(pending_transaction_response.pool_error, "");
    }

    #[test]
    fn test_json_preprocessing_doesnt_break_valid_json() {
        // Test that normal, valid JSON still works
        let valid_json = r#"{
  "confirmed-round": 1096,
  "pool-error": "test error message",
  "txn": "gqNzaWfEQOeqkYx2i+fq1t7p5y7Epr3BRDZ7yfAcGY0B8QgdfmOWN2TYWkverOJYdf+h+2Xp/R7tQqPmjI/2vzWYbhZbSQajdHhuiaNhbXTOAA9CQKNmZWXOAA9CQKJmds0D6KNnZW6sdGVzdG5ldC12MS4womx2zQfQpG5vdGXED0hlbGxvIEFsZ29yYW5kIaNyY3bEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPo3NuZMQgi1khV+YlkqSD1HzaHjPDm1ubtJRN2ItSK/SE513BSc+kdHlwZaNwYXk="
}"#;

        let result = PendingTransactionResponse::from_json(valid_json);
        assert!(result.is_ok());
        let pending_transaction_response = result.unwrap();
        assert_eq!(pending_transaction_response.confirmed_round, Some(1096));
        assert_eq!(
            pending_transaction_response.pool_error,
            "test error message"
        );
    }
}
