use crate::{
    ABIError, ABIType, ABIValue,
    constants::{BOOL_FALSE_BYTE, BOOL_TRUE_BYTE},
};

impl ABIType {
    pub(crate) fn encode_bool(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::Bool => {
                let bool_value = match value {
                    ABIValue::Bool(b) => b,
                    _ => {
                        return Err(ABIError::EncodingError {
                            message: "ABI value mismatch, expected boolean".to_string(),
                        });
                    }
                };

                match *bool_value {
                    true => Ok(vec![BOOL_TRUE_BYTE]),   // true -> 128 (MSB set)
                    false => Ok(vec![BOOL_FALSE_BYTE]), // false -> 0
                }
            }
            _ => Err(ABIError::EncodingError {
                message: "ABI type mismatch, expected bool".to_string(),
            }),
        }
    }

    pub(crate) fn decode_bool(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::Bool => {
                if bytes.len() != 1 {
                    return Err(ABIError::DecodingError {
                        message: "Bool string must be 1 byte long".to_string(),
                    });
                }

                match bytes[0] {
                    BOOL_TRUE_BYTE => Ok(ABIValue::Bool(true)),
                    BOOL_FALSE_BYTE => Ok(ABIValue::Bool(false)),
                    _ => Err(ABIError::DecodingError {
                        message: "Boolean could not be decoded from the byte string".to_string(),
                    }),
                }
            }
            _ => Err(ABIError::DecodingError {
                message: "ABI type mismatch, expected bool".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_wrong_type() {
        let abi_type = ABIType::Bool;
        let value = ABIValue::String("true".to_string());

        let result = abi_type.encode(&value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI encoding failed: ABI value mismatch, expected boolean"
        );
    }

    #[test]
    fn test_decode_wrong_length() {
        let abi_type = ABIType::Bool;
        let bytes = vec![0x80, 0x00]; // 2 bytes instead of 1

        let result = abi_type.decode(&bytes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Bool string must be 1 byte long"
        );
    }

    #[test]
    fn test_decode_invalid_value() {
        let abi_type = ABIType::Bool;
        let bytes = vec![0x30]; // Invalid value (not 0x80 or 0x00)

        let result = abi_type.decode(&bytes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Boolean could not be decoded from the byte string"
        );
    }
}
