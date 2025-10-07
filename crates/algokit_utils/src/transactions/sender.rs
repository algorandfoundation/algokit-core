use super::{
    app_call::{
        AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams, AppCreateParams,
        AppDeleteMethodCallParams, AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams,
    },
    asset_config::{AssetConfigParams, AssetCreateParams, AssetDestroyParams},
    asset_freeze::{AssetFreezeParams, AssetUnfreezeParams},
    asset_transfer::{
        AssetClawbackParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
    },
    composer::{ComposerError, SendParams, TransactionComposer, TransactionResult},
    key_registration::{
        NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
        OnlineKeyRegistrationParams,
    },
    payment::{AccountCloseParams, PaymentParams},
};
use crate::clients::asset_manager::{AssetManager, AssetManagerError};
use crate::{clients::app_manager::AppManagerError, transactions::TransactionComposerConfig};
use algod_client::apis::AlgodApiError;
use algod_client::models::PendingTransactionResponse;
use algokit_transact::{Address, Byte32, Transaction};
use snafu::Snafu;

use std::{str::FromStr, sync::Arc};

#[derive(Debug, Snafu)]
pub enum TransactionSenderError {
    #[snafu(display("Algod client error: {source}"))]
    AlgodClientError { source: AlgodApiError },
    #[snafu(display("Composer error: {source}"))]
    ComposerError { source: ComposerError },
    #[snafu(display("Asset manager error: {source}"))]
    AssetManagerError { source: AssetManagerError },
    #[snafu(display("App manager error: {source}"))]
    AppManagerError { source: AppManagerError },
    #[snafu(display("Invalid parameters: {message}"))]
    InvalidParameters { message: String },
    #[snafu(display("Transaction validation error: {message}"))]
    ValidationError { message: String },
}

impl From<AlgodApiError> for TransactionSenderError {
    fn from(e: AlgodApiError) -> Self {
        Self::AlgodClientError { source: e }
    }
}

impl From<ComposerError> for TransactionSenderError {
    fn from(e: ComposerError) -> Self {
        Self::ComposerError { source: e }
    }
}

impl From<AssetManagerError> for TransactionSenderError {
    fn from(e: AssetManagerError) -> Self {
        Self::AssetManagerError { source: e }
    }
}

impl From<AppManagerError> for TransactionSenderError {
    fn from(e: AppManagerError) -> Self {
        Self::AppManagerError { source: e }
    }
}

/// Result from sending a single transaction.
#[derive(Debug, Clone)]
pub struct SendResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
}

/// Result from sending an asset create transaction.
#[derive(Debug, Clone)]
pub struct SendAssetCreateResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The ID of the created asset
    pub asset_id: u64,
}

/// Result from sending an app create transaction.
#[derive(Debug, Clone)]
pub struct SendAppCreateResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
}

/// Result from sending an app method call transaction.
#[derive(Debug, Clone)]
pub struct SendAppMethodCallResult {
    /// The result of the primary (last) transaction
    pub result: TransactionResult,
    /// All transaction results from the composer
    pub group_results: Vec<TransactionResult>,
    /// The group ID (optional)
    pub group: Option<Byte32>,
}

/// Result from sending an app create method call transaction.
#[derive(Debug, Clone)]
pub struct SendAppCreateMethodCallResult {
    /// The result of the primary (last) transaction
    pub result: TransactionResult,
    /// All transaction results from the composer
    pub group_results: Vec<TransactionResult>,
    /// The group ID (optional)
    pub group: Option<Byte32>,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
}

/// Sends transactions and groups with validation and result processing.
#[derive(Clone)]
pub struct TransactionSender {
    asset_manager: AssetManager,
    new_composer: Arc<dyn Fn(Option<TransactionComposerConfig>) -> TransactionComposer>,
}

impl TransactionSender {
    /// Create a new transaction sender.
    ///
    /// # Arguments
    /// * `new_composer` - Factory function for creating new transaction composers
    /// * `asset_manager` - Asset manager for handling asset operations
    ///
    /// # Returns
    /// A new `TransactionSender` instance
    pub fn new(
        new_composer: impl Fn(Option<TransactionComposerConfig>) -> TransactionComposer + 'static,
        asset_manager: AssetManager,
    ) -> Self {
        Self {
            asset_manager,
            new_composer: Arc::new(new_composer),
        }
    }

