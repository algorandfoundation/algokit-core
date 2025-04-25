use crate::address::Address;
use crate::utils::{is_zero, is_zero_addr, is_zero_addr_opt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder, Default)]
#[builder(default)]
pub struct AssetTransferTransactionFields {
    #[serde(rename = "xaid")]
    #[serde(skip_serializing_if = "is_zero")]
    pub asset_id: u64,

    #[serde(rename = "aamt")]
    #[serde(skip_serializing_if = "is_zero")]
    pub amount: u64,

    #[serde(rename = "arcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    pub receiver: Address,

    #[serde(rename = "asnd")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    pub asset_sender: Option<Address>,

    #[serde(rename = "aclose")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    pub close_remainder_to: Option<Address>,
}
