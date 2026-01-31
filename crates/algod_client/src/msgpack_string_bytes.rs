/// Custom serde module for deserializing msgpack strings as raw bytes.
///
/// Msgpack strings may contain arbitrary bytes that aren't valid UTF-8.
/// This module deserializes the raw string bytes into Vec<u8> without
/// requiring UTF-8 validity.
use serde::{Deserialize, Deserializer, Serializer};

/// Deserialize a msgpack string as raw bytes
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Use rmpv::Value to capture the raw msgpack value
    let value: Option<rmpv::Value> = Option::deserialize(deserializer)?;

    match value {
        Some(rmpv::Value::String(s)) => {
            // rmpv::Utf8String gives us access to raw bytes even if not valid UTF-8
            Ok(Some(s.into_bytes()))
        }
        Some(rmpv::Value::Binary(b)) => Ok(Some(b)),
        Some(_) => Err(serde::de::Error::custom(
            "expected string or binary, got other type",
        )),
        None => Ok(None),
    }
}

/// Serialize bytes as a msgpack binary
pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(bytes) => serializer.serialize_str(unsafe { std::str::from_utf8_unchecked(bytes) }),
        None => serializer.serialize_none(),
    }
}
