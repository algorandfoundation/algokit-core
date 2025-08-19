//! Asset configuration transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing asset configuration transactions,
//! which are used to create, reconfige, or destroy Algorand Standard Assets (ASAs).

use crate::traits::Validate;
use crate::transactions::common::{TransactionHeader, TransactionValidationError};
use crate::utils::{is_false_opt, is_zero, is_zero_addr_opt, is_zero_opt};
use crate::{Address, Transaction};
use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{Bytes, serde_as, skip_serializing_none};

// Only used for serialise/deserialise
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
struct AssetParams {
    #[serde(rename = "t")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    pub total: Option<u64>,

    #[serde(rename = "dc")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    pub decimals: Option<u32>,

    #[serde(rename = "df")]
    #[serde(skip_serializing_if = "is_false_opt")]
    #[serde(default)]
    pub default_frozen: Option<bool>,

    #[serde(rename = "an")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub asset_name: Option<String>,

    #[serde(rename = "un")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unit_name: Option<String>,

    #[serde(rename = "au")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,

    #[serde(rename = "am")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub metadata_hash: Option<[u8; 32]>,

    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub manager: Option<Address>,

    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub reserve: Option<Address>,

    #[serde(rename = "f")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub freeze: Option<Address>,

    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub clawback: Option<Address>,
}

/// Represents an asset configuration transaction that creates, reconfigures, or destroys assets.
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "AssetConfigTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct AssetConfigTransactionFields {
    /// Common transaction header fields.
    pub header: TransactionHeader,

    /// ID of the asset to operate on.
    ///
    /// For asset creation, this must be 0.
    /// For asset reconfiguration this is the ID of the existing asset to be reconfigured,
    /// For asset destroy this is the ID of the existing asset to be destroyed.
    pub asset_id: u64,

    /// The total amount of the smallest divisible (decimal) unit to create.
    ///
    /// Required when creating a new asset.
    /// For example, if creating a asset with 2 decimals and wanting a total supply of 100 units, this value should be 10000.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub total: Option<u64>,

    /// The amount of decimal places the asset should have.
    ///
    /// If unspecified then the asset will be in whole units (i.e. `0`).
    /// * If 0, the asset is not divisible;
    /// * If 1, the base unit of the asset is in tenths;
    /// * If 2, the base unit of the asset is in hundredths;
    /// * If 3, the base unit of the asset is in thousandths;
    ///
    /// and so on up to 19 decimal places.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub decimals: Option<u32>,

    /// Whether the asset is frozen by default for all accounts.
    /// Defaults to `false`.
    ///
    /// If `true` then for anyone apart from the creator to hold the
    /// asset it needs to be unfrozen per account using an asset freeze
    /// transaction from the `freeze` account, which must be set on creation.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub default_frozen: Option<bool>,

    /// The optional name of the asset.
    ///
    /// Max size is 32 bytes.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub asset_name: Option<String>,

    /// The optional name of the unit of this asset (e.g. ticker name).
    ///
    /// Max size is 8 bytes.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub unit_name: Option<String>,

    /// Specifies an optional URL where more information about the asset can be retrieved (e.g. metadata).
    ///
    /// Max size is 96 bytes.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub url: Option<String>,

    /// 32-byte hash of some metadata that is relevant to your asset and/or asset holders.
    ///
    /// The format of this metadata is up to the application.
    ///
    /// This field can only be specified upon asset creation.
    #[builder(default)]
    pub metadata_hash: Option<[u8; 32]>,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The configuration fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
    #[builder(default)]
    pub manager: Option<Address>,

    /// The address of the optional account that holds the reserve (uncirculated supply) units of the asset.
    ///
    /// This address has no specific authority in the protocol itself and is informational only.
    ///
    /// Some standards like [ARC-19](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0019.md)
    /// rely on this field to hold meaningful data.
    ///
    /// It can be used in the case where you want to signal to holders of your asset that the uncirculated units
    /// of the asset reside in an account that is different from the default creator account.
    ///
    /// If not set or set to the Zero address the field is permanently empty.
    #[builder(default)]
    pub reserve: Option<Address>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address the field is permanently empty.
    #[builder(default)]
    pub freeze: Option<Address>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address the field is permanently empty.
    #[builder(default)]
    pub clawback: Option<Address>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct AssetConfigTransactionFieldsSerde {
    #[serde(flatten)]
    header: TransactionHeader,

    #[serde(rename = "caid")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    asset_id: u64,

    #[serde(rename = "apar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    asset_params: Option<AssetParams>,
}

