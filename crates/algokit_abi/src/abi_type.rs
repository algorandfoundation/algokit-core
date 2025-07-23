use crate::{
    constants::{
        ALGORAND_PUBLIC_KEY_BYTE_LENGTH, BITS_PER_BYTE, MAX_BIT_SIZE, MAX_PRECISION,
        STATIC_ARRAY_REGEX, UFIXED_REGEX,
    },
    types::collections::tuple::find_bool_sequence_end,
    ABIError, ABIValue,
};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// Represents a validated bit size for ABI uint and ufixed types (8-512, multiple of 8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitSize(u16);

impl BitSize {
    /// Creates a new [`BitSize`] with validation.
    ///
    /// # Arguments
    /// * `bits` - The bit size, must be 8-512 and divisible by 8.
    ///
    /// # Returns
    /// A new [`BitSize`] if valid, or an [`ABIError`] if invalid.
    pub fn new(bits: u16) -> Result<Self, ABIError> {
        if bits < BITS_PER_BYTE as u16 || bits > MAX_BIT_SIZE || bits % BITS_PER_BYTE as u16 != 0 {
            return Err(ABIError::ValidationError(format!(
                "Bit size must be between {} and {} and divisible by {}, got {}",
                BITS_PER_BYTE, MAX_BIT_SIZE, BITS_PER_BYTE, bits
            )));
        }
        Ok(BitSize(bits))
    }

    /// Returns the bit size value.
    ///
    /// # Returns
    /// The underlying bit size as a `u16`.
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// Represents a validated precision for ufixed ABI types (0-160).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Precision(u8);

impl Precision {
    /// Creates a new [`Precision`] with validation.
    ///
    /// # Arguments
    /// * `precision` - The precision value, must be 0-160.
    ///
    /// # Returns
    /// A new [`Precision`] if valid, or an [`ABIError`] if invalid.
    pub fn new(precision: u8) -> Result<Self, ABIError> {
        if precision > MAX_PRECISION {
            return Err(ABIError::ValidationError(format!(
                "Precision must be between 0 and {}, got {}",
                MAX_PRECISION, precision
            )));
        }
        Ok(Precision(precision))
    }

    /// Returns the precision value.
    ///
    /// # Returns
    /// The underlying precision as a `u8`.
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Represents an Algorand ABI type for encoding and decoding values as defined in [ARC-0004](https://arc.algorand.foundation/ARCs/arc-0004#types).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ABIType {
    /// An unsigned integer of a specific bit size.
    Uint(BitSize),
    /// A fixed-point number of a specific bit size and precision.
    UFixed(BitSize, Precision),
    /// An Algorand address.
    Address,
    /// A tuple of other ABI types.
    Tuple(Vec<ABIType>),
    /// A dynamic-length string.
    String,
    /// A single byte.
    Byte,
    /// A boolean value.
    Bool,
    /// A static-length array of another ABI type.
    StaticArray(Box<ABIType>, usize),
    /// A dynamic-length array of another ABI type.
    DynamicArray(Box<ABIType>),
}

impl AsRef<ABIType> for ABIType {
    fn as_ref(&self) -> &ABIType {
        self
    }
}

impl ABIType {
    /// Encodes an [`ABIValue`] according to this ABI type specification.
    ///
    /// # Arguments
    /// * `value` - The value to encode, must match this type.
    ///
    /// # Returns
    /// The encoded bytes if successful, or an [`ABIError`] if encoding fails.
    pub fn encode(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::Uint(_) => self.encode_uint(value),
            ABIType::UFixed(_, _) => self.encode_ufixed(value),
            ABIType::Address => self.encode_address(value),
            ABIType::Tuple(_) => self.encode_tuple(value),
            ABIType::StaticArray(_, _size) => self.encode_static_array(value),
            ABIType::DynamicArray(_) => self.encode_dynamic_array(value),
            ABIType::String => self.encode_string(value),
            ABIType::Byte => self.encode_byte(value),
            ABIType::Bool => self.encode_bool(value),
        }
    }

