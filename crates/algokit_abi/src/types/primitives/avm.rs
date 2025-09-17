use crate::{ABIError, ABIType, ABIValue};
use std::str::FromStr;

impl ABIType {
    pub(crate) fn encode_avm_bytes(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::AVMBytes => match value {
                ABIValue::Bytes(bytes) => Ok(bytes.clone()),
                _ => Err(ABIError::EncodingError {
                    message: "ABI value mismatch, expected bytes for AVMBytes".to_string(),
                }),
            },
            _ => Err(ABIError::EncodingError {
                message: "ABI type mismatch, expected AVMBytes".to_string(),
            }),
        }
    }

    pub(crate) fn decode_avm_bytes(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::AVMBytes => Ok(ABIValue::Bytes(bytes.to_vec())),
            _ => Err(ABIError::DecodingError {
                message: "ABI type mismatch, expected AVMBytes".to_string(),
            }),
        }
    }

    pub(crate) fn encode_avm_string(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::AVMString => match value {
                ABIValue::String(s) => Ok(s.as_bytes().to_vec()),
                _ => Err(ABIError::EncodingError {
                    message: "ABI value mismatch, expected string for AVMString".to_string(),
                }),
            },
            _ => Err(ABIError::EncodingError {
                message: "ABI type mismatch, expected AVMString".to_string(),
            }),
        }
    }

    pub(crate) fn decode_avm_string(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::AVMString => {
                let s = String::from_utf8(bytes.to_vec()).map_err(|e| ABIError::DecodingError {
                    message: format!("Invalid UTF-8 string for AVMString: {}", e),
                })?;
                Ok(ABIValue::String(s))
            }
            _ => Err(ABIError::DecodingError {
                message: "ABI type mismatch, expected AVMString".to_string(),
            }),
        }
    }

    pub(crate) fn encode_avm_uint64(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::AVMUint64 => ABIType::from_str("uint64")?.encode(value),
            _ => Err(ABIError::EncodingError {
                message: "ABI type mismatch, expected AVMUint64".to_string(),
            }),
        }
    }

    pub(crate) fn decode_avm_uint64(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::AVMUint64 => ABIType::from_str("uint64")?.decode(bytes),
            _ => Err(ABIError::DecodingError {
                message: "ABI type mismatch, expected AVMUint64".to_string(),
            }),
        }
    }
}
