use crate::arc56_contract::{
    StructField as Arc56StructField, StructFieldType as Arc56StructFieldType,
};
use crate::{ABIError, ABIType, ABIValue};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an ABI struct type with named fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABIStruct {
    /// The name of the struct type
    pub name: String,
    /// The fields of the struct in order
    pub fields: Vec<StructField>,
}

/// Represents the type of a struct field
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructFieldType {
    Type(ABIType),
    Fields(Vec<StructField>),
}

/// Represents a field in a struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub field_type: StructFieldType,
}

impl ABIStruct {
    pub(crate) fn get_abi_struct_type(
        struct_name: &str,
        structs: &HashMap<String, Vec<Arc56StructField>>,
    ) -> Result<Self, ABIError> {
        let arc56_fields = structs
            .get(struct_name)
            .ok_or_else(|| ABIError::ValidationError {
                message: format!("Struct '{}' not found in ARC-56 definition", struct_name),
            })?;

        let mut fields = Vec::new();
        for arc56_field in arc56_fields {
            let field_type = Self::resolve_field_type(&arc56_field.field_type, structs)?;
            fields.push(StructField {
                name: arc56_field.name.clone(),
                field_type,
            });
        }

        Ok(Self {
            name: struct_name.to_string(),
            fields,
        })
    }

    fn resolve_field_type(
        field_type: &Arc56StructFieldType,
        structs: &HashMap<String, Vec<Arc56StructField>>,
    ) -> Result<StructFieldType, ABIError> {
        match field_type {
            Arc56StructFieldType::Value(type_str) => {
                // Check if this is a reference to another struct
                if structs.contains_key(type_str) {
                    let nested_struct = Self::get_abi_struct_type(type_str, structs)?;
                    Ok(StructFieldType::Type(ABIType::Struct(nested_struct)))
                } else {
                    // Parse as regular ABI type
                    let abi_type = ABIType::from_str(type_str)?;
                    Ok(StructFieldType::Type(abi_type))
                }
            }
            Arc56StructFieldType::Nested(nested_fields) => {
                // Handle anonymous nested struct fields
                let mut resolved_fields = Vec::new();
                for nested_field in nested_fields {
                    let field_type = Self::resolve_field_type(&nested_field.field_type, structs)?;
                    resolved_fields.push(StructField {
                        name: nested_field.name.clone(),
                        field_type,
                    });
                }
                Ok(StructFieldType::Fields(resolved_fields))
            }
        }
    }

    pub(crate) fn to_tuple_type(&self) -> ABIType {
        Self::fields_to_tuple_type(&self.fields)
    }

    fn fields_to_tuple_type(fields: &[StructField]) -> ABIType {
        let child_types: Vec<ABIType> = fields
            .iter()
            .map(|field| match &field.field_type {
                StructFieldType::Fields(nested_fields) => Self::fields_to_tuple_type(nested_fields),
                StructFieldType::Type(ABIType::Struct(struct_type)) => struct_type.to_tuple_type(),
                StructFieldType::Type(other_type) => other_type.clone(),
            })
            .collect();
        ABIType::Tuple(child_types)
    }

    /// Encode struct value using tuple encoding
    pub(crate) fn encode(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match value {
            ABIValue::Struct(value) => {
                let tuple_values = self.value_to_tuple_values(value)?;
                let tuple_type = self.to_tuple_type();
                tuple_type.encode(&ABIValue::Array(tuple_values))
            }
            _ => Err(ABIError::ValidationError {
                message: format!("Cannot encode non-struct value as struct '{}'", self.name),
            }),
        }
    }

    /// Decode bytes using tuple decoding
    pub(crate) fn decode(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        let tuple_type = self.to_tuple_type();
        let decoded_tuple = tuple_type.decode(bytes)?;

        match decoded_tuple {
            ABIValue::Array(tuple_values) => {
                let value = self.get_value_from_tuple_values(tuple_values)?;
                Ok(ABIValue::Struct(value))
            }
            _ => Err(ABIError::DecodingError {
                message: format!(
                    "Expected array from tuple decode for struct '{}'",
                    self.name
                ),
            }),
        }
    }

    /// Convert a struct value (HashMap) to a tuple value (Vec) for encoding
    fn value_to_tuple_values(
        &self,
        value: &HashMap<String, ABIValue>,
    ) -> Result<Vec<ABIValue>, ABIError> {
        Self::field_values_to_tuple_values(&self.fields, value, &self.name)
    }

