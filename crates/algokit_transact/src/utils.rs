use crate::constants::{
    ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH, Byte32, HASH_BYTES_LENGTH,
};
use crate::traits::MsgPackEmpty;
use crate::{
    Address, AlgoKitTransactError, AlgorandMsgpack, MAX_TX_GROUP_SIZE, Transaction, TransactionId,
};
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as, skip_serializing_none};
use sha2::{Digest, Sha512_256};

pub fn sort_msgpack_value(value: rmpv::Value) -> Result<rmpv::Value, AlgoKitTransactError> {
    match value {
        rmpv::Value::Map(m) => {
            if m.is_empty() {
                return Ok(rmpv::Value::Map(Vec::new()));
            }

            // Categorize keys into integers, strings, and binary. Any other key types are unsupported.
            let mut int_entries: Vec<(rmpv::Integer, rmpv::Value)> = Vec::new();
            let mut string_entries: Vec<(String, rmpv::Value)> = Vec::new();
            let mut binary_entries: Vec<(Vec<u8>, rmpv::Value)> = Vec::new();

            for (k, v) in m {
                let sorted_v = sort_msgpack_value(v)?;
                match k {
                    rmpv::Value::Integer(int_key) => int_entries.push((int_key, sorted_v)),
                    rmpv::Value::String(key) => {
                        let s = key.as_str().unwrap_or("").to_string();
                        string_entries.push((s, sorted_v));
                    }
                    rmpv::Value::Binary(bytes) => binary_entries.push((bytes, sorted_v)),
                    _ => {
                        return Err(AlgoKitTransactError::InputError {
                            message: "Unsupported MessagePack map key type; only integer, string, and binary keys are supported".to_string(),
                        })
                    }
                }
            }

            // Sort each category
            int_entries.sort_by(|(a, _), (b, _)| {
                let a_val: i128 = if let Some(i) = a.as_i64() {
                    i as i128
                } else if let Some(u) = a.as_u64() {
                    u as i128
                } else {
                    0
                };
                let b_val: i128 = if let Some(i) = b.as_i64() {
                    i as i128
                } else if let Some(u) = b.as_u64() {
                    u as i128
                } else {
                    0
                };
                a_val.cmp(&b_val)
            });

            string_entries.sort_by(|(a, _), (b, _)| a.cmp(b));

            binary_entries.sort_by(|(a, _), (b, _)| a.cmp(b));

            // Concatenate in canonical category order: integers, then strings, then binary
            let mut result: Vec<(rmpv::Value, rmpv::Value)> =
                Vec::with_capacity(int_entries.len() + string_entries.len() + binary_entries.len());

            result.extend(
                int_entries
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::Integer(k), v)),
            );
            result.extend(
                string_entries
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::String(k.into()), v)),
            );
            result.extend(
                binary_entries
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::Binary(k), v)),
            );

            Ok(rmpv::Value::Map(result))
        }
        rmpv::Value::Array(arr) => {
            let result: Result<Vec<rmpv::Value>, AlgoKitTransactError> =
                arr.into_iter().map(sort_msgpack_value).collect();
            Ok(rmpv::Value::Array(result?))
        }
        // For all other types, return as-is
        v => Ok(v),
    }
}

#[cfg(test)]
mod tests {
    use super::sort_msgpack_value;
    use crate::AlgoKitTransactError;
    use base64::{Engine, prelude::BASE64_STANDARD};
    use rmpv::Value;

    fn map(entries: Vec<(Value, Value)>) -> Value {
        Value::Map(entries)
    }

    #[test]
    fn sort_integers_strings_binary_ordering() -> Result<(), AlgoKitTransactError> {
        let value = map(vec![
            (Value::String("b".into()), Value::from(2)),
            (Value::String("a".into()), Value::from(1)),
            (Value::Integer(10i64.into()), Value::from(3)),
            (Value::Integer(2i64.into()), Value::from(4)),
            (Value::Binary(vec![2, 0, 0]), Value::from(5)),
            (Value::Binary(vec![1, 0, 0]), Value::from(6)),
        ]);

        let sorted = sort_msgpack_value(value)?;
        let arr = match sorted {
            Value::Map(m) => m,
            _ => unreachable!(),
        };

        // integers come first and are ascending
        let (k0, k1) = (&arr[0].0, &arr[1].0);
        assert!(matches!(k0, Value::Integer(_)) && matches!(k1, Value::Integer(_)));
        let i0 = if let Value::Integer(i) = k0 {
            i.as_i64().unwrap()
        } else {
            0
        };
        let i1 = if let Value::Integer(i) = k1 {
            i.as_i64().unwrap()
        } else {
            0
        };
        assert_eq!((i0, i1), (2, 10));

        // then strings in lexicographic order
        let (k2, k3) = (&arr[2].0, &arr[3].0);
        let s2 = if let Value::String(s) = k2 {
            s.as_str().unwrap()
        } else {
            ""
        };
        let s3 = if let Value::String(s) = k3 {
            s.as_str().unwrap()
        } else {
            ""
        };
        assert_eq!((s2, s3), ("a", "b"));

        // then binary keys ascending
        let (k4, k5) = (&arr[4].0, &arr[5].0);
        let b4 = if let Value::Binary(b) = k4 {
            b.clone()
        } else {
            vec![]
        };
        let b5 = if let Value::Binary(b) = k5 {
            b.clone()
        } else {
            vec![]
        };
        assert!(b4 < b5);
        Ok(())
    }

