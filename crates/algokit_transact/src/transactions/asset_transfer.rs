//! Asset transfer transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing asset transfer transactions.

use crate::traits::Validate;
use crate::transactions::common::TransactionHeader;
use crate::utils::{is_zero, is_zero_addr, is_zero_addr_opt};
use crate::{Address, Transaction};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Represents an asset transfer transaction that moves ASAs between accounts.
///
/// Asset transfer transactions are used to transfer Algorand Standard Assets (ASAs)
/// from one account to another.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "AssetTransferTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct AssetTransferTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// The ID of the asset being transferred.
    #[serde(rename = "xaid")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub asset_id: u64,

    /// The amount of the asset to transfer.
    ///
    /// An integer value representing the number of units (to their smallest denomination) of
    /// the asset that are being transferred.
    /// In other words, the asset decimals don't play a role in this value.
    /// It should be up to the caller (or a higher abstraction) to handle the conversion based on
    /// the asset decimals.
    #[serde(rename = "aamt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub amount: u64,

    /// The address of the account that will receive the asset.
    ///
    /// The receiver must have opted-in to the asset before they can receive it.
    #[serde(rename = "arcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub receiver: Address,

    /// Optional address of the account that actually holds the asset being transferred.
    ///
    /// If provided, this indicates that the transaction is a clawback operation,
    /// where the sender is the asset clawback address and is forcibly moving assets
    /// from this account to the receiver.
    #[serde(rename = "asnd")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    #[builder(default)]
    pub asset_sender: Option<Address>,

    /// Optional address to send all remaining asset units to after the transfer.
    ///
    /// If specified, this indicates that the sender is closing out their position in the asset,
    /// and all remaining units of this asset owned by the sender will be transferred to this address.
    /// This effectively removes the asset from the sender's account.
    #[serde(rename = "aclose")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    #[builder(default)]
    pub close_remainder_to: Option<Address>,
}

impl AssetTransferTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, AssetTransferTransactionBuilderError> {
        let d = self.build_fields()?;
        d.validate().map_err(|errors| {
            AssetTransferTransactionBuilderError::ValidationError(format!(
                "Asset transfer validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::AssetTransfer(d))
    }
}

impl Validate for AssetTransferTransactionFields {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.asset_id == 0 {
            errors.push("Asset ID must not be 0".to_string());
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{AccountMother, TransactionHeaderMother};

    #[test]
    fn test_validate_asset_transfer_zero_asset_id() {
        let asset_transfer = AssetTransferTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 0, // Invalid asset ID
            amount: 1000,
            receiver: AccountMother::neil().address(),
            asset_sender: None,
            close_remainder_to: None,
        };

        let result = asset_transfer.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "Asset ID must not be 0");
    }

    #[test]
    fn test_validate_valid_asset_transfer() {
        let asset_transfer = AssetTransferTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 123, // Valid asset ID
            amount: 1000,
            receiver: AccountMother::neil().address(),
            asset_sender: None,
            close_remainder_to: None,
        };

        let result = asset_transfer.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_with_invalid_asset_id() {
        let result = AssetTransferTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .asset_id(0) // Invalid asset ID
            .amount(1000)
            .receiver(AccountMother::neil().address())
            .build();

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Asset transfer validation failed"));
        assert!(error_message.contains("Asset ID must not be 0"));
    }

    #[test]
    fn test_build_with_valid_asset_id() {
        let result = AssetTransferTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .asset_id(123) // Valid asset ID
            .amount(1000)
            .receiver(AccountMother::neil().address())
            .build();

        assert!(result.is_ok());
    }
}
