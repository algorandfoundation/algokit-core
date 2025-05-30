//! Transaction module for AlgoKit Core that provides functionality for creating, manipulating,
//! and managing different types of Algorand transactions.
//!
//! This module includes support for various transaction types, along with the ability to sign,
//! serialize, and deserialize them.

mod asset_transfer;
mod common;
mod payment;

use asset_transfer::AssetTransferTransactionBuilderError;
pub use asset_transfer::{AssetTransferTransactionBuilder, AssetTransferTransactionFields};
pub use common::{TransactionHeader, TransactionHeaderBuilder};
use payment::PaymentTransactionBuilderError;
pub use payment::{PaymentTransactionBuilder, PaymentTransactionFields};

use crate::constants::{
    ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, HASH_BYTES_LENGTH,
    MAX_TX_GROUP_SIZE,
};
use crate::error::AlgoKitTransactError;
use crate::traits::{AlgorandMsgpack, EstimateTransactionSize, TransactionId, Transactions};
use crate::utils::compute_group_id;
use crate::SignedTransactions;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::any::Any;

/// Enumeration of all transaction types.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Transaction {
    #[serde(rename = "pay")]
    Payment(PaymentTransactionFields),

    #[serde(rename = "axfer")]
    AssetTransfer(AssetTransferTransactionFields),
    // All the below transaction variants will be implemented in the future
    // #[serde(rename = "afrz")]
    // AssetFreeze(...),

    // #[serde(rename = "acfg")]
    // AssetConfig(...),

    // #[serde(rename = "keyreg")]
    // KeyRegistration(...),

    // #[serde(rename = "appl")]
    // ApplicationCall(...),
}

impl PaymentTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, PaymentTransactionBuilderError> {
        self.build_fields().map(|d| Transaction::Payment(d))
    }
}

impl AssetTransferTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, AssetTransferTransactionBuilderError> {
        self.build_fields().map(|d| Transaction::AssetTransfer(d))
    }
}

impl AlgorandMsgpack for Transaction {
    const PREFIX: &'static [u8] = b"TX";
}
impl TransactionId for Transaction {}

impl EstimateTransactionSize for Transaction {
    fn estimate_size(&self) -> Result<usize, AlgoKitTransactError> {
        return Ok(self.encode_raw()?.len() + ALGORAND_SIGNATURE_ENCODING_INCR);
    }
}

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
    pub signature: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH],
}

impl AlgorandMsgpack for SignedTransaction {
    /// Decodes MessagePack bytes into a SignedTransaction.
    ///
    /// # Parameters
    /// * `bytes` - The MessagePack encoded signed transaction bytes
    ///
    /// # Returns
    /// The decoded SignedTransaction or an error if decoding fails or the transaction type is not recognized.
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

                let stxn = SignedTransaction {
                    transaction: Transaction::decode(&txn_buf)?,
                    ..rmp_serde::from_slice(bytes)?
                };

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
    /// The transaction ID as a byte array or an error if generation fails.
    fn id_raw(&self) -> Result<[u8; HASH_BYTES_LENGTH], AlgoKitTransactError> {
        self.transaction.id_raw()
    }
}

impl EstimateTransactionSize for SignedTransaction {
    fn estimate_size(&self) -> Result<usize, AlgoKitTransactError> {
        return Ok(self.encode()?.len());
    }
}

impl Transactions for Vec<Transaction> {
    /// Groups the supplied transactions by calculating and assigning the group to each transaction.
    ///
    /// # Returns
    /// A result containing the transactions with group assign or an error if grouping fails.
    fn assign_group(&self) -> Result<Vec<Transaction>, AlgoKitTransactError> {
        if self.len() > MAX_TX_GROUP_SIZE {
            return Err(AlgoKitTransactError::InputError(format!(
                "Transaction group size exceeds the max limit of {}",
                MAX_TX_GROUP_SIZE
            )));
        }

        if self.len() == 0 {
            return Err(AlgoKitTransactError::InputError(String::from(
                "Transaction group size cannot be 0",
            )));
        }

        let group_id = compute_group_id(&self)?;
        Ok(self
            .iter()
            .map(|tx| {
                let mut tx = tx.clone();
                let header = match &mut tx {
                    Transaction::Payment(ref mut fields) => &mut fields.header,
                    Transaction::AssetTransfer(ref mut fields) => &mut fields.header,
                };
                header.group = Some(group_id);
                tx
            })
            .collect())
    }

    /// Encodes the supplied transactions to MessagePack format with the appropriate prefix (TX).
    ///
    /// This method performs canonical encoding and prepends the domain separation
    ///
    /// Use `encode_raw()` if you want to encode without the prefix.
    ///
    /// # Returns
    /// The encoded bytes with prefix for the supplied transactions or an AlgoKitTransactError if serialization fails.
    fn encode(&self) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
        self.iter()
            .map(|tx| tx.encode())
            .collect::<Result<Vec<Vec<u8>>, AlgoKitTransactError>>()
    }

    /// Encodes the supplied transactions to MessagePack format without any prefix.
    ///
    /// This method performs canonical encoding with sorted map keys and omitted empty fields,
    /// but does not include any domain separation prefix.
    ///
    /// # Returns
    /// The raw encoded bytes for the supplied transactions or an AlgoKitTransactError if serialization fails.
    fn encode_raw(&self) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
        self.iter()
            .map(|tx| tx.encode_raw())
            .collect::<Result<Vec<Vec<u8>>, AlgoKitTransactError>>()
    }

    /// Decodes a collection of MessagePack bytes into a transaction collection.
    ///
    /// If the bytes start with the expected PREFIX for this type, the prefix is
    /// automatically removed before decoding.
    ///
    /// # Parameters
    /// * `encoded_txs` - A collection of MessagePack encoded bytes, each representing a transaction.
    ///
    /// # Returns
    /// The decoded transactions or an AlgoKitTransactError if the input is empty or
    /// deserialization fails.
    fn decode(encoded_txs: &Vec<&[u8]>) -> Result<Vec<Transaction>, AlgoKitTransactError> {
        if encoded_txs.is_empty() {
            return Err(AlgoKitTransactError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        let txs = encoded_txs
            .iter()
            .copied()
            .map(|bytes| Transaction::decode(bytes))
            .collect::<Result<Vec<Transaction>, AlgoKitTransactError>>()?;

        Ok(txs)
    }
}

impl SignedTransactions for Vec<SignedTransaction> {
    /// Encodes the supplied signed transactions to MessagePack format.
    ///
    /// This method performs canonical encoding. No domain separation prefix is applicable.
    ///
    /// # Returns
    /// The encoded bytes for the supplied signed transactions or an AlgoKitTransactError if serialization fails.
    fn encode(&self) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
        self.iter()
            .map(|stx| stx.encode())
            .collect::<Result<Vec<Vec<u8>>, AlgoKitTransactError>>()
    }

    /// Decodes a collection of MessagePack bytes into a signed transaction collection.
    ///
    /// # Parameters
    /// * `encoded_txs` - A collection of MessagePack encoded bytes, each representing a signed transaction.
    ///
    /// # Returns
    /// The decoded signed transactions or an AlgoKitTransactError if the input is empty or
    /// deserialization fails.
    fn decode(encoded_txs: &Vec<&[u8]>) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        if encoded_txs.is_empty() {
            return Err(AlgoKitTransactError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        let stxs = encoded_txs
            .iter()
            .copied()
            .map(|bytes| SignedTransaction::decode(bytes))
            .collect::<Result<Vec<SignedTransaction>, AlgoKitTransactError>>()?;

        Ok(stxs)
    }
}
