#[cfg(test)]
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
// Primary using serde_with for Bytes, which is better than serde_bytes
// see https://docs.rs/serde_with/latest/serde_with/struct.Bytes.html
// It also has some other nice QOL features, like skip_serializing_none
use serde_with::{serde_as, skip_serializing_none, Bytes};
use sha2::{Digest, Sha512_256};
use std::collections::BTreeMap;
use thiserror::Error;

const HASH_BYTES_LENGTH: usize = 32;
const ALGORAND_CHECKSUM_BYTE_LENGTH: usize = 4;
const ALGORAND_ADDRESS_LENGTH: usize = 58;
const ALGORAND_PUBLIC_KEY_BYTE_LENGTH: usize = 32;

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
    const PREFIX: &'static [u8] = b"TX";

    /// msgpack encoding of the transaction with keys sorted and empty fields omitted
    /// This method does not include any prefix/domain separator
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

    /// Decode the bytes into Self. `PREFIX` is ignored if present
    fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
        if bytes.is_empty() {
            return Err(MsgPackError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        // If there is a PREFIX defined, bytes is longer than the prefix, and the bytes start
        // with the prefix, decode the bytes without the prefix
        if Self::PREFIX.len() > 0
            && bytes.len() > Self::PREFIX.len()
            && &bytes[..Self::PREFIX.len()] == Self::PREFIX
        {
            let without_prefix = &bytes[Self::PREFIX.len()..];
            Ok(rmp_serde::from_slice(&without_prefix)?)
        } else {
            Ok(rmp_serde::from_slice(bytes)?)
        }
    }

    /// msgpack encoding of the transaction with keys sorted and empty fields omitted
    /// To get the raw bytes without any domain separator (such as "TX" for transactions), use
    /// `encode_raw`
    fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        let encoded = self.encode_raw()?;
        if Self::PREFIX.is_empty() {
            return Ok(encoded);
        }

        let mut buf = Vec::with_capacity(encoded.len() + Self::PREFIX.len());
        buf.extend_from_slice(Self::PREFIX);
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

fn pub_key_to_checksum(pub_key: &Byte32) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
    let mut hasher = Sha512_256::new();
    hasher.update(&pub_key);

    let mut checksum = [0u8; ALGORAND_CHECKSUM_BYTE_LENGTH];
    checksum
        .copy_from_slice(&hasher.finalize()[(HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH)..]);
    checksum
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(transparent)]
pub struct Address {
    #[serde_as(as = "Bytes")]
    pub pub_key: Byte32,
}

impl Address {
    pub fn from_pubkey(pub_key: &Byte32) -> Self {
        Address { pub_key: *pub_key }
    }

    pub fn from_string(address: &str) -> Result<Self, MsgPackError> {
        if address.len() != ALGORAND_ADDRESS_LENGTH {
            return Err(MsgPackError::InputError(
                "address length is not 58".to_string(),
            ));
        }
        let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, address)
            .expect("decoded value should exist");

        let pub_key: [u8; 32] = decoded[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| {
                MsgPackError::InputError("could not decode address into public key".to_string())
            })?;

        let checksum: [u8; 4] = decoded[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| {
                MsgPackError::InputError("could not get checksum from decoded address".to_string())
            })?;

        let computed_checksum = pub_key_to_checksum(&pub_key);

        if computed_checksum != checksum {
            return Err(MsgPackError::InputError(
                "address checksum is invalid".to_string(),
            ));
        }

        Ok(Self { pub_key })
    }

    pub fn checksum(&self) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
        pub_key_to_checksum(&self.pub_key)
    }

    pub fn address(&self) -> String {
        let mut address_bytes = [0u8; 36]; // 32 bytes pub_key + 4 bytes checksum

        address_bytes[..32].copy_from_slice(&self.pub_key);

        let checksum = self.checksum();
        address_bytes[32..].copy_from_slice(&checksum);

        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &address_bytes)
    }
}

fn is_zero(n: &u64) -> bool {
    *n == 0u64
}

fn is_zero_addr(addr: &Address) -> bool {
    addr.pub_key == [0u8; 32]
}

fn is_zero_addr_opt(addr: &Option<Address>) -> bool {
    addr.as_ref().map_or(true, is_zero_addr)
}

fn is_empty_bytes32(bytes: &Byte32) -> bool {
    bytes == &[0u8; 32]
}

fn is_empty_bytes32_opt(bytes: &Option<Byte32>) -> bool {
    bytes.as_ref().map_or(true, is_empty_bytes32)
}

fn is_empty_string_opt(string: &Option<String>) -> bool {
    string.as_ref().map_or(true, String::is_empty)
}

