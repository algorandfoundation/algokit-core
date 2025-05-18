use crate::{ModelHandler, ModelRegistry, ModelType, MsgPackError, Result};
use serde::{Deserialize, Serialize};

/// SimulateRequest : Request type for simulation endpoint.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateRequest {
    /// Allows transactions without signatures to be simulated as if they had correct signatures.
    #[serde(
        rename = "allow-empty-signatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_empty_signatures: Option<bool>,

    /// Lifts limits on log opcode usage during simulation.
    #[serde(rename = "allow-more-logging", skip_serializing_if = "Option::is_none")]
    pub allow_more_logging: Option<bool>,

    /// Allows access to unnamed resources during simulation.
    #[serde(
        rename = "allow-unnamed-resources",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_unnamed_resources: Option<bool>,

    /// Configuration for execution traces.
    #[serde(rename = "exec-trace-config", skip_serializing_if = "Option::is_none")]
    pub exec_trace_config: Option<SimulateTraceConfig>,

    /// Applies extra opcode budget during simulation for each transaction group.
    #[serde(
        rename = "extra-opcode-budget",
        skip_serializing_if = "Option::is_none"
    )]
    pub extra_opcode_budget: Option<i64>,

    /// If true, signers for transactions that are missing signatures will be fixed during evaluation.
    #[serde(rename = "fix-signers", skip_serializing_if = "Option::is_none")]
    pub fix_signers: Option<bool>,

    /// If provided, specifies the round preceding the simulation. State changes through this round will be used to run this simulation.
    #[serde(rename = "round", skip_serializing_if = "Option::is_none")]
    pub round: Option<i64>,

    /// The transaction groups to simulate.
    #[serde(rename = "txn-groups")]
    pub txn_groups: Vec<SimulateRequestTransactionGroup>,
}

impl SimulateRequest {
    /// Request type for simulation endpoint.
    pub fn new(txn_groups: Vec<SimulateRequestTransactionGroup>) -> SimulateRequest {
        SimulateRequest {
            txn_groups,
            allow_empty_signatures: None,
            allow_more_logging: None,
            allow_unnamed_resources: None,
            exec_trace_config: None,
            extra_opcode_budget: None,
            fix_signers: None,
            round: None,
        }
    }
}

/// An object that configures simulation execution trace.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTraceConfig {
    /// A boolean option for opting in execution trace features simulation endpoint.
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// A boolean option enabling returning scratch slot changes together with execution trace during simulation.
    #[serde(rename = "scratch-change", skip_serializing_if = "Option::is_none")]
    pub scratch_change: Option<bool>,

    /// A boolean option enabling returning stack changes together with execution trace during simulation.
    #[serde(rename = "stack-change", skip_serializing_if = "Option::is_none")]
    pub stack_change: Option<bool>,

    /// A boolean option enabling returning application state changes (global, local, and box changes) with the execution trace during simulation.
    #[serde(rename = "state-change", skip_serializing_if = "Option::is_none")]
    pub state_change: Option<bool>,
}

impl SimulateTraceConfig {
    pub fn new() -> SimulateTraceConfig {
        SimulateTraceConfig {
            enable: None,
            scratch_change: None,
            stack_change: None,
            state_change: None,
        }
    }
}

/// SimulateRequestTransactionGroup : A transaction group to simulate.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateRequestTransactionGroup {
    /// An atomic transaction group.
    #[serde(rename = "txns")]
    pub txns: Vec<String>,
}

impl SimulateRequestTransactionGroup {
    /// A transaction group to simulate.
    pub fn new(txns: Vec<String>) -> SimulateRequestTransactionGroup {
        SimulateRequestTransactionGroup { txns }
    }
}

/// SimulationUnnamedResourcesAccessed : Placeholder type representing unnamed resources accessed during simulation.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateUnnamedResourcesAccessed {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

/// SimulationTransactionExecTrace : Placeholder for complex exec trace data structure.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationTransactionExecTrace {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

/// SimulationEvalOverrides : Model describing evaluation parameter overrides during simulation.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationEvalOverrides {
    #[serde(
        rename = "allow-empty-signatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_empty_signatures: Option<bool>,

    #[serde(
        rename = "allow-unnamed-resources",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_unnamed_resources: Option<bool>,

    #[serde(rename = "max-log-calls", skip_serializing_if = "Option::is_none")]
    pub max_log_calls: Option<i64>,

    #[serde(rename = "max-log-size", skip_serializing_if = "Option::is_none")]
    pub max_log_size: Option<i64>,

    #[serde(
        rename = "extra-opcode-budget",
        skip_serializing_if = "Option::is_none"
    )]
    pub extra_opcode_budget: Option<i64>,

    #[serde(rename = "fix-signers", skip_serializing_if = "Option::is_none")]
    pub fix_signers: Option<bool>,
}

