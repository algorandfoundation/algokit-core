use crate::{
    ABIError, ABIType, ABIValue,
    types::collections::tuple::{decode_abi_types, encode_abi_types},
};

impl ABIType {
    pub(crate) fn encode_static_array(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        let child_types = match self {
            ABIType::StaticArray(child_type, size) => vec![child_type.as_ref(); *size],
            _ => {
                return Err(ABIError::EncodingError(
                    "ABI type mismatch, expected static array".to_string(),
                ));
            }
        };

        let values = match value {
            ABIValue::Array(n) => n,
            _ => {
                return Err(ABIError::EncodingError(
                    "ABI value mismatch, expected an array of values".to_string(),
                ));
            }
        };

        encode_abi_types(&child_types, values)
    }

    pub(crate) fn decode_static_array(&self, value: &[u8]) -> Result<ABIValue, ABIError> {
        let child_types = match self {
            ABIType::StaticArray(child_type, size) => vec![child_type.as_ref(); *size],
            _ => {
                return Err(ABIError::EncodingError(
                    "ABI type mismatch, expected static array".to_string(),
                ));
            }
        };

        decode_abi_types(&child_types, value)
    }
}