    /// Create a new transaction composer group.
    ///
    /// # Arguments
    /// * `params` - Optional configuration for the transaction composer
    ///
    /// # Returns
    /// A new `Composer` instance
    pub fn new_composer(&self, params: Option<TransactionComposerConfig>) -> TransactionComposer {
        (self.new_composer)(params)
    }

    async fn send_single_transaction<F>(
        &self,
        add_transaction: F,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError>
    where
        F: FnOnce(&mut TransactionComposer) -> Result<(), ComposerError>,
    {
        let mut composer = self.new_composer(None);
        add_transaction(&mut composer)?;
        let composer_results = composer.send(send_params).await?;

        let result =
            composer_results
                .results
                .last()
                .ok_or(TransactionSenderError::ValidationError {
                    message: "No transaction returned".to_string(),
                })?;

        Ok(SendResult {
            transaction: result.transaction.clone(),
            confirmation: result.confirmation.clone(),
            transaction_id: result.transaction_id.clone(),
        })
    }

    async fn send_single_transaction_with_result<F, R, T>(
        &self,
        add_transaction: F,
        transform_result: T,
        send_params: Option<SendParams>,
    ) -> Result<R, TransactionSenderError>
    where
        F: FnOnce(&mut TransactionComposer) -> Result<(), ComposerError>,
        T: FnOnce(SendResult) -> Result<R, TransactionSenderError>,
    {
        let base_result = self
            .send_single_transaction(add_transaction, send_params)
            .await?;
        transform_result(base_result)
    }

    async fn send_method_call<F>(
        &self,
        add_transaction: F,
        send_params: Option<SendParams>,
    ) -> Result<SendAppMethodCallResult, TransactionSenderError>
    where
        F: FnOnce(&mut TransactionComposer) -> Result<(), ComposerError>,
    {
        let mut composer = self.new_composer(None);
        add_transaction(&mut composer)?;
        let composer_results = composer.send(send_params).await?;

        let result = composer_results
            .results
            .last()
            .ok_or(TransactionSenderError::ValidationError {
                message: "No transaction returned".to_string(),
            })?
            .clone();

        Ok(SendAppMethodCallResult {
            result,
            group_results: composer_results.results,
            group: composer_results.group,
        })
    }

    async fn send_method_call_with_result<F, R, T>(
        &self,
        add_transaction: F,
        transform_result: T,
        send_params: Option<SendParams>,
    ) -> Result<R, TransactionSenderError>
    where
        F: FnOnce(&mut TransactionComposer) -> Result<(), ComposerError>,
        T: FnOnce(SendAppMethodCallResult) -> Result<R, TransactionSenderError>,
    {
        let base_result = self.send_method_call(add_transaction, send_params).await?;
        transform_result(base_result)
    }

    /// Send a payment transaction to transfer Algo between accounts.
    ///
    /// # Arguments
    /// * `params` - The parameters for the payment transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the payment transaction and the transaction that was sent
    pub async fn payment(
        &self,
        params: PaymentParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_payment(params), send_params)
            .await
    }

