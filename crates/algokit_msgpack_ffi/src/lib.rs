#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

pub mod models;

// Re-export all models
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

/// Sorts a msgpack value recursively to ensure lexicographic field ordering
fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
    match value {
        rmpv::Value::Map(m) => {
            let mut sorted_map: BTreeMap<String, rmpv::Value> = BTreeMap::new();

            // Convert and sort all key-value pairs
            for (k, v) in m {
                if let rmpv::Value::String(key) = k {
                    let key_str = key.into_str().unwrap_or_default();
                    sorted_map.insert(key_str, sort_msgpack_value(v));
                }
            }

            // Convert back to rmpv::Value::Map
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
        // For all other types, return as-is
        v => v,
    }
}

/// Canonical msgpack encoding following Algorand's rules:
/// 1. Every integer must be encoded to the smallest type possible (0-255->8bit, 256-65535->16bit, etc)
/// 2. All field names must be sorted alphabetically
/// 3. All empty and 0 fields should be omitted (handled by serde skip_serializing_if)
/// 4. Every positive number must be encoded as uint (handled by rmp-serde)
/// 5. Binary blob should be used for binary data and string for strings (handled by rmp-serde)
pub fn encode_msgpack_canonical<T: MsgpackEncodable>(model: &T) -> Result<Vec<u8>, MsgpackError> {
    // Step 1: Serialize to msgpack using struct_map mode
    let mut temp_buf = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut temp_buf).with_struct_map();
    model.serialize(&mut serializer)?;

    // Step 2: Deserialize to msgpack Value for sorting
    let msgpack_value: rmpv::Value = rmpv::decode::read_value(&mut std::io::Cursor::new(temp_buf))
        .map_err(|e| {
            MsgpackError::SerializationError(format!("Failed to parse msgpack for sorting: {}", e))
        })?;

    // Step 3: Sort the msgpack value to ensure lexicographic field ordering
    let sorted_value = sort_msgpack_value(msgpack_value);

    // Step 4: Re-encode the sorted value to final msgpack bytes
    let mut final_buf = Vec::new();
    rmpv::encode::write_value(&mut final_buf, &sorted_value).map_err(|e| {
        MsgpackError::SerializationError(format!("Failed to encode sorted msgpack: {}", e))
    })?;

    Ok(final_buf)
}