/// SimulateInitialStates : Placeholder for initial states accessed during simulation.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateInitialStates {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

/// SimulateTransactionResult : Result for an individual transaction inside simulation response.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransactionResult {
    #[serde(rename = "txn-result")]
    pub txn_result: serde_json::Value,

    #[serde(
        rename = "app-budget-consumed",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_budget_consumed: Option<i64>,

    #[serde(
        rename = "logic-sig-budget-consumed",
        skip_serializing_if = "Option::is_none"
    )]
    pub logic_sig_budget_consumed: Option<i64>,

    #[serde(rename = "exec-trace", skip_serializing_if = "Option::is_none")]
    pub exec_trace: Option<SimulationTransactionExecTrace>,

    #[serde(
        rename = "unnamed-resources-accessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,

    #[serde(rename = "fixed-signer", skip_serializing_if = "Option::is_none")]
    pub fixed_signer: Option<String>,
}

/// SimulateTransactionGroupResult : Result for a group within simulation response.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransactionGroupResult {
    #[serde(rename = "txn-results")]
    pub txn_results: Vec<SimulateTransactionResult>,

    #[serde(rename = "failure-message", skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,

    #[serde(rename = "failed-at", skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<Vec<i64>>, // path indices

    #[serde(rename = "app-budget-added", skip_serializing_if = "Option::is_none")]
    pub app_budget_added: Option<i64>,

    #[serde(
        rename = "app-budget-consumed",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_budget_consumed: Option<i64>,

    #[serde(
        rename = "unnamed-resources-accessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
}

/// SimulateTransaction200Response : Successful response schema for /v2/transactions/simulate.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulateTransaction200Response {
    pub version: i64,

    #[serde(rename = "last-round")]
    pub last_round: i64,

    #[serde(rename = "txn-groups")]
    pub txn_groups: Vec<SimulateTransactionGroupResult>,

    #[serde(rename = "eval-overrides", skip_serializing_if = "Option::is_none")]
    pub eval_overrides: Option<SimulationEvalOverrides>,

    #[serde(rename = "exec-trace-config", skip_serializing_if = "Option::is_none")]
    pub exec_trace_config: Option<SimulateTraceConfig>,

    #[serde(rename = "initial-states", skip_serializing_if = "Option::is_none")]
    pub initial_states: Option<SimulateInitialStates>,
}

/// Custom handler for SimulateRequest to properly handle base64 transaction data
struct SimulateRequestHandler {}

impl SimulateRequestHandler {
    fn new() -> Self {
        Self {}
    }
}

impl ModelHandler for SimulateRequestHandler {
    fn encode_json_to_msgpack(&self, json_str: &str) -> Result<Vec<u8>> {
        // Parse the JSON string to process transactions
        let json_value: serde_json::Value = serde_json::from_str(json_str)?;

        // Direct encoding to avoid the verbose msgpack format
        // This mimics what algosdk does with the transactions
        let mut buf = Vec::new();

        // Start encoding the object
        if let serde_json::Value::Object(map) = &json_value {
            // First sort keys to ensure consistent output
            let mut sorted_keys: Vec<&String> = map.keys().collect();
            sorted_keys.sort();

            // Write map header
            rmp::encode::write_map_len(&mut buf, map.len() as u32)?;

            for key in sorted_keys {
                if let Some(value) = map.get(key) {
                    // Write key
                    rmp::encode::write_str(&mut buf, key)?;

                    // Write value based on type
                    match key.as_str() {
                        "txn-groups" => {
                            // Special handling for txn-groups to process base64 encoded transactions
                            self.encode_txn_groups(&mut buf, value)?;
                        }
                        _ => {
                            // For other fields, use generic msgpack encoding
                            self.encode_value(&mut buf, value)?;
                        }
                    }
                }
            }
        } else {
            return Err(MsgPackError::MsgPackWriteError(
                "Expected JSON object".to_string(),
            ));
        }

        Ok(buf)
    }

