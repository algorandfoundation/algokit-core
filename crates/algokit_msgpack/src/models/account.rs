use crate::{ModelHandler, MsgPackError, Result};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub amount: i64,
    #[serde(rename = "min-balance")]
    pub min_balance: i64,
    #[serde(rename = "amount-without-pending-rewards")]
    pub amount_without_pending_rewards: i64,
    #[serde(rename = "apps-local-state")]
    pub apps_local_state: Option<Vec<ApplicationLocalState>>,
    #[serde(rename = "total-apps-opted-in")]
    pub total_apps_opted_in: i64,
    #[serde(rename = "apps-total-schema")]
    pub apps_total_schema: Option<ApplicationStateSchema>,
    #[serde(rename = "apps-total-extra-pages")]
    pub apps_total_extra_pages: Option<i64>,
    pub assets: Option<Vec<AssetHolding>>,
    #[serde(rename = "total-assets-opted-in")]
    pub total_assets_opted_in: i64,
    #[serde(rename = "created-apps")]
    pub created_apps: Option<Vec<Application>>,
    #[serde(rename = "total-created-apps")]
    pub total_created_apps: i64,
    #[serde(rename = "created-assets")]
    pub created_assets: Option<Vec<Asset>>,
    #[serde(rename = "total-created-assets")]
    pub total_created_assets: i64,
    #[serde(rename = "total-boxes")]
    pub total_boxes: Option<i64>,
    #[serde(rename = "total-box-bytes")]
    pub total_box_bytes: Option<i64>,
    pub participation: Option<AccountParticipation>,
    #[serde(rename = "incentive-eligible")]
    pub incentive_eligible: Option<bool>,
    #[serde(rename = "pending-rewards")]
    pub pending_rewards: i64,
    #[serde(rename = "reward-base")]
    pub reward_base: Option<i64>,
    pub rewards: i64,
    pub round: i64,
    pub status: String,
    #[serde(rename = "sig-type")]
    pub sig_type: Option<String>,
    #[serde(rename = "auth-addr")]
    pub auth_addr: Option<String>,
    #[serde(rename = "last-proposed")]
    pub last_proposed: Option<i64>,
    #[serde(rename = "last-heartbeat")]
    pub last_heartbeat: Option<i64>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationLocalState {
    pub id: i64,
    pub schema: ApplicationStateSchema,
    #[serde(rename = "key-value")]
    pub key_value: Option<Vec<TealKeyValue>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationStateSchema {
    #[serde(rename = "num-uint")]
    pub num_uint: i64,
    #[serde(rename = "num-byte-slice")]
    pub num_byte_slice: i64,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TealKeyValue {
    pub key: String,
    pub value: TealValue,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TealValue {
    #[serde(rename = "type")]
    pub value_type: i64,
    pub bytes: String,
    pub uint: i64,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetHolding {
    pub amount: i64,
    #[serde(rename = "asset-id")]
    pub asset_id: i64,
    #[serde(rename = "is-frozen")]
    pub is_frozen: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub params: ApplicationParams,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationParams {
    pub creator: String,
    #[serde(rename = "approval-program")]
    pub approval_program: String,
    #[serde(rename = "clear-state-program")]
    pub clear_state_program: String,
    #[serde(rename = "extra-program-pages")]
    pub extra_program_pages: Option<i64>,
    #[serde(rename = "local-state-schema")]
    pub local_state_schema: Option<ApplicationStateSchema>,
    #[serde(rename = "global-state-schema")]
    pub global_state_schema: Option<ApplicationStateSchema>,
    #[serde(rename = "global-state")]
    pub global_state: Option<Vec<TealKeyValue>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Asset {
    pub index: i64,
    pub params: AssetParams,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetParams {
    pub clawback: Option<String>,
    pub creator: String,
    pub decimals: i64,
    #[serde(rename = "default-frozen")]
    pub default_frozen: Option<bool>,
    pub freeze: Option<String>,
    pub manager: Option<String>,
    #[serde(rename = "metadata-hash")]
    pub metadata_hash: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "name-b64")]
    pub name_b64: Option<String>,
    pub reserve: Option<String>,
    pub total: i64,
    #[serde(rename = "unit-name")]
    pub unit_name: Option<String>,
    #[serde(rename = "unit-name-b64")]
    pub unit_name_b64: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "url-b64")]
    pub url_b64: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountParticipation {
    #[serde(rename = "selection-participation-key")]
    pub selection_participation_key: String,
    #[serde(rename = "vote-first-valid")]
    pub vote_first_valid: i64,
    #[serde(rename = "vote-key-dilution")]
    pub vote_key_dilution: i64,
    #[serde(rename = "vote-last-valid")]
    pub vote_last_valid: i64,
    #[serde(rename = "vote-participation-key")]
    pub vote_participation_key: String,
    #[serde(rename = "state-proof-key")]
    pub state_proof_key: Option<String>,
}

pub struct AccountHandler;

impl ModelHandler for AccountHandler {
    /// The encode_json_to_msgpack function for Accounts is not implemented as it's typically
    /// not needed. The algod API only returns account data in MessagePack format, it doesn't
    /// accept account data as input.
    fn encode_json_to_msgpack(&self, _json_str: &str) -> Result<Vec<u8>> {
        Err(MsgPackError::MsgPackWriteError(
            "Account encoding is not implemented".into(),
        ))
    }

    fn decode_msgpack_to_json(&self, msgpack_bytes: &[u8]) -> Result<String> {
        // First try to parse as a full Account
        let result = rmp_serde::from_slice::<Account>(msgpack_bytes);

        if let Ok(account) = result {
            return Ok(serde_json::to_string(&account)?);
        }

        // Try parsing with the more flexible rmpv::Value
        use std::io::Cursor;
        let mut cursor = Cursor::new(msgpack_bytes);
        match rmpv::decode::read_value(&mut cursor) {
            Ok(root) => {
                // Convert rmpv::Value to serde_json::Value
                let json_value = Self::rmpv_to_json_value(&root);

                // Create a new Account object to populate with data from the JSON
                let mut account = Account::default();

                // Process "algo" field (account balance and basic info)
                if let Some(algo_value) = json_value.get("algo") {
                    if let Some(amount) = algo_value.as_u64() {
                        account.amount = amount as i64;
                    } else if let Some(amount) = algo_value.as_i64() {
                        account.amount = amount;
                    }
                }

                // Process "spend" field (auth address shorthand)
                if let Some(spend_value) = json_value.get("spend") {
                    if let Some(addr_str) = spend_value.as_str() {
                        account.auth_addr = Some(addr_str.to_string());
                    }
                }

                // Process "apar" field (asset parameters)
                if let Some(apar_value) = json_value.get("apar") {
                    if let Some(apar_map) = apar_value.as_object() {
                        let mut created_assets = Vec::new();

                        for (asset_id_str, asset_params) in apar_map {
                            if let Ok(asset_id) = asset_id_str.parse::<i64>() {
                                if let Some(params_obj) = asset_params.as_object() {
                                    let mut asset_params = AssetParams {
                                        creator: account.address.clone(),
                                        total: 0,
                                        decimals: 0,
                                        ..Default::default()
                                    };

                                    // Extract asset parameters
                                    if let Some(name) = params_obj.get("an") {
                                        asset_params.name = name.as_str().map(String::from);
                                    }

                                    if let Some(unit_name) = params_obj.get("un") {
                                        asset_params.unit_name =
                                            unit_name.as_str().map(String::from);
                                    }

                                    if let Some(total) = params_obj.get("t") {
                                        if let Some(t) = total.as_u64() {
                                            asset_params.total = t as i64;
                                        } else if let Some(t) = total.as_i64() {
                                            asset_params.total = t;
                                        }
                                    }

                                    if let Some(url) = params_obj.get("au") {
                                        asset_params.url = url.as_str().map(String::from);
                                    }

                                    if let Some(decimals) = params_obj.get("dc") {
                                        if let Some(d) = decimals.as_u64() {
                                            asset_params.decimals = d as i64;
                                        } else if let Some(d) = decimals.as_i64() {
                                            asset_params.decimals = d;
                                        }
                                    }

                                    // Add to created assets
                                    created_assets.push(Asset {
                                        index: asset_id,
                                        params: asset_params,
                                    });
                                }
                            }
                        }

                        if !created_assets.is_empty() {
                            account.total_created_assets = created_assets.len() as i64;
                            account.created_assets = Some(created_assets);
                        }
                    }
                }

                // Process "asset" field (assets owned by the account)
                if let Some(asset_value) = json_value.get("asset") {
                    if let Some(asset_map) = asset_value.as_object() {
                        let mut asset_holdings = Vec::new();

                        for (asset_id_str, asset_data) in asset_map {
                            if let Ok(asset_id) = asset_id_str.parse::<i64>() {
                                if let Some(asset_obj) = asset_data.as_object() {
                                    let mut asset_holding = AssetHolding {
                                        asset_id,
                                        amount: 0,
                                        is_frozen: false,
                                    };

                                    // Extract asset holding data
                                    if let Some(amount) = asset_obj.get("a") {
                                        if let Some(a) = amount.as_u64() {
                                            asset_holding.amount = a as i64;
                                        } else if let Some(a) = amount.as_i64() {
                                            asset_holding.amount = a;
                                        }
                                    }

                                    if let Some(frozen) = asset_obj.get("f") {
                                        asset_holding.is_frozen = frozen.as_bool().unwrap_or(false);
                                    }

                                    asset_holdings.push(asset_holding);
                                }
                            }
                        }

                        if !asset_holdings.is_empty() {
                            account.total_assets_opted_in = asset_holdings.len() as i64;
                            account.assets = Some(asset_holdings);
                        }
                    }
                }

                // Process "appp" field (created applications)
                if let Some(appp_value) = json_value.get("appp") {
                    if let Some(appp_map) = appp_value.as_object() {
                        let mut created_apps = Vec::new();

                        for (app_id_str, app_data) in appp_map {
                            if let Ok(app_id) = app_id_str.parse::<i64>() {
                                if let Some(app_obj) = app_data.as_object() {
                                    let mut app_params = ApplicationParams {
                                        creator: account.address.clone(),
                                        approval_program: String::new(),
                                        clear_state_program: String::new(),
                                        ..Default::default()
                                    };

                                    // Extract application parameters
                                    if let Some(approval_val) =
                                        app_obj.get("approv").or(app_obj.get("apap"))
                                    {
                                        if let Some(s) = approval_val.as_str() {
                                            app_params.approval_program = s.to_string();
                                        }
                                    }

                                    if let Some(clear_val) =
                                        app_obj.get("clearp").or(app_obj.get("apsu"))
                                    {
                                        if let Some(s) = clear_val.as_str() {
                                            app_params.clear_state_program = s.to_string();
                                        }
                                    }

                                    created_apps.push(Application {
                                        id: app_id,
                                        params: app_params,
                                    });
                                }
                            }
                        }

                        if !created_apps.is_empty() {
                            account.total_created_apps = created_apps.len() as i64;
                            account.created_apps = Some(created_apps);
                        }
                    }
                }

                // Process "appl" field (opted-in applications)
                if let Some(appl_value) = json_value.get("appl") {
                    if let Some(appl_map) = appl_value.as_object() {
                        let mut local_states = Vec::new();

                        for (app_id_str, _app_data) in appl_map {
                            if let Ok(app_id) = app_id_str.parse::<i64>() {
                                let local_state = ApplicationLocalState {
                                    id: app_id,
                                    schema: ApplicationStateSchema::default(),
                                    key_value: None,
                                };

                                local_states.push(local_state);
                            }
                        }

                        if !local_states.is_empty() {
                            account.total_apps_opted_in = local_states.len() as i64;
                            account.apps_local_state = Some(local_states);
                        }
                    }
                }

                // If we have min balance information
                if let Some(min_balance) = json_value.get("min-balance") {
                    if let Some(mb) = min_balance.as_u64() {
                        account.min_balance = mb as i64;
                    } else if let Some(mb) = min_balance.as_i64() {
                        account.min_balance = mb;
                    }
                }

                // Additional shorthand alias handling based on algod spec

                // "tsch" -> apps_total_schema (aggregate schema counts)
                if let Some(tsch_value) = json_value.get("tsch") {
                    if let Some(obj) = tsch_value.as_object() {
                        let mut schema = ApplicationStateSchema::default();
                        if let Some(nui_val) = obj.get("nui") {
                            if let Some(v) = nui_val.as_u64() {
                                schema.num_uint = v as i64;
                            } else if let Some(v) = nui_val.as_i64() {
                                schema.num_uint = v;
                            }
                        }
                        if let Some(nbs_val) = obj.get("nbs") {
                            if let Some(v) = nbs_val.as_u64() {
                                schema.num_byte_slice = v as i64;
                            } else if let Some(v) = nbs_val.as_i64() {
                                schema.num_byte_slice = v;
                            }
                        }
                        account.apps_total_schema = Some(schema);
                    }
                }

                // "teap" -> apps_total_extra_pages
                if let Some(val) = json_value.get("teap") {
                    if let Some(v) = val.as_u64() {
                        account.apps_total_extra_pages = Some(v as i64);
                    } else if let Some(v) = val.as_i64() {
                        account.apps_total_extra_pages = Some(v);
                    }
                }

                // "tbx" -> total_boxes
                if let Some(val) = json_value.get("tbx") {
                    if let Some(v) = val.as_u64() {
                        account.total_boxes = Some(v as i64);
                    } else if let Some(v) = val.as_i64() {
                        account.total_boxes = Some(v);
                    }
                }

                // "tbxb" -> total_box_bytes
                if let Some(val) = json_value.get("tbxb") {
                    if let Some(v) = val.as_u64() {
                        account.total_box_bytes = Some(v as i64);
                    } else if let Some(v) = val.as_i64() {
                        account.total_box_bytes = Some(v);
                    }
                }

                // "ebase" -> reward_base
                if let Some(val) = json_value.get("ebase") {
                    if let Some(v) = val.as_u64() {
                        account.reward_base = Some(v as i64);
                    } else if let Some(v) = val.as_i64() {
                        account.reward_base = Some(v);
                    }
                }

                // "ern" -> rewards
                if let Some(val) = json_value.get("ern") {
                    if let Some(v) = val.as_u64() {
                        account.rewards = v as i64;
                    } else if let Some(v) = val.as_i64() {
                        account.rewards = v;
                    }
                }

                // "onl" -> status
                if let Some(val) = json_value.get("onl") {
                    if let Some(s) = val.as_str() {
                        account.status = s.to_string();
                    }
                }

                // Direct field: "amount-without-pending-rewards"
                if let Some(val) = json_value.get("amount-without-pending-rewards") {
                    if let Some(v) = val.as_u64() {
                        account.amount_without_pending_rewards = v as i64;
                    } else if let Some(v) = val.as_i64() {
                        account.amount_without_pending_rewards = v;
                    }
                }

                // Standard "round" property
                if let Some(val) = json_value.get("round") {
                    if let Some(r) = val.as_u64() {
                        account.round = r as i64;
                    } else if let Some(r) = val.as_i64() {
                        account.round = r;
                    }
                }

                // If the original data didn't have an explicit address but we need one
                if account.address.is_empty() {
                    // If still empty, use an empty address as placeholder
                    if account.address.is_empty() {
                        account.address = "".to_string();
                    }
                }

                Ok(serde_json::to_string(&account)?)
            }
            Err(e) => Err(MsgPackError::IoError(e.to_string())),
        }
    }
}

impl AccountHandler {
    // Helper method to convert rmpv::Value to serde_json::Value
    fn rmpv_to_json_value(value: &rmpv::Value) -> serde_json::Value {
        match value {
            rmpv::Value::Nil => serde_json::Value::Null,
            rmpv::Value::Boolean(b) => serde_json::Value::Bool(*b),
            rmpv::Value::Integer(i) => {
                if let Some(n) = i.as_i64() {
                    serde_json::Value::Number(serde_json::Number::from(n))
                } else if let Some(n) = i.as_u64() {
                    serde_json::Value::Number(serde_json::Number::from(n))
                } else {
                    serde_json::Value::Null
                }
            }
            rmpv::Value::F32(f) => {
                if let Some(n) = serde_json::Number::from_f64(*f as f64) {
                    serde_json::Value::Number(n)
                } else {
                    serde_json::Value::Null
                }
            }
            rmpv::Value::F64(f) => {
                if let Some(n) = serde_json::Number::from_f64(*f) {
                    serde_json::Value::Number(n)
                } else {
                    serde_json::Value::Null
                }
            }
            rmpv::Value::String(s) => {
                serde_json::Value::String(s.as_str().unwrap_or_default().to_string())
            }
            rmpv::Value::Binary(b) => {
                // Encode binary data as base64 so that bytecode / hashes are
                // represented in a compact, portable form matching algod's
                // JSON output (e.g. approval / clear programs, metadata hash).
                // This also means downstream consumers can directly reuse the
                // value without additional transformations.
                let b64 = BASE64.encode(b);
                serde_json::Value::String(b64)
            }
            rmpv::Value::Array(arr) => {
                let values: Vec<serde_json::Value> =
                    arr.iter().map(Self::rmpv_to_json_value).collect();
                serde_json::Value::Array(values)
            }
            rmpv::Value::Map(m) => {
                let mut map = serde_json::Map::new();
                for (k, v) in m {
                    // Convert both string and integer keys to string
                    let key = match k {
                        rmpv::Value::String(s) => s.as_str().unwrap_or_default().to_string(),
                        rmpv::Value::Integer(i) => {
                            if let Some(n) = i.as_i64() {
                                n.to_string()
                            } else if let Some(n) = i.as_u64() {
                                n.to_string()
                            } else {
                                "unknown".to_string()
                            }
                        }
                        _ => continue, // Skip non-string/non-integer keys
                    };
                    map.insert(key, Self::rmpv_to_json_value(v));
                }
                serde_json::Value::Object(map)
            }
            rmpv::Value::Ext(_, _) => serde_json::Value::Null, // Ignore Ext values
        }
    }
}

pub fn register_account_model(registry: &mut crate::ModelRegistry) {
    use crate::ModelType;
    registry
        .registry
        .insert(ModelType::Account, Box::new(AccountHandler));
}
