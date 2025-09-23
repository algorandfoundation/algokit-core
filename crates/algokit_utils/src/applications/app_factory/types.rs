use std::collections::HashMap;
use std::sync::Arc;

use algod_client::models::PendingTransactionResponse;
use algokit_abi::{ABIReturn, ABIValue, Arc56Contract};
use algokit_transact::{Address, Transaction};

use crate::AlgorandClient;
use crate::AppSourceMaps;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::{AppMethodCallArg, TransactionComposerConfig, TransactionSigner};

#[derive(Clone, Debug)]
pub struct AppFactoryCompilationResult {
    pub approval_program: Vec<u8>,
    pub clear_state_program: Vec<u8>,
    pub compiled_approval: crate::clients::app_manager::CompiledTeal,
    pub compiled_clear: crate::clients::app_manager::CompiledTeal,
}

/// Result from sending an app create method call via AppFactory.
/// Contains transaction details, confirmation, and parsed ARC-56 return value.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateMethodCallResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The group ID for the transaction group (if any)
    pub group_id: String,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
    /// The compiled approval program (if provided)
    pub compiled_approval: Option<Vec<u8>>,
    /// The compiled clear state program (if provided)
    pub compiled_clear: Option<Vec<u8>>,
    /// The raw ABI return value (for compatibility)
    pub abi_return: Option<ABIReturn>,
    /// The parsed ARC-56 return value
    pub arc56_return: Option<ABIValue>,
}

/// Result from sending an app update method call via AppFactory.
/// Contains transaction details, confirmation, and parsed ARC-56 return value.
#[derive(Clone, Debug)]
pub struct AppFactoryUpdateMethodCallResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The group ID for the transaction group (if any)
    pub group_id: String,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
    /// The compiled approval program (if provided)
    pub compiled_approval: Option<Vec<u8>>,
    /// The compiled clear state program (if provided)
    pub compiled_clear: Option<Vec<u8>>,
    /// The approval program source map (if available)
    pub approval_source_map: Option<serde_json::Value>,
    /// The clear program source map (if available)
    pub clear_source_map: Option<serde_json::Value>,
    /// The raw ABI return value (for compatibility)
    pub abi_return: Option<ABIReturn>,
    /// The parsed ARC-56 return value
    pub arc56_return: Option<ABIValue>,
}

/// Result from sending an app delete method call via AppFactory.
/// Contains transaction details, confirmation, and parsed ARC-56 return value.
#[derive(Clone, Debug)]
pub struct AppFactoryDeleteMethodCallResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The group ID for the transaction group (if any)
    pub group_id: String,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
    /// The raw ABI return value (for compatibility)
    pub abi_return: Option<ABIReturn>,
    /// The parsed ARC-56 return value
    pub arc56_return: Option<ABIValue>,
}

pub struct AppFactoryParams {
    pub algorand: Arc<AlgorandClient>,
    pub app_spec: Arc56Contract,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub default_signer: Option<Arc<dyn TransactionSigner>>,
    pub version: Option<String>,
    pub deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
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

// Helper methods for creating these results
impl AppFactoryCreateMethodCallResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        group_id: String,
        transaction_ids: Vec<String>,
        transactions: Vec<Transaction>,
        confirmations: Vec<PendingTransactionResponse>,
        app_id: u64,
        app_address: Address,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
        abi_return: Option<ABIReturn>,
        arc56_return: Option<ABIValue>,
    ) -> Self {
        Self {
            transaction,
            confirmation,
            transaction_id,
            group_id,
            transaction_ids,
            transactions,
            confirmations,
            app_id,
            app_address,
            compiled_approval,
            compiled_clear,
            abi_return,
            arc56_return,
        }
    }
}

impl AppFactoryUpdateMethodCallResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        group_id: String,
        transaction_ids: Vec<String>,
        transactions: Vec<Transaction>,
        confirmations: Vec<PendingTransactionResponse>,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
        approval_source_map: Option<serde_json::Value>,
        clear_source_map: Option<serde_json::Value>,
        abi_return: Option<ABIReturn>,
        arc56_return: Option<ABIValue>,
    ) -> Self {
        Self {
            transaction,
            confirmation,
            transaction_id,
            group_id,
            transaction_ids,
            transactions,
            confirmations,
            compiled_approval,
            compiled_clear,
            approval_source_map,
            clear_source_map,
            abi_return,
            arc56_return,
        }
    }
}

impl AppFactoryDeleteMethodCallResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        group_id: String,
        transaction_ids: Vec<String>,
        transactions: Vec<Transaction>,
        confirmations: Vec<PendingTransactionResponse>,
        abi_return: Option<ABIReturn>,
        arc56_return: Option<ABIValue>,
    ) -> Self {
        Self {
            transaction,
            confirmation,
            transaction_id,
            group_id,
            transaction_ids,
            transactions,
            confirmations,
            abi_return,
            arc56_return,
        }
    }
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
pub struct AppFactoryUpdateParams {
    pub app_id: u64,
    pub args: Option<Vec<Vec<u8>>>,
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

#[derive(Clone, Default)]
pub struct AppFactoryDeleteParams {
    pub app_id: u64,
    pub args: Option<Vec<Vec<u8>>>,
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

/// Result from deploying an application via [`AppFactory`].
#[derive(Debug)]
pub struct AppFactoryDeployResult {
    /// Metadata for the deployed application.
    pub app: crate::applications::app_deployer::AppMetadata,
    /// The deployment outcome describing which operation was performed.
    pub operation_performed: crate::applications::app_deployer::AppDeployResult,
    /// Detailed result for the create transaction when a new application was created.
    pub create_result: Option<AppFactoryCreateMethodCallResult>,
    /// Detailed result for the update transaction when an application was updated.
    pub update_result: Option<AppFactoryUpdateMethodCallResult>,
    /// Detailed result for the delete transaction when an application was replaced.
    pub delete_result: Option<AppFactoryDeleteMethodCallResult>,
}