    fn decode_msgpack_to_json(&self, msgpack_bytes: &[u8]) -> Result<String> {
        // Custom decode implementation which correctly handles MessagePack binary values.
        // Using `rmpv` we can obtain a tree where binary values are represented explicitly.

        use rmpv::Value as RmpvValue;
        use std::io::Cursor;

        // Decode the root MessagePack value first.
        let mut cursor = Cursor::new(msgpack_bytes);
        let root_value: RmpvValue = rmpv::decode::read_value(&mut cursor)
            .map_err(|e| MsgPackError::IoError(e.to_string()))?;

        // Recursively transform it into a serde_json::Value converting binary to base64 strings.
        let mut json_value = Self::rmpv_to_json(&root_value);

        // Ensure we still post-process txn-groups for any edge-cases left (e.g. maps that have been
        // converted to JSON objects rather than binary). This keeps behaviour identical to the old
        // implementation while fixing the binary decode issue that was causing a panic.
        self.process_txn_groups_for_decode(&mut json_value)?;

        Ok(serde_json::to_string(&json_value)?)
    }
}

// Additional implementation methods for SimulateRequestHandler
impl SimulateRequestHandler {
    // Helper to encode a JSON value to MessagePack
    fn encode_value(&self, buf: &mut Vec<u8>, value: &serde_json::Value) -> Result<()> {
        // Re-use the crate level helper so that we have a single, well-tested
        // implementation for "generic" value encoding. This eliminates a large
        // chunk of duplicated logic that was previously living in this handler.
        crate::encode_value_to_msgpack(value, buf)
    }

    // Special handling for txn-groups to process the transactions efficiently
    fn encode_txn_groups(&self, buf: &mut Vec<u8>, value: &serde_json::Value) -> Result<()> {
        if let serde_json::Value::Array(groups) = value {
            // Write array header for txn-groups
            rmp::encode::write_array_len(buf, groups.len() as u32)?;

            for group in groups {
                if let serde_json::Value::Object(group_obj) = group {
                    // Write each group as an object
                    rmp::encode::write_map_len(buf, group_obj.len() as u32)?;

                    for (key, value) in group_obj {
                        // Write key
                        rmp::encode::write_str(buf, key)?;

                        if key == "txns" {
                            // Special handling for transactions array
                            if let serde_json::Value::Array(txns) = value {
                                // Write array header for transactions
                                rmp::encode::write_array_len(buf, txns.len() as u32)?;

                                for txn in txns {
                                    if let serde_json::Value::String(base64_txn) = txn {
                                        // Decode base64 to get the raw msgpack bytes
                                        let txn_bytes = base64::decode(base64_txn)?;

                                        // Write the raw txn bytes directly without re-encoding
                                        buf.extend_from_slice(&txn_bytes);
                                    } else {
                                        // If not a string, use generic encoding
                                        self.encode_value(buf, txn)?;
                                    }
                                }
                            } else {
                                // If txns isn't an array, encode normally
                                self.encode_value(buf, value)?;
                            }
                        } else {
                            // For other fields in the group, use standard encoding
                            self.encode_value(buf, value)?;
                        }
                    }
                } else {
                    // If the group isn't an object, encode normally
                    self.encode_value(buf, group)?;
                }
            }
        } else {
            // If txn-groups isn't an array, encode normally
            self.encode_value(buf, value)?;
        }

        Ok(())
    }