pub fn asset_config_serializer<S>(
    fields: &AssetConfigTransactionFields,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let fields = fields.clone();

    let has_asset_params = fields.total.is_some()
        || fields.decimals.is_some()
        || fields.default_frozen.is_some()
        || fields.asset_name.is_some()
        || fields.unit_name.is_some()
        || fields.url.is_some()
        || fields.metadata_hash.is_some()
        || fields.manager.is_some()
        || fields.reserve.is_some()
        || fields.freeze.is_some()
        || fields.clawback.is_some();

    let asset_params = match has_asset_params {
        true => Some(AssetParams {
            total: fields.total,
            decimals: fields.decimals,
            default_frozen: fields.default_frozen,
            asset_name: fields.asset_name,
            unit_name: fields.unit_name,
            url: fields.url,
            metadata_hash: fields.metadata_hash,
            manager: fields.manager,
            reserve: fields.reserve,
            freeze: fields.freeze,
            clawback: fields.clawback,
        }),
        false => None,
    };

    let serde_struct = AssetConfigTransactionFieldsSerde {
        header: fields.header,
        asset_id: fields.asset_id,
        asset_params,
    };

    serde_struct.serialize(serializer)
}

pub fn asset_config_deserializer<'de, D>(
    deserializer: D,
) -> Result<AssetConfigTransactionFields, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialised_fields = AssetConfigTransactionFieldsSerde::deserialize(deserializer)?;

    let (
        total,
        decimals,
        default_frozen,
        asset_name,
        unit_name,
        url,
        metadata_hash,
        manager,
        reserve,
        freeze,
        clawback,
    ) = match deserialised_fields.asset_params {
        Some(params) => (
            params.total,
            params.decimals,
            params.default_frozen,
            params.asset_name,
            params.unit_name,
            params.url,
            params.metadata_hash,
            params.manager,
            params.reserve,
            params.freeze,
            params.clawback,
        ),
        None => (
            None, None, None, None, None, None, None, None, None, None, None,
        ),
    };

    Ok(AssetConfigTransactionFields {
        header: deserialised_fields.header,
        asset_id: deserialised_fields.asset_id,
        total,
        decimals,
        default_frozen,
        asset_name,
        unit_name,
        url,
        metadata_hash,
        manager,
        reserve,
        freeze,
        clawback,
    })
}

impl AssetConfigTransactionFields {
    pub fn validate_for_creation(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        if self.total.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "Total".to_string(),
            ));
        }

        if let Some(decimals) = self.decimals {
            if decimals > 19 {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Decimals".to_string(),
                    actual: decimals as usize,
                    max: 19,
                    unit: "decimal places".to_string(),
                });
            }
        }

        if let Some(ref unit_name) = self.unit_name {
            if unit_name.len() > 8 {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Unit name".to_string(),
                    actual: unit_name.len(),
                    max: 8,
                    unit: "bytes".to_string(),
                });
            }
        }

        if let Some(ref asset_name) = self.asset_name {
            if asset_name.len() > 32 {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Asset name".to_string(),
                    actual: asset_name.len(),
                    max: 32,
                    unit: "bytes".to_string(),
                });
            }
        }

        if let Some(ref url) = self.url {
            if url.len() > 96 {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "URL".to_string(),
                    actual: url.len(),
                    max: 96,
                    unit: "bytes".to_string(),
                });
            }
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }

    pub fn validate_for_reconfigure(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        if self.total.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "total".to_string(),
            ));
        }

        if self.decimals.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "decimals".to_string(),
            ));
        }

        if self.default_frozen.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "default_frozen".to_string(),
            ));
        }

        if self.asset_name.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "asset_name".to_string(),
            ));
        }

        if self.unit_name.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "unit_name".to_string(),
            ));
        }

        if self.url.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "url".to_string(),
            ));
        }

        if self.metadata_hash.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                "metadata_hash".to_string(),
            ));
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}

