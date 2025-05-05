mod asset_transfer;
mod common;
mod payment;

use asset_transfer::AssetTransferTransactionBuilderError;
pub use asset_transfer::{AssetTransferTransactionBuilder, AssetTransferTransactionFields};
pub use common::{TransactionHeader, TransactionHeaderBuilder};
use payment::PaymentTransactionBuilderError;
pub use payment::{PaymentTransactionBuilder, PaymentTransactionFields};

use crate::constants::HASH_BYTES_LENGTH;
use crate::error::AlgoKitTransactError;
use crate::traits::{AlgorandMsgpack, TransactionId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::any::Any;

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
