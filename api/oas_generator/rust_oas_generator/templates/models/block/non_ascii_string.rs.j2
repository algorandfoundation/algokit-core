/// A wrapper type for msgpack strings that may contain non-UTF8 bytes.
///
/// Msgpack strings can contain arbitrary bytes that aren't valid UTF-8.
/// This type handles both string and binary msgpack values, storing them
/// internally as raw bytes.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct NonAsciiString(pub Vec<u8>);

impl NonAsciiString {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for NonAsciiString {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<NonAsciiString> for Vec<u8> {
    fn from(s: NonAsciiString) -> Self {
        s.0
    }
}

impl AsRef<[u8]> for NonAsciiString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for NonAsciiString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NonAsciiString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> Deserialize<'de> for NonAsciiString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Use rmpv::Value to capture the raw msgpack value
        let value = rmpv::Value::deserialize(deserializer)?;

        match value {
            rmpv::Value::String(s) => {
                // rmpv::Utf8String gives us access to raw bytes even if not valid UTF-8
                Ok(NonAsciiString(s.into_bytes()))
            }
            rmpv::Value::Binary(b) => Ok(NonAsciiString(b)),
            _ => Err(serde::de::Error::custom(
                "expected string or binary, got other type",
            )),
        }
    }
}

impl Serialize for NonAsciiString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as a msgpack string (not binary)
        let s = unsafe { std::str::from_utf8_unchecked(&self.0) };
        serializer.serialize_str(s)
    }
}
