use std::collections::HashMap;

use crate::{
    ABIError, ABIType, ABIValue,
    constants::{BOOL_FALSE_BYTE, BOOL_TRUE_BYTE, LENGTH_ENCODE_BYTE_SIZE},
};

struct Segment {
    left: u16,
    right: u16,
}

impl ABIType {
    pub(crate) fn encode_tuple(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        let child_types = match self {
            ABIType::Tuple(child_types) => child_types.iter().collect::<Vec<_>>(),
            _ => {
                return Err(ABIError::EncodingError {
                    message: "ABI type mismatch, expected tuple".to_string(),
                });
            }
        };

        let values = match value {
            ABIValue::Array(n) => n,
            _ => {
                return Err(ABIError::EncodingError {
                    message: "ABI value mismatch, expected an array of values".to_string(),
                });
            }
        };

        encode_abi_types(&child_types, values)
    }

    pub(crate) fn decode_tuple(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        let child_types = match self {
            ABIType::Tuple(child_types) => child_types.iter().collect::<Vec<_>>(),
            _ => {
                return Err(ABIError::DecodingError {
                    message: "ABI type mismatch, expected tuple".to_string(),
                });
            }
        };

        decode_abi_types(&child_types, bytes)
    }
}

pub fn encode_abi_types(abi_types: &[&ABIType], values: &[ABIValue]) -> Result<Vec<u8>, ABIError> {
    if abi_types.len() != values.len() {
        return Err(ABIError::EncodingError {
            message: "Mismatch lengths between the values and types".to_string(),
        });
    }

    let mut heads: Vec<Vec<u8>> = Vec::new();
    let mut tails: Vec<Vec<u8>> = Vec::new();
    let mut is_dynamic_index: HashMap<usize, bool> = HashMap::new();

    let mut abi_types_cursor = 0;
    while abi_types_cursor < abi_types.len() {
        let child_type = &abi_types[abi_types_cursor];

        if child_type.is_dynamic() {
            // Head is not pre-determined for dynamic types; store a placeholder for now
            is_dynamic_index.insert(heads.len(), true);
            heads.push(vec![0, 0]);
            tails.push(child_type.encode(&values[abi_types_cursor])?);
        } else {
            match child_type {
                ABIType::Bool => {
                    let bool_sequence_end_index =
                        find_bool_sequence_end(abi_types, abi_types_cursor);
                    let bool_values = &values[abi_types_cursor..=bool_sequence_end_index];
                    heads.push(compress_bools(bool_values)?.to_be_bytes().to_vec());

                    abi_types_cursor = bool_sequence_end_index;
                }
                _ => {
                    heads.push(child_type.encode(&values[abi_types_cursor])?);
                }
            }
            is_dynamic_index.insert(abi_types_cursor, false);
            tails.push(vec![]);
        }

        abi_types_cursor += 1;
    }

    let head_length: usize = heads.iter().map(|e| e.len()).sum();
    let mut tail_length = 0;

    for i in 0..heads.len() {
        if let Some(true) = is_dynamic_index.get(&i) {
            let head_value = head_length + tail_length;
            let head_value: u16 =
                u16::try_from(head_value).map_err(|_| ABIError::EncodingError {
                    message: format!("Value {} cannot fit in u16", head_value),
                })?;
            heads[i] = head_value.to_be_bytes().to_vec();
        }
        tail_length += tails[i].len();
    }

    let results = heads.into_iter().chain(tails).flatten().collect();

    Ok(results)
}

pub fn decode_abi_types(abi_types: &[&ABIType], bytes: &[u8]) -> Result<ABIValue, ABIError> {
    let value_partitions = extract_values(abi_types, bytes)?;

    let mut values: Vec<ABIValue> = Vec::new();
    for i in 0..abi_types.len() {
        let child_type = abi_types[i];
        let value_partition = &value_partitions[i];
        let child_type_value = child_type.decode(value_partition)?;
        values.push(child_type_value);
    }

    Ok(ABIValue::Array(values))
}

fn compress_bools(values: &[ABIValue]) -> Result<u8, ABIError> {
    if values.len() > 8 {
        return Err(ABIError::EncodingError {
            message: format!(
                "Expected no more than 8 bool values, received {}",
                values.len()
            ),
        });
    }

    let mut result: u8 = 0;
    for (i, value) in values.iter().enumerate() {
        match value {
            ABIValue::Bool(b) => {
                if *b {
                    result |= 1 << (7 - i);
                }
            }
            _ => {
                return Err(ABIError::EncodingError {
                    message: "Expected all values to be ABIValue::Bool".to_string(),
                });
            }
        }
    }
    Ok(result)
}

