/// Network-suggested parameters fetched from algod.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuggestedParams {
    /// Fee in microALGO. Used as-is when `flat_fee` is true, else a per-byte rate.
    pub fee: u64,
    pub flat_fee: bool,
    pub first_round_valid: u64,
    pub last_round_valid: u64,
    /// 32-byte genesis hash that pins the transaction to a specific chain.
    pub genesis_hash: Vec<u8>,
    pub genesis_id: String,
}

/// Composer-level defaults applied to every TxnParams unless an individual entry overrides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerParams {
    pub suggested_params: SuggestedParams,
    /// Number of rounds a transaction is valid by default when an individual TxnParams does not specify a validity window.
    pub default_validity_window: Option<u64>,
}
