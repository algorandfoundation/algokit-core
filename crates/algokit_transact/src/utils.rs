use crate::address::Address;
use crate::constants::{
    Byte32, ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH, HASH_BYTES_LENGTH,
};
use crate::{AlgoKitTransactError, AlgorandMsgpack, Transaction, TransactionId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, Bytes};
use sha2::{Digest, Sha512_256};
use std::collections::BTreeMap;

pub fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
    match value {
        rmpv::Value::Map(m) => {
            let mut sorted_map: BTreeMap<String, rmpv::Value> = BTreeMap::new();

            // Convert and sort all key-value pairs
            for (k, v) in m {
                if let rmpv::Value::String(key) = k {
                    let key_str = key.into_str().unwrap_or_default();
                    sorted_map.insert(key_str, sort_msgpack_value(v));
                }
            }

            // Convert back to rmpv::Value::Map
            rmpv::Value::Map(
                sorted_map
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::String(k.into()), v))
                    .collect(),
            )
        }
        rmpv::Value::Array(arr) => {
            rmpv::Value::Array(arr.into_iter().map(sort_msgpack_value).collect())
        }
        // For all other types, return as-is
        v => v,
    }
}

pub fn is_zero(n: &u64) -> bool {
    *n == 0u64
}

pub fn is_zero_opt(n: &Option<u64>) -> bool {
    n.as_ref().is_none_or(is_zero)
}

pub fn is_zero_addr(addr: &Address) -> bool {
    addr.pub_key == [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
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

    let mut group = [0u8; HASH_BYTES_LENGTH];
    group.copy_from_slice(&hasher.finalize()[..HASH_BYTES_LENGTH]);
    group
}

pub fn compute_group_id(txs: &[Transaction]) -> Result<Byte32, AlgoKitTransactError> {
    let tx_hashes: Result<Vec<Byte32>, AlgoKitTransactError> = txs
        .iter()
        .map(|tx| {
            if tx.header().group.is_some() {
                return Err(AlgoKitTransactError::InputError(
                    "Transactions must not already be grouped".to_string(),
                ));
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
