use num_bigint::BigUint;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Represents a value that can be encoded or decoded as an ABI type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ABIValue {
    /// A boolean value.
    Bool(bool),
    /// An unsigned integer value.
    Uint(BigUint),
    /// A string value.
    String(String),
    /// A byte value.
    Byte(u8),
    /// An array of ABI values.
    Array(Vec<ABIValue>),
    /// An Algorand address.
    Address(String),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// A struct value represented as a key-value map.
    Struct(HashMap<String, ABIValue>),
}

impl From<bool> for ABIValue {
    fn from(value: bool) -> Self {
        ABIValue::Bool(value)
    }
}

impl From<BigUint> for ABIValue {
    fn from(value: BigUint) -> Self {
        ABIValue::Uint(value)
    }
}

impl From<u8> for ABIValue {
    fn from(value: u8) -> Self {
        ABIValue::Uint(BigUint::from(value))
    }
}

impl From<u16> for ABIValue {
    fn from(value: u16) -> Self {
        ABIValue::Uint(BigUint::from(value))
    }
}

impl From<u32> for ABIValue {
    fn from(value: u32) -> Self {
        ABIValue::Uint(BigUint::from(value))
    }
}

impl From<u64> for ABIValue {
    fn from(value: u64) -> Self {
        ABIValue::Uint(BigUint::from(value))
    }
}

impl From<u128> for ABIValue {
    fn from(value: u128) -> Self {
        ABIValue::Uint(BigUint::from(value))
    }
}

impl From<usize> for ABIValue {
    fn from(value: usize) -> Self {
        ABIValue::Uint(BigUint::from(value))
    }
}

impl From<String> for ABIValue {
    fn from(value: String) -> Self {
        ABIValue::String(value)
    }
}

impl From<&str> for ABIValue {
    fn from(value: &str) -> Self {
        ABIValue::String(value.to_string())
    }
}

impl From<Vec<ABIValue>> for ABIValue {
    fn from(value: Vec<ABIValue>) -> Self {
        ABIValue::Array(value)
    }
}

impl From<HashMap<String, ABIValue>> for ABIValue {
    fn from(value: HashMap<String, ABIValue>) -> Self {
        ABIValue::Struct(value)
    }
}

impl ABIValue {
    /// Create an ABIValue::Byte from a u8 value
    pub fn from_byte(value: u8) -> Self {
        ABIValue::Byte(value)
    }

    /// Create an ABIValue::Address from a string
    pub fn from_address<S: Into<String>>(value: S) -> Self {
        ABIValue::Address(value.into())
    }

    /// Create an ABIValue::Struct from a HashMap
    pub fn from_struct(value: HashMap<String, ABIValue>) -> Self {
        ABIValue::Struct(value)
    }
}

impl Hash for ABIValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ABIValue::Bool(b) => {
                0u8.hash(state);
                b.hash(state);
            }
            ABIValue::Uint(u) => {
                1u8.hash(state);
                u.to_bytes_be().hash(state);
            }
            ABIValue::String(s) => {
                2u8.hash(state);
                s.hash(state);
            }
            ABIValue::Byte(b) => {
                3u8.hash(state);
                b.hash(state);
            }
            ABIValue::Array(arr) => {
                4u8.hash(state);
                arr.hash(state);
            }
            ABIValue::Address(addr) => {
                5u8.hash(state);
                addr.hash(state);
            }
            ABIValue::Bytes(bytes) => {
                6u8.hash(state);
                bytes.hash(state);
            }
            ABIValue::Struct(map) => {
                7u8.hash(state);
                // For HashMap, we need to hash in a consistent order
                let mut pairs: Vec<_> = map.iter().collect();
                pairs.sort_by_key(|(k, _)| *k);
                pairs.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_from_bool() {
        let value = ABIValue::from(true);
        assert_eq!(value, ABIValue::Bool(true));

        let value = ABIValue::from(false);
        assert_eq!(value, ABIValue::Bool(false));
    }

    #[rstest]
    #[case(ABIValue::from(42u8), ABIValue::Uint(BigUint::from(42u8)))]
    #[case(ABIValue::from(1000u16), ABIValue::Uint(BigUint::from(1000u16)))]
    #[case(ABIValue::from(100000u32), ABIValue::Uint(BigUint::from(100000u32)))]
    #[case(
        ABIValue::from(10000000000u64),
        ABIValue::Uint(BigUint::from(10000000000u64))
    )]
    #[case(
        ABIValue::from(340282366920938463463374607431768211455u128),
        ABIValue::Uint(BigUint::from(340282366920938463463374607431768211455u128))
    )]
    #[case(ABIValue::from(12345usize), ABIValue::Uint(BigUint::from(12345usize)))]
    #[case(ABIValue::from(100000u32), ABIValue::Uint(BigUint::from(100000u32)))]
    #[case(
        ABIValue::from(BigUint::from(999999u64)),
        ABIValue::Uint(BigUint::from(999999u64))
    )]
    fn test_from_uint_types(#[case] abi_value_1: ABIValue, #[case] abi_value_2: ABIValue) {
        assert_eq!(abi_value_1, abi_value_2);
    }

    #[test]
    fn test_from_string() {
        // Test String
        let value = ABIValue::from("hello world".to_string());
        assert_eq!(value, ABIValue::String("hello world".to_string()));

        // Test &str
        let value = ABIValue::from("hello world");
        assert_eq!(value, ABIValue::String("hello world".to_string()));
    }

    #[test]
    fn test_from_array() {
        let array = vec![
            ABIValue::Bool(true),
            ABIValue::Uint(BigUint::from(42u8)),
            ABIValue::String("test".to_string()),
        ];
        let value = ABIValue::from(array.clone());
        assert_eq!(value, ABIValue::Array(array));
    }

    #[test]
    fn test_from_byte() {
        let value = ABIValue::from_byte(255u8);
        assert_eq!(value, ABIValue::Byte(255u8));
    }

    #[test]
    fn test_from_address() {
        // Test with String
        let addr = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ".to_string();
        let value = ABIValue::from_address(addr.clone());
        assert_eq!(value, ABIValue::Address(addr));

        // Test with &str
        let addr_str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ";
        let value = ABIValue::from_address(addr_str);
        assert_eq!(value, ABIValue::Address(addr_str.to_string()));
    }

    #[test]
    fn test_from_struct() {
        let mut struct_map = HashMap::new();
        struct_map.insert("name".to_string(), ABIValue::String("Alice".to_string()));
        struct_map.insert("age".to_string(), ABIValue::Uint(BigUint::from(30u32)));

        let value = ABIValue::from_struct(struct_map.clone());
        assert_eq!(value, ABIValue::Struct(struct_map.clone()));

        // Test with From trait
        let value2 = ABIValue::from(struct_map.clone());
        assert_eq!(value2, ABIValue::Struct(struct_map));
    }
}
