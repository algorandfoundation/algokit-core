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
use crate::utils::{compute_group_id, is_zero_addr_opt};
use crate::{Address, SignedTransactions};
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

/// Network-level fee parameters that apply to all transactions
#[derive(Clone, Copy)]
pub struct NetworkFeeParams {
    pub fee_per_byte: u64,
    pub min_fee: u64,
}

/// Transaction-specific fee parameters
pub struct TransactionFeeParams {
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
}

impl Transaction {
    pub fn header(&self) -> &TransactionHeader {
        match self {
            Transaction::Payment(p) => &p.header,
            Transaction::AssetTransfer(a) => &a.header,
        }
    }

    pub fn header_mut(&mut self) -> &mut TransactionHeader {
        match self {
            Transaction::Payment(p) => &mut p.header,
            Transaction::AssetTransfer(a) => &mut a.header,
        }
    }

    pub fn assign_fee(
        &self,
        network_params: NetworkFeeParams,
        transaction_params: TransactionFeeParams,
    ) -> Result<Transaction, AlgoKitTransactError> {
        let mut tx = self.clone();
        let mut calculated_fee: u64 = 0;

        if network_params.fee_per_byte > 0 {
            let estimated_size = tx.estimate_size()?;
            calculated_fee = network_params.fee_per_byte * estimated_size as u64;
        }

        if calculated_fee < network_params.min_fee {
            calculated_fee = network_params.min_fee;
        }

        if let Some(extra_fee) = transaction_params.extra_fee {
            calculated_fee += extra_fee;
        }

        if let Some(max_fee) = transaction_params.max_fee {
            if calculated_fee > max_fee {
                return Err(AlgoKitTransactError::InputError(format!(
                    "Transaction fee {} µALGO is greater than max fee {} µALGO",
                    calculated_fee, max_fee
                )));
            }
        }

        let header = tx.header_mut();
        header.fee = Some(calculated_fee);

        return Ok(tx);
    }
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

    /// Optional Ed25519 signature authorizing the transaction.
    #[serde(rename = "sig")]
    #[serde_as(as = "Option<Bytes>")]
    pub signature: Option<[u8; ALGORAND_SIGNATURE_BYTE_LENGTH]>,

    /// Optional auth address applicable if the transaction sender is a rekeyed account.
    #[serde(rename = "sgnr")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub auth_address: Option<Address>,
}

impl AlgorandMsgpack for SignedTransaction {
    /// Decodes MsgPack bytes into a SignedTransaction.
    ///
    /// # Parameters
    /// * `bytes` - The MsgPack encoded signed transaction bytes
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

impl Transactions for &[Transaction] {
    /// Groups the supplied transactions by calculating and assigning the group to each transaction.
    ///
    /// # Returns
    /// A result containing the transactions with group assign or an error if grouping fails.
    fn assign_group(self) -> Result<Vec<Transaction>, AlgoKitTransactError> {
        if self.len() > MAX_TX_GROUP_SIZE {
            return Err(AlgoKitTransactError::InputError(format!(
                "Transaction group size exceeds the max limit of {}",
                MAX_TX_GROUP_SIZE
            )));
        }

        if self.is_empty() {
            return Err(AlgoKitTransactError::InputError(String::from(
                "Transaction group size cannot be 0",
            )));
        }

        let group_id = compute_group_id(self)?;
        Ok(self
            .iter()
            .map(|tx| {
                let mut tx = tx.clone();
                tx.header_mut().group = Some(group_id);
                tx
            })
            .collect())
    }

