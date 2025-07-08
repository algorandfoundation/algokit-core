//! Transaction module for AlgoKit Core that provides functionality for creating, manipulating,
//! and managing different types of Algorand transactions.
//!
//! This module includes support for various transaction types, along with the ability to sign,
//! serialize, and deserialize them.

mod application_call;
mod asset_config;
mod asset_transfer;
mod common;
mod key_registration;
mod payment;

use application_call::{application_call_deserializer, application_call_serializer};
pub use application_call::{
    ApplicationCallTransactionBuilder, ApplicationCallTransactionFields, BoxReference,
    OnApplicationComplete, StateSchema,
};
pub use asset_config::{
    asset_config_deserializer, asset_config_serializer, AssetConfigTransactionBuilder,
    AssetConfigTransactionFields,
};
pub use asset_transfer::{AssetTransferTransactionBuilder, AssetTransferTransactionFields};
pub use common::{TransactionHeader, TransactionHeaderBuilder};
pub use key_registration::{KeyRegistrationTransactionBuilder, KeyRegistrationTransactionFields};
pub use payment::{PaymentTransactionBuilder, PaymentTransactionFields};

use crate::constants::{
    ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, HASH_BYTES_LENGTH,
    MAX_TX_GROUP_SIZE,
};
use crate::error::AlgoKitTransactError;
use crate::traits::{AlgorandMsgpack, EstimateTransactionSize, TransactionId, Transactions};
use crate::utils::{compute_group_id, is_zero_addr_opt};
use crate::Address;
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

    #[serde(serialize_with = "asset_config_serializer")]
    #[serde(deserialize_with = "asset_config_deserializer")]
    #[serde(rename = "acfg")]
    AssetConfig(AssetConfigTransactionFields),

    #[serde(serialize_with = "application_call_serializer")]
    #[serde(deserialize_with = "application_call_deserializer")]
    #[serde(rename = "appl")]
    ApplicationCall(ApplicationCallTransactionFields),

    #[serde(rename = "keyreg")]
    KeyRegistration(KeyRegistrationTransactionFields),
}

pub struct FeeParams {
    pub fee_per_byte: u64,
    pub min_fee: u64,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
}

impl Transaction {
    pub fn header(&self) -> &TransactionHeader {
        match self {
            Transaction::Payment(p) => &p.header,
            Transaction::AssetTransfer(a) => &a.header,
            Transaction::AssetConfig(a) => &a.header,
            Transaction::ApplicationCall(a) => &a.header,
            Transaction::KeyRegistration(k) => &k.header,
        }
    }

    pub fn header_mut(&mut self) -> &mut TransactionHeader {
        match self {
            Transaction::Payment(p) => &mut p.header,
            Transaction::AssetTransfer(a) => &mut a.header,
            Transaction::AssetConfig(a) => &mut a.header,
            Transaction::ApplicationCall(a) => &mut a.header,
            Transaction::KeyRegistration(k) => &mut k.header,
        }
    }

    pub fn assign_fee(&self, request: FeeParams) -> Result<Transaction, AlgoKitTransactError> {
        let mut tx = self.clone();
        let mut calculated_fee: u64 = 0;

        if request.fee_per_byte > 0 {
            let estimated_size = tx.estimate_size()?;
            calculated_fee = request.fee_per_byte * estimated_size as u64;
        }

        if calculated_fee < request.min_fee {
            calculated_fee = request.min_fee;
        }

        if let Some(extra_fee) = request.extra_fee {
            calculated_fee += extra_fee;
        }

        if let Some(max_fee) = request.max_fee {
            if calculated_fee > max_fee {
                return Err(AlgoKitTransactError::InputError(format!(
                    "Transaction fee {} µALGO is greater than max fee {} µALGO",
                    calculated_fee, max_fee
                )));
            }
        }

        let header = tx.header_mut();
        header.fee = Some(calculated_fee);

        Ok(tx)
    }
}

impl AlgorandMsgpack for Transaction {
    const PREFIX: &'static [u8] = b"TX";
}

impl TransactionId for Transaction {}

impl EstimateTransactionSize for Transaction {
    fn estimate_size(&self) -> Result<usize, AlgoKitTransactError> {
        Ok(self.encode_raw()?.len() + ALGORAND_SIGNATURE_ENCODING_INCR)
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
                rmpv::encode::write_value(&mut txn_buf, txn_value)?;

                let stxn = SignedTransaction {
                    transaction: Transaction::decode(&txn_buf)?,
                    ..rmp_serde::from_slice(bytes)?
                };

                Ok(stxn)
            }
            _ => Err(AlgoKitTransactError::InputError(format!(
                "expected signed transaction to be a map, but got a: {:#?}",
                value.type_id()
            ))),
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
        Ok(self.encode()?.len())
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
}
