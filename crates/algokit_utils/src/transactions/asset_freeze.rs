use algokit_transact::{Address, AssetFreezeTransactionFields, Transaction, TransactionHeader};

use super::common::CommonParams;

/// Parameters to freeze an asset for a target account.
#[derive(Debug, Default, Clone)]
pub struct AssetFreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being frozen.
    pub asset_id: u64,

    /// The target account whose asset holdings will be frozen.
    pub target_address: Address,
}

/// Parameters to unfreeze an asset for a target account.
#[derive(Debug, Default, Clone)]
pub struct AssetUnfreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being unfrozen.
    pub asset_id: u64,

    /// The target account whose asset holdings will be unfrozen.
    pub target_address: Address,
}

pub fn build_asset_freeze(params: &AssetFreezeParams, header: TransactionHeader) -> Transaction {
    Transaction::AssetFreeze(AssetFreezeTransactionFields {
        header,
        asset_id: params.asset_id,
        freeze_target: params.target_address.clone(),
        frozen: true,
    })
}

pub fn build_asset_unfreeze(
    params: &AssetUnfreezeParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::AssetFreeze(AssetFreezeTransactionFields {
        header,
        asset_id: params.asset_id,
        freeze_target: params.target_address.clone(),
        frozen: false,
    })
}
