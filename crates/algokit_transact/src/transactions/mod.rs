//! Transaction module for AlgoKit Core that provides functionality for creating, manipulating,
//! and managing different types of Algorand transactions.
//!
//! This module includes support for various transaction types, along with the ability to sign,
//! serialize, and deserialize them.

mod asset_transfer;
mod common;
mod payment;

pub use asset_transfer::AssetTransferTransactionFields;
pub use common::{TransactionHeader, TransactionType};
pub use payment::PayTransactionFields;

use crate::constants::HASH_BYTES_LENGTH;
use crate::error::AlgoKitTransactError;
use crate::traits::{AlgorandMsgpack, TransactionId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::any::Any;

/// Enumeration of all transaction types.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Transaction {
    Payment(PayTransactionFields),
    AssetTransfer(AssetTransferTransactionFields),
}

impl AlgorandMsgpack for Transaction {
    /// Encodes a transaction into MessagePack format.
    ///
    /// # Returns
    /// A Result containing the encoded bytes.
    /// 
    /// # Errors
    /// If the transaction type is not recognized or if encoding fails.
    fn encode(&self) -> Result<Vec<u8>, AlgoKitTransactError> {
        match self {
            Transaction::Payment(tx) => tx.encode(),
            Transaction::AssetTransfer(tx) => tx.encode(),
        }
    }

    /// Decodes MessagePack bytes into a Transaction.
    ///
    /// # Parameters
    /// * `bytes` - The MessagePack encoded transaction bytes
    ///
    /// # Returns
    /// A Result containing the decoded Transaction.
    /// 
    /// # Errors
    /// If the decoding fails or if the transaction type is not recognized.
    fn decode(bytes: &[u8]) -> Result<Self, AlgoKitTransactError> {
        let header = TransactionHeader::decode(bytes)?;
        match header.transaction_type {
            TransactionType::Payment => {
                Ok(Transaction::Payment(PayTransactionFields::decode(bytes)?))
            }
            TransactionType::AssetTransfer => Ok(Transaction::AssetTransfer(
                AssetTransferTransactionFields::decode(bytes)?,
            )),
            _ => Err(AlgoKitTransactError::UnknownTransactionType(format!(
                "{:?}",
                header.transaction_type
            ))),
        }
    }
}
impl TransactionId for Transaction {}

/// A signed transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignedTransaction {
    /// The transaction that has been signed.
    #[serde(rename = "txn")]
    pub transaction: Transaction,

    /// The Ed25519 signature authorizing the transaction.
    #[serde(rename = "sig")]
    #[serde_as(as = "Bytes")]
    pub signature: [u8; 64],
}

impl AlgorandMsgpack for SignedTransaction {
    /// The prefix used for MessagePack encoding, empty for signed transactions.
    const PREFIX: &'static [u8] = b"";

    /// Decodes MessagePack bytes into a SignedTransaction.
    ///
    /// This implementation handles the complexity of determining the transaction type
    /// correctly during deserialization.
    ///
    /// # Parameters
    /// * `bytes` - The MessagePack encoded signed transaction bytes
    ///
    /// # Returns
    /// A Result containing the decoded SignedTransaction.
    /// 
    /// # Errors
    /// If the decoding fails or if the transaction type is not recognized.
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
    /// Generates the raw transaction ID as a hash of the transaction data.
    ///
    /// # Returns
    /// A Result containing the transaction ID as a byte array.
    /// 
    /// # Errors
    /// If the transaction ID cannot be generated.
    fn raw_id(&self) -> Result<[u8; HASH_BYTES_LENGTH], AlgoKitTransactError> {
        self.transaction.raw_id()
    }
}
