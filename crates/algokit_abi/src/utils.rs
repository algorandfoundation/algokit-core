use num_bigint::BigUint;

pub fn big_uint_to_bytes(value: &BigUint, len: usize) -> Vec<u8> {
    let bytes = &value.to_bytes_be();
    let mut result = vec![0u8; len - bytes.len()];
    result.extend_from_slice(bytes);
    result
}