    /// Close an account and transfer remaining balance to another account.
    ///
    /// **Warning:** Be careful this can lead to loss of funds if not used correctly.
    ///
    /// # Arguments
    /// * `params` - The parameters for the account close transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the account close transaction and the transaction that was sent
    pub async fn account_close(
        &self,
        params: AccountCloseParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_account_close(params), send_params)
            .await
    }

    /// Transfer an Algorand Standard Asset.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset transfer transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset transfer transaction and the transaction that was sent
    pub async fn asset_transfer(
        &self,
        params: AssetTransferParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_transfer(params), send_params)
            .await
    }

    /// Opt an account into an Algorand Standard Asset.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset opt-in transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset opt-in transaction and the transaction that was sent
    pub async fn asset_opt_in(
        &self,
        params: AssetOptInParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_opt_in(params), send_params)
            .await
    }

    /// Opt an account out of an Algorand Standard Asset.
    ///
    /// **Note:** If the account has a balance of the asset,
    /// it will not be able to opt-out unless `ensure_zero_balance`
    /// is set to `false` (but then the account will lose the assets).
    /// When no close remainder to address is specified, the asset creator will be resolved and used.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset opt-out transaction
    /// * `send_params` - Optional parameters for sending the transaction
    /// * `ensure_zero_balance` - Whether to ensure the account has zero balance before opting out
    ///
    /// # Returns
    /// The result of the asset opt-out transaction and the transaction that was sent
    pub async fn asset_opt_out(
        &self,
        params: AssetOptOutParams,
        send_params: Option<SendParams>,
        ensure_zero_balance: Option<bool>,
    ) -> Result<SendResult, TransactionSenderError> {
        if ensure_zero_balance.unwrap_or(true) {
            // Ensure account has zero balance before opting out
            let account_info = self
                .asset_manager
                .get_account_information(&params.sender, params.asset_id)
                .await
                .map_err(|e| TransactionSenderError::ValidationError {
                    message: format!(
                        "Account {} validation failed for Asset {}: {}",
                        params.sender, params.asset_id, e
                    ),
                })?;

            let balance = account_info
                .asset_holding
                .as_ref()
                .map(|h| h.amount)
                .unwrap_or(0);
            if balance != 0 {
                return Err(TransactionSenderError::ValidationError {
                    message: format!(
                        "Account {} does not have a zero balance for Asset {}; can't opt-out.",
                        params.sender, params.asset_id
                    ),
                });
            }
        }

        // Resolve close_remainder_to to asset creator if not specified
        let params = if params.close_remainder_to.is_none() {
            let asset_info = self
                .asset_manager
                .get_by_id(params.asset_id)
                .await
                .map_err(|e| TransactionSenderError::ValidationError {
                    message: format!("Failed to get asset {} information: {}", params.asset_id, e),
                })?;

            let creator = Address::from_str(&asset_info.creator).map_err(|e| {
                TransactionSenderError::ValidationError {
                    message: format!(
                        "Invalid creator address for asset {}: {}",
                        params.asset_id, e
                    ),
                }
            })?;

            AssetOptOutParams {
                close_remainder_to: Some(creator),
                ..params
            }
        } else {
            params
        };

        self.send_single_transaction(|composer| composer.add_asset_opt_out(params), send_params)
            .await
    }

    /// Create a new Algorand Standard Asset.
    ///
    /// The account that sends this transaction will automatically be
    /// opted in to the asset and will hold all units after creation.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset creation transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset create transaction and the transaction that was sent
    pub async fn asset_create(
        &self,
        params: AssetCreateParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAssetCreateResult, TransactionSenderError> {
        self.send_single_transaction_with_result(
            |composer| composer.add_asset_create(params),
            |base_result| {
                let asset_id = base_result.confirmation.asset_id.ok_or_else(|| {
                    TransactionSenderError::ValidationError {
                        message: "Asset creation confirmation missing asset-index".to_string(),
                    }
                })?;
                Ok(SendAssetCreateResult {
                    transaction: base_result.transaction,
                    confirmation: base_result.confirmation,
                    transaction_id: base_result.transaction_id,
                    asset_id,
                })
            },
            send_params,
        )
        .await
    }

    /// Configure an existing Algorand Standard Asset.
    ///
    /// **Note:** The manager, reserve, freeze, and clawback addresses
    /// are immutably empty if they are not set. If manager is not set then
    /// all fields are immutable from that point forward.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset config transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset config transaction and the transaction that was sent
    pub async fn asset_config(
        &self,
        params: AssetConfigParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_config(params), send_params)
            .await
    }

    /// Destroys an Algorand Standard Asset.
    ///
    /// Created assets can be destroyed only by the asset manager account.
    /// All of the assets must be owned by the creator of the asset before
    /// the asset can be deleted.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset destroy transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset destroy transaction and the transaction that was sent
    pub async fn asset_destroy(
        &self,
        params: AssetDestroyParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_destroy(params), send_params)
            .await
    }

    /// Freeze an Algorand Standard Asset for an account.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset freeze transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset freeze transaction and the transaction that was sent
    pub async fn asset_freeze(
        &self,
        params: AssetFreezeParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_freeze(params), send_params)
            .await
    }

    /// Unfreeze an Algorand Standard Asset for an account.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset unfreeze transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset unfreeze transaction and the transaction that was sent
    pub async fn asset_unfreeze(
        &self,
        params: AssetUnfreezeParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_unfreeze(params), send_params)
            .await
    }

    /// Clawback an Algorand Standard Asset from an account.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset clawback transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the asset clawback transaction and the transaction that was sent
    pub async fn asset_clawback(
        &self,
        params: AssetClawbackParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_asset_clawback(params), send_params)
            .await
    }

    /// Call a smart contract.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app call transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the app call transaction and the transaction that was sent
    pub async fn app_call(
        &self,
        params: AppCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_app_call(params), send_params)
            .await
    }

    /// Create a smart contract.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app creation transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the app create transaction and the transaction that was sent
    pub async fn app_create(
        &self,
        params: AppCreateParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCreateResult, TransactionSenderError> {
        self.send_single_transaction_with_result(
            |composer| composer.add_app_create(params),
            |base_result| {
                let app_id = base_result.confirmation.app_id.ok_or_else(|| {
                    TransactionSenderError::ValidationError {
                        message: "App creation confirmation missing application-index".to_string(),
                    }
                })?;
                Ok(SendAppCreateResult {
                    transaction: base_result.transaction,
                    confirmation: base_result.confirmation,
                    transaction_id: base_result.transaction_id,
                    app_id,
                    app_address: Address::from_app_id(&app_id),
                })
            },
            send_params,
        )
        .await
    }

    /// Update a smart contract.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app update transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the app update transaction and the transaction that was sent
    pub async fn app_update(
        &self,
        params: AppUpdateParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_app_update(params), send_params)
            .await
    }

    /// Delete a smart contract.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app deletion transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the app delete transaction and the transaction that was sent
    pub async fn app_delete(
        &self,
        params: AppDeleteParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(|composer| composer.add_app_delete(params), send_params)
            .await
    }

    /// Call a smart contract via an ABI method.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app call transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the application ABI method call transaction and the transaction that was sent
    pub async fn app_call_method_call(
        &self,
        params: AppCallMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppMethodCallResult, TransactionSenderError> {
        self.send_method_call(
            |composer| composer.add_app_call_method_call(params),
            send_params,
        )
        .await
    }

    /// Create a smart contract via an ABI method.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app creation transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the application ABI method create transaction and the transaction that was sent
    pub async fn app_create_method_call(
        &self,
        params: AppCreateMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCreateMethodCallResult, TransactionSenderError> {
        self.send_method_call_with_result(
            |composer| composer.add_app_create_method_call(params),
            |base_result| {
                let app_id = base_result.result.confirmation.app_id.ok_or_else(|| {
                    TransactionSenderError::ValidationError {
                        message: "App creation confirmation missing application-index".to_string(),
                    }
                })?;
                Ok(SendAppCreateMethodCallResult {
                    result: base_result.result,
                    group_results: base_result.group_results,
                    group: base_result.group,
                    app_id,
                    app_address: Address::from_app_id(&app_id),
                })
            },
            send_params,
        )
        .await
    }

    /// Update a smart contract via an ABI method.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app update transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the application ABI method update transaction and the transaction that was sent
    pub async fn app_update_method_call(
        &self,
        params: AppUpdateMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppMethodCallResult, TransactionSenderError> {
        self.send_method_call(
            |composer| composer.add_app_update_method_call(params),
            send_params,
        )
        .await
    }

    /// Delete a smart contract via an ABI method.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app deletion transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the application ABI method delete transaction and the transaction that was sent
    pub async fn app_delete_method_call(
        &self,
        params: AppDeleteMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppMethodCallResult, TransactionSenderError> {
        self.send_method_call(
            |composer| composer.add_app_delete_method_call(params),
            send_params,
        )
        .await
    }

    /// Register an online key.
    ///
    /// # Arguments
    /// * `params` - The parameters for the key registration transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the online key registration transaction and the transaction that was sent
    pub async fn online_key_registration(
        &self,
        params: OnlineKeyRegistrationParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(
            |composer| composer.add_online_key_registration(params),
            send_params,
        )
        .await
    }

    /// Register an offline key.
    ///
    /// # Arguments
    /// * `params` - The parameters for the key registration transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the offline key registration transaction and the transaction that was sent
    pub async fn offline_key_registration(
        &self,
        params: OfflineKeyRegistrationParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(
            |composer| composer.add_offline_key_registration(params),
            send_params,
        )
        .await
    }

    /// Register a non-participation key.
    ///
    /// # Arguments
    /// * `params` - The parameters for the key registration transaction
    /// * `send_params` - Optional parameters for sending the transaction
    ///
    /// # Returns
    /// The result of the non-participation key registration transaction and the transaction that was sent
    pub async fn non_participation_key_registration(
        &self,
        params: NonParticipationKeyRegistrationParams,
        send_params: Option<SendParams>,
    ) -> Result<SendResult, TransactionSenderError> {
        self.send_single_transaction(
            |composer| composer.add_non_participation_key_registration(params),
            send_params,
        )
        .await
    }
}