    /// Assigns fees to each transaction in the group.
    ///
    /// # Parameters
    /// * `network_params` - Network-level fee parameters that apply to all transactions
    /// * `transaction_params` - A vector of tuples containing (index, fee_params) for transactions that should have fees assigned
    ///
    /// # Returns
    /// A result containing the transactions with fees assigned or an error if the operation fails.
    fn assign_fees(
        self,
        network_params: NetworkFeeParams,
        transaction_params: Vec<(usize, TransactionFeeParams)>,
    ) -> Result<Vec<Transaction>, AlgoKitTransactError> {
        if self.is_empty() {
            return Err(AlgoKitTransactError::InputError(String::from(
                "Transaction group size cannot be 0",
            )));
        }

        let mut result = self.to_vec();

        for (index, tx_param) in transaction_params {
            if index >= result.len() {
                return Err(AlgoKitTransactError::InputError(format!(
                    "Transaction index {} is out of bounds for transaction group of size {}",
                    index,
                    result.len()
                )));
            }

            result[index] = result[index].assign_fee(network_params, tx_param)?;
        }

        Ok(result)
    }

    /// Encodes the supplied transactions to MsgPack format with the appropriate prefix (TX).
    ///
    /// This method performs canonical encoding and prepends the domain separation
    ///
    /// Use `encode_raw()` if you want to encode without the prefix.
    ///
    /// # Returns
    /// The encoded bytes with prefix for the supplied transactions or an AlgoKitTransactError if serialization fails.
    fn encode(self) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
        self.iter()
            .map(|tx| tx.encode())
            .collect::<Result<Vec<Vec<u8>>, AlgoKitTransactError>>()
    }

    /// Encodes the supplied transactions to MsgPack format without any prefix.
    ///
    /// This method performs canonical encoding with sorted map keys and omitted empty fields,
    /// but does not include any domain separation prefix.
    ///
    /// # Returns
    /// The raw encoded bytes for the supplied transactions or an AlgoKitTransactError if serialization fails.
    fn encode_raw(self) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
        self.iter()
            .map(|tx| tx.encode_raw())
            .collect::<Result<Vec<Vec<u8>>, AlgoKitTransactError>>()
    }

    /// Decodes a collection of MsgPack bytes into a transaction collection.
    ///
    /// If the bytes start with the expected PREFIX for this type, the prefix is
    /// automatically removed before decoding.
    ///
    /// # Parameters
    /// * `encoded_txs` - A collection of MsgPack encoded bytes, each representing a transaction.
    ///
    /// # Returns
    /// The decoded transactions or an AlgoKitTransactError if the input is empty or
    /// deserialization fails.
    fn decode(encoded_txs: &[Vec<u8>]) -> Result<Vec<Transaction>, AlgoKitTransactError> {
        if encoded_txs.is_empty() {
            return Err(AlgoKitTransactError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        let txs = encoded_txs
            .iter()
            .map(|bytes| Transaction::decode(bytes))
            .collect::<Result<Vec<Transaction>, AlgoKitTransactError>>()?;

        Ok(txs)
    }
}

impl SignedTransactions for &[SignedTransaction] {
    /// Encodes signed transactions to MsgPack for sending on the network.
    ///
    /// This method performs canonical encoding. No domain separation prefix is applicable.
    ///
    /// # Parameters
    /// * `self` - A collection of signed transactions to encode
    ///
    /// # Returns
    /// A collection of MsgPack encoded bytes or an AlgoKitTransactError if serialization fails.
    fn encode(self) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
        self.iter()
            .map(|stx| stx.encode())
            .collect::<Result<Vec<Vec<u8>>, AlgoKitTransactError>>()
    }

    /// Decodes a collection of MsgPack bytes into a signed transaction collection.
    ///
    /// # Parameters
    /// * `encoded_signed_txs` - A collection of MsgPack encoded bytes, each representing a signed transaction.
    ///
    /// # Returns
    /// A collection of decoded SignedTransaction or an AlgoKitTransactError if the input is empty or
    /// deserialization fails.
    fn decode(
        encoded_signed_txs: &[Vec<u8>],
    ) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
        if encoded_signed_txs.is_empty() {
            return Err(AlgoKitTransactError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        let stxs = encoded_signed_txs
            .iter()
            .map(|bytes| SignedTransaction::decode(bytes))
            .collect::<Result<Vec<SignedTransaction>, AlgoKitTransactError>>()?;

        Ok(stxs)
    }
}
