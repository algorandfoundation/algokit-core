use crate::msgpack::{
    encode_value_to_msgpack, AlgoKitMsgPackError, ModelHandler, ModelRegistry, ModelType, Result,
};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -----------------------------
// Simulation request structures
// -----------------------------

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateRequest {
    #[serde(rename = "allow-empty-signatures")]
    pub allow_empty_signatures: Option<bool>,
    #[serde(rename = "allow-more-logging")]
    pub allow_more_logging: Option<bool>,
    #[serde(rename = "allow-unnamed-resources")]
    pub allow_unnamed_resources: Option<bool>,
    #[serde(rename = "exec-trace-config")]
    pub exec_trace_config: Option<SimulateTraceConfig>,
    #[serde(rename = "extra-opcode-budget")]
    pub extra_opcode_budget: Option<i64>,
    #[serde(rename = "fix-signers")]
    pub fix_signers: Option<bool>,
    #[serde(rename = "round")]
    pub round: Option<i64>,
    #[serde(rename = "txn-groups")]
    pub txn_groups: Vec<SimulateRequestTransactionGroup>,
}

impl SimulateRequest {
    pub fn new(txn_groups: Vec<SimulateRequestTransactionGroup>) -> Self {
        Self {
            txn_groups,
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTraceConfig {
    #[serde(rename = "enable")]
    pub enable: Option<bool>,
    #[serde(rename = "scratch-change")]
    pub scratch_change: Option<bool>,
    #[serde(rename = "stack-change")]
    pub stack_change: Option<bool>,
    #[serde(rename = "state-change")]
    pub state_change: Option<bool>,
}

impl SimulateTraceConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateRequestTransactionGroup {
    #[serde(rename = "txns")]
    pub txns: Vec<String>,
}

impl SimulateRequestTransactionGroup {
    pub fn new(txns: Vec<String>) -> Self {
        Self { txns }
    }
}

// ---------------------
// Simulation responses
// ---------------------

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateUnnamedResourcesAccessed {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationTransactionExecTrace {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationEvalOverrides {
    #[serde(rename = "allow-empty-signatures")]
    pub allow_empty_signatures: Option<bool>,
    #[serde(rename = "allow-unnamed-resources")]
    pub allow_unnamed_resources: Option<bool>,
    #[serde(rename = "max-log-calls")]
    pub max_log_calls: Option<i64>,
    #[serde(rename = "max-log-size")]
    pub max_log_size: Option<i64>,
    #[serde(rename = "extra-opcode-budget")]
    pub extra_opcode_budget: Option<i64>,
    #[serde(rename = "fix-signers")]
    pub fix_signers: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateInitialStates {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransactionResult {
    #[serde(rename = "txn-result")]
    pub txn_result: serde_json::Value,
    #[serde(rename = "app-budget-consumed")]
    pub app_budget_consumed: Option<i64>,
    #[serde(rename = "logic-sig-budget-consumed")]
    pub logic_sig_budget_consumed: Option<i64>,
    #[serde(rename = "exec-trace")]
    pub exec_trace: Option<SimulationTransactionExecTrace>,
    #[serde(rename = "unnamed-resources-accessed")]
    pub unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
    #[serde(rename = "fixed-signer")]
    pub fixed_signer: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransactionGroupResult {
    #[serde(rename = "txn-results")]
    pub txn_results: Vec<SimulateTransactionResult>,
    #[serde(rename = "failure-message")]
    pub failure_message: Option<String>,
    #[serde(rename = "failed-at")]
    pub failed_at: Option<Vec<i64>>, // path indices
    #[serde(rename = "app-budget-added")]
    pub app_budget_added: Option<i64>,
    #[serde(rename = "app-budget-consumed")]
    pub app_budget_consumed: Option<i64>,
    #[serde(rename = "unnamed-resources-accessed")]
    pub unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransaction200Response {
    pub version: i64,
    #[serde(rename = "last-round")]
    pub last_round: i64,
    #[serde(rename = "txn-groups")]
    pub txn_groups: Vec<SimulateTransactionGroupResult>,
    #[serde(rename = "eval-overrides")]
    pub eval_overrides: Option<SimulationEvalOverrides>,
    #[serde(rename = "exec-trace-config")]
    pub exec_trace_config: Option<SimulateTraceConfig>,
    #[serde(rename = "initial-states")]
    pub initial_states: Option<SimulateInitialStates>,
}

// --------------------------------------
// Custom encoding/decoding for simulation
// --------------------------------------

struct SimulateRequestHandler;

impl ModelHandler for SimulateRequestHandler {
    fn encode_json_to_msgpack(&self, json_str: &str) -> Result<Vec<u8>> {
        let json_value: serde_json::Value = serde_json::from_str(json_str)?;
        let mut buf = Vec::new();
        match &json_value {
            serde_json::Value::Object(map) => {
                // Sort keys for consistent output
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                rmp::encode::write_map_len(&mut buf, map.len() as u32)?;
                for key in keys {
                    let value = map.get(key).expect("key just taken exists");
                    rmp::encode::write_str(&mut buf, key)?;
                    match key.as_str() {
                        "txn-groups" => Self::encode_txn_groups(&mut buf, value)?,
                        _ => encode_value_to_msgpack(value, &mut buf)?,
                    }
                }
            }
            _ => {
                return Err(AlgoKitMsgPackError::MsgpackWriteError(
                    "Expected JSON object".into(),
                ))
            }
        }
        Ok(buf)
    }

    fn decode_msgpack_to_json(&self, _msgpack_bytes: &[u8]) -> Result<String> {
        Err(AlgoKitMsgPackError::MsgpackWriteError(
            "Simulate request decoding is not supported".into(),
        ))
    }
}

impl SimulateRequestHandler {
    fn encode_txn_groups(buf: &mut Vec<u8>, value: &serde_json::Value) -> Result<()> {
        if let serde_json::Value::Array(groups) = value {
            rmp::encode::write_array_len(buf, groups.len() as u32)?;
            for group in groups {
                match group {
                    serde_json::Value::Object(group_obj) => {
                        rmp::encode::write_map_len(buf, group_obj.len() as u32)?;
                        for (key, val) in group_obj {
                            rmp::encode::write_str(buf, key)?;
                            if key == "txns" {
                                if let serde_json::Value::Array(txns) = val {
                                    rmp::encode::write_array_len(buf, txns.len() as u32)?;
                                    for txn in txns {
                                        match txn {
                                            serde_json::Value::String(s) => {
                                                buf.extend_from_slice(&BASE64.decode(s)?);
                                            }
                                            _ => encode_value_to_msgpack(txn, buf)?,
                                        }
                                    }
                                } else {
                                    encode_value_to_msgpack(val, buf)?;
                                }
                            } else {
                                encode_value_to_msgpack(val, buf)?;
                            }
                        }
                    }
                    _ => encode_value_to_msgpack(group, buf)?,
                }
            }
        } else {
            encode_value_to_msgpack(value, buf)?;
        }
        Ok(())
    }

    fn rmpv_to_json(value: &rmpv::Value) -> serde_json::Value {
        use rmpv::Value as V;
        match value {
            V::Nil => serde_json::Value::Null,
            V::Boolean(b) => serde_json::Value::Bool(*b),
            V::Integer(i) => {
                if let Some(n) = i.as_i64() {
                    serde_json::Value::Number(n.into())
                } else if let Some(n) = i.as_u64() {
                    serde_json::Value::Number(serde_json::Number::from(n))
                } else {
                    serde_json::Value::String(i.to_string())
                }
            }
            V::F32(f) => serde_json::Number::from_f64(*f as f64).unwrap().into(),
            V::F64(f) => serde_json::Number::from_f64(*f).unwrap().into(),
            V::String(s) => serde_json::Value::String(s.as_str().unwrap_or_default().into()),
            V::Binary(b) | V::Ext(_, b) => serde_json::Value::String(BASE64.encode(b)),
            V::Array(arr) => serde_json::Value::Array(arr.iter().map(Self::rmpv_to_json).collect()),
            V::Map(map) => {
                let mut m = serde_json::Map::with_capacity(map.len());
                for (k, v) in map {
                    let key = match k {
                        V::String(s) => s.as_str().unwrap_or_default().to_string(),
                        _ => k.to_string(),
                    };
                    m.insert(key, Self::rmpv_to_json(v));
                }
                serde_json::Value::Object(m)
            }
        }
    }
}

struct SimulateResponseHandler;

impl ModelHandler for SimulateResponseHandler {
    fn encode_json_to_msgpack(&self, json_str: &str) -> Result<Vec<u8>> {
        let v: serde_json::Value = serde_json::from_str(json_str)?;
        Ok(rmp_serde::to_vec_named(&v)?)
    }

    fn decode_msgpack_to_json(&self, msgpack_bytes: &[u8]) -> Result<String> {
        use std::io::Cursor;
        let mut cursor = Cursor::new(msgpack_bytes);
        let root: rmpv::Value = rmpv::decode::read_value(&mut cursor)
            .map_err(|e| AlgoKitMsgPackError::IoError(e.to_string()))?;
        let json_val = SimulateRequestHandler::rmpv_to_json(&root);
        Ok(serde_json::to_string(&json_val)?)
    }
}

// -----------------------------
// Registration helper
// -----------------------------

pub fn register_simulation_models(registry: &mut ModelRegistry) {
    registry
        .registry
        .insert(ModelType::SimulateRequest, Box::new(SimulateRequestHandler));
    registry.registry.insert(
        ModelType::SimulateTransaction200Response,
        Box::new(SimulateResponseHandler),
    );
}
