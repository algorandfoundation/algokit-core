use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rmp::encode::{self as rmp_encode, ValueWriteError};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

mod models;
pub use models::*;

#[derive(Debug, Error)]
pub enum AlgoKitMsgPackError {
    #[error("Error occurred during serialization: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error occurred during msgpack encoding: {0}")]
    MsgpackEncodingError(#[from] rmp_serde::encode::Error),
    #[error("Error occurred during msgpack decoding: {0}")]
    MsgpackDecodingError(#[from] rmp_serde::decode::Error),
    #[error("Error occurred during base64 decoding: {0}")]
    Base64DecodingError(#[from] base64::DecodeError),
    #[error("Error occurred during msgpack writing: {0}")]
    MsgpackWriteError(String),
    #[error("Unknown model type: {0}")]
    UnknownModelError(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Error occurred during value writing: {0}")]
    ValueWriteError(String),
}

impl From<std::io::Error> for AlgoKitMsgPackError {
    fn from(err: std::io::Error) -> Self {
        AlgoKitMsgPackError::IoError(err.to_string())
    }
}

impl From<ValueWriteError> for AlgoKitMsgPackError {
    fn from(err: ValueWriteError) -> Self {
        AlgoKitMsgPackError::ValueWriteError(format!("{:?}", err))
    }
}

pub type Result<T> = std::result::Result<T, AlgoKitMsgPackError>;

pub trait ToMsgPack: Serialize {
    fn to_msg_pack(&self) -> Result<Vec<u8>> {
        let json_value = serde_json::to_value(self)?;
        let processed_value = sort_and_filter_json(json_value)?;
        let mut buf = Vec::new();
        encode_value_to_msgpack(&processed_value, &mut buf)?;
        Ok(buf)
    }

    fn to_msg_pack_base64(&self) -> Result<String> {
        let bytes = self.to_msg_pack()?;
        Ok(BASE64.encode(&bytes))
    }
}

pub fn sort_and_filter_json(value: Value) -> Result<Value> {
    match value {
        Value::Object(map) => {
            let mut sorted_map = BTreeMap::new();
            for (key, val) in map {
                let processed_val = sort_and_filter_json(val)?;
                if !is_zero_value(&processed_val) {
                    sorted_map.insert(key, processed_val);
                }
            }
            Ok(Value::Object(serde_json::Map::from_iter(sorted_map)))
        }
        Value::Array(arr) => {
            let mut new_arr = Vec::with_capacity(arr.len());
            for item in arr {
                new_arr.push(sort_and_filter_json(item)?);
            }
            Ok(Value::Array(new_arr))
        }
        _ => Ok(value),
    }
}

fn is_zero_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(b) => !b,
        Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap() == 0
            } else if n.is_u64() {
                n.as_u64().unwrap() == 0
            } else if n.is_f64() {
                n.as_f64().unwrap() == 0.0
            } else {
                false
            }
        }
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(o) => o.is_empty(),
    }
}

pub(crate) fn encode_value_to_msgpack(value: &Value, buf: &mut Vec<u8>) -> Result<()> {
    match value {
        Value::Null => rmp_encode::write_nil(buf)?,
        Value::Bool(b) => rmp_encode::write_bool(buf, *b)?,
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                rmp_encode::write_i64(buf, i)?;
            } else if let Some(u) = n.as_u64() {
                rmp_encode::write_u64(buf, u)?;
            } else if let Some(f) = n.as_f64() {
                rmp_encode::write_f64(buf, f)?;
            }
        }
        Value::String(s) => rmp_encode::write_str(buf, s)?,
        Value::Array(arr) => {
            if arr.iter().all(
                |item| matches!(item, Value::Number(n) if n.is_u64() && n.as_u64().unwrap() <= 255),
            ) {
                let bin_data: Vec<u8> = arr
                    .iter()
                    .filter_map(|v| v.as_u64().map(|n| n as u8))
                    .collect();
                rmp_encode::write_bin(buf, &bin_data)?;
                return Ok(());
            }
            rmp_encode::write_array_len(buf, arr.len() as u32)?;
            for item in arr {
                encode_value_to_msgpack(item, buf)?;
            }
        }
        Value::Object(obj) => {
            rmp_encode::write_map_len(buf, obj.len() as u32)?;
            for (key, value) in obj {
                rmp_encode::write_str(buf, key)?;
                encode_value_to_msgpack(value, buf)?;
            }
        }
    }
    Ok(())
}

pub struct ModelRegistry {
    registry: HashMap<ModelType, Box<dyn ModelHandler>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, model_type: ModelType)
    where
        T: DeserializeOwned + Serialize + 'static,
    {
        self.registry
            .insert(model_type, Box::new(TypedModelHandler::<T>::new()));
    }

    pub fn encode_json_to_msgpack(&self, model_type: ModelType, json_str: &str) -> Result<Vec<u8>> {
        if let Some(handler) = self.registry.get(&model_type) {
            handler.encode_json_to_msgpack(json_str)
        } else {
            Err(AlgoKitMsgPackError::UnknownModelError(
                model_type.as_str().to_string(),
            ))
        }
    }

    pub fn decode_msgpack_to_json(
        &self,
        model_type: ModelType,
        msgpack_bytes: &[u8],
    ) -> Result<String> {
        if let Some(handler) = self.registry.get(&model_type) {
            handler.decode_msgpack_to_json(msgpack_bytes)
        } else {
            Err(AlgoKitMsgPackError::UnknownModelError(
                model_type.as_str().to_string(),
            ))
        }
    }

    pub fn encode_json_to_base64_msgpack(
        &self,
        model_type: ModelType,
        json_str: &str,
    ) -> Result<String> {
        let msgpack_bytes = self.encode_json_to_msgpack(model_type, json_str)?;
        Ok(BASE64.encode(&msgpack_bytes))
    }

    pub fn decode_base64_msgpack_to_json(
        &self,
        model_type: ModelType,
        base64_str: &str,
    ) -> Result<String> {
        let msgpack_bytes = BASE64.decode(base64_str)?;
        self.decode_msgpack_to_json(model_type, &msgpack_bytes)
    }

    pub fn list_models(&self) -> Vec<ModelType> {
        self.registry.keys().cloned().collect()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        models::register_all_models(&mut registry);
        registry
    }
}

