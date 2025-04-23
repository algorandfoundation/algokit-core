use crate::address::Address;
use crate::utils::{is_zero, is_zero_addr, is_zero_addr_opt};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PaymentTransactionFields {
    #[serde(rename = "rcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    pub receiver: Address,

    #[serde(rename = "amt")]
    #[serde(skip_serializing_if = "is_zero")]
    pub amount: u64,

    #[serde(rename = "close")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    pub close_remainder_to: Option<Address>,
}
