/// Shared header parameters every transaction accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonTxnParams {
    pub sender: String,
    pub note: Option<Vec<u8>>,
    pub lease: Option<Vec<u8>>,
    pub rekey_to: Option<String>,
    /// Fixed fee in microALGO. When set, overrides any per-byte fee calculation.
    pub static_fee: Option<u64>,
    /// Additional microALGO added on top of a computed fee.
    pub extra_fee: Option<u64>,
    /// Cap applied to a computed fee.
    pub max_fee: Option<u64>,
    /// Number of rounds the transaction is valid. Overrides ComposerParams.default_validity_window.
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

/// One transaction's input — one variant per supported transaction type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxnParams {
    Payment(PaymentParams),
    AssetTransfer(AssetTransferParams),
    AssetOptIn(AssetOptInParams),
    OnlineKeyReg(OnlineKeyRegParams),
    OfflineKeyReg(OfflineKeyRegParams),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentParams {
    pub common: CommonTxnParams,
    pub receiver: String,
    pub amount: u64,
    pub close_remainder_to: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetTransferParams {
    pub common: CommonTxnParams,
    pub asset_id: u64,
    pub receiver: String,
    pub amount: u64,
    pub clawback_target: Option<String>,
    pub close_asset_to: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetOptInParams {
    pub common: CommonTxnParams,
    pub asset_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnlineKeyRegParams {
    pub common: CommonTxnParams,
    pub vote_key: Vec<u8>,
    pub selection_key: Vec<u8>,
    pub state_proof_key: Vec<u8>,
    pub vote_first: u64,
    pub vote_last: u64,
    pub vote_key_dilution: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineKeyRegParams {
    pub common: CommonTxnParams,
}
