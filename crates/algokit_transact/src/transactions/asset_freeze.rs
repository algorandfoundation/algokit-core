//! Asset freeze transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing asset freeze transactions,
//! which are used to freeze or unfreeze asset holdings for specific accounts.

use crate::Transaction;
use crate::address::Address;
use crate::traits::Validate;
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
        let d = self.build_fields()?;
        d.validate().map_err(|errors| {
            AssetFreezeTransactionBuilderError::ValidationError(format!(
                "Asset freeze validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::AssetFreeze(d))
    }
}

impl Validate for AssetFreezeTransactionFields {
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
    fn test_validate_asset_freeze_zero_asset_id() {
        let asset_freeze = AssetFreezeTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 0, // Invalid asset ID
            freeze_target: AccountMother::neil().address(),
            frozen: true,
        };

        let result = asset_freeze.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "Asset ID must not be 0");
    }

    #[test]
    fn test_validate_valid_asset_freeze() {
        let asset_freeze = AssetFreezeTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 123, // Valid asset ID
            freeze_target: AccountMother::neil().address(),
            frozen: true,
        };

        let result = asset_freeze.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_with_invalid_asset_id() {
        let result = AssetFreezeTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .asset_id(0) // Invalid asset ID
            .freeze_target(AccountMother::neil().address())
            .frozen(true)
            .build();

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Asset freeze validation failed"));
        assert!(error_message.contains("Asset ID must not be 0"));
    }

    #[test]
    fn test_build_with_valid_asset_id() {
        let result = AssetFreezeTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .asset_id(123) // Valid asset ID
            .freeze_target(AccountMother::neil().address())
            .frozen(true)
            .build();

        assert!(result.is_ok());
    }
}