    /// Decodes bytes according to this ABI type specification.
    ///
    /// # Arguments
    /// * `bytes` - The encoded bytes to decode.
    ///
    /// # Returns
    /// The decoded [`ABIValue`] if successful, or an [`ABIError`] if decoding fails.
    pub fn decode(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::Uint(_) => self.decode_uint(bytes),
            ABIType::UFixed(_, _) => self.decode_ufixed(bytes),
            ABIType::Address => self.decode_address(bytes),
            ABIType::String => self.decode_string(bytes),
            ABIType::Bool => self.decode_bool(bytes),
            ABIType::Byte => self.decode_byte(bytes),
            ABIType::Tuple(_) => self.decode_tuple(bytes),
            ABIType::StaticArray(_, _size) => self.decode_static_array(bytes),
            ABIType::DynamicArray(_) => self.decode_dynamic_array(bytes),
        }
    }

    pub(crate) fn is_dynamic(&self) -> bool {
        match self {
            ABIType::StaticArray(child_type, _) => child_type.is_dynamic(),
            ABIType::Tuple(child_types) => child_types.iter().any(|t| t.is_dynamic()),
            ABIType::DynamicArray(_) | ABIType::String => true,
            _ => false,
        }
    }

    pub(crate) fn get_size(abi_type: &ABIType) -> Result<usize, ABIError> {
        match abi_type {
            ABIType::Uint(bit_size) => Ok((bit_size.value() / BITS_PER_BYTE as u16) as usize),
            ABIType::UFixed(bit_size, _) => Ok((bit_size.value() / BITS_PER_BYTE as u16) as usize),
            ABIType::Address => Ok(ALGORAND_PUBLIC_KEY_BYTE_LENGTH),
            ABIType::Bool => Ok(1),
            ABIType::Byte => Ok(1),
            ABIType::StaticArray(child_type, size) => match child_type.as_ref() {
                ABIType::Bool => Ok((*size).div_ceil(BITS_PER_BYTE as usize)),
                _ => Ok(Self::get_size(child_type)? * *size),
            },
            ABIType::Tuple(child_types) => {
                let mut size = 0;
                let mut i = 0;
                while i < child_types.len() {
                    let child_type = &child_types[i];
                    match child_type {
                        ABIType::Bool => {
                            let sequence_end_index = find_bool_sequence_end(child_types, i);
                            let bool_count = sequence_end_index - i + 1;

                            size += bool_count.div_ceil(BITS_PER_BYTE as usize);
                            i = sequence_end_index + 1;
                        }
                        _ => {
                            size += Self::get_size(child_type)?;
                            i += 1;
                        }
                    }
                }
                Ok(size)
            }
            ABIType::String => Err(ABIError::DecodingError(format!(
                "Failed to get size, {} is a dynamic type",
                abi_type
            ))),
            ABIType::DynamicArray(_) => Err(ABIError::DecodingError(format!(
                "Failed to get size, {} is a dynamic type",
                abi_type
            ))),
        }
    }
}

impl Display for ABIType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ABIType::Uint(bit_size) => write!(f, "uint{}", bit_size.value()),
            ABIType::UFixed(bit_size, precision) => {
                write!(f, "ufixed{}x{}", bit_size.value(), precision.value())
            }
            ABIType::Address => write!(f, "address"),
            ABIType::Tuple(child_types) => {
                let type_strings: Vec<String> = child_types.iter().map(|t| t.to_string()).collect();
                write!(f, "({})", type_strings.join(","))
            }
            ABIType::String => write!(f, "string"),
            ABIType::Byte => write!(f, "byte"),
            ABIType::Bool => write!(f, "bool"),
            ABIType::StaticArray(child_type, length) => {
                write!(f, "{}[{}]", child_type, length)
            }
            ABIType::DynamicArray(child_type) => {
                write!(f, "{}[]", child_type)
            }
        }
    }
}