fn is_empty_vec_opt<T>(vec: &Option<Vec<T>>) -> bool {
    vec.as_ref().map_or(true, Vec::is_empty)
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionHeader {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    #[serde(rename = "snd")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub sender: Address,

    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub fee: u64,

    #[serde(rename = "fv")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub first_valid: u64,

    #[serde(rename = "lv")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub last_valid: u64,

    #[serde(rename = "gh")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    pub genesis_hash: Option<Byte32>,

    #[serde(rename = "gen")]
    #[serde(skip_serializing_if = "is_empty_string_opt")]
    #[serde(default)]
    pub genesis_id: Option<String>,

    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    pub note: Option<Vec<u8>>,

    #[serde(rename = "rekey")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub rekey_to: Option<Address>,

    #[serde(rename = "lx")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    pub lease: Option<Byte32>,

    #[serde(rename = "grp")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
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
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub receiver: Address,

    #[serde(rename = "amt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub amount: u64,

    #[serde(rename = "close")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub close_remainder_to: Option<Address>,
}

impl AlgorandMsgpack for PayTransactionFields {}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AssetTransferTransactionFields {
    #[serde(flatten)]
    pub header: TransactionHeader,

    #[serde(rename = "xaid")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub asset_id: u64,

    #[serde(rename = "aamt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub amount: u64,

    #[serde(rename = "arcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub receiver: Address,

    #[serde(rename = "asnd")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub asset_sender: Option<Address>,

    #[serde(rename = "aclose")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub close_remainder_to: Option<Address>,
}

impl AlgorandMsgpack for AssetTransferTransactionFields {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Transaction {
    Payment(PayTransactionFields),
    AssetTransfer(AssetTransferTransactionFields),
}

impl AlgorandMsgpack for Transaction {
    fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        match self {
            Transaction::Payment(tx) => tx.encode(),
            Transaction::AssetTransfer(tx) => tx.encode(),
        }
    }

    fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
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
    const PREFIX: &'static [u8] = b"";

    // Since we provide default values for all transaction fields, serde will not know which
    // transaction type the bytes actually correspond with. To fix this we need to manually
    // decode the transaction using Transaction::decode (which does check the type) and
    // then add it to the decoded struct
    fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
        let value: rmpv::Value = rmp_serde::from_slice(bytes)?;

        match value {
            rmpv::Value::Map(map) => {
                let txn_value = &map
                    .iter()
                    .find(|(k, _)| k.as_str() == Some("txn"))
                    .unwrap()
                    .1;

                let mut txn_buf = Vec::new();
                rmpv::encode::write_value(&mut txn_buf, &txn_value)?;

                let txn = Transaction::decode(&txn_buf)?;
                let mut stxn: SignedTransaction = rmp_serde::from_slice(bytes)?;

                stxn.transaction = txn;

                return Ok(stxn);
            }
            _ => {
                return Err(MsgPackError::InputError(format!(
                    "Invalid input: {}",
                    value
                )))
            }
        }
    }
}

#[test]
fn test_pay_transaction() {
    let tx_struct = PayTransactionFields {
        header: TransactionHeader {
            genesis_id: None,
            transaction_type: TransactionType::Payment,
            sender: Address::from_pubkey(&[1; 32]),
            fee: 0,
            first_valid: 1000,
            last_valid: 1000,
            genesis_hash: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        receiver: Address::from_pubkey(&[1; 32]),
        amount: 1000,
        close_remainder_to: None,
    };

    let encoded_struct = tx_struct.encode().unwrap();
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
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, tx_enum);

    let raw_encoding = tx_struct.encode_raw().unwrap();
    assert_eq!(encoded_struct[0], b'T');
    assert_eq!(encoded_struct[1], b'X');
    assert_eq!(encoded_struct.len(), raw_encoding.len() + 2);
    assert_eq!(encoded_struct[2..], raw_encoding);
    assert_eq!(encoded_struct.len(), 107);
}

#[test]
fn test_asset_transfer_transaction() {
    let tx_struct = AssetTransferTransactionFields {
        header: TransactionHeader {
            genesis_id: None,
            transaction_type: TransactionType::AssetTransfer,
            sender: Address::from_pubkey(&[1; 32]),
            fee: 0,
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
        receiver: Address::from_pubkey(&[1; 32]),
        asset_sender: None,
        close_remainder_to: None,
    };

    let encoded_struct = tx_struct.encode().unwrap();
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
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, tx_enum);

    let raw_encoding = tx_struct.encode_raw().unwrap();
    assert_eq!(encoded_struct[0], b'T');
    assert_eq!(encoded_struct[1], b'X');
    assert_eq!(encoded_struct.len(), raw_encoding.len() + 2);
    assert_eq!(encoded_struct[2..], raw_encoding);
    assert_eq!(encoded_struct.len(), 117);
}

#[test]
fn test_address() {
    let addr = Address::from_pubkey(&[0; 32]);
    assert_eq!(
        addr.address(),
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ"
    );

    let addr_from_str = Address::from_string(&addr.address()).unwrap();
    assert_eq!(addr, addr_from_str);
}
