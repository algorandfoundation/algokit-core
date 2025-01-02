use serde::{Deserialize, Serialize};
// Primary using serde_with for Bytes, which is better than serde_bytes
// see https://docs.rs/serde_with/latest/serde_with/struct.Bytes.html
// It also has some other nice QOL features, like skip_serializing_none
use serde_with::{serde_as, skip_serializing_none, Bytes};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MsgPackError {
    #[error("Error ocurred during encoding: {0}")]
    EncodeError(#[from] rmp_serde::encode::Error),

    #[error("Error ocurred during decoding: {0}")]
    DecodeError(#[from] rmp_serde::decode::Error),

    #[error("Error ocurred during encoding: {0}")]
    RmpvEncodeError(#[from] rmpv::encode::Error),

    #[error("Error ocurred during encoding: {0}")]
    RmpvDecodeError(#[from] rmpv::decode::Error),

    #[error("Unknown transaction type")]
    UnknownTransactionType,

    #[error("Invalid input: {0}")]
    InputError(String),
}

pub trait AlgorandMsgpack: Serialize + for<'de> Deserialize<'de> {
    /// msgpack encoding of the transaction with keys sorted and empty fields omitted
    /// Use `encode_raw` for encoding transactions for signing
    fn encode_raw(&self) -> Result<Vec<u8>, MsgPackError> {
        // First serialize to a temporary buffer to get the map entries
        let mut temp_buf = Vec::new();
        let mut temp_serializer = rmp_serde::Serializer::new(&mut temp_buf)
            .with_struct_map()
            .with_bytes(rmp_serde::config::BytesMode::ForceAll);

        self.serialize(&mut temp_serializer)?;

        // Deserialize into a Value and sort recursively
        let value = rmpv::decode::read_value(&mut temp_buf.as_slice())?;
        let sorted_value = sort_msgpack_value(value);

        // Serialize the sorted value
        let mut final_buf = Vec::new();
        rmpv::encode::write_value(&mut final_buf, &sorted_value)?;

        Ok(final_buf)
    }

    /// Decode the bytes into Self. "TX" prefix is ignored if present
    fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
        if bytes.is_empty() {
            return Err(MsgPackError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        if bytes.len() > 2 && bytes[0] == b'T' && bytes[1] == b'X' {
            let without_prefix = bytes[2..].to_vec();
            Ok(rmp_serde::from_slice(&without_prefix)?)
        } else {
            Ok(rmp_serde::from_slice(bytes)?)
        }
    }

    /// Default implementation for encoding a transaction with the prefix "TX"
    fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        let encoded = self.encode_raw()?;
        let mut buf = Vec::with_capacity(encoded.len() + 2);
        buf.extend_from_slice(b"TX");
        buf.extend_from_slice(&encoded);
        Ok(buf)
    }
}

fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionType {
    #[serde(rename = "pay")]
    Payment,

    #[serde(rename = "axfer")]
    AssetTransfer,

    #[serde(rename = "afrz")]
    AssetFreeze,

    #[serde(rename = "acfg")]
    AssetConfig,

    #[serde(rename = "keyreg")]
    KeyRegistration,

    #[serde(rename = "appl")]
    ApplicationCall,
}

type Byte32 = [u8; 32];
type Pubkey = Byte32;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionHeader {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    #[serde(rename = "snd")]
    #[serde_as(as = "Bytes")]
    pub sender: Pubkey,

    pub fee: u64,

    #[serde(rename = "fv")]
    pub first_valid: u64,

    #[serde(rename = "lv")]
    pub last_valid: u64,

    #[serde(rename = "gh")]
    #[serde_as(as = "Option<Bytes>")]
    pub genesis_hash: Option<Byte32>,

    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,

    #[serde_as(as = "Option<Bytes>")]
    pub note: Option<Vec<u8>>,

    #[serde(rename = "rekey")]
    #[serde_as(as = "Option<Bytes>")]
    pub rekey_to: Option<Pubkey>,

    #[serde(rename = "lx")]
    #[serde_as(as = "Option<Bytes>")]
    pub lease: Option<Byte32>,

    #[serde(rename = "grp")]
    #[serde_as(as = "Option<Bytes>")]
    pub group: Option<Byte32>,
}

impl AlgorandMsgpack for TransactionHeader {}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PayTransactionFields {
    #[serde(flatten)]
    pub header: TransactionHeader,

    #[serde(rename = "rcv")]
    #[serde_as(as = "Bytes")]
    pub receiver: Pubkey,

    #[serde(rename = "amt")]
    pub amount: u64,

    #[serde(rename = "close")]
    pub close_remainder_to: Option<Pubkey>,
}

impl AlgorandMsgpack for PayTransactionFields {}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AssetTransferTransactionFields {
    #[serde(flatten)]
    pub header: TransactionHeader,

    #[serde(rename = "xaid")]
    pub asset_id: u64,

    #[serde(rename = "aamt")]
    pub amount: u64,

    #[serde(rename = "arcv")]
    #[serde_as(as = "Bytes")]
    pub receiver: Pubkey,

    #[serde(rename = "asnd")]
    #[serde_as(as = "Option<Bytes>")]
    pub asset_sender: Option<Pubkey>,

    #[serde(rename = "aclose")]
    #[serde_as(as = "Option<Bytes>")]
    pub close_remainder_to: Option<Pubkey>,
}

impl AlgorandMsgpack for AssetTransferTransactionFields {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Transaction {
    Payment(PayTransactionFields),
    AssetTransfer(AssetTransferTransactionFields),
}

impl AlgorandMsgpack for Transaction {}

impl Transaction {
    pub fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        match self {
            Transaction::Payment(tx) => tx.encode_raw(),
            Transaction::AssetTransfer(tx) => tx.encode_raw(),
        }
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
        let header = TransactionHeader::decode(bytes)?;
        match header.transaction_type {
            TransactionType::Payment => {
                Ok(Transaction::Payment(PayTransactionFields::decode(bytes)?))
            }
            TransactionType::AssetTransfer => Ok(Transaction::AssetTransfer(
                AssetTransferTransactionFields::decode(bytes)?,
            )),
            _ => Err(MsgPackError::UnknownTransactionType),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignedTransaction {
    #[serde(rename = "txn")]
    pub transaction: Transaction,

    #[serde(rename = "sig")]
    #[serde_as(as = "Bytes")]
    pub signature: [u8; 64],
}

impl AlgorandMsgpack for SignedTransaction {
    fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        self.encode_raw()
    }
}

#[test]
fn test_pay_transaction() {
    let tx_struct = PayTransactionFields {
        header: TransactionHeader {
            genesis_id: None,
            transaction_type: TransactionType::Payment,
            sender: [0; 32],
            fee: 1000,
            first_valid: 1000,
            last_valid: 1000,
            genesis_hash: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        receiver: [0; 32],
        amount: 1000,
        close_remainder_to: None,
    };

    let encoded_struct = tx_struct.encode_raw().unwrap();
    let decoded_struct = PayTransactionFields::decode(&encoded_struct).unwrap();
    assert_eq!(decoded_struct, tx_struct);

    let tx_enum = Transaction::Payment(tx_struct.clone());
    let encoded_enum = tx_enum.encode().unwrap();
    let decoded_enum = Transaction::decode(&encoded_enum).unwrap();
    assert_eq!(decoded_enum, tx_enum);
    assert_eq!(decoded_enum, Transaction::Payment(tx_struct.clone()));

    let signed_tx = SignedTransaction {
        transaction: tx_enum.clone(),
        signature: [0; 64],
    };
    let encoded_stx = signed_tx.encode_raw().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, tx_enum);

    let prefix_encoded = tx_struct.encode().unwrap();
    assert_eq!(prefix_encoded[0], b'T');
    assert_eq!(prefix_encoded[1], b'X');
    assert_eq!(prefix_encoded.len(), encoded_struct.len() + 2);
    assert_eq!(prefix_encoded[2..], encoded_struct);
    assert_eq!(encoded_struct.len(), 112);
}

#[test]
fn test_asset_transfer_transaction() {
    let tx_struct = AssetTransferTransactionFields {
        header: TransactionHeader {
            genesis_id: None,
            transaction_type: TransactionType::AssetTransfer,
            sender: [0; 32],
            fee: 1000,
            first_valid: 1000,
            last_valid: 1000,
            genesis_hash: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        asset_id: 1,
        amount: 1000,
        receiver: [0; 32],
        asset_sender: None,
        close_remainder_to: None,
    };

    let encoded_struct = tx_struct.encode_raw().unwrap();
    let decoded_struct = AssetTransferTransactionFields::decode(&encoded_struct).unwrap();
    assert_eq!(decoded_struct, tx_struct);

    let tx_enum = Transaction::AssetTransfer(tx_struct.clone());
    let encoded_enum = tx_enum.encode().unwrap();
    let decoded_enum = Transaction::decode(&encoded_enum).unwrap();
    assert_eq!(decoded_enum, tx_enum);

    let signed_tx = SignedTransaction {
        transaction: tx_enum.clone(),
        signature: [0; 64],
    };
    let encoded_stx = signed_tx.encode_raw().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, tx_enum);

    let prefix_encoded = tx_struct.encode().unwrap();
    assert_eq!(prefix_encoded[0], b'T');
    assert_eq!(prefix_encoded[1], b'X');
    assert_eq!(prefix_encoded.len(), encoded_struct.len() + 2);
    assert_eq!(prefix_encoded[2..], encoded_struct);
}