fn extract_values(abi_types: &[&ABIType], bytes: &[u8]) -> Result<Vec<Vec<u8>>, ABIError> {
    let mut dynamic_segments: Vec<Segment> = Vec::new();
    let mut value_partitions: Vec<Option<Vec<u8>>> = Vec::new();
    let mut bytes_cursor: usize = 0;

    let mut abi_types_cursor = 0;
    while abi_types_cursor < abi_types.len() {
        let child_type = abi_types[abi_types_cursor];

        if child_type.is_dynamic() {
            if bytes[bytes_cursor..].len() < LENGTH_ENCODE_BYTE_SIZE {
                return Err(ABIError::DecodingError {
                    message: "Byte array is too short to be decoded".to_string(),
                });
            }

            let dynamic_index = u16::from_be_bytes([bytes[bytes_cursor], bytes[bytes_cursor + 1]]);
            if let Some(last_segment) = dynamic_segments.last_mut() {
                if dynamic_index < last_segment.left {
                    return Err(ABIError::DecodingError {
                        message:
                            "Dynamic index segment miscalculation: left is greater than right index"
                                .to_string(),
                    });
                }
                last_segment.right = dynamic_index;
            }

            dynamic_segments.push(Segment {
                left: dynamic_index,
                right: 0,
            });
            value_partitions.push(None);
            bytes_cursor += LENGTH_ENCODE_BYTE_SIZE;
        } else {
            match child_type {
                ABIType::Bool => {
                    let bool_sequence_end_index =
                        find_bool_sequence_end(abi_types, abi_types_cursor);

                    for j in 0..=(bool_sequence_end_index - abi_types_cursor) {
                        let bool_mask: u8 = BOOL_TRUE_BYTE >> j;
                        if bytes[bytes_cursor] & bool_mask > 0 {
                            value_partitions.push(Some(vec![BOOL_TRUE_BYTE]));
                        } else {
                            value_partitions.push(Some(vec![BOOL_FALSE_BYTE]));
                        }
                    }

                    abi_types_cursor = bool_sequence_end_index;
                    bytes_cursor += 1;
                }
                _ => {
                    let child_type_size = ABIType::get_size(child_type)?;
                    let slice = bytes
                        .get(bytes_cursor..bytes_cursor + child_type_size)
                        .ok_or_else(|| {
                            ABIError::DecodingError { message: format!(
                                "Index out of bounds: trying to access bytes[{}..{}] but slice has length {}",
                                bytes_cursor,
                                bytes_cursor + child_type_size,
                                bytes.len()
                            ) }
                        })?;

                    value_partitions.push(Some(slice.to_vec()));
                    bytes_cursor += child_type_size;
                }
            }
        }
        if abi_types_cursor != abi_types.len() - 1 && bytes_cursor >= bytes.len() {
            return Err(ABIError::DecodingError {
                message: "Input bytes not enough to decode".to_string(),
            });
        }
        abi_types_cursor += 1;
    }

    if let Some(last_segment) = dynamic_segments.last_mut() {
        let bytes_length = bytes.len();
        last_segment.right = u16::try_from(bytes_length).map_err(|_| ABIError::EncodingError {
            message: format!("Value {} cannot fit in u16", bytes_length),
        })?;
    } else if bytes_cursor < bytes.len() {
        return Err(ABIError::DecodingError {
            message: "Input bytes not fully consumed".to_string(),
        });
    }

    for i in 0..dynamic_segments.len() {
        let segment = &dynamic_segments[i];
        if segment.left > segment.right {
            return Err(ABIError::DecodingError {
                message: "Dynamic segment should display a [l, r] space with l <= r".to_string(),
            });
        }
        if i != dynamic_segments.len() - 1 && segment.right != dynamic_segments[i + 1].left {
            return Err(ABIError::DecodingError {
                message: "Dynamic segments should be consecutive".to_string(),
            });
        }
    }

    let mut segment_index: usize = 0;
    for i in 0..abi_types.len() {
        let child_type = &abi_types[i];
        if child_type.is_dynamic() {
            value_partitions[i] = Some(
                bytes[dynamic_segments[segment_index].left as usize
                    ..dynamic_segments[segment_index].right as usize]
                    .to_vec(),
            );
            segment_index += 1;
        }
    }

    // Check that all items in value_partitions are Some and convert to Vec<Vec<u8>>
    let value_partitions: Vec<Vec<u8>> = value_partitions
        .into_iter()
        .enumerate()
        .map(|(i, partition)| {
            partition.ok_or_else(|| ABIError::DecodingError {
                message: format!("Value partition at index {} is None", i),
            })
        })
        .collect::<Result<Vec<Vec<u8>>, ABIError>>()?;

    Ok(value_partitions)
}

/// Finds the end index of a consecutive bool sequence in an ABI type array.
///
/// Bool types in tuples are packed together for efficient encoding, with a maximum
/// of 8 consecutive bools allowed per sequence.
pub(crate) fn find_bool_sequence_end<T>(abi_types: &[T], current_index: usize) -> usize
where
    T: AsRef<ABIType>,
{
    let mut cursor: usize = current_index;
    loop {
        match abi_types[cursor].as_ref() {
            ABIType::Bool => {
                if cursor - current_index + 1 == 8 || cursor == abi_types.len() - 1 {
                    return cursor;
                }
                cursor += 1;
            }
            _ => {
                return cursor - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use crate::{ABIType, ABIValue, abi_type::BitSize};

    #[test]
    fn test_wrong_value_length() {
        let uint32_type1 = ABIType::Uint(BitSize::new(32).unwrap());
        let uint32_type2 = ABIType::Uint(BitSize::new(32).unwrap());
        let tuple_type = ABIType::Tuple(vec![uint32_type1, uint32_type2]);

        let value = ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u32))]);
        let result = tuple_type.encode(&value);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI encoding failed: Mismatch lengths between the values and types"
        );
    }

    #[test]
    fn test_decode_malformed_tuple_insufficient_bytes() {
        let uint32_type1 = ABIType::Uint(BitSize::new(32).unwrap());
        let uint32_type2 = ABIType::Uint(BitSize::new(32).unwrap());
        let tuple_type = ABIType::Tuple(vec![uint32_type1, uint32_type2]);
        let bytes = vec![0x00, 0x00, 0x00]; // Too few bytes for two uint32s
        let result = tuple_type.decode(&bytes);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Index out of bounds: trying to access bytes[0..4] but slice has length 3"
        );
    }

    #[test]
    fn test_decode_malformed_tuple_extra_bytes() {
        let uint8_type = ABIType::Uint(BitSize::new(8).unwrap());
        let tuple_type = ABIType::Tuple(vec![uint8_type]);
        let bytes = vec![0x01, 0x02, 0x03]; // Extra bytes after the uint8
        let result = tuple_type.decode(&bytes);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Input bytes not fully consumed")
        );
    }
}
