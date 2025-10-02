use std::{collections::HashMap, sync::Arc};

use algokit_abi::ABIValue as RustABIValue;
use num_bigint::BigUint;

use crate::transactions::common::UtilsError;

// NOTE: Once we get a release that enables custom types with Python (it's on main), we can use them to provide a better ux: https://github.com/mozilla/uniffi-rs/issues/2652#issuecomment-3307297845

#[derive(uniffi::Object, Debug, Clone, PartialEq)]
#[uniffi::export(Eq)]
pub struct ABIValue {
    pub rust_value: RustABIValue,
}

#[uniffi::export]
impl ABIValue {
    #[uniffi::constructor]
    pub fn bool(value: bool) -> Self {
        ABIValue {
            rust_value: RustABIValue::Bool(value),
        }
    }

    pub fn get_bool(&self) -> Result<bool, UtilsError> {
        if let RustABIValue::Bool(b) = &self.rust_value {
            Ok(*b)
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a bool".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn array(values: Vec<Arc<ABIValue>>) -> Self {
        ABIValue {
            rust_value: RustABIValue::Array(
                values.into_iter().map(|v| v.rust_value.clone()).collect(),
            ),
        }
    }

    pub fn get_array(&self) -> Result<Vec<Arc<ABIValue>>, UtilsError> {
        if let RustABIValue::Array(arr) = &self.rust_value {
            Ok(arr
                .iter()
                .cloned()
                .map(|v| Arc::new(ABIValue { rust_value: v }))
                .collect())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not an array".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn uint(value: u64) -> Self {
        ABIValue {
            rust_value: RustABIValue::Uint(num_bigint::BigUint::from(value)),
        }
    }

    pub fn get_uint(&self) -> Result<u64, UtilsError> {
        if let RustABIValue::Uint(u) = &self.rust_value {
            let digits = u.to_u64_digits();
            if digits.len() == 1 {
                Ok(digits[0])
            } else if digits.is_empty() {
                Ok(0)
            } else {
                Err(UtilsError::UtilsError {
                    message: "ABI uint value is too large to fit in u64".to_string(),
                })
            }
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a uint".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn biguint(value: Vec<u8>) -> Self {
        ABIValue {
            rust_value: RustABIValue::Uint(BigUint::from_bytes_be(&value)),
        }
    }

    pub fn get_big_uint(&self) -> Result<Vec<u8>, UtilsError> {
        if let RustABIValue::Uint(u) = &self.rust_value {
            Ok(u.to_bytes_be())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a uint".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn string(value: String) -> Self {
        ABIValue {
            rust_value: RustABIValue::String(value),
        }
    }

    pub fn get_string(&self) -> Result<String, UtilsError> {
        if let RustABIValue::String(s) = &self.rust_value {
            Ok(s.clone())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a string".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn byte(value: u8) -> Self {
        ABIValue {
            rust_value: RustABIValue::Byte(value),
        }
    }

    pub fn get_byte(&self) -> Result<u8, UtilsError> {
        if let RustABIValue::Byte(b) = &self.rust_value {
            Ok(*b)
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a byte".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn address(value: String) -> Self {
        ABIValue {
            rust_value: RustABIValue::Address(value),
        }
    }

    pub fn get_address(&self) -> Result<String, UtilsError> {
        if let RustABIValue::Address(a) = &self.rust_value {
            Ok(a.clone())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not an address".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn bytes(value: Vec<u8>) -> Self {
        ABIValue {
            rust_value: RustABIValue::Bytes(value),
        }
    }

    pub fn get_bytes(&self) -> Result<Vec<u8>, UtilsError> {
        if let RustABIValue::Bytes(b) = &self.rust_value {
            Ok(b.clone())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not bytes".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn struct_fields(fields: HashMap<String, Arc<ABIValue>>) -> Self {
        ABIValue {
            rust_value: RustABIValue::Struct(
                fields
                    .into_iter()
                    .map(|(k, v)| (k, v.rust_value.clone()))
                    .collect(),
            ),
        }
    }

    pub fn get_struct_fields(&self) -> Result<HashMap<String, Arc<ABIValue>>, UtilsError> {
        if let RustABIValue::Struct(map) = &self.rust_value {
            Ok(map
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        Arc::new(ABIValue {
                            rust_value: v.clone(),
                        }),
                    )
                })
                .collect())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a struct".to_string(),
            })
        }
    }
}
