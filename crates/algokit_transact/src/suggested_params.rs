use serde::{Deserialize, Serialize};

/// Contains parameters relevant to the creation of a new transaction in a specific network at a specific time
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SuggestedParams {
    /// Set this to true to specify fee as microalgos-per-txn.
    /// If the final calculated fee is lower than the protocol minimum fee, the fee will be increased to match the minimum.
    #[serde(rename = "flat-fee")]
    pub flat_fee: bool,
    /// Integer fee per byte, in microAlgos. For a flat fee, set flat_fee to true.
    #[serde(rename = "fee")]
    pub fee: u64,
    /// Minimum fee (not per byte) required for the transaction to be confirmed.
    #[serde(rename = "min-fee")]
    pub min_fee: u64,
    /// First protocol round on which this txn is valid.
    #[serde(rename = "first-round")]
    pub first_valid: u64,
    /// Last protocol round on which this txn is valid.
    #[serde(rename = "last-round")]
    pub last_valid: u64,
    /// Specifies genesis ID of network in use.
    #[serde(rename = "genesis-id")]
    pub genesis_id: String,
    /// Specifies hash genesis block of network in use.
    #[serde(rename = "genesis-hash")]
    pub genesis_hash: Vec<u8>,
    /// ConsensusVersion indicates the consensus protocol version as of the last round.
    #[serde(rename = "consensus-version")]
    pub consensus_version: String,
}
