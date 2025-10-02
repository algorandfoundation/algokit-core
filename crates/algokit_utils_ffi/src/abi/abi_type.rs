use std::{str::FromStr, sync::Arc};

use algokit_abi::ABIType as RustABIType;
use algokit_abi::types::r#struct::ABIStruct as RustABIStruct;
use algokit_abi::types::r#struct::StructField as RustStructField;
use algokit_abi::types::r#struct::StructFieldType as RustStructFieldType;

use crate::transactions::common::UtilsError;

#[derive(Debug, Clone, uniffi::Enum)]
pub enum StructFieldType {
    Type(Arc<ABIType>),
    Fields(Vec<Arc<StructField>>),
}

impl From<RustStructFieldType> for StructFieldType {
    fn from(value: RustStructFieldType) -> Self {
        match value {
            RustStructFieldType::Type(t) => {
                StructFieldType::Type(Arc::new(ABIType { abi_type: t }))
            }
            RustStructFieldType::Fields(f) => StructFieldType::Fields(
                f.into_iter()
                    .map(|sf| {
                        Arc::new(StructField {
                            name: sf.name,
                            field_type: sf.field_type.into(),
                        })
                    })
                    .collect(),
            ),
        }
    }
}

impl From<StructFieldType> for RustStructFieldType {
    fn from(value: StructFieldType) -> Self {
        match value {
            StructFieldType::Type(t) => RustStructFieldType::Type(t.abi_type.clone()),
            StructFieldType::Fields(f) => RustStructFieldType::Fields(
                f.iter()
                    .map(|sf| RustStructField {
                        name: sf.name.clone(),
                        field_type: sf.field_type.clone().into(),
                    })
                    .collect(),
            ),
        }
    }
}

/// Represents a field in a struct
#[derive(Debug, uniffi::Object)]
pub struct StructField {
    pub name: String,
    pub field_type: StructFieldType,
}

#[uniffi::export]
impl StructField {
    #[uniffi::constructor]
    pub fn new(name: String, field_type: StructFieldType) -> Self {
        StructField { name, field_type }
    }
}

impl From<RustStructField> for StructField {
    fn from(value: RustStructField) -> Self {
        StructField {
            name: value.name,
            field_type: value.field_type.into(),
        }
    }
}

impl From<&StructField> for RustStructField {
    fn from(value: &StructField) -> Self {
        RustStructField {
            name: value.name.clone(),
            field_type: value.field_type.clone().into(),
        }
    }
}

#[derive(uniffi::Object)]
pub struct ABIStruct {
    /// The name of the struct type
    pub name: String,
    /// The fields of the struct in order
    pub fields: Vec<StructField>,
}

impl From<RustABIStruct> for ABIStruct {
    fn from(value: RustABIStruct) -> Self {
        ABIStruct {
            name: value.name,
            fields: value.fields.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl From<ABIStruct> for RustABIStruct {
    fn from(value: ABIStruct) -> Self {
        RustABIStruct {
            name: value.name,
            fields: value.fields.into_iter().map(|f| (&f).into()).collect(),
        }
    }
}

#[derive(uniffi::Object, Debug)]
pub struct ABIType {
    pub abi_type: RustABIType,
}

#[uniffi::export]
impl ABIType {
    #[uniffi::constructor]
    pub fn from_string(type_str: &str) -> Result<Self, UtilsError> {
        RustABIType::from_str(type_str)
            .map(|abi_type| ABIType { abi_type })
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.abi_type.to_string()
    }

    pub fn encode(&self, value: &crate::abi::abi_value::ABIValue) -> Result<Vec<u8>, UtilsError> {
        self.abi_type
            .encode(&value.rust_value)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn decode(&self, data: &[u8]) -> Result<crate::abi::abi_value::ABIValue, UtilsError> {
        self.abi_type
            .decode(data)
            .map(|v| crate::abi::abi_value::ABIValue { rust_value: v })
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    #[uniffi::constructor]
    pub fn uint(bit_size: u16) -> Result<Self, UtilsError> {
        let abi_type =
            RustABIType::Uint(algokit_abi::abi_type::BitSize::new(bit_size).map_err(|_| {
                UtilsError::UtilsError {
                    message: format!("Invalid bit size: {}", bit_size),
                }
            })?);

        Ok(ABIType { abi_type })
    }

    #[uniffi::constructor]
    pub fn ufixed(bit_size: u16, precision: u8) -> Result<Self, UtilsError> {
        let abi_type = RustABIType::UFixed(
            algokit_abi::abi_type::BitSize::new(bit_size).map_err(|_| UtilsError::UtilsError {
                message: format!("Invalid bit size: {}", bit_size),
            })?,
            algokit_abi::abi_type::Precision::new(precision).map_err(|_| {
                UtilsError::UtilsError {
                    message: format!("Invalid precision: {}", precision),
                }
            })?,
        );

        Ok(ABIType { abi_type })
    }

    #[uniffi::constructor]
    pub fn address() -> Self {
        ABIType {
            abi_type: RustABIType::Address,
        }
    }

    #[uniffi::constructor]
    pub fn tuple(elements: Vec<Arc<ABIType>>) -> Self {
        let rust_elements = elements.into_iter().map(|e| e.abi_type.clone()).collect();
        ABIType {
            abi_type: RustABIType::Tuple(rust_elements),
        }
    }

    #[uniffi::constructor]
    pub fn string() -> Self {
        ABIType {
            abi_type: RustABIType::String,
        }
    }

    #[uniffi::constructor]
    pub fn byte() -> Self {
        ABIType {
            abi_type: RustABIType::Byte,
        }
    }

    #[uniffi::constructor]
    pub fn bool() -> Self {
        ABIType {
            abi_type: RustABIType::Bool,
        }
    }

    #[uniffi::constructor]
    pub fn static_array(element_type: Arc<ABIType>, length: u16) -> Result<Self, UtilsError> {
        let abi_type =
            RustABIType::StaticArray(Box::new(element_type.abi_type.clone()), length as usize);
        Ok(ABIType { abi_type })
    }

    #[uniffi::constructor]
    pub fn dynamic_array(element_type: Arc<ABIType>) -> Self {
        let abi_type = RustABIType::DynamicArray(Box::new(element_type.abi_type.clone()));
        ABIType { abi_type }
    }

    #[uniffi::constructor]
    pub fn struct_fields(name: String, fields: Vec<Arc<StructField>>) -> Self {
        let rust_fields = fields.into_iter().map(|f| f.as_ref().into()).collect();
        let abi_type = RustABIType::Struct(RustABIStruct {
            name,
            fields: rust_fields,
        });
        ABIType { abi_type }
    }

    #[uniffi::constructor]
    pub fn avm_bytes() -> Self {
        ABIType {
            abi_type: RustABIType::AVMBytes,
        }
    }

    #[uniffi::constructor]
    pub fn avm_string() -> Self {
        ABIType {
            abi_type: RustABIType::AVMString,
        }
    }

    #[uniffi::constructor]
    pub fn avm_uint64() -> Self {
        ABIType {
            abi_type: RustABIType::AVMUint64,
        }
    }
}
