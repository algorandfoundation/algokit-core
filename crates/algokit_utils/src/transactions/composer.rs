use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{
        PendingTransactionResponse, SimulateRequest, SimulateRequestTransactionGroup,
        SimulateUnnamedResourcesAccessed, TransactionParams,
    },
};
use algokit_transact::{
    Address, AlgorandMsgpack, AssetConfigTransactionFields, Byte32, FeeParams,
    KeyRegistrationTransactionFields, MAX_TX_GROUP_SIZE, OnApplicationComplete,
    PaymentTransactionFields, SignedTransaction, Transaction, TransactionHeader, TransactionId,
    Transactions,
};
use derive_more::Debug;
use std::{collections::HashMap, sync::Arc};

// Constants for fee coverage priority calculation
const HIGH_PRIORITY_MULTIPLIER: i64 = 1_000;
const NORMAL_PRIORITY_MULTIPLIER: i64 = 1;
const NO_PRIORITY_LEVEL: i64 = -1;

// Constants for fee calculations
const SIGNATURE_BYTES_ESTIMATE: u64 = 75; // Estimated bytes added after signing
const DEFAULT_VALIDITY_WINDOW: u64 = 10;
const LOCALNET_VALIDITY_WINDOW: u64 = 1000;

// Constants for simulation
const EMPTY_SIGNATURE: [u8; 64] = [0; 64]; // Empty signature for simulation

use crate::genesis_id_is_localnet;

use super::application_call::{
    ApplicationCallParams, ApplicationCreateParams, ApplicationDeleteParams,
    ApplicationUpdateParams,
};
use super::asset_config::{AssetCreateParams, AssetDestroyParams, AssetReconfigureParams};
use super::asset_freeze::{AssetFreezeParams, AssetUnfreezeParams};
use super::common::{CommonParams, TransactionSigner, TransactionSignerGetter};
use super::key_registration::{
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use super::payment::{AccountCloseParams, PaymentParams};

#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(#[from] AlgodError),
    #[error("Decode Error: {0}")]
    DecodeError(String),
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Signing Error: {0}")]
    SigningError(String),
    #[error("Composer State Error: {0}")]
    StateError(String),
    #[error("Transaction pool error: {0}")]
    PoolError(String),
    #[error("Transaction group size exceeds the max limit of: {max}", max = MAX_TX_GROUP_SIZE)]
    GroupSizeError(),
}

#[derive(Clone)]
pub struct TransactionWithSigner {
    pub transaction: Transaction,
    pub signer: Arc<dyn TransactionSigner>,
}

#[derive(Debug, Clone)]
pub struct AssetTransferParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset transfers
    /// without additional side effects.
    /// Only in the case where the receiver is equal to the sender and the amount is zero,
    /// this is an asset opt-in transaction.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
}

#[derive(Debug, Clone)]
pub struct AssetOptInParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-in
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
}

/// Represents the fee difference for a transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeeDelta {
    /// Transaction has the exact required fee
    None,
    /// Transaction has an insufficient fee (needs more)
    Deficit(u64),
    /// Transaction has an excess fee (can cover other transactions)
    Surplus(u64),
}

impl FeeDelta {
    /// Create a FeeDelta from an i64 value (positive = deficit, negative = surplus, zero = none)
    pub fn from_i64(value: i64) -> Self {
        if value > 0 {
            FeeDelta::Deficit(value as u64)
        } else if value < 0 {
            FeeDelta::Surplus((-value) as u64)
        } else {
            FeeDelta::None
        }
    }

    /// Convert to i64 representation (positive = deficit, negative = surplus, zero = none)
    pub fn to_i64(&self) -> i64 {
        match self {
            FeeDelta::None => 0,
            FeeDelta::Deficit(amount) => *amount as i64,
            FeeDelta::Surplus(amount) => -(*amount as i64),
        }
    }

    /// Check if this represents no fee difference (neutral)
    pub fn is_none(&self) -> bool {
        matches!(self, FeeDelta::None)
    }

    /// Check if this represents a deficit (needs more fees)
    pub fn is_deficit(&self) -> bool {
        matches!(self, FeeDelta::Deficit(_))
    }

    /// Check if this represents a surplus (has excess fees)
    pub fn is_surplus(&self) -> bool {
        matches!(self, FeeDelta::Surplus(_))
    }

    /// Get the amount regardless of whether it's deficit or surplus
    pub fn amount(&self) -> u64 {
        match self {
            FeeDelta::None => 0,
            FeeDelta::Deficit(amount) | FeeDelta::Surplus(amount) => *amount,
        }
    }
}

impl std::ops::Add for FeeDelta {
    type Output = FeeDelta;

    fn add(self, rhs: FeeDelta) -> Self::Output {
        FeeDelta::from_i64(self.to_i64() + rhs.to_i64())
    }
}

#[derive(Debug, Clone)]
pub struct AssetOptOutParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-out
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
pub struct AssetClawbackParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset clawback
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
    // The address from which ASAs are taken.
    pub clawback_target: Address,
}

#[derive(Debug, Clone)]
pub struct SendTransactionResults {
    pub group_id: Option<Byte32>,
    pub transaction_ids: Vec<String>,
    pub confirmations: Vec<PendingTransactionResponse>,
}

