use crate::address::Address;
use crate::constants::Byte32;
use crate::traits::AlgorandMsgpack;
use crate::utils::{
    is_empty_bytes32_opt, is_empty_string_opt, is_empty_vec_opt, is_zero, is_zero_addr,
    is_zero_addr_opt,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, Bytes};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionType {
    #[serde(rename = "pay")]
    Payment,

    #[serde(rename = "axfer")]
    AssetTransfer,

    #[serde(rename = "afrz")]
    AssetFreeze,

    #[serde(rename = "acfg")]
    AssetConfig,

    #[serde(rename = "keyreg")]
    KeyRegistration,

    #[serde(rename = "appl")]
    ApplicationCall,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TransactionHeader {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    #[serde(rename = "snd")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub sender: Address,

    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub fee: u64,

    #[serde(rename = "fv")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub first_valid: u64,

    #[serde(rename = "lv")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub last_valid: u64,

    #[serde(rename = "gh")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    #[builder(default)]
    pub genesis_hash: Option<Byte32>,

    #[serde(rename = "gen")]
    #[serde(skip_serializing_if = "is_empty_string_opt")]
    #[serde(default)]
    #[builder(default)]
    pub genesis_id: Option<String>,

    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub note: Option<Vec<u8>>,

    #[serde(rename = "rekey")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    #[builder(default)]
    pub rekey_to: Option<Address>,

    #[serde(rename = "lx")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    #[builder(default)]
    pub lease: Option<Byte32>,

    #[serde(rename = "grp")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_bytes32_opt")]
    #[serde(default)]
    #[builder(default)]
    pub group: Option<Byte32>,
}

impl AlgorandMsgpack for TransactionHeader {}