impl FromStr for ABIType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Dynamic array
        if let Some(element_type_str) = s.strip_suffix("[]") {
            let element_type = ABIType::from_str(element_type_str)?;
            return Ok(ABIType::DynamicArray(Box::new(element_type)));
        }

        // Static array
        if s.ends_with(']') {
            let regex = &*STATIC_ARRAY_REGEX;
            if let Some(captures) = regex.captures(s) {
                let element_type_str = &captures[1];
                let length_str = &captures[2];

                let length = length_str.parse::<usize>().map_err(|_| {
                    ABIError::ValidationError(format!("Invalid array length: {}", length_str))
                })?;

                let element_type = ABIType::from_str(element_type_str)?;
                return Ok(ABIType::StaticArray(Box::new(element_type), length));
            } else {
                return Err(ABIError::ValidationError(format!(
                    "Malformed static array string: {}",
                    s
                )));
            }
        }

        // Uint type
        if let Some(size_str) = s.strip_prefix("uint") {
            if size_str.chars().all(|c| c.is_ascii_digit()) {
                let size = size_str.parse::<u16>().map_err(|_| {
                    ABIError::ValidationError(format!("Invalid uint size: {}", size_str))
                })?;
                let bit_size = BitSize::new(size)?;
                return Ok(ABIType::Uint(bit_size));
            } else {
                return Err(ABIError::ValidationError(format!(
                    "Malformed uint string: {}",
                    size_str
                )));
            }
        }

        // UFixed type
        if s.starts_with("ufixed") {
            let regex = &*UFIXED_REGEX;
            if let Some(captures) = regex.captures(s) {
                let size_str = &captures[1];
                let precision_str = &captures[2];

                let size = size_str.parse::<u16>().map_err(|_| {
                    ABIError::ValidationError(format!("Invalid ufixed size: {}", size_str))
                })?;
                let precision = precision_str.parse::<u8>().map_err(|_| {
                    ABIError::ValidationError(format!(
                        "Invalid ufixed precision: {}",
                        precision_str
                    ))
                })?;

                let bit_size = BitSize::new(size)?;
                let precision = Precision::new(precision)?;
                return Ok(ABIType::UFixed(bit_size, precision));
            } else {
                return Err(ABIError::ValidationError(format!(
                    "Malformed ufixed type: {}",
                    s
                )));
            }
        }

        // Tuple type
        if s.len() >= 2 && s.starts_with('(') && s.ends_with(')') {
            let tuple_type_strings: Vec<_> = parse_tuple_content(&s[1..s.len() - 1])?;
            let child_types: Result<Vec<_>, _> = tuple_type_strings
                .iter()
                .map(|str| ABIType::from_str(str))
                .collect();

            return Ok(ABIType::Tuple(child_types?));
        }

        // Simple types
        match s {
            "byte" => Ok(ABIType::Byte),
            "bool" => Ok(ABIType::Bool),
            "address" => Ok(ABIType::Address),
            "string" => Ok(ABIType::String),
            _ => Err(ABIError::ValidationError(format!(
                "Cannot convert string '{}' to an ABI type",
                s
            ))),
        }
    }
}

