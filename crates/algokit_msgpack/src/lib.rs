use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// Error types for MessagePack encoding/decoding
#[derive(Debug, Error)]
pub enum MsgPackError {
    /// Error during encoding
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Error during decoding
    #[error("Decoding error: {0}")]
    DecodingError(String),

    /// Error during JSON conversion
    #[error("JSON conversion error: {0}")]
    JsonConversionError(String),
}

/// Implements MessagePack encoding and decoding functions
pub struct MsgPack;

impl MsgPack {
    /// Encode a Serialize object to MessagePack bytes
    pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, MsgPackError> {
        rmp_serde::to_vec(value).map_err(|e| MsgPackError::EncodingError(e.to_string()))
    }

    /// Decode MessagePack bytes to a specified type
    pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, MsgPackError> {
        rmp_serde::from_slice(bytes).map_err(|e| MsgPackError::DecodingError(e.to_string()))
    }

    /// Convert JSON string to MessagePack bytes
    pub fn json_to_msgpack(json_str: &str) -> Result<Vec<u8>, MsgPackError> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| MsgPackError::JsonConversionError(e.to_string()))?;

        Self::encode(&value)
    }

    /// Convert MessagePack bytes to JSON string
    pub fn msgpack_to_json(bytes: &[u8]) -> Result<String, MsgPackError> {
        let value: serde_json::Value = Self::decode(bytes)?;

        serde_json::to_string(&value).map_err(|e| MsgPackError::JsonConversionError(e.to_string()))
    }

    /// Check if the content type is MessagePack
    pub fn is_msgpack_content_type(content_type: &str) -> bool {
        content_type.contains("application/msgpack")
            || content_type.contains("application/x-msgpack")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_bytes::ByteBuf;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        name: String,
        age: u32,
        data: ByteBuf,
    }

    #[test]
    fn test_encode_decode() {
        let test_struct = TestStruct {
            name: "Test".to_string(),
            age: 30,
            data: ByteBuf::from(vec![1, 2, 3, 4]),
        };

        let encoded = MsgPack::encode(&test_struct).unwrap();
        let decoded: TestStruct = MsgPack::decode(&encoded).unwrap();

        assert_eq!(test_struct, decoded);
    }

    #[test]
    fn test_json_to_msgpack_to_json() {
        let json_str = r#"{"name":"Test","age":30,"data":[1,2,3,4]}"#;

        let msgpack = MsgPack::json_to_msgpack(json_str).unwrap();
        let json = MsgPack::msgpack_to_json(&msgpack).unwrap();

        // Parse both as Value to handle formatting differences
        let original_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let converted_value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(original_value, converted_value);
    }

    #[test]
    fn test_is_msgpack_content_type() {
        assert!(MsgPack::is_msgpack_content_type("application/msgpack"));
        assert!(MsgPack::is_msgpack_content_type("application/x-msgpack"));
        assert!(!MsgPack::is_msgpack_content_type("application/json"));
    }
}
