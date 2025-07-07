use algokit_transact::Address;

use super::common::CommonParams;

/// Parameters to define an asset creation transaction.
#[derive(Debug, Clone)]
pub struct AssetCreateParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The total amount of the smallest divisible (decimal) unit to create.
    ///
    /// For example, if creating a asset with 2 decimals and wanting a total supply of 100 units, this value should be 10000.
    pub total: u64,

    /// The amount of decimal places the asset should have.
    ///
    /// If unspecified then the asset will be in whole units (i.e. `0`).
    /// * If 0, the asset is not divisible;
    /// * If 1, the base unit of the asset is in tenths;
    /// * If 2, the base unit of the asset is in hundredths;
    /// * If 3, the base unit of the asset is in thousandths;
    ///
    /// and so on up to 19 decimal places.
    pub decimals: Option<u32>,

    /// Whether the asset is frozen by default for all accounts.
    /// Defaults to `false`.
    ///
    /// If `true` then for anyone apart from the creator to hold the
    /// asset it needs to be unfrozen per account using an asset freeze
    /// transaction from the `freeze` account, which must be set on creation.
    pub default_frozen: Option<bool>,

    /// The optional name of the asset.
    ///
    /// Max size is 32 bytes.
    pub asset_name: Option<String>,

    /// The optional name of the unit of this asset (e.g. ticker name).
    ///
    /// Max size is 8 bytes.
    pub unit_name: Option<String>,

    /// Specifies an optional URL where more information about the asset can be retrieved (e.g. metadata).
    ///
    /// Max size is 96 bytes.
    pub url: Option<String>,

    /// 32-byte hash of some metadata that is relevant to your asset and/or asset holders.
    ///
    /// The format of this metadata is up to the application.
    pub metadata_hash: Option<[u8; 32]>,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
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
    /// If not set or set to the Zero address is permanently empty.
    pub reserve: Option<Address>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub freeze: Option<Address>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub clawback: Option<Address>,
}

/// Parameters to define an asset reconfiguration transaction.
///
/// For asset reconfiguration, the asset ID field must be set. Only fields manager, reserve, freeze, and clawback can be set.
///
/// **Note:** The manager, reserve, freeze, and clawback addresses
/// are immutably empty if they are not set. If manager is not set then
/// all fields are immutable from that point forward.
#[derive(Debug, Clone)]
pub struct AssetReconfigureParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// ID of the existing asset to be reconfigured.
    pub asset_id: u64,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The configuration fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
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
    /// If not set or set to the Zero address is permanently empty.
    pub reserve: Option<Address>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub freeze: Option<Address>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub clawback: Option<Address>,
}

/// Parameters to define an asset destroy transaction.
///
/// For asset destroy, the asset ID field must be set, all other fields must not be set.
#[derive(Debug, Clone)]
pub struct AssetDestroyParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// ID of the existing asset to be destroyed.
    pub asset_id: u64,
}
