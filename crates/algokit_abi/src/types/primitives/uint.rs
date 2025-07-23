use num_bigint::BigUint;

use crate::{utils, ABIError, ABIType, ABIValue};

impl ABIType {
    pub(crate) fn encode_uint(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::Uint(bit_size) => {
                let bit_size = bit_size.value();
                let value = match value {
                    ABIValue::Uint(n) => n,
                    _ => {
                        return Err(ABIError::EncodingError(
                            "ABI value mismatch, expected uint".to_string(),
                        ));
                    }
                };

                if value >= &BigUint::from(2u64).pow(bit_size as u32) {
                    return Err(ABIError::EncodingError(format!(
                        "{} is too big to fit in uint{}",
                        value, bit_size
                    )));
                }

                Ok(utils::big_uint_to_bytes(value, (bit_size / 8) as usize))
            }
            _ => Err(ABIError::EncodingError(
                "ABI type mismatch, expected uint".to_string(),
            )),
        }
    }

    pub(crate) fn decode_uint(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::Uint(bit_size) => {
                let bit_size = bit_size.value();
                let expected_len = (bit_size / 8) as usize;
                if bytes.len() != expected_len {
                    return Err(ABIError::DecodingError(format!(
                        "Invalid byte array length, expected {} bytes, got {}",
                        expected_len,
                        bytes.len()
                    )));
                }

                Ok(ABIValue::Uint(BigUint::from_bytes_be(bytes)))
            }
            _ => Err(ABIError::DecodingError(
                "ABI type mismatch, expected uint".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::abi_type::BitSize;

    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_uint_decode_wrong_length() {
        let abi_type = ABIType::Uint(BitSize::new(8).unwrap());
        let abi_value = vec![0u8, 0];

        let result = abi_type.decode(&abi_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Invalid byte array length, expected 1 bytes, got 2"
        );
    }

    #[test]
    fn test_uint_decode_wrong_abi_type() {
        let abi_type = ABIType::String;
        let abi_value = vec![0u8, 0, 0, 42];

        let result = abi_type.decode(&abi_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Invalid byte array length for string, expected 0 value, got 2"
        );
    }

    #[test]
    fn test_uint_max_size() {
        let abi_type = ABIType::Uint(BitSize::new(512).unwrap());
        let value = ABIValue::Uint(BigUint::from(1u64) << 511); // 2^511
        let encoded = abi_type.encode(&value).unwrap();
        assert_eq!(encoded.len(), 64); // 512 bits = 64 bytes
        let decoded = abi_type.decode(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_uint_leading_zeros() {
        // Test that leading zeros are handled correctly
        let abi_type = ABIType::Uint(BitSize::new(32).unwrap());
        let value = ABIValue::Uint(BigUint::from(1u32));
        let encoded = abi_type.encode(&value).unwrap();
        assert_eq!(encoded, vec![0, 0, 0, 1]); // Should have leading zeros
        let decoded = abi_type.decode(&encoded).unwrap();
        assert_eq!(decoded, value);
    }
}
