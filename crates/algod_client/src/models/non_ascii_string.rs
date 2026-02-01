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
        struct NonAsciiStringVisitor;

        impl serde::de::Visitor<'_> for NonAsciiStringVisitor {
            type Value = NonAsciiString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string (possibly non-UTF8)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonAsciiString(v.as_bytes().to_vec()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonAsciiString(v.into_bytes()))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonAsciiString(v.to_vec()))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonAsciiString(v))
            }
        }

        deserializer.deserialize_any(NonAsciiStringVisitor)
    }
}

impl Serialize for NonAsciiString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Use the new MSGPACK_RAW_STR_STRUCT_NAME mechanism to serialize
        // raw bytes as MessagePack string without UTF-8 validation
        serializer.serialize_newtype_struct(
            rmp_serde::MSGPACK_RAW_STR_STRUCT_NAME,
            serde_bytes::Bytes::new(&self.0),
        )
    }
}
