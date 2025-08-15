//! Transaction module for AlgoKit Core that provides functionality for creating, manipulating,
//! and managing different types of Algorand transactions.
//!
//! This module includes support for various transaction types, along with the ability to sign,
//! serialize, and deserialize them.

mod application_call;
mod asset_config;
mod asset_freeze;
mod asset_transfer;
mod common;
mod key_registration;
mod payment;

pub use application_call::{
    ApplicationCallTransactionBuilder, ApplicationCallTransactionFields, BoxReference,
    OnApplicationComplete, StateSchema,
};
use application_call::{application_call_deserializer, application_call_serializer};
pub use asset_config::{
    AssetConfigTransactionBuilder, AssetConfigTransactionFields, asset_config_deserializer,
    asset_config_serializer,
};
pub use asset_freeze::{AssetFreezeTransactionBuilder, AssetFreezeTransactionFields};
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
use crate::{Address, MultisigSignature};
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
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

    #[serde(rename = "afrz")]
    AssetFreeze(AssetFreezeTransactionFields),

    #[serde(rename = "keyreg")]
    KeyRegistration(KeyRegistrationTransactionFields),
}

#[derive(Default)]
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
            Transaction::AssetFreeze(f) => &f.header,
        }
    }

    pub fn header_mut(&mut self) -> &mut TransactionHeader {
        match self {
            Transaction::Payment(p) => &mut p.header,
            Transaction::AssetTransfer(a) => &mut a.header,
            Transaction::AssetConfig(a) => &mut a.header,
            Transaction::ApplicationCall(a) => &mut a.header,
            Transaction::KeyRegistration(k) => &mut k.header,
            Transaction::AssetFreeze(f) => &mut f.header,
        }
    }

    pub fn calculate_fee(&self, request: FeeParams) -> Result<u64, AlgoKitTransactError> {
        let mut calculated_fee: u64 = 0;

        if request.fee_per_byte > 0 {
            let estimated_size = self.estimate_size()?;
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

        Ok(calculated_fee)
    }

    pub fn assign_fee(&self, request: FeeParams) -> Result<Transaction, AlgoKitTransactError> {
        let mut tx = self.clone();
        let header = tx.header_mut();
        header.fee = Some(self.calculate_fee(request)?);

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Bytes>")]
    pub signature: Option<[u8; ALGORAND_SIGNATURE_BYTE_LENGTH]>,

    /// Optional auth address applicable if the transaction sender is a rekeyed account.
    #[serde(rename = "sgnr")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub auth_address: Option<Address>,

    /// Optional multisignature signature for the transaction.
    #[serde(rename = "msig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisignature: Option<MultisigSignature>,
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

impl Transaction {
    // Essential header accessors that are actually used in the codebase

    /// Returns the sender address of the transaction
    pub fn sender(&self) -> &Address {
        &self.header().sender
    }

    /// Returns the fee of the transaction
    pub fn fee(&self) -> Option<u64> {
        self.header().fee
    }

    /// Returns the first valid round of the transaction
    pub fn first_valid_round(&self) -> u64 {
        self.header().first_valid
    }

    /// Returns the last valid round of the transaction
    pub fn last_valid_round(&self) -> u64 {
        self.header().last_valid
    }

    /// Returns the note of the transaction
    pub fn note(&self) -> Option<&Vec<u8>> {
        self.header().note.as_ref()
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;

    fn create_test_header() -> TransactionHeader {
        TransactionHeader {
            sender: Address([0u8; 32]),
            fee: Some(1000),
            first_valid: 100,
            last_valid: 200,
            genesis_hash: None,
            genesis_id: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        }
    }

    #[test]
    fn test_header_accessors() {
        let payment = PaymentTransactionFields {
            header: create_test_header(),
            receiver: Address([1u8; 32]),
            amount: 1000,
            close_remainder_to: None,
        };
        let transaction = Transaction::Payment(payment);

        // Test header accessors
        assert_eq!(transaction.fee(), Some(1000));
        assert_eq!(transaction.first_valid_round(), 100);
        assert_eq!(transaction.last_valid_round(), 200);
        assert_eq!(transaction.sender(), &Address([0u8; 32]));
        assert_eq!(transaction.note(), None);
    }

    #[test]
    fn test_app_call_accessor() {
        let app_call = Transaction::ApplicationCall(ApplicationCallTransactionFields {
            header: create_test_header(),
            app_id: 321,
            on_complete: OnApplicationComplete::NoOp,
            approval_program: None,
            clear_state_program: None,
            global_state_schema: None,
            local_state_schema: None,
            extra_program_pages: None,
            args: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
        });

        // Test pattern matching for app call
        if let Transaction::ApplicationCall(app_fields) = &app_call {
            assert_eq!(app_fields.app_id, 321);
        } else {
            panic!("Expected ApplicationCall transaction");
        }

        // Test with non-app transaction
        let payment = Transaction::Payment(PaymentTransactionFields {
            header: create_test_header(),
            receiver: Address([1u8; 32]),
            amount: 1000,
            close_remainder_to: None,
        });

        // Verify payment is not an app call
        match &payment {
            Transaction::ApplicationCall(_) => panic!("Expected non-ApplicationCall transaction"),
            _ => {} // This is what we expect
        }
    }

    #[test]
    fn test_idiomatic_pattern_matching() {
        let transactions = vec![
            Transaction::Payment(PaymentTransactionFields {
                header: create_test_header(),
                receiver: Address([1u8; 32]),
                amount: 1000,
                close_remainder_to: None,
            }),
            Transaction::AssetTransfer(AssetTransferTransactionFields {
                header: create_test_header(),
                asset_id: 123,
                amount: 500,
                receiver: Address([2u8; 32]),
                asset_sender: None,
                close_remainder_to: None,
            }),
            Transaction::ApplicationCall(ApplicationCallTransactionFields {
                header: create_test_header(),
                app_id: 321,
                on_complete: OnApplicationComplete::NoOp,
                approval_program: None,
                clear_state_program: None,
                global_state_schema: None,
                local_state_schema: None,
                extra_program_pages: None,
                args: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
            }),
        ];

        // Test idiomatic Rust pattern matching (zero cost)
        let mut payment_count = 0;
        let mut asset_count = 0;
        let mut app_count = 0;

        for tx in &transactions {
            match tx {
                Transaction::Payment(_) => payment_count += 1,
                Transaction::AssetTransfer(_) => asset_count += 1,
                Transaction::ApplicationCall(_) => app_count += 1,
                _ => {}
            }
        }

        assert_eq!(payment_count, 1);
        assert_eq!(asset_count, 1);
        assert_eq!(app_count, 1);

        // Test filtering with matches! macro (idiomatic for boolean checks)
        let payments: Vec<_> = transactions
            .iter()
            .filter(|tx| matches!(tx, Transaction::Payment(_)))
            .collect();
        assert_eq!(payments.len(), 1);

        // Test accessing fields directly via pattern matching
        for tx in &transactions {
            match tx {
                Transaction::Payment(payment) => {
                    assert_eq!(payment.amount, 1000);
                    assert_eq!(payment.receiver, Address([1u8; 32]));
                }
                Transaction::AssetTransfer(asset) => {
                    assert_eq!(asset.asset_id, 123);
                    assert_eq!(asset.amount, 500);
                }
                Transaction::ApplicationCall(app) => {
                    assert_eq!(app.app_id, 321);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_safe_extraction_pattern() {
        let payment = Transaction::Payment(PaymentTransactionFields {
            header: create_test_header(),
            receiver: Address([1u8; 32]),
            amount: 1000,
            close_remainder_to: None,
        });

        // Test safe extraction with if let (idiomatic Rust)
        if let Transaction::Payment(payment_fields) = &payment {
            assert_eq!(payment_fields.amount, 1000);
        } else {
            panic!("Expected payment transaction");
        }

        // Test that it doesn't match wrong type
        if let Transaction::ApplicationCall(_) = &payment {
            panic!("Should not match app call");
        }
    }
}