    // Process transaction groups during decoding, detecting binary data and converting to base64
    #[allow(clippy::collapsible_match)]
    fn process_txn_groups_for_decode(&self, value: &mut serde_json::Value) -> Result<()> {
        if let serde_json::Value::Object(obj) = value {
            if let Some(txn_groups) = obj.get_mut("txn-groups") {
                if let serde_json::Value::Array(groups) = txn_groups {
                    for group in groups {
                        if let serde_json::Value::Object(group_obj) = group {
                            if let Some(txns) = group_obj.get_mut("txns") {
                                if let serde_json::Value::Array(txns_array) = txns {
                                    for txn in txns_array.iter_mut() {
                                        // Try to identify any transaction object form and convert it to base64
                                        match txn {
                                            // Object is a direct transaction object
                                            serde_json::Value::Object(_) => {
                                                // If it's a JSON object, serialize it to msgpack and then base64
                                                let mut buf = Vec::new();
                                                self.encode_value(&mut buf, txn)?;
                                                *txn =
                                                    serde_json::Value::String(base64::encode(&buf));
                                            }
                                            // Binary data is serialized as binary value in rmp_serde
                                            serde_json::Value::String(s) if s.is_empty() => {
                                                // Empty string might be incorrectly decoded binary
                                                *txn = serde_json::Value::String("".to_string());
                                            }
                                            // Other non-string values might be binary data
                                            _ if !txn.is_string() => {
                                                // Encode any non-string transaction to msgpack and base64
                                                let mut buf = Vec::new();
                                                self.encode_value(&mut buf, txn)?;
                                                *txn =
                                                    serde_json::Value::String(base64::encode(&buf));
                                            }
                                            // Already a string, don't do anything
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Helper that converts an `rmpv::Value` into a `serde_json::Value`, turning any binary blobs
    /// into base64 encoded strings so that they can be represented in JSON.
    fn rmpv_to_json(value: &rmpv::Value) -> serde_json::Value {
        match value {
            rmpv::Value::Nil => serde_json::Value::Null,
            rmpv::Value::Boolean(b) => serde_json::Value::Bool(*b),
            rmpv::Value::Integer(i) => {
                if let Some(n) = i.as_i64() {
                    serde_json::Value::Number(n.into())
                } else if let Some(n) = i.as_u64() {
                    serde_json::Value::Number(serde_json::Number::from(n))
                } else {
                    // Fallback to string if too big (shouldn't happen for Algorand transactions)
                    serde_json::Value::String(i.to_string())
                }
            }
            rmpv::Value::F32(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f as f64).unwrap())
            }
            rmpv::Value::F64(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap())
            }
            rmpv::Value::String(s) => {
                serde_json::Value::String(s.as_str().unwrap_or_default().to_string())
            }
            rmpv::Value::Binary(b) => {
                // Represent binary blobs as base64 strings in the resulting JSON
                serde_json::Value::String(base64::encode(b))
            }
            rmpv::Value::Array(arr) => {
                let json_arr = arr.iter().map(Self::rmpv_to_json).collect();
                serde_json::Value::Array(json_arr)
            }
            rmpv::Value::Map(map) => {
                let mut json_map = serde_json::Map::with_capacity(map.len());
                for (key, val) in map {
                    // Convert keys to strings (MessagePack map keys in this context are always strings)
                    let key_str = match key {
                        rmpv::Value::String(s) => s.as_str().unwrap_or_default().to_string(),
                        _ => key.to_string(),
                    };
                    json_map.insert(key_str, Self::rmpv_to_json(val));
                }
                serde_json::Value::Object(json_map)
            }
            rmpv::Value::Ext(_, data) => {
                // Treat ext types as raw binary as well.
                serde_json::Value::String(base64::encode(data))
            }
        }
    }
}

/// Register all Algorand models with the registry
pub fn register_all_models(registry: &mut ModelRegistry) {
    // Register SimulateRequest with custom handler
    registry.registry.insert(
        ModelType::SimulateRequest,
        Box::new(SimulateRequestHandler::new()),
    );

    // Register other models with standard handlers
    registry.register::<SimulateTraceConfig>(ModelType::SimulateTraceConfig);
    registry
        .register::<SimulateRequestTransactionGroup>(ModelType::SimulateRequestTransactionGroup);
    registry.registry.insert(
        ModelType::SimulateTransaction200Response,
        Box::new(SimulateResponseHandler::new()),
    );
    registry
        .register::<SimulateUnnamedResourcesAccessed>(ModelType::SimulateUnnamedResourcesAccessed);
    registry.register::<SimulationTransactionExecTrace>(ModelType::SimulationTransactionExecTrace);
    registry.register::<SimulationEvalOverrides>(ModelType::SimulationEvalOverrides);
    registry.register::<SimulateInitialStates>(ModelType::SimulateInitialStates);
    registry.register::<SimulateTransactionResult>(ModelType::SimulateTransactionResult);
    registry.register::<SimulateTransactionGroupResult>(ModelType::SimulateTransactionGroupResult);
}

/// Custom handler for SimulateTransaction200Response to handle binary fields gracefully
struct SimulateResponseHandler {}

impl SimulateResponseHandler {
    fn new() -> Self {
        Self {}
    }
}

impl ModelHandler for SimulateResponseHandler {
    fn encode_json_to_msgpack(&self, json_str: &str) -> Result<Vec<u8>> {
        // Use standard named encoding; response encoding isn't a priority
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        Ok(rmp_serde::to_vec_named(&value)?)
    }

    fn decode_msgpack_to_json(&self, msgpack_bytes: &[u8]) -> Result<String> {
        use std::io::Cursor;
        let mut cursor = Cursor::new(msgpack_bytes);
        let root: rmpv::Value = rmpv::decode::read_value(&mut cursor)
            .map_err(|e| crate::MsgPackError::IoError(e.to_string()))?;
        let json_val = SimulateRequestHandler::rmpv_to_json(&root);
        Ok(serde_json::to_string(&json_val)?)
    }
}
