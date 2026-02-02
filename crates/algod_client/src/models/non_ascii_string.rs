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

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use std::collections::HashMap;

    /// Test that NonAsciiString preserves non-UTF8 bytes when used as map keys
    /// during serialization/deserialization roundtrip.
    ///
    /// This reproduces the data loss issue found in block transaction data where
    /// non-UTF8 byte sequences in map keys were being converted to empty strings.
    #[test]
    fn test_non_utf8_map_keys_roundtrip() {
        // These are actual byte sequences from the failing test that were being
        // converted to empty strings during roundtrip
        let test_keys = vec![
            vec![0, 0, 0, 0, 0, 4, 197, 193],    // UTF-8 error at position 6
            vec![0, 0, 0, 0, 1, 225, 171, 112],  // UTF-8 error at position 5
            vec![0, 0, 0, 0, 17, 40, 131, 228],  // UTF-8 error at position 6
            vec![0, 0, 0, 0, 23, 4, 213, 85],    // UTF-8 error at position 6
            vec![0, 0, 0, 0, 41, 100, 51, 129],  // UTF-8 error at position 7
            vec![0, 0, 0, 0, 52, 228, 193, 3],   // UTF-8 error at position 5
            vec![0, 0, 0, 0, 52, 232, 113, 71],  // UTF-8 error at position 5
            vec![0, 0, 0, 0, 71, 135, 254, 137], // UTF-8 error at position 7
        ];

        // Create a map with NonAsciiString keys containing non-UTF8 bytes
        let mut original_map: HashMap<NonAsciiString, String> = HashMap::new();
        for (i, key_bytes) in test_keys.iter().enumerate() {
            let key = NonAsciiString::new(key_bytes.clone());
            original_map.insert(key, format!("value_{}", i));
        }

        // Serialize the map to MessagePack
        let mut serialized = Vec::new();
        original_map
            .serialize(&mut rmp_serde::Serializer::new(&mut serialized))
            .expect("Failed to serialize map");

        // Deserialize back
        let deserialized_map: HashMap<NonAsciiString, String> =
            rmp_serde::from_slice(&serialized).expect("Failed to deserialize map");

        // Check that all keys are preserved
        assert_eq!(
            original_map.len(),
            deserialized_map.len(),
            "Map lost keys during roundtrip! Original had {} keys, deserialized has {} keys",
            original_map.len(),
            deserialized_map.len()
        );

        // Check that each key's bytes are preserved exactly
        for (original_key, original_value) in &original_map {
            let found = deserialized_map
                .iter()
                .find(|(k, _)| k.as_bytes() == original_key.as_bytes());

            assert!(
                found.is_some(),
                "Key with bytes {:?} was lost during roundtrip",
                original_key.as_bytes()
            );

            let (deserialized_key, deserialized_value) = found.unwrap();
            assert_eq!(
                original_key.as_bytes(),
                deserialized_key.as_bytes(),
                "Key bytes changed during roundtrip: {:?} -> {:?}",
                original_key.as_bytes(),
                deserialized_key.as_bytes()
            );
            assert_eq!(
                original_value,
                deserialized_value,
                "Value changed for key {:?}",
                original_key.as_bytes()
            );
        }
    }

    /// Test that empty byte sequences are preserved (not converted to empty strings)
    #[test]
    fn test_empty_bytes_roundtrip() {
        let empty = NonAsciiString::new(vec![]);

        let mut serialized = Vec::new();
        empty
            .serialize(&mut rmp_serde::Serializer::new(&mut serialized))
            .expect("Failed to serialize empty NonAsciiString");

        let deserialized: NonAsciiString =
            rmp_serde::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            empty.as_bytes(),
            deserialized.as_bytes(),
            "Empty bytes were not preserved during roundtrip"
        );
    }

    /// Test valid UTF-8 strings are preserved
    #[test]
    fn test_valid_utf8_roundtrip() {
        let test_strings = vec!["", "hello", "世界", "\0\0\0\0\0\0\0\0"];

        for s in test_strings {
            let original = NonAsciiString::new(s.as_bytes().to_vec());

            let mut serialized = Vec::new();
            original
                .serialize(&mut rmp_serde::Serializer::new(&mut serialized))
                .expect("Failed to serialize NonAsciiString");

            let deserialized: NonAsciiString =
                rmp_serde::from_slice(&serialized).expect("Failed to deserialize");

            assert_eq!(
                original.as_bytes(),
                deserialized.as_bytes(),
                "UTF-8 string '{}' was not preserved during roundtrip",
                s
            );
        }
    }

    /// Test that NonAsciiString preserves non-UTF8 bytes when used as IndexMap keys.
    /// This is important because BlockStateDelta uses IndexMap<NonAsciiString, ...>
    #[test]
    fn test_non_utf8_indexmap_keys_roundtrip() {
        // These are actual byte sequences from the failing test
        let test_keys = vec![
            vec![0, 0, 0, 0, 0, 4, 197, 193],
            vec![0, 0, 0, 0, 1, 225, 171, 112],
            vec![0, 0, 0, 0, 17, 40, 131, 228],
        ];

        // Create an IndexMap with NonAsciiString keys containing non-UTF8 bytes
        let mut original_map: IndexMap<NonAsciiString, String> = IndexMap::new();
        for (i, key_bytes) in test_keys.iter().enumerate() {
            let key = NonAsciiString::new(key_bytes.clone());
            original_map.insert(key, format!("value_{}", i));
        }

        // Serialize the map to MessagePack
        let mut serialized = Vec::new();
        original_map
            .serialize(&mut rmp_serde::Serializer::new(&mut serialized))
            .expect("Failed to serialize IndexMap");

        // Deserialize back
        let deserialized_map: IndexMap<NonAsciiString, String> =
            rmp_serde::from_slice(&serialized).expect("Failed to deserialize IndexMap");

        // Check that all keys are preserved
        assert_eq!(
            original_map.len(),
            deserialized_map.len(),
            "IndexMap lost keys during roundtrip! Original had {} keys, deserialized has {} keys",
            original_map.len(),
            deserialized_map.len()
        );

        // Check that each key's bytes are preserved exactly
        for (original_key, original_value) in &original_map {
            let found = deserialized_map.get(original_key);

            assert!(
                found.is_some(),
                "Key with bytes {:?} was lost during roundtrip",
                original_key.as_bytes()
            );

            let deserialized_value = found.unwrap();
            assert_eq!(
                original_value,
                deserialized_value,
                "Value changed for key {:?}",
                original_key.as_bytes()
            );
        }

        // Also check byte-by-byte
        for (original_key, _) in &original_map {
            let found = deserialized_map
                .keys()
                .find(|k| k.as_bytes() == original_key.as_bytes());

            assert!(
                found.is_some(),
                "Key with bytes {:?} was lost during roundtrip",
                original_key.as_bytes()
            );
        }
    }

    /// Test nested structures similar to the actual BlockStateDelta structure
    #[test]
    fn test_nested_indexmap_with_non_utf8_keys() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct InnerValue {
            at: u64,
            bs: Vec<u8>,
        }

        // Create a nested structure: IndexMap<NonAsciiString, IndexMap<String, InnerValue>>
        let mut outer_map: IndexMap<NonAsciiString, IndexMap<String, InnerValue>> = IndexMap::new();

        let non_utf8_key = NonAsciiString::new(vec![0, 0, 0, 0, 0, 4, 197, 193]);
        let mut inner_map = IndexMap::new();
        inner_map.insert(
            "bs".to_string(),
            InnerValue {
                at: 1,
                bs: vec![1, 2, 3, 4],
            },
        );
        outer_map.insert(non_utf8_key.clone(), inner_map);

        // Serialize
        let mut serialized = Vec::new();
        outer_map
            .serialize(&mut rmp_serde::Serializer::new(&mut serialized))
            .expect("Failed to serialize nested structure");

        // Deserialize
        let deserialized: IndexMap<NonAsciiString, IndexMap<String, InnerValue>> =
            rmp_serde::from_slice(&serialized).expect("Failed to deserialize nested structure");

        assert_eq!(
            outer_map.len(),
            deserialized.len(),
            "Outer map lost keys during roundtrip"
        );

        // Check the non-UTF8 key is preserved
        let found_key = deserialized
            .keys()
            .find(|k| k.as_bytes() == non_utf8_key.as_bytes());

        assert!(
            found_key.is_some(),
            "Non-UTF8 key {:?} was lost in nested structure",
            non_utf8_key.as_bytes()
        );

        // Check the inner value is preserved
        let inner = deserialized.get(&non_utf8_key);
        assert!(inner.is_some(), "Inner map not found for non-UTF8 key");
        assert_eq!(
            outer_map.get(&non_utf8_key),
            inner,
            "Inner map values differ"
        );
    }

    /// Test that mimics the exact flow from the failing block test:
    /// raw msgpack -> rmpv::Value -> deserialize to struct -> serialize back -> rmpv::Value
    /// This tests if the rmpv intermediate representation causes data loss
    #[test]
    fn test_rmpv_roundtrip_with_non_utf8_keys() {
        use std::io::Cursor;

        // Create raw msgpack with non-UTF8 string keys (like from the blockchain)
        // This simulates what comes from the API
        let mut original_map: IndexMap<NonAsciiString, u32> = IndexMap::new();
        original_map.insert(NonAsciiString::new(vec![0, 0, 0, 0, 0, 4, 197, 193]), 1);
        original_map.insert(NonAsciiString::new(vec![0, 0, 0, 0, 1, 225, 171, 112]), 2);
        original_map.insert(NonAsciiString::new(vec![0, 0, 0, 0, 0, 0, 0, 0]), 3);

        // Serialize to msgpack bytes
        let mut raw_bytes = Vec::new();
        original_map
            .serialize(&mut rmp_serde::Serializer::new(&mut raw_bytes))
            .expect("Failed to serialize");

        // Decode to rmpv::Value (simulating raw API response)
        let raw_value = rmpv::decode::read_value(&mut Cursor::new(&raw_bytes))
            .expect("Failed to decode to rmpv::Value");

        // Deserialize from bytes to struct (simulating API client deserialization)
        let deserialized_map: IndexMap<NonAsciiString, u32> =
            rmp_serde::from_slice(&raw_bytes).expect("Failed to deserialize");

        // Serialize back to bytes (simulating to_msgpack())
        let mut reserialized_bytes = Vec::new();
        deserialized_map
            .serialize(&mut rmp_serde::Serializer::new(&mut reserialized_bytes))
            .expect("Failed to reserialize");

        // Decode reserialized bytes to rmpv::Value
        let reserialized_value = rmpv::decode::read_value(&mut Cursor::new(&reserialized_bytes))
            .expect("Failed to decode reserialized to rmpv::Value");

        // Compare the rmpv::Value representations
        // Extract maps
        let raw_map = raw_value.as_map().expect("Expected map in raw value");
        let reserialized_map = reserialized_value
            .as_map()
            .expect("Expected map in reserialized value");

        assert_eq!(
            raw_map.len(),
            reserialized_map.len(),
            "Maps have different sizes after roundtrip: raw has {} keys, reserialized has {} keys",
            raw_map.len(),
            reserialized_map.len()
        );

        // Check each key exists in both
        for (raw_key, raw_val) in raw_map {
            let raw_key_bytes = if let Some(s) = raw_key.as_str() {
                s.as_bytes()
            } else if let Some(b) = raw_key.as_slice() {
                b
            } else {
                panic!("Unexpected key type: {:?}", raw_key);
            };

            let found = reserialized_map.iter().any(|(k, v)| {
                let k_bytes = if let Some(s) = k.as_str() {
                    s.as_bytes()
                } else if let Some(b) = k.as_slice() {
                    b
                } else {
                    return false;
                };
                k_bytes == raw_key_bytes && v == raw_val
            });

            assert!(
                found,
                "Key {:?} with value {:?} from raw map not found in reserialized map",
                raw_key_bytes, raw_val
            );
        }
    }
}
