use crate::transactions::common::UtilsError;
use async_trait::async_trait;

#[derive(uniffi::Record)]
pub struct AccountInfo {
    pub balance: u64,
    pub min_balance: u64,
    pub created_apps: Vec<u64>,
    pub created_assets: Vec<u64>,
}

#[derive(uniffi::Record)]
pub struct TransactionInfo {
    pub tx_id: String,
    pub confirmed_round: Option<u64>,
    pub asset_id: Option<u64>,
    pub app_id: Option<u64>,
}

#[derive(uniffi::Record)]
pub struct SuggestedParams {
    pub fee: u64,
    pub first_valid_round: u64,
    pub last_valid_round: u64,
    pub genesis_hash: Vec<u8>,
    pub genesis_id: String,
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait AlgodClientTrait: Send + Sync {
    async fn send_transaction(&self, txn: Vec<u8>) -> Result<String, UtilsError>;
    async fn get_account_info(&self, address: String) -> Result<AccountInfo, UtilsError>;
    async fn get_transaction_info(&self, tx_id: String) -> Result<TransactionInfo, UtilsError>;
    async fn wait_for_confirmation(&self, tx_id: String) -> Result<TransactionInfo, UtilsError>;
    async fn get_suggested_params(&self) -> Result<SuggestedParams, UtilsError>;
}