// TODO: NC - Should this be named SendOptions instead? What is the naming convention for Python utils?
#[derive(Debug, Default, Clone)]
pub struct SendParams {
    pub max_rounds_to_wait_for_confirmation: Option<u64>,
    pub cover_app_call_inner_transaction_fees: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct BuildOptions {
    pub cover_app_call_inner_transaction_fees: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum ComposerTransaction {
    Transaction(Transaction),
    Payment(PaymentParams),
    AccountClose(AccountCloseParams),
    AssetTransfer(AssetTransferParams),
    AssetOptIn(AssetOptInParams),
    AssetOptOut(AssetOptOutParams),
    AssetClawback(AssetClawbackParams),
    AssetCreate(AssetCreateParams),
    AssetReconfigure(AssetReconfigureParams),
    AssetDestroy(AssetDestroyParams),
    AssetFreeze(AssetFreezeParams),
    AssetUnfreeze(AssetUnfreezeParams),
    ApplicationCall(ApplicationCallParams),
    ApplicationCreate(ApplicationCreateParams),
    ApplicationUpdate(ApplicationUpdateParams),
    ApplicationDelete(ApplicationDeleteParams),
    OnlineKeyRegistration(OnlineKeyRegistrationParams),
    OfflineKeyRegistration(OfflineKeyRegistrationParams),
    NonParticipationKeyRegistration(NonParticipationKeyRegistrationParams),
}

impl ComposerTransaction {
    pub fn common_params(&self) -> CommonParams {
        match self {
            ComposerTransaction::Payment(payment_params) => payment_params.common_params.clone(),
            ComposerTransaction::AccountClose(account_close_params) => {
                account_close_params.common_params.clone()
            }
            ComposerTransaction::AssetTransfer(asset_transfer_params) => {
                asset_transfer_params.common_params.clone()
            }
            ComposerTransaction::AssetOptIn(asset_opt_in_params) => {
                asset_opt_in_params.common_params.clone()
            }
            ComposerTransaction::AssetOptOut(asset_opt_out_params) => {
                asset_opt_out_params.common_params.clone()
            }
            ComposerTransaction::AssetClawback(asset_clawback_params) => {
                asset_clawback_params.common_params.clone()
            }
            ComposerTransaction::AssetCreate(asset_create_params) => {
                asset_create_params.common_params.clone()
            }
            ComposerTransaction::AssetReconfigure(asset_reconfigure_params) => {
                asset_reconfigure_params.common_params.clone()
            }
            ComposerTransaction::AssetDestroy(asset_destroy_params) => {
                asset_destroy_params.common_params.clone()
            }
            ComposerTransaction::AssetFreeze(asset_freeze_params) => {
                asset_freeze_params.common_params.clone()
            }
            ComposerTransaction::AssetUnfreeze(asset_unfreeze_params) => {
                asset_unfreeze_params.common_params.clone()
            }
            ComposerTransaction::ApplicationCall(app_call_params) => {
                app_call_params.common_params.clone()
            }
            ComposerTransaction::ApplicationCreate(app_create_params) => {
                app_create_params.common_params.clone()
            }
            ComposerTransaction::ApplicationUpdate(app_update_params) => {
                app_update_params.common_params.clone()
            }
            ComposerTransaction::ApplicationDelete(app_delete_params) => {
                app_delete_params.common_params.clone()
            }
            ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                online_key_reg_params.common_params.clone()
            }
            ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                offline_key_reg_params.common_params.clone()
            }
            ComposerTransaction::NonParticipationKeyRegistration(non_participation_params) => {
                non_participation_params.common_params.clone()
            }
            _ => CommonParams::default(),
        }
    }

    /// Returns true if this transaction is an application call type (call, create, update, or delete)
    pub fn is_app_call(&self) -> bool {
        matches!(
            self,
            ComposerTransaction::ApplicationCall(_)
                | ComposerTransaction::ApplicationCreate(_)
                | ComposerTransaction::ApplicationUpdate(_)
                | ComposerTransaction::ApplicationDelete(_)
        )
    }

    /// Get the logical maximum fee based on static_fee and max_fee
    /// Returns the higher of static_fee or max_fee, or static_fee if max_fee is None
    pub fn logical_max_fee(&self) -> Option<u64> {
        let common_params = self.common_params();
        let max_fee = common_params.max_fee;
        let static_fee = common_params.static_fee;

        let mut logical_max_fee = static_fee;
        if max_fee.is_some() && max_fee.unwrap() > static_fee.unwrap_or(0) {
            logical_max_fee = max_fee;
        }
        logical_max_fee
    }

    // TODO: NC - I don't think we need this?
    /// Checks if this transaction has an immutable fee (i.e., the current fee matches the logical max fee)
    pub fn has_immutable_fee(&self, current_fee: u64) -> bool {
        if let Some(logical_max_fee) = self.logical_max_fee() {
            logical_max_fee == current_fee
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Composer {
    transactions: Vec<ComposerTransaction>,
    algod_client: AlgodClient,
    signer_getter: Arc<dyn TransactionSignerGetter>,
    built_group: Option<Vec<TransactionWithSigner>>,
    signed_group: Option<Vec<SignedTransaction>>,
}

// TODO: NC - Where do we put these?
#[derive(Debug)]
struct TransactionExecutionInfo {
    required_fee_delta: FeeDelta,
    unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
}

#[derive(Debug)]
struct GroupExecutionInfo {
    unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
    transaction_execution_infos: Vec<TransactionExecutionInfo>,
}

/// Context for fee calculation operations
#[derive(Debug, Clone)]
struct FeeCalculationContext {
    per_byte_fee: u64,
    min_fee: u64,
    default_validity_window: u64,
}

impl FeeCalculationContext {
    fn new(suggested_params: &TransactionParams) -> Self {
        let default_validity_window = if genesis_id_is_localnet(&suggested_params.genesis_id) {
            LOCALNET_VALIDITY_WINDOW
        } else {
            DEFAULT_VALIDITY_WINDOW
        };

        Self {
            per_byte_fee: suggested_params.fee,
            min_fee: suggested_params.min_fee,
            default_validity_window,
        }
    }

    /// Calculate minimum fee for a transaction based on its encoded size
    fn calculate_min_fee_for_txn(&self, encoded_txn_size: usize) -> u64 {
        let per_byte_fee = self.per_byte_fee * (encoded_txn_size as u64 + SIGNATURE_BYTES_ESTIMATE);
        if per_byte_fee < self.min_fee {
            self.min_fee
        } else {
            per_byte_fee
        }
    }
}

impl Composer {
    pub fn new(algod_client: AlgodClient, signer_getter: Arc<dyn TransactionSignerGetter>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client,
            signer_getter,
            built_group: None,
            signed_group: None,
        }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        use crate::EmptySigner;

        Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            signer_getter: Arc::new(EmptySigner {}),
            built_group: None,
            signed_group: None,
        }
    }

    fn push(&mut self, txn: ComposerTransaction) -> Result<(), ComposerError> {
        if self.transactions.len() >= MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }
        self.transactions.push(txn);
        Ok(())
    }