    fn field_values_to_tuple_values(
        fields: &[StructField],
        struct_value: &HashMap<String, ABIValue>,
        struct_name: &str,
    ) -> Result<Vec<ABIValue>, ABIError> {
        fields
            .iter()
            .map(|field| {
                let value =
                    struct_value
                        .get(&field.name)
                        .ok_or_else(|| ABIError::ValidationError {
                            message: format!(
                                "Missing field '{}' in struct '{}'",
                                field.name, struct_name
                            ),
                        })?;

                match (&field.field_type, value) {
                    (
                        StructFieldType::Fields(nested_fields),
                        ABIValue::Struct(nested_struct_value),
                    ) => {
                        let nested_tuple_values = Self::field_values_to_tuple_values(
                            nested_fields,
                            nested_struct_value,
                            "anonymous",
                        )?;
                        Ok(ABIValue::Array(nested_tuple_values))
                    }
                    (
                        StructFieldType::Type(ABIType::Struct(nested_struct)),
                        ABIValue::Struct(nested_struct_value),
                    ) => {
                        let nested_tuple_values =
                            nested_struct.value_to_tuple_values(nested_struct_value)?;
                        Ok(ABIValue::Array(nested_tuple_values))
                    }
                    _ => Ok(value.clone()),
                }
            })
            .collect()
    }

    fn get_value_from_tuple_values(
        &self,
        tuple_values: Vec<ABIValue>,
    ) -> Result<HashMap<String, ABIValue>, ABIError> {
        if tuple_values.len() != self.fields.len() {
            return Err(ABIError::ValidationError {
                message: format!(
                    "Tuple length {} doesn't match struct '{}' field count {}",
                    tuple_values.len(),
                    self.name,
                    self.fields.len()
                ),
            });
        }

        Self::get_field_values(&self.fields, tuple_values)
    }

    fn get_field_values(
        fields: &[StructField],
        values: Vec<ABIValue>,
    ) -> Result<HashMap<String, ABIValue>, ABIError> {
        fields
            .iter()
            .zip(values)
            .map(|(field, value)| {
                let processed_value = match (&field.field_type, value) {
                    (StructFieldType::Fields(nested_fields), ABIValue::Array(nested_tuple)) => {
                        let nested_map = Self::get_field_values(nested_fields, nested_tuple)?;
                        ABIValue::Struct(nested_map)
                    }
                    (
                        StructFieldType::Type(ABIType::Struct(nested_struct)),
                        ABIValue::Array(nested_tuple),
                    ) => {
                        let nested_value =
                            nested_struct.get_value_from_tuple_values(nested_tuple)?;
                        ABIValue::Struct(nested_value)
                    }
                    (_, other_value) => other_value,
                };
                Ok((field.name.clone(), processed_value))
            })
            .collect()
    }
}

