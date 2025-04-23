use crate::address::Address;
use crate::traits::{AlgorandMsgpack, TransactionId};
use crate::transactions::common::TransactionHeader;
use crate::utils::{is_zero, is_zero_addr, is_zero_addr_opt};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PayTransactionFields {
    #[serde(flatten)]
    pub header: TransactionHeader,

    #[serde(rename = "rcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub receiver: Address,

    #[serde(rename = "amt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub amount: u64,

    #[serde(rename = "close")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub close_remainder_to: Option<Address>,
}

impl AlgorandMsgpack for PayTransactionFields {}
impl TransactionId for PayTransactionFields {}
