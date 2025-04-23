mod asset_transfer;
mod common;
mod payment;

pub use asset_transfer::AssetTransferTransactionFields;
pub use common::TransactionType;
pub use payment::PaymentTransactionFields;

use crate::address::Address;
use crate::constants::Byte32;
use crate::constants::HASH_BYTES_LENGTH;
use crate::error::AlgoKitTransactError;
use crate::traits::{AlgorandMsgpack, TransactionId};
use crate::utils::{
    is_empty_bytes32_opt, is_empty_string_opt, is_empty_vec_opt, is_zero, is_zero_addr,
    is_zero_addr_opt,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, Bytes};
use std::any::Any;

impl AlgorandMsgpack for Transaction {}
impl TransactionId for Transaction {}

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
    fn decode(bytes: &[u8]) -> Result<Self, AlgoKitTransactError> {
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
                return Err(AlgoKitTransactError::InputError(format!(
                    "expected signed transaction to be a map, but got a: {:#?}",
                    value.type_id()
                )))
            }
        }
    }
}
impl TransactionId for SignedTransaction {
    fn raw_id(&self) -> Result<[u8; HASH_BYTES_LENGTH], AlgoKitTransactError> {
        self.transaction.raw_id()
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
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

    #[serde(flatten)]
    pub payment: Option<PaymentTransactionFields>,

    #[serde(flatten)]
    pub asset_transfer: Option<AssetTransferTransactionFields>,
}
