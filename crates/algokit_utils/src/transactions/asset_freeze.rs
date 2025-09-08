use crate::create_transaction_params;
use algokit_transact::{Address, AssetFreezeTransactionFields, Transaction, TransactionHeader};

create_transaction_params! {
    /// Parameters for creating an asset freeze transaction.
    #[derive(Clone, Default)]
    pub struct AssetFreezeParams {
        /// The ID of the asset to freeze
        pub asset_id: u64,
        /// The address of the account to freeze
        pub target_address: Address,
    }
}

create_transaction_params! {
    /// Parameters for creating an asset unfreeze transaction.
    #[derive(Clone, Default)]
    pub struct AssetUnfreezeParams {
      /// The ID of the asset to unfreeze
        pub asset_id: u64,
       /// The address of the account to unfreeze
        pub target_address: Address,
    }
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