/// Custom msgpack encoding for SimulateRequest that handles base64-encoded signed transactions
pub fn encode_simulate_request_canonical(model: &SimulateRequest) -> Result<Vec<u8>, MsgpackError> {
    use base64::{engine::general_purpose, Engine as _};

    // Convert to JSON first to leverage existing serialization, then encode manually
    let json_str = serde_json::to_string(model)?;
    let json_value: serde_json::Value = serde_json::from_str(&json_str)?;

    let mut buf = Vec::new();
    match &json_value {
        serde_json::Value::Object(map) => {
            // Sort keys for consistent output
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

/// Helper function to encode transaction groups with proper handling of base64 msgpack
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
                                            // Decode base64 and write raw msgpack bytes directly
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

/// Helper function to encode JSON values to msgpack
fn encode_value_to_msgpack(
    value: &serde_json::Value,
    buf: &mut Vec<u8>,
) -> Result<(), MsgpackError> {
    use serde_json::Value;

    match value {
        Value::Null => {
            rmp::encode::write_nil(buf)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
            Ok(())
        }
        Value::Bool(b) => {
            rmp::encode::write_bool(buf, *b)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
            Ok(())
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
            Ok(())
        }
        Value::String(s) => {
            rmp::encode::write_str(buf, s)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
            Ok(())
        }
        Value::Array(arr) => {
            rmp::encode::write_array_len(buf, arr.len() as u32)
                .map_err(|e| MsgpackError::SerializationError(e.to_string()))?;
            for item in arr {
                encode_value_to_msgpack(item, buf)?;
            }
            Ok(())
        }
        Value::Object(obj) => {
            // Sort keys for canonical encoding
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
            Ok(())
        }
    }
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
// GENERIC JSON CONVERSION UTILITIES
// =============================================================================

/// Serialize any model implementing JsonSerializable to JSON string
/// This works across both WASM and UniFFI boundaries
pub fn serialize_to_json<T: JsonSerializable>(model: &T) -> Result<String, MsgpackError> {
    model.to_json()
}

/// Deserialize JSON string to any model implementing JsonSerializable
/// This works across both WASM and UniFFI boundaries
pub fn deserialize_from_json<T: JsonSerializable>(json: &str) -> Result<T, MsgpackError> {
    T::from_json(json)
}

/// Convert JSON string to JsValue for WASM (TypeScript side can use JSON.parse)
#[cfg(feature = "ffi_wasm")]
pub fn json_string_to_js_value(json: &str) -> Result<wasm_bindgen::JsValue, MsgpackError> {
    let json_value: serde_json::Value = serde_json::from_str(json)?;
    serde_wasm_bindgen::to_value(&json_value)
        .map_err(|e| MsgpackError::SerializationError(e.to_string()))
}

/// Convert JsValue to JSON string for WASM (TypeScript side can use JSON.stringify)
#[cfg(feature = "ffi_wasm")]
pub fn js_value_to_json_string(value: wasm_bindgen::JsValue) -> Result<String, MsgpackError> {
    let json_value: serde_json::Value = serde_wasm_bindgen::from_value(value)
        .map_err(|e| MsgpackError::DeserializationError(e.to_string()))?;
    serde_json::to_string(&json_value).map_err(|e| e.into())
}

/// Runtime JSON conversion helpers - work with serde_json::Value for maximum flexibility
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn parse_json_string(json: String) -> Result<String, MsgpackError> {
    // Validates JSON and returns pretty-formatted version
    let value: serde_json::Value = serde_json::from_str(&json)?;
    serde_json::to_string_pretty(&value).map_err(|e| e.into())
}

/// Convert between different JSON formats - useful for data transformation
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn minify_json(json: String) -> Result<String, MsgpackError> {
    let value: serde_json::Value = serde_json::from_str(&json)?;
    serde_json::to_string(&value).map_err(|e| e.into())
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
    encode_simulate_request_canonical(&model)
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

// =============================================================================
// JSON CONVERSION FUNCTIONS - Generated using macros for key models
// =============================================================================

// Generate JSON conversion functions for key models
json_model_functions!(Account, "account");
json_model_functions!(SimulateRequest, "simulateRequest");
json_model_functions!(SimulateTransaction200Response, "simulateResponse");
json_model_functions!(DryrunRequest, "dryrunRequest");
json_model_functions!(ErrorResponse, "errorResponse");
json_model_functions!(PendingTransactionResponse, "pendingTransaction");

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

// Test SimulateRequest encoding
#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose, Engine as _};

    #[test]
    fn test_simulate_request_encoding() {
        // Example: Properly encoded base64 strings of already msgpack-encoded signed transactions
        // In real usage, these would come from the client with transactions already encoded
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

        println!("Properly encoded SimulateRequest: {}", base64);
    }

    #[test]
    fn test_simulate_response_decoding() {}
}

// =============================================================================
// JSON HELPERS - Macros and utilities for easy JSON conversion
// =============================================================================

/// Macro to generate JSON conversion functions for a specific model
/// Usage: json_model_functions!(Account, "account");
#[macro_export]
macro_rules! json_model_functions {
    ($model:ty, $name:literal) => {
        paste::paste! {
            #[doc = "Convert " $name " to JSON string"]
            #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
            pub fn [<$name _to_json>](model: $model) -> Result<String, MsgpackError> {
                model.to_json()
            }

            #[doc = "Convert JSON string to " $name]
            #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
            pub fn [<$name _from_json>](json: String) -> Result<$model, MsgpackError> {
                <$model>::from_json(&json)
            }

            #[cfg(feature = "ffi_wasm")]
            #[doc = "Convert " $name " to JsValue (preserves types for TypeScript)"]
            #[wasm_bindgen(js_name = [<$name ToJsValue>])]
            pub fn [<$name _to_js_value>](model: $model) -> Result<wasm_bindgen::JsValue, MsgpackError> {
                serde_wasm_bindgen::to_value(&model).map_err(|e| MsgpackError::SerializationError(e.to_string()))
            }

            #[cfg(feature = "ffi_wasm")]
            #[doc = "Convert JsValue to " $name " (from TypeScript object)"]
            #[wasm_bindgen(js_name = [<$name FromJsValue>])]
            pub fn [<$name _from_js_value>](value: wasm_bindgen::JsValue) -> Result<$model, MsgpackError> {
                serde_wasm_bindgen::from_value(value).map_err(|e| MsgpackError::DeserializationError(e.to_string()))
            }
        }
    };
}
