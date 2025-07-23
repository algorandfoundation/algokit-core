use crate::{constants::LENGTH_ENCODE_BYTE_SIZE, ABIError, ABIType, ABIValue};

impl ABIType {
    pub(crate) fn encode_string(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::String => {
                let value = match value {
                    ABIValue::String(s) => s,
                    _ => {
                        return Err(ABIError::EncodingError(
                            "ABI value mismatch, expected string".to_string(),
                        ));
                    }
                };

                let utf8_bytes = value.as_bytes().to_vec();
                let length = utf8_bytes.len() as u16;
                let mut result = Vec::with_capacity(2 + utf8_bytes.len());
                result.extend_from_slice(&length.to_be_bytes());
                result.extend_from_slice(&utf8_bytes);

                Ok(result)
            }
            _ => Err(ABIError::EncodingError(
                "ABI type mismatch, expected string".to_string(),
            )),
        }
    }

    pub(crate) fn decode_string(&self, value: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::String => {
                if value.len() < LENGTH_ENCODE_BYTE_SIZE {
                    return Err(ABIError::DecodingError(
                        "Byte array is too short for string".to_string(),
                    ));
                }

                let length = u16::from_be_bytes([value[0], value[1]]) as usize;
                let content_bytes = &value[LENGTH_ENCODE_BYTE_SIZE..];
                if content_bytes.len() != length {
                    return Err(ABIError::DecodingError(format!(
                        "Invalid byte array length for string, expected {} value, got {}",
                        length,
                        content_bytes.len()
                    )));
                }

                let string_value = String::from_utf8(content_bytes.to_vec())
                    .map_err(|_| ABIError::DecodingError("Invalid UTF-8 encoding".to_string()))?;
                Ok(ABIValue::String(string_value))
            }
            _ => Err(ABIError::DecodingError(
                "ABI type mismatch, expected string".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insufficient_bytes() {
        let abi_type = ABIType::String;
        let bytes = vec![0]; // Only 1 byte, need 2 for length

        let result = abi_type.decode(&bytes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Byte array is too short for string"
        );
    }

    #[test]
    fn test_length_mismatch() {
        let abi_type = ABIType::String;
        let bytes = vec![0, 5, 65, 66]; // Claims 5 bytes but only has 2

        let result = abi_type.decode(&bytes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Invalid byte array length for string, expected 5 value, got 2"
        );
    }

    #[test]
    fn test_wrong_input_type() {
        let abi_type = ABIType::String;
        let value = ABIValue::Uint(num_bigint::BigUint::from(42u32));

        let result = abi_type.encode(&value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI encoding failed: ABI value mismatch, expected string"
        );
    }
}