    #[test]
    fn nested_maps_sorted_recursively() -> Result<(), AlgoKitTransactError> {
        let inner = map(vec![
            (Value::String("z".into()), Value::from(1)),
            (Value::String("a".into()), Value::from(2)),
        ]);
        let outer = map(vec![(Value::Integer(1i64.into()), inner)]);

        let sorted = sort_msgpack_value(outer)?;
        if let Value::Map(m) = sorted {
            if let Value::Map(inner_sorted) = &m[0].1 {
                let first_key = if let Value::String(s) = &inner_sorted[0].0 {
                    s.as_str().unwrap()
                } else {
                    ""
                };
                assert_eq!(first_key, "a");
            }
        }
        Ok(())
    }

    #[test]
    fn fixture_global_state_delta_binary_keys_sorted() -> Result<(), AlgoKitTransactError> {
        let json = algokit_test_artifacts::msgpack::TESTNET_GLOBAL_STATE_DELTA_TX;
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        let entries = v["transaction"]["global-state-delta"].as_array().unwrap();
        let mut m = Vec::new();
        for e in entries {
            let k_b64 = e["key"].as_str().unwrap();
            let key_bytes = BASE64_STANDARD.decode(k_b64).unwrap();
            m.push((Value::Binary(key_bytes), Value::from(0)));
        }
        let sorted = sort_msgpack_value(Value::Map(m))?;
        if let Value::Map(arr) = sorted {
            let keys: Vec<Vec<u8>> = arr
                .iter()
                .map(|(k, _)| {
                    if let Value::Binary(b) = k {
                        b.clone()
                    } else {
                        vec![]
                    }
                })
                .collect();
            let mut sorted_keys = keys.clone();
            sorted_keys.sort();
            assert_eq!(keys, sorted_keys);
        }
        Ok(())
    }
}

pub fn is_zero<T>(n: &T) -> bool
where
    T: PartialEq + From<u8>,
{
    *n == T::from(0u8)
}

pub fn is_zero_opt<T>(n: &Option<T>) -> bool
where
    T: PartialEq + From<u8>,
{
    n.as_ref().is_none_or(is_zero)
}

pub fn is_zero_addr(addr: &Address) -> bool {
    addr.as_bytes() == &[0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
}

pub fn is_zero_addr_opt(addr: &Option<Address>) -> bool {
    addr.as_ref().is_none_or(is_zero_addr)
}

pub fn is_empty_bytes32(bytes: &Byte32) -> bool {
    bytes == &[0u8; 32]
}

pub fn is_empty_bytes32_opt(bytes: &Option<Byte32>) -> bool {
    bytes.as_ref().is_none_or(is_empty_bytes32)
}

pub fn is_empty_string_opt(string: &Option<String>) -> bool {
    string.as_ref().is_none_or(String::is_empty)
}

pub fn is_empty_vec_opt<T>(vec: &Option<Vec<T>>) -> bool {
    vec.as_ref().is_none_or(Vec::is_empty)
}

pub fn is_empty_struct_opt<T>(val: &Option<T>) -> bool
where
    T: MsgPackEmpty,
{
    val.as_ref().is_none_or(|v| v.is_empty())
}

pub fn pub_key_to_checksum(pub_key: &Byte32) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
    let mut hasher = Sha512_256::new();
    hasher.update(pub_key);

    let mut checksum = [0u8; ALGORAND_CHECKSUM_BYTE_LENGTH];
    checksum
        .copy_from_slice(&hasher.finalize()[(HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH)..]);
    checksum
}

pub fn hash(bytes: &Vec<u8>) -> Byte32 {
    let mut hasher = Sha512_256::new();
    hasher.update(bytes);

    let mut hash_bytes = [0u8; HASH_BYTES_LENGTH];
    hash_bytes.copy_from_slice(&hasher.finalize()[..HASH_BYTES_LENGTH]);
    hash_bytes
}

pub fn compute_group(txs: &[Transaction]) -> Result<Byte32, AlgoKitTransactError> {
    if txs.is_empty() {
        return Err(AlgoKitTransactError::InputError {
            message: String::from("Transaction group size cannot be 0"),
        });
    }

    if txs.len() > MAX_TX_GROUP_SIZE {
        return Err(AlgoKitTransactError::InputError {
            message: format!(
                "Transaction group size exceeds the max limit of {}",
                MAX_TX_GROUP_SIZE
            ),
        });
    }

    let tx_hashes: Result<Vec<Byte32>, AlgoKitTransactError> = txs
        .iter()
        .map(|tx| {
            if tx.header().group.is_some() {
                return Err(AlgoKitTransactError::InputError {
                    message: "Transactions must not already be grouped".to_string(),
                });
            }
            tx.id_raw()
        })
        .collect();
    let grouped = (GroupedTransactions {
        tx_hashes: tx_hashes?,
    })
    .encode()
    .unwrap();

    Ok(hash(&grouped))
}

pub fn is_false_opt(bool: &Option<bool>) -> bool {
    bool.as_ref().is_none_or(|b| !b)
}

// This struct is only used internally for generating the group id
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct GroupedTransactions {
    #[serde(rename = "txlist")]
    #[serde_as(as = "Vec<Bytes>")]
    pub tx_hashes: Vec<Byte32>,
}

impl AlgorandMsgpack for GroupedTransactions {
    const PREFIX: &'static [u8] = b"TG";
}
