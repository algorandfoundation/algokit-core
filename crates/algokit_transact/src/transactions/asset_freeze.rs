//! Asset freeze transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing asset freeze transactions,
//! which are used to freeze or unfreeze asset holdings for specific accounts.

use crate::Transaction;
use crate::address::Address;
use crate::transactions::common::TransactionHeader;
use crate::utils::{is_zero, is_zero_addr};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Represents an asset freeze transaction that freezes or unfreezes asset holdings.
///
/// Asset freeze transactions are used by the asset freeze account to control
/// whether a specific account can transfer a particular asset.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "AssetFreezeTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct AssetFreezeTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// The ID of the asset being frozen/unfrozen.
    #[serde(rename = "faid")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub asset_id: u64,

    /// The target account whose asset holdings will be affected.
    #[serde(rename = "fadd")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub freeze_target: Address,

    /// The new freeze status.
    ///
    /// `true` to freeze the asset holdings (prevent transfers),
    /// `false` to unfreeze the asset holdings (allow transfers).
    #[serde(rename = "afrz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    #[builder(default)]
    pub frozen: bool,
}

impl AssetFreezeTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, AssetFreezeTransactionBuilderError> {
        self.build_fields().map(Transaction::AssetFreeze)
    }
}