trait ModelHandler {
    fn encode_json_to_msgpack(&self, json_str: &str) -> Result<Vec<u8>>;
    fn decode_msgpack_to_json(&self, msgpack_bytes: &[u8]) -> Result<String>;
}

struct TypedModelHandler<T>
where
    T: DeserializeOwned + Serialize,
{
    _phantom: std::marker::PhantomData<T>,
}

impl<T> TypedModelHandler<T>
where
    T: DeserializeOwned + Serialize,
{
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> ModelHandler for TypedModelHandler<T>
where
    T: DeserializeOwned + Serialize,
{
    fn encode_json_to_msgpack(&self, json_str: &str) -> Result<Vec<u8>> {
        let model: T = serde_json::from_str(json_str)?;
        Ok(rmp_serde::to_vec_named(&model)?)
    }

    fn decode_msgpack_to_json(&self, msgpack_bytes: &[u8]) -> Result<String> {
        let model: T = rmp_serde::from_slice(msgpack_bytes)?;
        Ok(serde_json::to_string(&model)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    SimulateRequest,
    SimulateTransaction200Response,
}

impl ModelType {
    /// Convert a ModelType to its string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelType::SimulateRequest => "SimulateRequest",
            ModelType::SimulateTransaction200Response => "SimulateTransaction200Response",
        }
    }

    /// Convert a string to a ModelType, returning None if the string doesn't match any model
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "SimulateRequest" => Some(ModelType::SimulateRequest),
            "SimulateTransaction200Response" => Some(ModelType::SimulateTransaction200Response),
            _ => None,
        }
    }

    /// Get all available model types
    pub fn all() -> Vec<ModelType> {
        vec![
            ModelType::SimulateRequest,
            ModelType::SimulateTransaction200Response,
        ]
    }
}

// Public API functions
pub fn encode_json_to_msgpack(model_type: ModelType, json_str: &str) -> Result<Vec<u8>> {
    let registry = ModelRegistry::default();
    registry.encode_json_to_msgpack(model_type, json_str)
}

pub fn decode_msgpack_to_json(model_type: ModelType, msgpack_bytes: &[u8]) -> Result<String> {
    let registry = ModelRegistry::default();
    registry.decode_msgpack_to_json(model_type, msgpack_bytes)
}

pub fn encode_json_to_base64_msgpack(model_type: ModelType, json_str: &str) -> Result<String> {
    let registry = ModelRegistry::default();
    registry.encode_json_to_base64_msgpack(model_type, json_str)
}

pub fn decode_base64_msgpack_to_json(model_type: ModelType, base64_str: &str) -> Result<String> {
    let registry = ModelRegistry::default();
    registry.decode_base64_msgpack_to_json(model_type, base64_str)
}

pub fn supported_models() -> Vec<ModelType> {
    let registry = ModelRegistry::default();
    registry.list_models()
}

// Allow users to use the standard `str::parse()` style API as well.
impl std::str::FromStr for ModelType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        ModelType::from_str(s).ok_or(())
    }
}

// Provide a blanket implementation so every `Serialize` type automatically
// gets the `ToMsgPack` methods.
impl<T> ToMsgPack for T where T: Serialize {}

// Implement `Display` for `ModelType` so it can be printed directly and used in
// other error types if needed.
impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sort_and_filter_json() {
        // Test object with unsorted keys and zero values
        let input = json!({
            "z": "z-value",
            "a": "a-value",
            "empty_string": "",
            "empty_array": [],
            "zero_number": 0,
            "non_zero": 42,
            "false_value": false,
            "true_value": true,
            "nested": {
                "y": "y-value",
                "x": "x-value",
                "empty_obj": {},
                "zero_num": 0
            }
        });

        let result = sort_and_filter_json(input).unwrap();

        // Expected: keys sorted alphabetically, zero values removed
        let expected = json!({
            "a": "a-value",
            "nested": {
                "x": "x-value",
                "y": "y-value"
            },
            "non_zero": 42,
            "true_value": true,
            "z": "z-value"
        });

        assert_eq!(result, expected);

        // Check JSON string representation to verify key order
        let result_str = serde_json::to_string(&result).unwrap();
        assert!(result_str.find("\"a\"").unwrap() < result_str.find("\"nested\"").unwrap());
        assert!(result_str.find("\"nested\"").unwrap() < result_str.find("\"non_zero\"").unwrap());
        assert!(
            result_str.find("\"non_zero\"").unwrap() < result_str.find("\"true_value\"").unwrap()
        );
        assert!(result_str.find("\"true_value\"").unwrap() < result_str.find("\"z\"").unwrap());

        // Check nested sorting
        let nested_obj = result.as_object().unwrap().get("nested").unwrap();
        let nested_str = serde_json::to_string(nested_obj).unwrap();
        assert!(nested_str.find("\"x\"").unwrap() < nested_str.find("\"y\"").unwrap());
    }
}
