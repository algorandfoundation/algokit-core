use crate::AlgorandClient;
use crate::clients::app_manager::{CompiledPrograms, TealTemplateValue};
use crate::transactions::TransactionComposerConfig;
use crate::transactions::TransactionSigner;
use crate::transactions::app_call::AppMethodCallArg;
use algod_client::models::PendingTransactionResponse;
use algokit_abi::ABIReturn;
use algokit_abi::Arc56Contract;
use algokit_transact::BoxReference;
use algokit_transact::Byte32;
use algokit_transact::Transaction;
use derive_more::Debug;
use std::collections::HashMap;
use std::sync::Arc;

/// Container for source maps captured during compilation/simulation.
#[derive(Debug, Clone, Default)]
pub struct AppSourceMaps {
    pub approval_source_map: Option<serde_json::Value>,
    pub clear_source_map: Option<serde_json::Value>,
}

/// Parameters required to construct an AppClient instance.
#[derive(Clone)]
pub struct AppClientParams {
    pub app_id: u64,
    pub app_spec: Arc56Contract,
    pub algorand: Arc<AlgorandClient>,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub default_signer: Option<Arc<dyn TransactionSigner>>,
    pub source_maps: Option<AppSourceMaps>,
    pub transaction_composer_config: Option<TransactionComposerConfig>,
}

/// Parameters for funding an application's account.
#[derive(Debug, Clone, Default)]
pub struct FundAppAccountParams {
    pub amount: u64,
    pub sender: Option<String>,
    #[debug(skip)]
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub close_remainder_to: Option<String>,
}

/// Parameters for ABI method call operations
#[derive(Debug, Clone, Default)]
pub struct AppClientMethodCallParams {
    pub method: String,
    pub args: Vec<AppMethodCallArg>,
    pub sender: Option<String>,
    #[debug(skip)]
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub account_references: Option<Vec<String>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for bare (non-ABI) app call operations
#[derive(Debug, Clone, Default)]
pub struct AppClientBareCallParams {
    pub args: Option<Vec<Vec<u8>>>,
    pub sender: Option<String>,
    #[debug(skip)]
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub account_references: Option<Vec<String>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<BoxReference>>,
}

/// Enriched logic error details with source map information.
#[derive(Debug, Clone, Default)]
pub struct LogicError {
    pub message: String,
    pub program: Option<Vec<u8>>,
    pub source_map: Option<serde_json::Value>,
    pub transaction_id: Option<String>,
    pub pc: Option<u64>,
    pub line_no: Option<u64>,
    pub lines: Option<Vec<String>>,
    pub traces: Option<Vec<serde_json::Value>>,
    /// Original logic error string if parsed
    pub logic_error_str: Option<String>,
}

impl std::fmt::Display for LogicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tx = self.transaction_id.as_deref().unwrap_or("N/A");
        let pc = self
            .pc
            .map(|p| p.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let mut base = format!("Txn {} had error '{}' at PC {}", tx, self.message, pc);
        if let Some(line) = self.line_no {
            base.push_str(&format!(" and Source Line {}", line));
        }
        writeln!(f, "{}", base)?;
        if let Some(trace) = self.annotated_trace() {
            write!(f, "{}", trace)?;
        }
        Ok(())
    }
}

impl LogicError {
    /// Build a simple annotated snippet string from stored lines and line number.
    pub fn annotated_trace(&self) -> Option<String> {
        let lines = self.lines.as_ref()?;
        let line_no = self.line_no? as usize;
        let mut out = String::new();
        for entry in lines {
            out.push_str(entry);
            if entry.starts_with(&format!("{:>4} |", line_no)) {
                out.push_str("\t<--- Error");
            }
            out.push('\n');
        }
        if out.is_empty() { None } else { Some(out) }
    }
}

/// Compilation configuration for update/compile flows
#[derive(Debug, Clone, Default)]
pub struct CompilationParams {
    pub deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct AppClientUpdateMethodCallResult {
    /// The primary transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the primary transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID of the primary transaction that has been sent
    pub transaction_id: String,
    /// The ABI return value from the primary method call (optional)
    pub abi_return: Option<ABIReturn>,
    /// The transactions that were sent
    pub transactions: Vec<Transaction>,
    /// The responses from sending and waiting for the transactions
    pub confirmations: Vec<PendingTransactionResponse>,
    /// The transaction IDs that have been sent
    pub transaction_ids: Vec<String>,
    /// The returned values of ABI methods
    pub abi_returns: Vec<ABIReturn>,
    /// The group ID (optional)
    pub group: Option<Byte32>,
    /// The compiled programs (approval and clear state)
    pub compiled_programs: CompiledPrograms,
}

#[derive(Clone, Debug)]
pub struct AppClientUpdateResult {
    /// The primary transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the primary transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID of the primary transaction that has been sent
    pub transaction_id: String,
    /// The compiled programs (approval and clear state)
    pub compiled_programs: CompiledPrograms,
}
