/// Custom serde module for deserializing a vector of msgpack strings as raw bytes.
///
/// Msgpack strings may contain arbitrary bytes that aren't valid UTF-8.
/// This module deserializes the raw string bytes into Vec<Vec<u8>> without
/// requiring UTF-8 validity.
use serde::{Deserialize, Deserializer, Serializer};

/// Deserialize a vector of msgpack strings as raw bytes
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Vec<u8>>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Use rmpv::Value to capture the raw msgpack value
    let value: Option<rmpv::Value> = Option::deserialize(deserializer)?;

    match value {
        Some(rmpv::Value::Array(arr)) => {
            let mut result = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    rmpv::Value::String(s) => {
                        // rmpv::Utf8String gives us access to raw bytes even if not valid UTF-8
                        result.push(s.into_bytes());
                    }
                    rmpv::Value::Binary(b) => result.push(b),
                    _ => {
                        return Err(serde::de::Error::custom(
                            "expected string or binary in array, got other type",
                        ));
                    }
                }
            }
            Ok(Some(result))
        }
        Some(_) => Err(serde::de::Error::custom("expected array, got other type")),
        None => Ok(None),
    }
}

/// Serialize a vector of bytes as msgpack strings
pub fn serialize<S>(value: &Option<Vec<Vec<u8>>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(vec) => {
            use serde::ser::SerializeSeq;
            let mut seq = serializer.serialize_seq(Some(vec.len()))?;
            for bytes in vec {
                // Serialize each byte vector as a string (not binary)
                let s = unsafe { std::str::from_utf8_unchecked(bytes) };
                seq.serialize_element(s)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}