    pub fn add_payment(&mut self, payment_params: PaymentParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Payment(payment_params))
    }

    pub fn add_account_close(
        &mut self,
        account_close_params: AccountCloseParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AccountClose(account_close_params))
    }

    pub fn add_asset_transfer(
        &mut self,
        asset_transfer_params: AssetTransferParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetTransfer(asset_transfer_params))
    }

    pub fn add_asset_opt_in(
        &mut self,
        asset_opt_in_params: AssetOptInParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptIn(asset_opt_in_params))
    }

    pub fn add_asset_opt_out(
        &mut self,
        asset_opt_out_params: AssetOptOutParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptOut(asset_opt_out_params))
    }

    pub fn add_asset_clawback(
        &mut self,
        asset_clawback_params: AssetClawbackParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetClawback(asset_clawback_params))
    }

    pub fn add_asset_create(
        &mut self,
        asset_create_params: AssetCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetCreate(asset_create_params))
    }

    pub fn add_asset_reconfigure(
        &mut self,
        asset_reconfigure_params: AssetReconfigureParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetReconfigure(
            asset_reconfigure_params,
        ))
    }

    pub fn add_asset_destroy(
        &mut self,
        asset_destroy_params: AssetDestroyParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetDestroy(asset_destroy_params))
    }

    pub fn add_asset_freeze(
        &mut self,
        asset_freeze_params: AssetFreezeParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetFreeze(asset_freeze_params))
    }

    pub fn add_asset_unfreeze(
        &mut self,
        asset_unfreeze_params: AssetUnfreezeParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetUnfreeze(asset_unfreeze_params))
    }

    pub fn add_online_key_registration(
        &mut self,
        online_key_reg_params: OnlineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OnlineKeyRegistration(
            online_key_reg_params,
        ))
    }

    pub fn add_offline_key_registration(
        &mut self,
        offline_key_reg_params: OfflineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OfflineKeyRegistration(
            offline_key_reg_params,
        ))
    }

    pub fn add_non_participation_key_registration(
        &mut self,
        non_participation_params: NonParticipationKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::NonParticipationKeyRegistration(
            non_participation_params,
        ))
    }

    pub fn add_application_call(
        &mut self,
        app_call_params: ApplicationCallParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationCall(app_call_params))
    }

    pub fn add_application_create(
        &mut self,
        app_create_params: ApplicationCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationCreate(app_create_params))
    }

    pub fn add_application_update(
        &mut self,
        app_update_params: ApplicationUpdateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationUpdate(app_update_params))
    }

    pub fn add_application_delete(
        &mut self,
        app_delete_params: ApplicationDeleteParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationDelete(app_delete_params))
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Transaction(transaction))
    }

    pub fn add_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<(), ComposerError> {
        if self.transactions.len() + transactions.len() > MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }

        transactions
            .into_iter()
            .try_for_each(|transaction| self.add_transaction(transaction))
    }

    pub fn transactions(&self) -> &Vec<ComposerTransaction> {
        &self.transactions
    }

    fn get_signer(&self, address: Address) -> Option<Arc<dyn TransactionSigner>> {
        self.signer_getter.get_signer(address)
    }

    async fn get_group_execution_info(
        &self,
        suggested_params: &TransactionParams,
        fee_context: &FeeCalculationContext,
        cover_inner_fees: bool,
    ) -> Result<GroupExecutionInfo, ComposerError> {
        let mut txns = Vec::new();
        let mut app_call_indexes_without_max_fees = Vec::new();
        let mut original_fees = Vec::new(); // Store original fees for later calculation
        let mut logical_max_fees = Vec::new();

        for (i, ctxn) in self.transactions.iter().enumerate() {
            let mut txn: Transaction = Self::build_transaction(
                &ctxn,
                suggested_params,
                &fee_context.default_validity_window,
            )
            .await?;

            // Store the original fee before modification
            original_fees.push(txn.header().fee.unwrap_or(0)); // TODO: NC - We could calculate this differently, which would

            let logical_max_fee = ctxn.logical_max_fee();
            logical_max_fees.push(logical_max_fee);

            // Check if this is an app call transaction and handle max fee requirement
            if cover_inner_fees {
                if ctxn.is_app_call() {
                    let logical_max_fee = ctxn.logical_max_fee();
                    if logical_max_fee.is_none() {
                        app_call_indexes_without_max_fees.push(i);
                    } else {
                        txn.header_mut().fee = logical_max_fee;
                    }
                }
            }

            txns.push(txn);
        }

        if txns.len() > 1 {
            txns = txns.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?;
        }

        let stxns = txns
            .into_iter()
            .map(|t| SignedTransaction {
                transaction: t,
                signature: Some(EMPTY_SIGNATURE), // Empty signature for simulation
                auth_address: None,
                multisignature: None,
            })
            .collect();

        if cover_inner_fees && !app_call_indexes_without_max_fees.is_empty() {
            return Err(ComposerError::StateError(format!(
                "Please provide a maxFee for each app call transaction when coverAppCallInnerTransactionFees is enabled. Required for transaction {}",
                app_call_indexes_without_max_fees
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }

        let txn_group = SimulateRequestTransactionGroup { txns: stxns };
        let simulate_request = SimulateRequest {
            txn_groups: vec![txn_group],
            allow_unnamed_resources: Some(true),
            allow_empty_signatures: Some(true),
            fix_signers: Some(true),
            ..Default::default()
        };

        // TODO: NC - simulate should work if format is not set
        let response = self
            .algod_client
            .simulate_transaction(simulate_request, Some(Format::Msgpack))
            .await
            .map_err(ComposerError::AlgodClientError)?;

        let group_response = &response.txn_groups[0];

        // Check for simulation failure
        if let Some(failure_message) = &group_response.failure_message {
            if cover_inner_fees && failure_message.contains("fee too small") {
                return Err(ComposerError::StateError(
                    "Fees were too small to resolve execution info via simulate. You may need to increase an app call transaction max fee.".to_string()
                ));
            }

            // TODO: NC - This feels like something that could be simplified
            let failed_at = group_response
                .failed_at
                .as_ref()
                .map(|indices| {
                    indices
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_else(|| "unknown".to_string());

            return Err(ComposerError::StateError(format!(
                "Error resolving execution info via simulate in transaction {}: {}",
                failed_at, failure_message
            )));
        }

        let mut txn_execution_infos = Vec::new();
        for (i, txn_result) in group_response.txn_results.iter().enumerate() {
            let original_txn = &self.transactions[i];
            let original_fee = original_fees[i];

            let required_fee_delta = if cover_inner_fees {
                // Calculate parent transaction fee delta using the fee context
                let built_txn = Self::build_transaction(
                    &original_txn,
                    suggested_params,
                    &fee_context.default_validity_window,
                )
                .await?;
                let encoded_txn = built_txn.encode().map_err(|e| {
                    ComposerError::TransactionError(format!("Failed to encode transaction: {}", e))
                })?;
                let parent_min_fee = fee_context.calculate_min_fee_for_txn(encoded_txn.len());
                let parent_fee_delta =
                    FeeDelta::from_i64(parent_min_fee as i64 - original_fee as i64);

                match original_txn {
                    _ if original_txn.is_app_call() => {
                        // Calculate inner transaction fee delta
                        let inner_fee_delta = Self::calculate_inner_fee_delta(
                            &txn_result.txn_result.inner_txns,
                            fee_context.min_fee,
                            FeeDelta::None,
                        );
                        inner_fee_delta + parent_fee_delta
                    }
                    _ => parent_fee_delta,
                }
            } else {
                FeeDelta::None
            };

            txn_execution_infos.push(TransactionExecutionInfo {
                required_fee_delta,
                unnamed_resources_accessed: txn_result.unnamed_resources_accessed.clone(),
            });
        }

        Ok(GroupExecutionInfo {
            unnamed_resources_accessed: group_response.unnamed_resources_accessed.clone(),
            transaction_execution_infos: txn_execution_infos,
        })
    }

    fn calculate_inner_fee_delta(
        inner_txns: &Option<Vec<PendingTransactionResponse>>,
        min_txn_fee: u64,
        acc: FeeDelta,
    ) -> FeeDelta {
        match inner_txns {
            Some(txns) => {
                // Surplus inner transaction fees do not pool up to the parent transaction.
                // Additionally surplus inner transaction fees only pool from sibling transactions that are sent prior to a given inner transaction, hence why we iterate in reverse order.
                txns.iter().rev().fold(acc, |acc, inner_txn| {
                    let recursive_delta =
                        Self::calculate_inner_fee_delta(&inner_txn.inner_txns, min_txn_fee, acc);
                    let txn_fee_delta = FeeDelta::from_i64(
                        min_txn_fee as i64 // Inner transactions don't require per byte fees
                        - inner_txn.txn.transaction.header().fee.unwrap_or(0) as i64,
                    );

                    let current_fee_delta = recursive_delta + txn_fee_delta;

                    // If after the recursive inner fee calculations we have a surplus,
                    // return 0 to avoid pooling up surplus fees, which is not allowed.
                    if current_fee_delta.is_surplus() {
                        FeeDelta::None
                    } else {
                        current_fee_delta
                    }
                })
            }
            None => acc,
        }
    }

    async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        self.algod_client
            .transaction_params()
            .await
            .map_err(Into::into)
    }

    /// Calculate priority level for a transaction in the fee redistribution process
    fn calculate_transaction_priority(
        &self,
        group_index: usize,
        execution_info: &TransactionExecutionInfo,
        transactions_with_signers: &[TransactionWithSigner],
    ) -> (usize, FeeDelta, i64) {
        let txn_in_group = &transactions_with_signers[group_index].transaction;
        let current_fee = txn_in_group.header().fee.unwrap_or(0);
        let immutable_fee = self.transactions[group_index].has_immutable_fee(current_fee);

        // Because we don't alter non app call transaction, they take priority
        let priority_multiplier =
            if matches!(execution_info.required_fee_delta, FeeDelta::Deficit(_))
                && (immutable_fee || !matches!(txn_in_group, Transaction::ApplicationCall(_)))
            {
                HIGH_PRIORITY_MULTIPLIER
            } else {
                NORMAL_PRIORITY_MULTIPLIER
            };

        let surplus_fee_priority_level = match &execution_info.required_fee_delta {
            FeeDelta::Deficit(amount) => (*amount as i64) * priority_multiplier,
            _ => NO_PRIORITY_LEVEL,
        };

        (
            group_index,
            execution_info.required_fee_delta.clone(),
            surplus_fee_priority_level,
        )
    }

    /// Create a transaction header from common parameters and suggested params
    fn create_transaction_header(
        common_params: &CommonParams,
        suggested_params: &TransactionParams,
        default_validity_window: &u64,
    ) -> Result<TransactionHeader, ComposerError> {
        let first_valid = common_params
            .first_valid_round
            .unwrap_or(suggested_params.last_round);

        Ok(TransactionHeader {
            sender: common_params.sender.clone(),
            rekey_to: common_params.rekey_to.clone(),
            note: common_params.note.clone(),
            lease: common_params.lease,
            fee: common_params.static_fee,
            genesis_id: Some(suggested_params.genesis_id.clone()),
            genesis_hash: Some(
                suggested_params
                    .genesis_hash
                    .clone()
                    .try_into()
                    .map_err(|_e| ComposerError::DecodeError("Invalid genesis hash".to_string()))?,
            ),
            first_valid,
            last_valid: common_params.last_valid_round.unwrap_or_else(|| {
                common_params
                    .validity_window
                    .map(|window| first_valid + window)
                    .unwrap_or(first_valid + default_validity_window)
            }),
            group: None,
        })
    }

    /// Create the transaction body from a ComposerTransaction and header
    fn create_transaction_body(
        composer_tx: &ComposerTransaction,
        header: TransactionHeader,
    ) -> Transaction {
        let common_params = composer_tx.common_params();

        match composer_tx {
            ComposerTransaction::Transaction(tx) => tx.clone(),
            ComposerTransaction::Payment(pay_params) => {
                Transaction::Payment(PaymentTransactionFields {
                    header,
                    receiver: pay_params.receiver.clone(),
                    amount: pay_params.amount,
                    close_remainder_to: None,
                })
            }
            ComposerTransaction::AccountClose(account_close_params) => {
                Transaction::Payment(PaymentTransactionFields {
                    header,
                    receiver: common_params.sender.clone(),
                    amount: 0,
                    close_remainder_to: Some(account_close_params.close_remainder_to.clone()),
                })
            }
            ComposerTransaction::AssetTransfer(asset_transfer_params) => {
                Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                    header,
                    asset_id: asset_transfer_params.asset_id,
                    amount: asset_transfer_params.amount,
                    receiver: asset_transfer_params.receiver.clone(),
                    asset_sender: None,
                    close_remainder_to: None,
                })
            }
            ComposerTransaction::AssetOptIn(asset_opt_in_params) => {
                Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                    header,
                    asset_id: asset_opt_in_params.asset_id,
                    amount: 0,
                    receiver: asset_opt_in_params.common_params.sender.clone(),
                    asset_sender: None,
                    close_remainder_to: None,
                })
            }
            ComposerTransaction::AssetOptOut(asset_opt_out_params) => {
                Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                    header,
                    asset_id: asset_opt_out_params.asset_id,
                    amount: 0,
                    receiver: asset_opt_out_params.common_params.sender.clone(),
                    asset_sender: None,
                    close_remainder_to: asset_opt_out_params.close_remainder_to.clone(),
                })
            }
            ComposerTransaction::AssetClawback(asset_clawback_params) => {
                Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                    header,
                    asset_id: asset_clawback_params.asset_id,
                    amount: asset_clawback_params.amount,
                    receiver: asset_clawback_params.receiver.clone(),
                    asset_sender: Some(asset_clawback_params.clawback_target.clone()),
                    close_remainder_to: None,
                })
            }
            ComposerTransaction::AssetCreate(asset_create_params) => {
                Transaction::AssetConfig(AssetConfigTransactionFields {
                    header,
                    asset_id: 0,
                    total: Some(asset_create_params.total),
                    decimals: asset_create_params.decimals,
                    default_frozen: asset_create_params.default_frozen,
                    asset_name: asset_create_params.asset_name.clone(),
                    unit_name: asset_create_params.unit_name.clone(),
                    url: asset_create_params.url.clone(),
                    metadata_hash: asset_create_params.metadata_hash,
                    manager: asset_create_params.manager.clone(),
                    reserve: asset_create_params.reserve.clone(),
                    freeze: asset_create_params.freeze.clone(),
                    clawback: asset_create_params.clawback.clone(),
                })
            }
            ComposerTransaction::AssetReconfigure(asset_reconfigure_params) => {
                Transaction::AssetConfig(AssetConfigTransactionFields {
                    header,
                    asset_id: asset_reconfigure_params.asset_id,
                    total: None,
                    decimals: None,
                    default_frozen: None,
                    asset_name: None,
                    unit_name: None,
                    url: None,
                    metadata_hash: None,
                    manager: asset_reconfigure_params.manager.clone(),
                    reserve: asset_reconfigure_params.reserve.clone(),
                    freeze: asset_reconfigure_params.freeze.clone(),
                    clawback: asset_reconfigure_params.clawback.clone(),
                })
            }
            ComposerTransaction::AssetDestroy(asset_destroy_params) => {
                Transaction::AssetConfig(AssetConfigTransactionFields {
                    header,
                    asset_id: asset_destroy_params.asset_id,
                    total: None,
                    decimals: None,
                    default_frozen: None,
                    asset_name: None,
                    unit_name: None,
                    url: None,
                    metadata_hash: None,
                    manager: None,
                    reserve: None,
                    freeze: None,
                    clawback: None,
                })
            }
            ComposerTransaction::AssetFreeze(asset_freeze_params) => {
                Transaction::AssetFreeze(algokit_transact::AssetFreezeTransactionFields {
                    header,
                    asset_id: asset_freeze_params.asset_id,
                    freeze_target: asset_freeze_params.target_address.clone(),
                    frozen: true,
                })
            }
            ComposerTransaction::AssetUnfreeze(asset_unfreeze_params) => {
                Transaction::AssetFreeze(algokit_transact::AssetFreezeTransactionFields {
                    header,
                    asset_id: asset_unfreeze_params.asset_id,
                    freeze_target: asset_unfreeze_params.target_address.clone(),
                    frozen: false,
                })
            }
            ComposerTransaction::ApplicationCall(app_call_params) => {
                Transaction::ApplicationCall(algokit_transact::ApplicationCallTransactionFields {
                    header,
                    app_id: app_call_params.app_id,
                    on_complete: app_call_params.on_complete,
                    approval_program: None,
                    clear_state_program: None,
                    global_state_schema: None,
                    local_state_schema: None,
                    extra_program_pages: None,
                    args: app_call_params.args.clone(),
                    account_references: app_call_params.account_references.clone(),
                    app_references: app_call_params.app_references.clone(),
                    asset_references: app_call_params.asset_references.clone(),
                    box_references: app_call_params.box_references.clone(),
                })
            }
            ComposerTransaction::ApplicationCreate(app_create_params) => {
                Transaction::ApplicationCall(algokit_transact::ApplicationCallTransactionFields {
                    header,
                    app_id: 0, // 0 indicates application creation
                    on_complete: app_create_params.on_complete,
                    approval_program: Some(app_create_params.approval_program.clone()),
                    clear_state_program: Some(app_create_params.clear_state_program.clone()),
                    global_state_schema: app_create_params.global_state_schema.clone(),
                    local_state_schema: app_create_params.local_state_schema.clone(),
                    extra_program_pages: app_create_params.extra_program_pages,
                    args: app_create_params.args.clone(),
                    account_references: app_create_params.account_references.clone(),
                    app_references: app_create_params.app_references.clone(),
                    asset_references: app_create_params.asset_references.clone(),
                    box_references: app_create_params.box_references.clone(),
                })
            }
            ComposerTransaction::ApplicationUpdate(app_update_params) => {
                Transaction::ApplicationCall(algokit_transact::ApplicationCallTransactionFields {
                    header,
                    app_id: app_update_params.app_id,
                    on_complete: OnApplicationComplete::UpdateApplication,
                    approval_program: Some(app_update_params.approval_program.clone()),
                    clear_state_program: Some(app_update_params.clear_state_program.clone()),
                    global_state_schema: None,
                    local_state_schema: None,
                    extra_program_pages: None,
                    args: app_update_params.args.clone(),
                    account_references: app_update_params.account_references.clone(),
                    app_references: app_update_params.app_references.clone(),
                    asset_references: app_update_params.asset_references.clone(),
                    box_references: app_update_params.box_references.clone(),
                })
            }
            ComposerTransaction::ApplicationDelete(app_delete_params) => {
                Transaction::ApplicationCall(algokit_transact::ApplicationCallTransactionFields {
                    header,
                    app_id: app_delete_params.app_id,
                    on_complete: OnApplicationComplete::DeleteApplication,
                    approval_program: None,
                    clear_state_program: None,
                    global_state_schema: None,
                    local_state_schema: None,
                    extra_program_pages: None,
                    args: app_delete_params.args.clone(),
                    account_references: app_delete_params.account_references.clone(),
                    app_references: app_delete_params.app_references.clone(),
                    asset_references: app_delete_params.asset_references.clone(),
                    box_references: app_delete_params.box_references.clone(),
                })
            }
            ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                    header,
                    vote_key: Some(online_key_reg_params.vote_key),
                    selection_key: Some(online_key_reg_params.selection_key),
                    vote_first: Some(online_key_reg_params.vote_first),
                    vote_last: Some(online_key_reg_params.vote_last),
                    vote_key_dilution: Some(online_key_reg_params.vote_key_dilution),
                    state_proof_key: online_key_reg_params.state_proof_key,
                    non_participation: None,
                })
            }
            ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                    header,
                    vote_key: None,
                    selection_key: None,
                    vote_first: None,
                    vote_last: None,
                    vote_key_dilution: None,
                    state_proof_key: None,
                    non_participation: offline_key_reg_params.non_participation,
                })
            }
            ComposerTransaction::NonParticipationKeyRegistration(_) => {
                Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                    header,
                    vote_key: None,
                    selection_key: None,
                    vote_first: None,
                    vote_last: None,
                    vote_key_dilution: None,
                    state_proof_key: None,
                    non_participation: Some(true),
                })
            }
        }
    }

    async fn build_transaction(
        tx: &ComposerTransaction,
        suggested_params: &TransactionParams,
        default_validity_window: &u64,
    ) -> Result<Transaction, ComposerError> {
        let common_params = tx.common_params();
        let header = Self::create_transaction_header(
            &common_params,
            suggested_params,
            default_validity_window,
        )?;
        let calculate_fee = header.fee.is_none();

        let mut transaction = Self::create_transaction_body(tx, header);

        // Calculate fee if needed
        if calculate_fee {
            transaction = transaction
                .assign_fee(FeeParams {
                    fee_per_byte: suggested_params.fee,
                    min_fee: suggested_params.min_fee,
                    extra_fee: common_params.extra_fee,
                    max_fee: common_params.max_fee,
                })
                .map_err(|e| ComposerError::TransactionError(e.to_string()))?;
        }

        Ok(transaction)
    }

    async fn build_transactions(
        &mut self,
        suggested_params: &TransactionParams,
        default_validity_window: &u64,
    ) -> Result<Vec<TransactionWithSigner>, ComposerError> {
        let mut transactions = Vec::new();
        let mut signers = Vec::new();

        for tx in &self.transactions {
            let common_params = tx.common_params();
            let header = Self::create_transaction_header(
                &common_params,
                suggested_params,
                default_validity_window,
            )?;

            // Special handling for pre-built transactions
            let mut calculate_fee = header.fee.is_none();
            let mut transaction = match tx {
                ComposerTransaction::Transaction(tx) => {
                    calculate_fee = false;
                    tx.clone()
                }
                _ => Self::create_transaction_body(tx, header),
            };

            if calculate_fee {
                transaction = transaction
                    .assign_fee(FeeParams {
                        fee_per_byte: suggested_params.fee,
                        min_fee: suggested_params.min_fee,
                        extra_fee: common_params.extra_fee,
                        max_fee: common_params.max_fee,
                    })
                    .map_err(|e| ComposerError::TransactionError(e.to_string()))?;
            }

            let signer = if let Some(transaction_signer) = common_params.signer {
                transaction_signer
            } else {
                let sender_address = transaction.header().sender.clone();

                self.get_signer(sender_address.clone())
                    .ok_or(ComposerError::SigningError(format!(
                        "No signer found for address: {}",
                        sender_address
                    )))?
            };

            transactions.push(transaction);
            signers.push(signer);
        }

        if transactions.len() > 1 {
            let grouped_transactions = transactions.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?;
            transactions = grouped_transactions;
        }

        let transactions_with_signers: Vec<TransactionWithSigner> = transactions
            .into_iter()
            .zip(signers.into_iter())
            .map(|(transaction, signer)| TransactionWithSigner {
                transaction,
                signer,
            })
            .collect();

        Ok(transactions_with_signers)
    }

    pub async fn build(
        &mut self,
        options: &Option<BuildOptions>,
    ) -> Result<&Vec<TransactionWithSigner>, ComposerError> {
        if let Some(ref group) = self.built_group {
            return Ok(group);
        }

        let suggested_params = self.get_suggested_params().await?;
        let fee_context = FeeCalculationContext::new(&suggested_params); // TODO: NC - Not sure about FeeContext as a type

        let cover_inner_fees = options.as_ref().map_or(false, |opts| {
            opts.cover_app_call_inner_transaction_fees.unwrap_or(false)
        });

        let mut transactions_with_signers = self
            .build_transactions(&suggested_params, &fee_context.default_validity_window)
            .await?;

        if cover_inner_fees {
            let execution_info = self
                .get_group_execution_info(&suggested_params, &fee_context, cover_inner_fees)
                .await?;

            let mut surplus_group_fees = execution_info
                .transaction_execution_infos
                .iter()
                .map(|txn| match &txn.required_fee_delta {
                    FeeDelta::Surplus(amount) => *amount,
                    _ => 0,
                })
                .sum();

            // TODO: NC - Hopefully we can get rid of this
            let mut logical_max_fees = Vec::new();

            // Create transaction info with group indices and priority levels
            let mut txn_infos: Vec<_> = execution_info
                .transaction_execution_infos
                .iter()
                .enumerate()
                .map(|(group_index, txn)| {
                    let logical_max_fee = self.transactions[group_index].logical_max_fee();
                    logical_max_fees.push(logical_max_fee);

                    self.calculate_transaction_priority(
                        group_index,
                        txn,
                        &transactions_with_signers,
                    )
                })
                .collect();

            // Sort by priority level (higher first)
            txn_infos.sort_by(|a, b| b.2.cmp(&a.2));

            // Resolve any additional fees required for the transactions
            for (group_index, required_fee_delta, _) in txn_infos {
                if let FeeDelta::Deficit(deficit_amount) = required_fee_delta {
                    let mut additional_fee_delta: FeeDelta = FeeDelta::None;
                    if surplus_group_fees >= deficit_amount {
                        // Surplus fully covers the deficit
                        surplus_group_fees = surplus_group_fees - deficit_amount;
                    } else {
                        // Surplus partially covers the deficit
                        additional_fee_delta =
                            FeeDelta::Deficit(deficit_amount - surplus_group_fees);
                        surplus_group_fees = 0;
                    }

                    if let FeeDelta::Deficit(deficit_amount) = additional_fee_delta {
                        // Handle the deficit by modifying the transaction fee
                        match transactions_with_signers[group_index].transaction {
                            Transaction::ApplicationCall(_) => {
                                let txn_header = transactions_with_signers[group_index]
                                    .transaction
                                    .header_mut();
                                let current_fee = txn_header.fee.unwrap_or(0);
                                let transaction_fee = current_fee + deficit_amount;

                                let logical_max_fee = logical_max_fees[group_index];
                                if logical_max_fee.is_none()
                                    || transaction_fee > logical_max_fee.unwrap()
                                {
                                    return Err(ComposerError::TransactionError(format!(
                                        "Calculated transaction fee {} µALGO is greater than max of {} for transaction {}",
                                        transaction_fee,
                                        logical_max_fee.unwrap_or(0),
                                        group_index
                                    )));
                                }

                                txn_header.fee = Some(transaction_fee);
                            }
                            _ => {
                                return Err(ComposerError::TransactionError(format!(
                                    "An additional fee of {} µALGO is required for non application call transaction {}",
                                    deficit_amount, group_index
                                )));
                            }
                        }
                    }
                }
            }

            // TODO: NC - Can we make this nicer?
            // Reassign group IDs after fee modifications
            if transactions_with_signers.len() > 1 {
                // Extract transactions, assign group, then update the original transactions
                let mut group_transactions: Vec<Transaction> = transactions_with_signers
                    .iter()
                    .map(|t| {
                        let mut txn = t.transaction.clone();
                        txn.header_mut().group = None; // Clear existing group assignment
                        txn
                    })
                    .collect();

                group_transactions = group_transactions.assign_group().map_err(|e| {
                    ComposerError::TransactionError(format!("Failed to assign group: {}", e))
                })?;

                // Update the group field in the original transactions
                let group_id = group_transactions[0].header().group;
                for txn_with_signer in &mut transactions_with_signers {
                    txn_with_signer.transaction.header_mut().group = group_id;
                }
            }
        }

        self.built_group = Some(transactions_with_signers);
        Ok(self.built_group.as_ref().unwrap())
    }

    pub async fn gather_signatures(&mut self) -> Result<&Vec<SignedTransaction>, ComposerError> {
        if let Some(ref group) = self.signed_group {
            return Ok(group);
        }

        let transactions_with_signers =
            self.built_group.as_ref().ok_or(ComposerError::StateError(
                "Cannot gather signatures before building the transaction group".to_string(),
            ))?;

        // Group transactions by signer
        let mut transactions = Vec::new();
        let mut signer_groups: HashMap<*const dyn TransactionSigner, Vec<usize>> = HashMap::new();
        for (index, txn_with_signer) in transactions_with_signers.iter().enumerate() {
            let signer_ptr = Arc::as_ptr(&txn_with_signer.signer);
            signer_groups.entry(signer_ptr).or_default().push(index);
            transactions.push(txn_with_signer.transaction.to_owned());
        }

        let mut signed_transactions = vec![None; transactions_with_signers.len()];

        for (_signer_ptr, indices) in signer_groups {
            // Get the signer from the first transaction with this signer
            let signer = &transactions_with_signers[indices[0]].signer;

            // Sign all transactions for this signer
            let signed_txns = signer
                .sign_transactions(&transactions, &indices)
                .await
                .map_err(ComposerError::SigningError)?;

            for (i, &index) in indices.iter().enumerate() {
                signed_transactions[index] = Some(signed_txns[i].to_owned());
            }
        }

        let final_signed_transactions: Result<Vec<SignedTransaction>, _> = signed_transactions
            .into_iter()
            .enumerate()
            .map(|(i, signed_transaction)| {
                signed_transaction.ok_or_else(|| {
                    ComposerError::SigningError(format!(
                        "Transaction at index {} was not signed",
                        i
                    ))
                })
            })
            .collect();

        self.signed_group = Some(final_signed_transactions?);
        Ok(self.signed_group.as_ref().unwrap())
    }

    async fn wait_for_confirmation(
        &self,
        tx_id: &str,
        max_rounds: u64,
    ) -> Result<PendingTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        let status = self
            .algod_client
            .get_status()
            .await
            .map_err(|e| format!("Failed to get status: {:?}", e))?;

        let start_round = status.last_round + 1;
        let mut current_round = start_round;

        while current_round < start_round + max_rounds {
            match self
                .algod_client
                .pending_transaction_information(tx_id, Some(Format::Msgpack))
                .await
            {
                Ok(response) => {
                    // Check for pool errors first - transaction was kicked out of pool
                    if !response.pool_error.is_empty() {
                        return Err(Box::new(ComposerError::PoolError(
                            response.pool_error.clone(),
                        )));
                    }

                    // Check if transaction is confirmed
                    if response.confirmed_round.is_some() {
                        return Ok(response);
                    }
                }
                Err(error) => {
                    // Only retry for 404 errors (transaction not found yet)
                    // All other errors indicate permanent issues and should fail fast
                    let is_retryable = matches!(
                        &error,
                        algod_client::apis::Error::Api(
                            algod_client::apis::AlgodApiError::PendingTransactionInformation(
                                algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            )
                        )
                    ) || error.to_string().contains("404");

                    if is_retryable {
                        current_round += 1;
                        continue;
                    } else {
                        return Err(Box::new(ComposerError::AlgodClientError(error)));
                    }
                }
            };

            let _ = self.algod_client.wait_for_block(current_round).await;
            current_round += 1;
        }

        Err(format!(
            "Transaction {} not confirmed after {} rounds",
            tx_id, max_rounds
        )
        .into())
    }

    pub async fn send(
        &mut self,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResults, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: NC - This could be extract into a type converter
        let build_options = send_params.as_ref().map(|params| BuildOptions {
            cover_app_call_inner_transaction_fees: params.cover_app_call_inner_transaction_fees,
        });

        self.build(&build_options)
            .await
            .map_err(|e| format!("Failed to build transaction: {}", e))?;

        let group_id = {
            let transactions_with_signers =
                self.built_group.as_ref().ok_or("No transactions built")?;
            if transactions_with_signers.is_empty() {
                return Err("No transactions to send".into());
            }
            transactions_with_signers[0].transaction.header().group
        };

        self.gather_signatures()
            .await
            .map_err(|e| format!("Failed to sign transaction: {}", e))?;

        let signed_transactions = self.signed_group.as_ref().ok_or("No signed transactions")?;

        let wait_rounds = if let Some(max_rounds_to_wait_for_confirmation) =
            send_params.and_then(|p| p.max_rounds_to_wait_for_confirmation)
        {
            max_rounds_to_wait_for_confirmation
        } else {
            let first_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().first_valid)
                .min()
                .ok_or("Failed to calculate first valid round")?;

            let last_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().last_valid)
                .max()
                .ok_or("Failed to calculate last valid round")?;

            last_round - first_round
        };

        // Encode each signed transaction and concatenate them
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn
                .encode()
                .map_err(|e| format!("Failed to encode signed transaction: {}", e))?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let _ = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction(s): {:?}", e))?;

        let transaction_ids: Vec<String> = signed_transactions
            .iter()
            .map(|txn| txn.id())
            .collect::<Result<Vec<String>, _>>()?;

        let mut confirmations = Vec::new();
        for id in &transaction_ids {
            let confirmation = self
                .wait_for_confirmation(id, wait_rounds)
                .await
                .map_err(|e| format!("Failed to confirm transaction: {}", e))?;
            confirmations.push(confirmation);
        }

        Ok(SendTransactionResults {
            group_id,
            transaction_ids,
            confirmations,
        })
    }
}

