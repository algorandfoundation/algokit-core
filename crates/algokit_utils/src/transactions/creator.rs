use algokit_transact::Transaction;
use std::sync::Arc;

use crate::{AccountCloseParams, transactions::TransactionComposerConfig};

use super::{
    AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams, AppCreateParams,
    AppDeleteMethodCallParams, AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams,
    AssetClawbackParams, AssetConfigParams, AssetCreateParams, AssetDestroyParams,
    AssetFreezeParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
    AssetUnfreezeParams, NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams, PaymentParams,
    composer::{ComposerError, TransactionComposer},
};

/// Creates Algorand transactions.
pub struct TransactionCreator {
    new_composer: Arc<dyn Fn(Option<TransactionComposerConfig>) -> TransactionComposer>,
}

impl TransactionCreator {
    pub fn new(
        new_composer: impl Fn(Option<TransactionComposerConfig>) -> TransactionComposer + 'static,
    ) -> Self {
        Self {
            new_composer: Arc::new(new_composer),
        }
    }

    pub(crate) async fn transaction<F>(
        &self,
        composer_method: F,
    ) -> Result<Transaction, ComposerError>
    where
        F: FnOnce(&mut TransactionComposer) -> Result<(), ComposerError>,
    {
        let mut composer = (self.new_composer)(None);
        composer_method(&mut composer)?;
        let built_transactions = composer.build().await?;

        built_transactions
            .last()
            .map(|tx_with_signer| tx_with_signer.transaction.clone())
            .ok_or(ComposerError::StateError {
                message: "No transactions were built by the composer".to_string(),
            })
    }

