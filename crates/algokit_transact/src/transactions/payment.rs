//! Payment transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing payment transactions,
//! which are used to transfer ALGO between accounts.

use crate::transactions::common::TransactionHeader;
use crate::utils::{is_zero, is_zero_addr, is_zero_addr_opt};
use crate::{Address, Transaction};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Represents a payment transaction that transfers ALGO between accounts.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "PaymentTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct PaymentTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// The address of the account receiving the ALGO payment.
    #[serde(rename = "rcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub receiver: Address,

    /// The amount of microALGO to send.
    ///
    /// Specified in microALGO (1 ALGO = 1,000,000 microALGO).
    #[serde(rename = "amt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub amount: u64,

    /// Optional address to send all remaining funds to after the transfer.
    ///
    /// If specified, this indicates that the sender account should be closed after the transaction,
    /// and all remaining funds (minus fees) should be transferred to the specified address.
    /// This effectively removes the sender account from the ledger.
    #[serde(rename = "close")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    #[builder(default)]
    pub close_remainder_to: Option<Address>,
}

impl PaymentTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, PaymentTransactionBuilderError> {
        self.build_fields().map(Transaction::Payment)
    }
}