pub(crate) fn parse_tuple_content(content: &str) -> Result<Vec<String>, ABIError> {
    if content.is_empty() {
        return Ok(Vec::new());
    }

    if content.starts_with(",") {
        return Err(ABIError::ValidationError(
            "Tuple name should not start with comma".to_string(),
        ));
    }
    if content.ends_with(",") {
        return Err(ABIError::ValidationError(
            "Tuple name should not start with comma".to_string(),
        ));
    }
    if content.contains(",,") {
        return Err(ABIError::ValidationError(
            "tuple string should not have consecutive commas".to_string(),
        ));
    }

    let mut tuple_strings: Vec<String> = Vec::new();
    let mut depth = 0;
    let mut word: String = String::new();

    for ch in content.chars() {
        word.push(ch);
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                word.pop();
                tuple_strings.push(word);
                word = String::new();
            }
            _ => {}
        }
    }
    if !word.is_empty() {
        tuple_strings.push(word);
    }
    if depth != 0 {
        return Err(ABIError::ValidationError(
            "Tuple string has mismatched parentheses".to_string(),
        ));
    }

    Ok(tuple_strings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use rstest::rstest;

    #[rstest]
    #[case(
        ABIType::Uint(BitSize::new(8).unwrap()),
        ABIValue::Uint(BigUint::from(0u8)),
        &[0]
    )]
    #[case(
        ABIType::Uint(BitSize::new(16).unwrap()),
        ABIValue::Uint(BigUint::from(3u16)),
        &[0, 3]
    )]
    #[case(
        ABIType::Uint(BitSize::new(64).unwrap()),
        ABIValue::Uint(BigUint::from(256u64)),
        &[0, 0, 0, 0, 0, 0, 1, 0]
    )]
    #[case(
        ABIType::UFixed(BitSize::new(8).unwrap(), Precision::new(30).unwrap()),
        ABIValue::Uint(BigUint::from(255u8)),
        &[255]
    )]
    #[case(
        ABIType::UFixed(BitSize::new(32).unwrap(), Precision::new(10).unwrap()),
        ABIValue::Uint(BigUint::from(33u32)),
        &[0, 0, 0, 33]
    )]
    #[case(
        ABIType::Address,
        ABIValue::Address("MO2H6ZU47Q36GJ6GVHUKGEBEQINN7ZWVACMWZQGIYUOE3RBSRVYHV4ACJI".to_string()),
        &[
            99, 180, 127, 102, 156, 252, 55, 227, 39, 198, 169, 232, 163, 16, 36, 130, 26, 223, 230, 213, 0, 153, 108, 192, 200, 197, 28, 77, 196, 50, 141, 112
        ]
    )]
    #[case(
        ABIType::String,
        ABIValue::String("What’s new".to_string()),
        &[0, 12, 87, 104, 97, 116, 226, 128, 153, 115, 32, 110, 101, 119]
    )]
    #[case(
        ABIType::String,
        ABIValue::String("😅🔨".to_string()),
        &[0, 8, 240, 159, 152, 133, 240, 159, 148, 168]
    )]
    #[case(
        ABIType::Byte,
        ABIValue::Byte(10),
        &[10]
    )]
    #[case(
        ABIType::Byte,
        ABIValue::Byte(255),
        &[255]
    )]
    #[case(
        ABIType::Bool,
        ABIValue::Bool(true),
        &[128]
    )]
    #[case(
        ABIType::Bool,
        ABIValue::Bool(false),
        &[0]
    )]
    #[case(
        ABIType::String,
        ABIValue::String("asdf".to_string()),
        &[0, 4, 97, 115, 100, 102]
    )]
    #[case(
        ABIType::StaticArray(Box::new(ABIType::Bool), 3),
        ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(false)]),
        &[192]
    )]
    #[case(
        ABIType::StaticArray(Box::new(ABIType::Bool), 8),
        ABIValue::Array(vec![ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false)]),
        &[64]
    )]
    #[case(
        ABIType::StaticArray(Box::new(ABIType::Bool), 8),
        ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(true)]),
        &[255]
    )]
    #[case(
        ABIType::StaticArray(Box::new(ABIType::Bool), 9),
        ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(true)]),
        &[146, 128]
    )]
    #[case(
        ABIType::StaticArray(Box::new(ABIType::Uint(BitSize::new(64).unwrap())), 3),
        ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u64)), ABIValue::Uint(BigUint::from(2u64)), ABIValue::Uint(BigUint::from(3u64))]),
        &[0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3]
    )]
    #[case(
        ABIType::DynamicArray(Box::new(ABIType::Bool)),
        ABIValue::Array(vec![]),
        &[0, 0]
    )]
    #[case(
        ABIType::DynamicArray(Box::new(ABIType::Bool)),
        ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(true), ABIValue::Bool(false)]),
        &[0, 3, 192]
    )]
    #[case(
        ABIType::DynamicArray(Box::new(ABIType::Bool)),
        ABIValue::Array(vec![ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(false)]),
        &[0, 8, 64]
    )]
    #[case(
        ABIType::DynamicArray(Box::new(ABIType::Bool)),
        ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(true)]),
        &[0, 9, 146, 128]
    )]
    #[case(
        ABIType::from_str("()").unwrap(),
        ABIValue::Array(vec![]),
        &[]
    )]
    #[case(
        ABIType::from_str("(bool,bool,bool)").unwrap(),
        ABIValue::Array(vec![ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(true)]),
        &[96]
    )]
    #[case(
        ABIType::from_str("(bool[3])").unwrap(),
        ABIValue::Array(vec![ABIValue::Array(vec![ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(true)])]),
        &[96]
    )]
    #[case(
        ABIType::from_str("(bool[])").unwrap(),
        ABIValue::Array(vec![ABIValue::Array(vec![ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(true)])]),
        &[0, 2, 0, 3, 96]
    )]
    #[case(
        ABIType::from_str("(bool[2],bool[])").unwrap(),
        ABIValue::Array(vec![ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(true)]), ABIValue::Array(vec![ABIValue::Bool(true), ABIValue::Bool(true)])]),
        &[192, 0, 3, 0, 2, 192]
    )]
    #[case(
        ABIType::from_str("(bool[],bool[])").unwrap(),
        ABIValue::Array(vec![ABIValue::Array(vec![]), ABIValue::Array(vec![])]),
        &[0, 4, 0, 6, 0, 0, 0, 0]
    )]
    #[case(
        ABIType::from_str("(string,bool,bool,bool,bool,string)").unwrap(),
        ABIValue::Array(vec![ABIValue::String("AB".to_string()), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::Bool(true), ABIValue::Bool(false), ABIValue::String("DE".to_string())]),
        &[0, 5, 160, 0, 9, 0, 2, 65, 66, 0, 2, 68, 69]
    )]
    #[case(
        ABIType::Tuple(vec![ABIType::Uint(BitSize::new(8).unwrap()),
        ABIType::Uint(BitSize::new(16).unwrap())]), ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u8)), ABIValue::Uint(BigUint::from(2u16))]),
        &[1, 0, 2]
    )]
    #[case(
        ABIType::Tuple(vec![ABIType::Uint(BitSize::new(32).unwrap()),
        ABIType::Uint(BitSize::new(32).unwrap())]), ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u32)), ABIValue::Uint(BigUint::from(2u16))]),
        &[0, 0, 0, 1, 0, 0, 0, 2]
    )]
    #[case(
        ABIType::Tuple(vec![ABIType::Uint(BitSize::new(32).unwrap()),
        ABIType::String]), ABIValue::Array(vec![ABIValue::Uint(BigUint::from(42u32)), ABIValue::String("hello".to_string())]),
        &[0, 0, 0, 42, 0, 6, 0, 5, 104, 101, 108, 108, 111]
    )]
    #[case(
        ABIType::Tuple(vec![ABIType::Uint(BitSize::new(16).unwrap()),
        ABIType::Bool]), ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1234u32)), ABIValue::Bool(false)]),
        &[4, 210, 0]
    )]
    #[case(
        ABIType::Tuple(vec![ABIType::Uint(BitSize::new(32).unwrap()),
        ABIType::String, ABIType::Bool]), ABIValue::Array(vec![ABIValue::Uint(BigUint::from(42u32)), ABIValue::String("test".to_string()), ABIValue::Bool(false)]),
        &[0, 0, 0, 42, 0, 7, 0, 0, 4, 116, 101, 115, 116]
    )]
    #[case(
        ABIType::from_str("(uint16,(byte,address))").unwrap(),
        ABIValue::Array(vec![ABIValue::Uint(BigUint::from(42u32)), ABIValue::Array(vec![ABIValue::Byte(234), ABIValue::Address("MO2H6ZU47Q36GJ6GVHUKGEBEQINN7ZWVACMWZQGIYUOE3RBSRVYHV4ACJI".to_string())])]),
        &[0, 42, 234, 99, 180, 127, 102, 156, 252, 55, 227, 39, 198, 169, 232, 163, 16, 36, 130, 26, 223, 230, 213, 0, 153, 108, 192, 200, 197, 28, 77, 196, 50, 141, 112])]
    fn should_round_trip(
        #[case] abi_type: ABIType,
        #[case] abi_value: ABIValue,
        #[case] expected_encoded_value: &[u8],
    ) {
        let encoded = abi_type.encode(&abi_value).expect("Failed to encode");
        assert_eq!(encoded, expected_encoded_value);
        let decoded = abi_type.decode(&encoded).expect("Failed to decode");
        assert_eq!(decoded, abi_value);
    }
}
