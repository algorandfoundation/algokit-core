use algokit_transact::Address;

use super::common::CommonParams;

/// Parameters to freeze an asset for a target account.
#[derive(Debug, Clone)]
pub struct AssetFreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being frozen.
    pub asset_id: u64,

    /// The target account whose asset holdings will be frozen.
    pub target_address: Address,
}

/// Parameters to unfreeze an asset for a target account.
#[derive(Debug, Clone)]
pub struct AssetUnfreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being unfrozen.
    pub asset_id: u64,

    /// The target account whose asset holdings will be unfrozen.
    pub target_address: Address,
}
