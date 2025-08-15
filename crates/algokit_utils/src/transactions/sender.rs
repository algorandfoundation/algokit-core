use super::{
    application_call::{
        AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams, AppCreateParams,
        AppDeleteMethodCallParams, AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams,
    },
    asset_config::{AssetCreateParams, AssetDestroyParams, AssetReconfigureParams},
    asset_freeze::{AssetFreezeParams, AssetUnfreezeParams},
    asset_transfer::{AssetOptInParams, AssetOptOutParams, AssetTransferParams},
    composer::{Composer, ComposerError, SendParams},
    key_registration::{OfflineKeyRegistrationParams, OnlineKeyRegistrationParams},
    payment::{AccountCloseParams, PaymentParams},
    sender_results::{
        SendAppCallResult, SendAppCreateResult, SendAppUpdateResult, SendAssetCreateResult,
        SendTransactionResult, TransactionResultError,
    },
};
use crate::clients::app_manager::{AppManager, AppManagerError, CompiledTeal};
use crate::clients::asset_manager::{AssetManager, AssetManagerError};
use algod_client::apis::AlgodApiError;
use algokit_abi::{ABIMethod, ABIReturn};
use algokit_transact::Address;

use std::{str::FromStr, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum TransactionSenderError {
    #[error("Algod client error: {0}")]
    AlgodClientError(#[from] AlgodApiError),
    #[error("Composer error: {0}")]
    ComposerError(#[from] ComposerError),
    #[error("Asset manager error: {0}")]
    AssetManagerError(#[from] AssetManagerError),
    #[error("App manager error: {0}")]
    AppManagerError(#[from] AppManagerError),
    #[error("Transaction result error: {0}")]
    TransactionResultError(#[from] TransactionResultError),
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("Transaction validation error: {0}")]
    ValidationError(String),
}

/// Sends transactions and groups with validation and result processing.
#[derive(Clone)]
pub struct TransactionSender {
    asset_manager: AssetManager,
    app_manager: AppManager,
    new_group: Arc<dyn Fn() -> Composer>,
}

pub trait HasMethod {
    fn method(&self) -> &ABIMethod;
}

pub trait HasPrograms {
    fn approval_program(&self) -> &[u8];
    fn clear_state_program(&self) -> &[u8];
}

// Implement HasMethod for method call parameters
impl<T> HasMethod for AppCallMethodCallParams<T>
where
    T: super::application_call::ValidMethodCallArg,
{
    fn method(&self) -> &ABIMethod {
        &self.method
    }
}

impl<T> HasMethod for AppCreateMethodCallParams<T>
where
    T: super::application_call::ValidMethodCallArg,
{
    fn method(&self) -> &ABIMethod {
        &self.method
    }
}

impl<T> HasMethod for AppUpdateMethodCallParams<T>
where
    T: super::application_call::ValidMethodCallArg,
{
    fn method(&self) -> &ABIMethod {
        &self.method
    }
}

impl<T> HasMethod for AppDeleteMethodCallParams<T>
where
    T: super::application_call::ValidMethodCallArg,
{
    fn method(&self) -> &ABIMethod {
        &self.method
    }
}

impl HasPrograms for AppCreateParams {
    fn approval_program(&self) -> &[u8] {
        &self.approval_program
    }

    fn clear_state_program(&self) -> &[u8] {
        &self.clear_state_program
    }
}

impl HasPrograms for AppUpdateParams {
    fn approval_program(&self) -> &[u8] {
        &self.approval_program
    }

    fn clear_state_program(&self) -> &[u8] {
        &self.clear_state_program
    }
}

impl<T> HasPrograms for AppCreateMethodCallParams<T>
where
    T: super::application_call::ValidMethodCallArg,
{
    fn approval_program(&self) -> &[u8] {
        &self.approval_program
    }

    fn clear_state_program(&self) -> &[u8] {
        &self.clear_state_program
    }
}

impl<T> HasPrograms for AppUpdateMethodCallParams<T>
where
    T: super::application_call::ValidMethodCallArg,
{
    fn approval_program(&self) -> &[u8] {
        &self.approval_program
    }

    fn clear_state_program(&self) -> &[u8] {
        &self.clear_state_program
    }
}

impl TransactionSender {
    /// Create a new TransactionSender instance.
    pub fn new(
        new_group: impl Fn() -> Composer + 'static,
        asset_manager: AssetManager,
        app_manager: AppManager,
    ) -> Self {
        Self {
            asset_manager,
            app_manager,
            new_group: Arc::new(new_group),
        }
    }

    pub fn new_group(&self) -> Composer {
        (self.new_group)()
    }

    async fn send_and_parse(
        &self,
        mut composer: Composer,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let built_transactions = composer.build(None).await?;

        let raw_transactions: Vec<algokit_transact::Transaction> = built_transactions
            .iter()
            .map(|tx_with_signer| tx_with_signer.transaction.clone())
            .collect();

        let composer_results = composer.send(send_params).await?;

        let group_id = composer_results
            .group_id
            .map(hex::encode)
            .unwrap_or_else(|| "".to_string());

        // Enhanced ABI return processing using app_manager
        let abi_returns: Option<Vec<ABIReturn>> = if !composer_results.abi_returns.is_empty() {
            let returns: Result<Vec<_>, _> = composer_results
                .abi_returns
                .into_iter()
                .map(|result| result.map_err(TransactionSenderError::ComposerError))
                .collect();
            match returns {
                Ok(returns) => {
                    // Process ABI returns with app_manager for enhanced parsing
                    let processed_returns: Vec<_> = returns.into_iter().flatten().collect();
                    Some(processed_returns)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        let result = SendTransactionResult::new(
            group_id,
            composer_results.transaction_ids,
            raw_transactions,
            composer_results.confirmations,
            abi_returns,
        )?;

        Ok(result)
    }

    /// Helper method to send a single transaction using the standard 3-line pattern.
    /// Creates a new group, adds the transaction using the provided closure, and sends it.
    async fn send_single_transaction<F>(
        &self,
        send_params: Option<SendParams>,
        add_transaction: F,
    ) -> Result<SendTransactionResult, TransactionSenderError>
    where
        F: FnOnce(&mut Composer) -> Result<(), ComposerError>,
    {
        let mut composer = self.new_group();
        add_transaction(&mut composer)?;
        self.send_and_parse(composer, send_params).await
    }

    /// Helper method to send a single transaction and wrap the result in a specific type.
    /// Creates a new group, adds the transaction using the provided closure, sends it,
    /// and applies a result transformer function.
    async fn send_single_transaction_with_result<F, R, T>(
        &self,
        send_params: Option<SendParams>,
        add_transaction: F,
        transform_result: T,
    ) -> Result<R, TransactionSenderError>
    where
        F: FnOnce(&mut Composer) -> Result<(), ComposerError>,
        T: FnOnce(SendTransactionResult) -> Result<R, TransactionSenderError>,
    {
        let mut composer = self.new_group();
        add_transaction(&mut composer)?;
        let base_result = self.send_and_parse(composer, send_params).await?;
        transform_result(base_result)
    }

    /// Extract ABI return from transaction result using app manager for enhanced processing.
    ///
    /// This method takes a transaction result and method parameter to extract and parse
    /// ABI return values with proper type information from the app manager.
    ///
    /// # Arguments
    /// * `result` - The transaction result containing potential ABI returns
    /// * `params` - Parameters containing the method definition for ABI processing
    ///
    /// # Returns
    /// * `Option<ABIReturn>` - The processed ABI return if available and valid
    fn extract_abi_return_from_result(
        &self,
        result: &SendTransactionResult,
        params: &impl HasMethod,
    ) -> Option<ABIReturn> {
        // Get the last ABI return from the result (most recent transaction)
        let abi_return = result.abi_returns.as_ref()?.last()?.clone();

        // Use app manager to enhance the ABI return processing
        let method = params.method();

        // If the method has a return type, validate and enhance the return
        if method.returns.is_some() {
            // Use app manager static method to parse the return value with proper method information
            match AppManager::get_abi_return(&abi_return.raw_return_value, method) {
                Ok(Some(parsed)) => {
                    // Return enhanced ABIReturn with validated parsing
                    Some(parsed)
                }
                Ok(None) => {
                    // Method has no return type
                    Some(abi_return)
                }
                Err(_) => {
                    // Return original if parsing fails
                    Some(abi_return)
                }
            }
        } else {
            // Method has no return type, return as-is
            Some(abi_return)
        }
    }

    /// Extract compilation metadata for TEAL programs using app manager caching.
    fn extract_compilation_metadata(
        &self,
        params: &impl HasPrograms,
    ) -> (Option<CompiledTeal>, Option<CompiledTeal>) {
        let approval_program = params.approval_program();
        let clear_state_program = params.clear_state_program();

        // Convert program bytes to TEAL strings for compilation lookup
        let approval_teal = String::from_utf8(approval_program.to_vec()).ok();
        let clear_state_teal = String::from_utf8(clear_state_program.to_vec()).ok();

        let compiled_approval = if let Some(teal) = approval_teal {
            self.app_manager.get_compilation_result(&teal)
        } else {
            None
        };

        let compiled_clear = if let Some(teal) = clear_state_teal {
            self.app_manager.get_compilation_result(&teal)
        } else {
            None
        };

        (compiled_approval, compiled_clear)
    }

    /// Send payment transaction to transfer Algo between accounts.
    pub async fn payment(
        &self,
        params: PaymentParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_payment(params))
            .await
    }

    /// Send account close transaction.
    pub async fn account_close(
        &self,
        params: AccountCloseParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_account_close(params))
            .await
    }

    /// Send asset transfer transaction.
    pub async fn asset_transfer(
        &self,
        params: AssetTransferParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        // Enhanced parameter validation
        if params.asset_id == 0 {
            return Err(TransactionSenderError::InvalidParameters(
                "Asset ID must be greater than 0".to_string(),
            ));
        }
        // Note: amount can be 0 for opt-in transactions, so we don't validate it here

        self.send_single_transaction(send_params, |composer| composer.add_asset_transfer(params))
            .await
    }

    /// Send asset opt-in transaction.
    pub async fn asset_opt_in(
        &self,
        params: AssetOptInParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_asset_opt_in(params))
            .await
    }

    /// Send asset opt-out transaction.
    pub async fn asset_opt_out(
        &self,
        params: AssetOptOutParams,
        send_params: Option<SendParams>,
        ensure_zero_balance: Option<bool>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        // Resolve close_remainder_to to asset creator if not specified
        let params = if params.close_remainder_to.is_none() {
            let asset_info = self
                .asset_manager
                .get_by_id(params.asset_id)
                .await
                .map_err(|e| {
                    TransactionSenderError::ValidationError(format!(
                        "Failed to get asset {} information: {}",
                        params.asset_id, e
                    ))
                })?;

            let creator = Address::from_str(&asset_info.creator).map_err(|e| {
                TransactionSenderError::ValidationError(format!(
                    "Invalid creator address for asset {}: {}",
                    params.asset_id, e
                ))
            })?;

            AssetOptOutParams {
                close_remainder_to: Some(creator),
                ..params
            }
        } else {
            params
        };

        if ensure_zero_balance.unwrap_or(true) {
            // Ensure account has zero balance before opting out
            let account_info = self
                .asset_manager
                .get_account_information(&params.common_params.sender, params.asset_id)
                .await
                .map_err(|e| {
                    TransactionSenderError::ValidationError(format!(
                        "Account {} validation failed for Asset {}: {}",
                        params.common_params.sender, params.asset_id, e
                    ))
                })?;

            let balance = account_info
                .asset_holding
                .as_ref()
                .map(|h| h.amount)
                .unwrap_or(0);
            if balance != 0 {
                return Err(TransactionSenderError::ValidationError(format!(
                    "Account {} does not have a zero balance for Asset {}; can't opt-out.",
                    params.common_params.sender, params.asset_id
                )));
            }
        }

        self.send_single_transaction(send_params, |composer| composer.add_asset_opt_out(params))
            .await
    }

    /// Send asset creation transaction.
    pub async fn asset_create(
        &self,
        params: AssetCreateParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAssetCreateResult, TransactionSenderError> {
        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_asset_create(params),
            |base_result| {
                SendAssetCreateResult::new(base_result)
                    .map_err(TransactionSenderError::TransactionResultError)
            },
        )
        .await
    }

    /// Send asset configuration transaction.
    pub async fn asset_config(
        &self,
        params: AssetReconfigureParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| {
            composer.add_asset_reconfigure(params)
        })
        .await
    }

    /// Send asset destroy transaction.
    pub async fn asset_destroy(
        &self,
        params: AssetDestroyParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_asset_destroy(params))
            .await
    }

    /// Send asset freeze transaction.
    pub async fn asset_freeze(
        &self,
        params: AssetFreezeParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_asset_freeze(params))
            .await
    }

    /// Send asset unfreeze transaction.
    pub async fn asset_unfreeze(
        &self,
        params: AssetUnfreezeParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_asset_unfreeze(params))
            .await
    }

    /// Send application call transaction.
    pub async fn app_call(
        &self,
        params: AppCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_app_call(params))
            .await
    }

    /// Send application creation transaction.
    pub async fn app_create(
        &self,
        params: AppCreateParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCreateResult, TransactionSenderError> {
        // Extract compilation metadata using helper method
        let (compiled_approval, compiled_clear) = self.extract_compilation_metadata(&params);

        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_app_create(params),
            |base_result| {
                // Convert CompiledTeal to Vec<u8> for the result
                let approval_bytes = compiled_approval.map(|ct| ct.compiled_base64_to_bytes);
                let clear_bytes = compiled_clear.map(|ct| ct.compiled_base64_to_bytes);

                SendAppCreateResult::new(base_result, None, approval_bytes, clear_bytes)
                    .map_err(TransactionSenderError::TransactionResultError)
            },
        )
        .await
    }

    /// Send application update transaction.
    pub async fn app_update(
        &self,
        params: AppUpdateParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppUpdateResult, TransactionSenderError> {
        // Extract compilation metadata using helper method
        let (compiled_approval, compiled_clear) = self.extract_compilation_metadata(&params);

        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_app_update(params),
            |base_result| {
                // Convert CompiledTeal to Vec<u8> for the result
                let approval_bytes = compiled_approval.map(|ct| ct.compiled_base64_to_bytes);
                let clear_bytes = compiled_clear.map(|ct| ct.compiled_base64_to_bytes);

                Ok(SendAppUpdateResult::new(
                    base_result,
                    None,
                    approval_bytes,
                    clear_bytes,
                ))
            },
        )
        .await
    }

    /// Send application delete transaction.
    pub async fn app_delete(
        &self,
        params: AppDeleteParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| composer.add_app_delete(params))
            .await
    }

    /// Send ABI method call transaction.
    pub async fn app_call_method_call(
        &self,
        params: AppCallMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCallResult, TransactionSenderError> {
        let params_clone = params.clone();
        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_app_call_method_call(params),
            |base_result| {
                // Extract ABI return using helper method for enhanced processing
                let abi_return = self.extract_abi_return_from_result(&base_result, &params_clone);
                Ok(SendAppCallResult::new(base_result, abi_return))
            },
        )
        .await
    }

    /// Send ABI method call for app creation.
    pub async fn app_create_method_call(
        &self,
        params: AppCreateMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCreateResult, TransactionSenderError> {
        // Extract compilation metadata using helper method
        let (compiled_approval, compiled_clear) = self.extract_compilation_metadata(&params);
        let params_clone = params.clone();

        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_app_create_method_call(params),
            |base_result| {
                // Extract ABI return using helper method for enhanced processing
                let abi_return = self.extract_abi_return_from_result(&base_result, &params_clone);

                // Convert CompiledTeal to Vec<u8> for the result
                let approval_bytes = compiled_approval.map(|ct| ct.compiled_base64_to_bytes);
                let clear_bytes = compiled_clear.map(|ct| ct.compiled_base64_to_bytes);

                SendAppCreateResult::new(base_result, abi_return, approval_bytes, clear_bytes)
                    .map_err(TransactionSenderError::TransactionResultError)
            },
        )
        .await
    }

    /// Send ABI method call for app update.
    pub async fn app_update_method_call(
        &self,
        params: AppUpdateMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppUpdateResult, TransactionSenderError> {
        // Extract compilation metadata using helper method
        let (compiled_approval, compiled_clear) = self.extract_compilation_metadata(&params);
        let params_clone = params.clone();

        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_app_update_method_call(params),
            |base_result| {
                // Extract ABI return using helper method for enhanced processing
                let abi_return = self.extract_abi_return_from_result(&base_result, &params_clone);

                // Convert CompiledTeal to Vec<u8> for the result
                let approval_bytes = compiled_approval.map(|ct| ct.compiled_base64_to_bytes);
                let clear_bytes = compiled_clear.map(|ct| ct.compiled_base64_to_bytes);

                Ok(SendAppUpdateResult::new(
                    base_result,
                    abi_return,
                    approval_bytes,
                    clear_bytes,
                ))
            },
        )
        .await
    }

    /// Send ABI method call for app deletion.
    pub async fn app_delete_method_call(
        &self,
        params: AppDeleteMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCallResult, TransactionSenderError> {
        let params_clone = params.clone();
        self.send_single_transaction_with_result(
            send_params,
            |composer| composer.add_app_delete_method_call(params),
            |base_result| {
                // Extract ABI return using helper method for enhanced processing
                let abi_return = self.extract_abi_return_from_result(&base_result, &params_clone);
                Ok(SendAppCallResult::new(base_result, abi_return))
            },
        )
        .await
    }

    /// Send online key registration transaction.
    pub async fn online_key_registration(
        &self,
        params: OnlineKeyRegistrationParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| {
            composer.add_online_key_registration(params)
        })
        .await
    }

    /// Send offline key registration transaction.
    pub async fn offline_key_registration(
        &self,
        params: OfflineKeyRegistrationParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        self.send_single_transaction(send_params, |composer| {
            composer.add_offline_key_registration(params)
        })
        .await
    }

    /// Generate lease from arbitrary data.
    pub fn encode_lease(&self, lease_data: &[u8]) -> Result<[u8; 32], TransactionSenderError> {
        if lease_data.len() <= 32 {
            let mut lease = [0u8; 32];
            lease[..lease_data.len()].copy_from_slice(lease_data);
            Ok(lease)
        } else {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(lease_data);
            let hash_result = hasher.finalize();
            let mut lease = [0u8; 32];
            lease.copy_from_slice(&hash_result);
            Ok(lease)
        }
    }

    /// Generate unique lease from string identifier.
    pub fn string_lease(&self, identifier: &str) -> [u8; 32] {
        self.encode_lease(identifier.as_bytes()).unwrap()
    }
}