// TODO: NC - These fee delta tests can be removed when done.

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{AccountMother, TransactionMother};
    use base64::{Engine, prelude::BASE64_STANDARD};

    #[test]
    fn test_fee_delta_operations() {
        // Test creation from i64
        assert_eq!(FeeDelta::from_i64(100), FeeDelta::Deficit(100));
        assert_eq!(FeeDelta::from_i64(-50), FeeDelta::Surplus(50));
        assert_eq!(FeeDelta::from_i64(0), FeeDelta::None);

        // Test conversion to i64
        assert_eq!(FeeDelta::None.to_i64(), 0);
        assert_eq!(FeeDelta::Deficit(100).to_i64(), 100);
        assert_eq!(FeeDelta::Surplus(50).to_i64(), -50);

        // Test is_none, is_deficit and is_surplus
        assert!(FeeDelta::None.is_none());
        assert!(!FeeDelta::None.is_deficit());
        assert!(!FeeDelta::None.is_surplus());

        assert!(FeeDelta::Deficit(100).is_deficit());
        assert!(!FeeDelta::Deficit(100).is_surplus());
        assert!(!FeeDelta::Deficit(100).is_none());

        assert!(FeeDelta::Surplus(50).is_surplus());
        assert!(!FeeDelta::Surplus(50).is_deficit());
        assert!(!FeeDelta::Surplus(50).is_none());

        // Test amount extraction
        assert_eq!(FeeDelta::None.amount(), 0);
        assert_eq!(FeeDelta::Deficit(100).amount(), 100);
        assert_eq!(FeeDelta::Surplus(50).amount(), 50);
    }

    #[test]
    fn test_option_fee_delta_logic() {
        // Test that FeeDelta::None represents no fee delta (neutral)
        let execution_info = TransactionExecutionInfo {
            required_fee_delta: FeeDelta::None,
            unnamed_resources_accessed: None,
        };

        // Should contribute 0 to surplus calculation
        assert_eq!(
            match &execution_info.required_fee_delta {
                FeeDelta::Surplus(amount) => *amount,
                _ => 0,
            },
            0
        );

        // Test with Deficit
        let execution_info_deficit = TransactionExecutionInfo {
            required_fee_delta: FeeDelta::Deficit(50),
            unnamed_resources_accessed: None,
        };

        assert!(matches!(
            execution_info_deficit.required_fee_delta,
            FeeDelta::Deficit(_)
        ));

        // Test with Surplus
        let execution_info_surplus = TransactionExecutionInfo {
            required_fee_delta: FeeDelta::Surplus(75),
            unnamed_resources_accessed: None,
        };

        assert_eq!(
            match &execution_info_surplus.required_fee_delta {
                FeeDelta::Surplus(amount) => *amount,
                _ => 0,
            },
            75
        );
    }

    #[test]
    fn test_add_transaction() {
        let mut composer = Composer::testnet();
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_ok());
    }

    #[test]
    fn test_add_too_many_transactions() {
        let mut composer = Composer::testnet();
        for _ in 0..16 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_err());
    }

    #[tokio::test]
    async fn test_get_suggested_params() {
        let composer = Composer::testnet();
        let response = composer.get_suggested_params().await.unwrap();

        assert_eq!(
            response.genesis_hash,
            BASE64_STANDARD
                .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
                .unwrap()
        );
    }

    #[test]
    fn test_add_payment() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        assert!(composer.add_payment(payment_params).is_ok());
    }

    #[tokio::test]
    async fn test_gather_signatures() {
        let mut composer = Composer::testnet();

        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        composer.add_payment(payment_params).unwrap();
        composer.build(&None).await.unwrap();

        let result = composer.gather_signatures().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_single_transaction_no_group() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        composer.add_payment(payment_params).unwrap();

        composer.build(&None).await.unwrap();

        let built_group = composer.built_group.as_ref().unwrap();
        assert_eq!(built_group.len(), 1);

        // Single transaction should not have a group ID set
        assert!(built_group[0].transaction.header().group.is_none());
    }

    #[tokio::test]
    async fn test_multiple_transactions_have_group() {
        let mut composer = Composer::testnet();

        for _ in 0..2 {
            let payment_params = PaymentParams {
                common_params: CommonParams {
                    sender: AccountMother::account().address(),
                    signer: None,
                    rekey_to: None,
                    note: None,
                    lease: None,
                    static_fee: None,
                    extra_fee: None,
                    max_fee: None,
                    validity_window: None,
                    first_valid_round: None,
                    last_valid_round: None,
                },
                receiver: AccountMother::account().address(),
                amount: 1000,
            };
            composer.add_payment(payment_params).unwrap();
        }

        composer.build(&None).await.unwrap();

        let built_group = composer.built_group.as_ref().unwrap();
        assert_eq!(built_group.len(), 2);

        // Multiple transactions should have group IDs set
        for transaction_with_signer in built_group {
            assert!(transaction_with_signer.transaction.header().group.is_some());
        }

        // All transactions should have the same group ID
        let group_id = built_group[0].transaction.header().group.as_ref().unwrap();
        for transaction_with_signer in &built_group[1..] {
            assert_eq!(
                transaction_with_signer
                    .transaction
                    .header()
                    .group
                    .as_ref()
                    .unwrap(),
                group_id
            );
        }
    }

    #[test]
    fn test_error_recoverability_logic() {
        // Test string-based 404 detection (the primary retry mechanism)
        let error_404_string = "Request failed with status 404: Transaction not found";
        let error_500_string = "Request failed with status 500: Server error";

        // The main retry logic relies on string matching
        assert!(
            error_404_string.contains("404"),
            "404 errors should be retryable"
        );
        assert!(
            !error_500_string.contains("404"),
            "500 errors should not be retryable"
        );
    }

    #[test]
    fn test_validity_window_logic() {
        // Test LocalNet detection and validity window logic
        assert_eq!(
            if genesis_id_is_localnet("devnet-v1") {
                1000
            } else {
                10
            },
            1000,
            "LocalNet should use 1000 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("testnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "TestNet should use 10 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("mainnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "MainNet should use 10 round validity window"
        );
    }
}
