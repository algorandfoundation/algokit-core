use algokit_transact::{Address, AssetFreezeTransactionFields, Transaction, TransactionHeader};

use super::common::CommonTransactionParams;

/// Parameters for creating an asset freeze transaction.
#[derive(Debug, Default, Clone)]
pub struct AssetFreezeParams {
    /// Common parameters used across all transaction types
    pub common_params: CommonTransactionParams,

    /// The ID of the asset to freeze
    pub asset_id: u64,

    /// The address of the account to freeze
    pub target_address: Address,
}

/// Parameters for creating an asset unfreeze transaction.
#[derive(Debug, Default, Clone)]
pub struct AssetUnfreezeParams {
    /// Common parameters used across all transaction types
    pub common_params: CommonTransactionParams,

    /// The ID of the asset to unfreeze
    pub asset_id: u64,

    /// The address of the account to unfreeze
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
