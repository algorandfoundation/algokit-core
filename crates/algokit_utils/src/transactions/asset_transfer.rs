use algokit_transact::{Address, AssetTransferTransactionFields, Transaction, TransactionHeader};

use super::common::CommonParams;

#[derive(Debug, Clone)]
pub struct AssetTransferParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset transfers
    /// without additional side effects.
    /// Only in the case where the receiver is equal to the sender and the amount is zero,
    /// this is an asset opt-in transaction.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
}

#[derive(Debug, Clone)]
pub struct AssetOptInParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-in
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
}

#[derive(Debug, Clone)]
pub struct AssetOptOutParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-out
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
pub struct AssetClawbackParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset clawback
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
    // The address from which ASAs are taken.
    pub clawback_target: Address,
}

pub fn build_asset_transfer(
    params: &AssetTransferParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: params.amount,
        receiver: params.receiver.clone(),
        asset_sender: None,
        close_remainder_to: None,
    })
}

pub fn build_asset_opt_in(params: &AssetOptInParams, header: TransactionHeader) -> Transaction {
    let sender = header.sender.clone();
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: 0,
        receiver: sender,
        asset_sender: None,
        close_remainder_to: None,
    })
}

pub fn build_asset_opt_out(params: &AssetOptOutParams, header: TransactionHeader) -> Transaction {
    let sender = header.sender.clone();
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: 0,
        receiver: sender,
        asset_sender: None,
        close_remainder_to: params.close_remainder_to.clone(),
    })
}

pub fn build_asset_clawback(
    params: &AssetClawbackParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: params.amount,
        receiver: params.receiver.clone(),
        asset_sender: Some(params.clawback_target.clone()),
        close_remainder_to: None,
    })
}