impl AssetConfigTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, AssetConfigTransactionBuilderError> {
        let d = self.build_fields()?;
        d.validate().map_err(|errors| {
            AssetConfigTransactionBuilderError::ValidationError(format!(
                "Asset config validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::AssetConfig(d))
    }
}

impl Validate for AssetConfigTransactionFields {
    fn validate(&self) -> Result<(), Vec<String>> {
        match self.asset_id {
            0 => {
                // Asset creation
                self.validate_for_creation()
                    .map_err(|errors| errors.iter().map(|e| e.to_string()).collect())
            }
            _ => {
                // Asset reconfiguration or destroy
                let has_asset_params = self.total.is_some()
                    || self.decimals.is_some()
                    || self.default_frozen.is_some()
                    || self.asset_name.is_some()
                    || self.unit_name.is_some()
                    || self.url.is_some()
                    || self.metadata_hash.is_some()
                    || self.manager.is_some()
                    || self.reserve.is_some()
                    || self.freeze.is_some()
                    || self.clawback.is_some();

                match has_asset_params {
                    true => self
                        .validate_for_reconfigure()
                        .map_err(|errors| errors.iter().map(|e| e.to_string()).collect()),
                    false => Ok(()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{AccountMother, TransactionHeaderMother};

    #[test]
    fn test_validate_asset_creation_multiple_errors() {
        let long_url = "https://".to_string() + &"a".repeat(100); // More than 96 bytes total
        let asset_config = AssetConfigTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 0,        // Asset creation
            total: None,        // Missing total - ERROR 1
            decimals: Some(20), // Too large (max is 19) - ERROR 2
            default_frozen: Some(false),
            asset_name: Some(
                "ThisIsAVeryLongAssetNameThatExceedsTheMaximumLengthOf32Bytes".to_string(),
            ), // More than 32 bytes - ERROR 3
            unit_name: Some("VERYLONGUNITNAME".to_string()), // More than 8 bytes - ERROR 4
            url: Some(long_url),                             // More than 96 bytes - ERROR 5
            metadata_hash: None,
            manager: Some(AccountMother::neil().address()),
            reserve: None,
            freeze: None,
            clawback: None,
        };

        let result = asset_config.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 5);

        // Check that all errors are present
        let error_text = errors.join("\n");
        assert!(error_text.contains("Total is required"));
        assert!(error_text.contains("Decimals cannot exceed 19 decimal places"));
        assert!(error_text.contains("Asset name cannot exceed 32 bytes"));
        assert!(error_text.contains("Unit name cannot exceed 8 bytes"));
        assert!(error_text.contains("URL cannot exceed 96 bytes"));
    }

    #[test]
    fn test_validate_valid_asset_creation() {
        let asset_config = AssetConfigTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 0,
            total: Some(1000),
            decimals: Some(2),
            default_frozen: Some(false),
            asset_name: Some("TestAsset".to_string()),
            unit_name: Some("TA".to_string()),
            url: Some("https://example.com".to_string()),
            metadata_hash: Some([1; 32]),
            manager: Some(AccountMother::neil().address()),
            reserve: Some(AccountMother::neil().address()),
            freeze: Some(AccountMother::neil().address()),
            clawback: Some(AccountMother::neil().address()),
        };

        let result = asset_config.validate();
        assert!(result.is_ok());
    }

    // Asset Reconfigure Validation Tests

    #[test]
    fn test_validate_valid_asset_reconfigure() {
        let asset_config = AssetConfigTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 123,                                   // Existing asset
            total: None,                                     // Can't modify
            decimals: None,                                  // Can't modify
            default_frozen: None,                            // Can't modify
            asset_name: None,                                // Can't modify
            unit_name: None,                                 // Can't modify
            url: None,                                       // Can't modify
            metadata_hash: None,                             // Can't modify
            manager: Some(AccountMother::neil().address()),  // Can modify
            reserve: Some(AccountMother::neil().address()),  // Can modify
            freeze: Some(AccountMother::neil().address()),   // Can modify
            clawback: Some(AccountMother::neil().address()), // Can modify
        };

        let result = asset_config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_valid_asset_destroy() {
        let asset_config = AssetConfigTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 123, // Existing asset
            total: None,   // No params for destroy
            decimals: None,
            default_frozen: None,
            asset_name: None,
            unit_name: None,
            url: None,
            metadata_hash: None,
            manager: None,
            reserve: None,
            freeze: None,
            clawback: None,
        };

        let result = asset_config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_asset_reconfigure_multiple_immutable_field_errors() {
        let asset_config = AssetConfigTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            asset_id: 123,                                  // Existing asset
            total: Some(2000),                              // Can't modify total - ERROR 1
            decimals: Some(3),                              // Can't modify decimals - ERROR 2
            default_frozen: Some(true),                     // Can't modify default_frozen - ERROR 3
            asset_name: Some("NewName".to_string()),        // Can't modify asset_name - ERROR 4
            unit_name: Some("NEW".to_string()),             // Can't modify unit_name - ERROR 5
            url: Some("https://newurl.com".to_string()),    // Can't modify url - ERROR 6
            metadata_hash: Some([2; 32]),                   // Can't modify metadata_hash - ERROR 7
            manager: Some(AccountMother::neil().address()), // This is allowed
            reserve: None,
            freeze: None,
            clawback: None,
        };

        let result = asset_config.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 7);

        // Check that all immutable field errors are present
        let error_text = errors.join("\n");
        assert!(error_text.contains("total") && error_text.contains("immutable"));
        assert!(error_text.contains("decimals") && error_text.contains("immutable"));
        assert!(error_text.contains("default_frozen") && error_text.contains("immutable"));
        assert!(error_text.contains("asset_name") && error_text.contains("immutable"));
        assert!(error_text.contains("unit_name") && error_text.contains("immutable"));
        assert!(error_text.contains("url") && error_text.contains("immutable"));
        assert!(error_text.contains("metadata_hash") && error_text.contains("immutable"));
    }
}