impl Display for ABIStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let tuple_type = self.to_tuple_type();
        write!(f, "{}", tuple_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi_type::BitSize;
    use std::collections::HashMap;

    #[test]
    fn test_struct_and_tuple_encode_decode_should_match() {
        // Create tuple type: (uint8,(uint16,string,string[]),(bool,byte),(byte,address))
        let tuple_type = ABIType::Tuple(vec![
            ABIType::Uint(BitSize::new(8).unwrap()),
            ABIType::Tuple(vec![
                ABIType::Uint(BitSize::new(16).unwrap()),
                ABIType::String,
                ABIType::DynamicArray(Box::new(ABIType::String)),
            ]),
            ABIType::Tuple(vec![ABIType::Bool, ABIType::Byte]),
            ABIType::Tuple(vec![ABIType::Byte, ABIType::Address]),
        ]);

        // Create nested struct type "Struct 2"
        let struct2 = ABIStruct {
            name: "Struct 2".to_string(),
            fields: vec![
                StructField {
                    name: "Struct 2 field 1".to_string(),
                    field_type: StructFieldType::Type(ABIType::Uint(BitSize::new(16).unwrap())),
                },
                StructField {
                    name: "Struct 2 field 2".to_string(),
                    field_type: StructFieldType::Type(ABIType::String),
                },
                StructField {
                    name: "Struct 2 field 3".to_string(),
                    field_type: StructFieldType::Type(ABIType::DynamicArray(Box::new(
                        ABIType::String,
                    ))),
                },
            ],
        };

        // Create main struct type "Struct 1"
        let struct_type = ABIStruct {
            name: "Struct 1".to_string(),
            fields: vec![
                StructField {
                    name: "field 1".to_string(),
                    field_type: StructFieldType::Type(ABIType::Uint(BitSize::new(8).unwrap())),
                },
                StructField {
                    name: "field 2".to_string(),
                    field_type: StructFieldType::Type(ABIType::Struct(struct2)),
                },
                StructField {
                    name: "field 3".to_string(),
                    field_type: StructFieldType::Fields(vec![
                        StructField {
                            name: "field 3 child 1".to_string(),
                            field_type: StructFieldType::Type(ABIType::Bool),
                        },
                        StructField {
                            name: "field 3 child 2".to_string(),
                            field_type: StructFieldType::Type(ABIType::Byte),
                        },
                    ]),
                },
                StructField {
                    name: "field 4".to_string(),
                    field_type: StructFieldType::Type(ABIType::Tuple(vec![
                        ABIType::Byte,
                        ABIType::Address,
                    ])),
                },
            ],
        };

        // Create tuple value: [123, [65432, 'hello', ['world 1', 'world 2', 'world 3']], [false, 88], [222, 'BEKKSMPBTPIGBYJGKD4XK7E7ZQJNZIHJVYFQWW3HNI32JHSH3LOGBRY3LE']]
        let tuple_value = ABIValue::Array(vec![
            ABIValue::Uint(123u8.into()),
            ABIValue::Array(vec![
                ABIValue::Uint(65432u16.into()),
                ABIValue::String("hello".to_string()),
                ABIValue::Array(vec![
                    ABIValue::String("world 1".to_string()),
                    ABIValue::String("world 2".to_string()),
                    ABIValue::String("world 3".to_string()),
                ]),
            ]),
            ABIValue::Array(vec![ABIValue::Bool(false), ABIValue::Byte(88)]),
            ABIValue::Array(vec![
                ABIValue::Byte(222),
                ABIValue::Address(
                    "BEKKSMPBTPIGBYJGKD4XK7E7ZQJNZIHJVYFQWW3HNI32JHSH3LOGBRY3LE".to_string(),
                ),
            ]),
        ]);

        // Create struct value
        let mut field3_value = HashMap::new();
        field3_value.insert("field 3 child 1".to_string(), ABIValue::Bool(false));
        field3_value.insert("field 3 child 2".to_string(), ABIValue::Byte(88));

        let mut field2_value = HashMap::new();
        field2_value.insert(
            "Struct 2 field 1".to_string(),
            ABIValue::Uint(65432u16.into()),
        );
        field2_value.insert(
            "Struct 2 field 2".to_string(),
            ABIValue::String("hello".to_string()),
        );
        field2_value.insert(
            "Struct 2 field 3".to_string(),
            ABIValue::Array(vec![
                ABIValue::String("world 1".to_string()),
                ABIValue::String("world 2".to_string()),
                ABIValue::String("world 3".to_string()),
            ]),
        );

        let mut struct_value_map = HashMap::new();
        struct_value_map.insert("field 1".to_string(), ABIValue::Uint(123u8.into()));
        struct_value_map.insert("field 2".to_string(), ABIValue::Struct(field2_value));
        struct_value_map.insert("field 3".to_string(), ABIValue::Struct(field3_value));
        struct_value_map.insert(
            "field 4".to_string(),
            ABIValue::Array(vec![
                ABIValue::Byte(222),
                ABIValue::Address(
                    "BEKKSMPBTPIGBYJGKD4XK7E7ZQJNZIHJVYFQWW3HNI32JHSH3LOGBRY3LE".to_string(),
                ),
            ]),
        );

        let struct_value = ABIValue::Struct(struct_value_map);

        // Test encoding - tuple and struct should produce same bytes
        let encoded_tuple = tuple_type.encode(&tuple_value).unwrap();
        let encoded_struct = struct_type.encode(&struct_value).unwrap();
        assert_eq!(encoded_tuple, encoded_struct);

        // Test decoding tuple
        let decoded_tuple = tuple_type.decode(&encoded_tuple).unwrap();
        assert_eq!(decoded_tuple, tuple_value);

        // Test decoding struct from tuple encoding
        let decoded_struct = struct_type.decode(&encoded_tuple).unwrap();
        assert_eq!(decoded_struct, struct_value);

        // Verify struct to tuple type conversion matches expected tuple type
        let converted_tuple_type = struct_type.to_tuple_type();
        assert_eq!(converted_tuple_type, tuple_type);
    }
}