    /// Create a payment transaction to transfer Algo between accounts.
    ///
    /// # Arguments
    /// * `params` - The parameters for the payment transaction
    ///
    /// # Returns
    /// The payment transaction
    pub async fn payment(&self, params: PaymentParams) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_payment(params))
            .await
    }

    /// Create an account close transaction to close an account and transfer all remaining funds.
    ///
    /// # Arguments
    /// * `params` - The parameters for the account close transaction
    ///
    /// # Returns
    /// The account close transaction
    pub async fn account_close(
        &self,
        params: AccountCloseParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_account_close(params))
            .await
    }

    /// Create a create Algorand Standard Asset transaction.
    ///
    /// The account that sends this transaction will automatically be
    /// opted in to the asset and will hold all units after creation.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset creation transaction
    ///
    /// # Returns
    /// The asset create transaction
    pub async fn asset_create(
        &self,
        params: AssetCreateParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_create(params))
            .await
    }

    /// Create an Algorand Standard Asset transfer transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset transfer transaction
    ///
    /// # Returns
    /// The asset transfer transaction
    pub async fn asset_transfer(
        &self,
        params: AssetTransferParams,
    ) -> Result<Transaction, ComposerError> {
        if params.asset_id == 0 {
            return Err(ComposerError::TransactionError {
                message: "Asset ID must be greater than 0".to_string(),
            });
        }
        self.transaction(|composer| composer.add_asset_transfer(params))
            .await
    }

    /// Create an Algorand Standard Asset opt-in transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset opt-in transaction
    ///
    /// # Returns
    /// The asset opt-in transaction
    pub async fn asset_opt_in(
        &self,
        params: AssetOptInParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_opt_in(params))
            .await
    }

    /// Create an asset opt-out transaction.
    ///
    /// **Note:** If the account has a balance of the asset, it will lose those assets
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset opt-out transaction
    ///
    /// # Returns
    /// The asset opt-out transaction
    pub async fn asset_opt_out(
        &self,
        params: AssetOptOutParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_opt_out(params))
            .await
    }

    /// Create an asset config transaction to reconfigure an existing Algorand Standard Asset.
    ///
    /// **Note:** The manager, reserve, freeze, and clawback addresses
    /// are immutably empty if they are not set. If manager is not set then
    /// all fields are immutable from that point forward.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset config transaction
    ///
    /// # Returns
    /// The asset config transaction
    pub async fn asset_config(
        &self,
        params: AssetConfigParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_config(params))
            .await
    }

    /// Create an Algorand Standard Asset destroy transaction.
    ///
    /// Created assets can be destroyed only by the asset manager account.
    /// All of the assets must be owned by the creator of the asset before
    /// the asset can be deleted.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset destroy transaction
    ///
    /// # Returns
    /// The asset destroy transaction
    pub async fn asset_destroy(
        &self,
        params: AssetDestroyParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_destroy(params))
            .await
    }

    /// Create an Algorand Standard Asset freeze transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset freeze transaction
    ///
    /// # Returns
    /// The asset freeze transaction
    pub async fn asset_freeze(
        &self,
        params: AssetFreezeParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_freeze(params))
            .await
    }

    /// Create an Algorand Standard Asset unfreeze transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset unfreeze transaction
    ///
    /// # Returns
    /// The asset unfreeze transaction
    pub async fn asset_unfreeze(
        &self,
        params: AssetUnfreezeParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_unfreeze(params))
            .await
    }

    /// Create an Algorand Standard Asset clawback transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the asset clawback transaction
    ///
    /// # Returns
    /// The asset clawback transaction
    pub async fn asset_clawback(
        &self,
        params: AssetClawbackParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_clawback(params))
            .await
    }

    /// Create an application call transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app call transaction
    ///
    /// # Returns
    /// The application call transaction
    pub async fn app_call(&self, params: AppCallParams) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_call(params))
            .await
    }

    /// Create an application create transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app creation transaction
    ///
    /// # Returns
    /// The application create transaction
    pub async fn app_create(&self, params: AppCreateParams) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_create(params))
            .await
    }

    /// Create an application update transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app update transaction
    ///
    /// # Returns
    /// The application update transaction
    pub async fn app_update(&self, params: AppUpdateParams) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_update(params))
            .await
    }

    /// Create an application delete transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the app deletion transaction
    ///
    /// # Returns
    /// The application delete transaction
    pub async fn app_delete(&self, params: AppDeleteParams) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_delete(params))
            .await
    }

    /// Create an application call with ABI method call transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the ABI method call transaction
    ///
    /// # Returns
    /// The application ABI method call transaction
    pub async fn app_call_method_call(
        &self,
        params: AppCallMethodCallParams,
    ) -> Result<Vec<Transaction>, ComposerError> {
        self.built_transactions(|composer| composer.add_app_call_method_call(params))
            .await
    }

    /// Create an application create call with ABI method call transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the ABI method creation transaction
    ///
    /// # Returns
    /// The application ABI method create transaction
    pub async fn app_create_method_call(
        &self,
        params: AppCreateMethodCallParams,
    ) -> Result<Vec<Transaction>, ComposerError> {
        self.built_transactions(|composer| composer.add_app_create_method_call(params))
            .await
    }

    /// Create an application update call with ABI method call transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the ABI method update transaction
    ///
    /// # Returns
    /// The application ABI method update transaction
    pub async fn app_update_method_call(
        &self,
        params: AppUpdateMethodCallParams,
    ) -> Result<Vec<Transaction>, ComposerError> {
        self.built_transactions(|composer| composer.add_app_update_method_call(params))
            .await
    }

    /// Create an application delete call with ABI method call transaction.
    ///
    /// **Note**: you may prefer to use `algorand.client` to get an app client for more advanced functionality.
    ///
    /// # Arguments
    /// * `params` - The parameters for the ABI method deletion transaction
    ///
    /// # Returns
    /// The application ABI method delete transaction
    pub async fn app_delete_method_call(
        &self,
        params: AppDeleteMethodCallParams,
    ) -> Result<Vec<Transaction>, ComposerError> {
        self.built_transactions(|composer| composer.add_app_delete_method_call(params))
            .await
    }

    /// Create an online key registration transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the key registration transaction
    ///
    /// # Returns
    /// The online key registration transaction
    pub async fn online_key_registration(
        &self,
        params: OnlineKeyRegistrationParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_online_key_registration(params))
            .await
    }

    /// Create an offline key registration transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the key registration transaction
    ///
    /// # Returns
    /// The offline key registration transaction
    pub async fn offline_key_registration(
        &self,
        params: OfflineKeyRegistrationParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_offline_key_registration(params))
            .await
    }

    /// Create a non-participation key registration transaction.
    ///
    /// # Arguments
    /// * `params` - The parameters for the non-participation key registration transaction
    ///
    /// # Returns
    /// The non participating key registration transaction
    pub async fn non_participation_key_registration(
        &self,
        params: NonParticipationKeyRegistrationParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_non_participation_key_registration(params))
            .await
    }

    async fn built_transactions<F>(
        &self,
        composer_method: F,
    ) -> Result<Vec<Transaction>, ComposerError>
    where
        F: FnOnce(&mut TransactionComposer) -> Result<(), ComposerError>,
    {
        let mut composer = (self.new_composer)(None);
        composer_method(&mut composer)?;
        let transactions_with_signers = composer.build().await?;

        Ok(transactions_with_signers
            .iter()
            .map(|ts| ts.transaction.clone())
            .collect())
    }
}
