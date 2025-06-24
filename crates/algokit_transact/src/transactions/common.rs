//! Common types and structures for Algorand transactions.
//!
//! This module provides the fundamental transaction types and headers used
//! across different transaction types.

use crate::address::Address;
use crate::constants::Byte32;
use crate::utils::{
    is_empty_bytes32_opt, is_empty_string_opt, is_empty_vec_opt, is_zero, is_zero_addr,
    is_zero_addr_opt, is_zero_opt,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, Bytes};

/// Common header fields shared by all transaction types.
///
/// This structure contains the fields that are present in every transaction,
/// regardless of transaction type.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TransactionHeader {
    /// The account that authorized the transaction.
    ///
    /// Fees are deducted from this account.
    #[serde(rename = "snd")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub sender: Address,

    /// Optional transaction fee in microALGO.
    ///
    /// When None, the fee will be interpreted as 0 by the network.
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub fee: Option<u64>,

    /// First round for when the transaction is valid.
    #[serde(rename = "fv")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub first_valid: u64,

    /// Last round for when the transaction is valid.
    ///
    /// After this round, the transaction will be expired.
    #[serde(rename = "lv")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub last_valid: u64,

    /// Hash of the genesis block of the network.
    ///
    /// Used to identify which network the transaction is for.
    #[serde(rename = "gh")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    #[builder(default)]
    pub genesis_hash: Option<Byte32>,

    /// Genesis ID of the network.
    ///
    /// A human-readable string used alongside genesis hash to identify the network.
    #[serde(rename = "gen")]
    #[serde(skip_serializing_if = "is_empty_string_opt")]
    #[serde(default)]
    #[builder(default)]
    pub genesis_id: Option<String>,

    /// Optional user-defined note field.
    ///
    /// Can contain arbitrary data up to 1KB in size.
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub note: Option<Vec<u8>>,

    /// Optional authorized account for future transactions.
    ///
    /// If set, only this account will be used for transaction authorization going forward.
    /// Reverting back control to the original address must be done by setting this field to
    /// the original address.
    #[serde(rename = "rekey")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    #[builder(default)]
    pub rekey_to: Option<Address>,

    /// Optional lease value to enforce mutual transaction exclusion.
    ///
    /// When a transaction with a non-empty lease field is confirmed, the lease is acquired.
    /// A lease X is acquired by the sender, generating the (sender, X) lease.
    /// The lease is kept active until the last_valid round of the transaction has elapsed.
    /// No other transaction sent by the same sender can be confirmed until the lease expires.
    #[serde(rename = "lx")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    #[builder(default)]
    pub lease: Option<Byte32>,

    /// Optional group ID for atomic transaction grouping.
    ///
    /// Transactions with the same group ID must execute together or not at all.
    #[serde(rename = "grp")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    #[builder(default)]
    pub group: Option<Byte32>,
}
