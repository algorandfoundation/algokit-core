use crate::AlgorandClient;
use crate::AppSourceMaps;
use crate::applications::app_client::CompilationParams;
use crate::clients::app_manager::CompiledPrograms;
use crate::transactions::{
    AppMethodCallArg, TransactionComposerConfig, TransactionResult, TransactionSigner,
};
use algod_client::models::PendingTransactionResponse;
use algokit_abi::Arc56Contract;
use algokit_transact::Byte32;
use algokit_transact::{Address, Transaction};
use std::sync::Arc;

/// Result from sending an app create call via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateResult {
    /// The create transaction
    pub transaction: Transaction,
    /// The response from sending and waiting for the create transaction
    pub confirmation: PendingTransactionResponse,
    /// The create transaction ID
    pub transaction_id: String,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
    /// The compiled approval and clear programs
    pub compiled_programs: CompiledPrograms,
}

/// Result from sending an app create method call via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateMethodCallResult {
    /// The result of the primary (last) transaction
    pub result: TransactionResult,
    /// All transaction results
    pub group_results: Vec<TransactionResult>,
    /// The group ID for the transaction group (if any)
    pub group: Option<Byte32>,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
    /// The compiled approval and clear programs
    pub compiled_programs: CompiledPrograms,
}

pub struct AppFactoryParams {
    pub algorand: Arc<AlgorandClient>,
    pub app_spec: Arc56Contract,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub default_signer: Option<Arc<dyn TransactionSigner>>,
    pub version: Option<String>,
    pub compilation_params: Option<CompilationParams>,
    pub source_maps: Option<AppSourceMaps>,
    pub transaction_composer_config: Option<TransactionComposerConfig>,
}
#[derive(Clone, Default)]
pub struct AppFactoryCreateParams {
    pub on_complete: Option<algokit_transact::OnApplicationComplete>,
    pub args: Option<Vec<Vec<u8>>>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub global_state_schema: Option<algokit_transact::StateSchema>,
    pub local_state_schema: Option<algokit_transact::StateSchema>,
    pub extra_program_pages: Option<u32>,
    pub sender: Option<String>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Clone, Default)]
pub struct AppFactoryCreateMethodCallParams {
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub on_complete: Option<algokit_transact::OnApplicationComplete>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub global_state_schema: Option<algokit_transact::StateSchema>,
    pub local_state_schema: Option<algokit_transact::StateSchema>,
    pub extra_program_pages: Option<u32>,
    pub sender: Option<String>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Clone, Default)]
pub struct AppFactoryUpdateMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Clone, Default)]
pub struct AppFactoryDeleteMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}
