use algokit_abi::ABIMethod;
use algokit_transact::Transaction;
use derive_more::Debug;
use std::{collections::HashMap, sync::Arc};

use super::{
    AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams, AppCreateParams,
    AppDeleteMethodCallParams, AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams,
    AssetClawbackParams, AssetCreateParams, AssetDestroyParams, AssetFreezeParams,
    AssetOptInParams, AssetOptOutParams, AssetReconfigureParams, AssetTransferParams,
    AssetUnfreezeParams, NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams, PaymentParams,
    common::TransactionSigner,
    composer::{Composer, ComposerError},
};

#[derive(Debug, Clone)]
pub struct BuiltTransactions {
    pub transactions: Vec<Transaction>,
    pub method_calls: HashMap<usize, ABIMethod>,
    #[debug(skip)]
    pub signers: HashMap<usize, Arc<dyn TransactionSigner>>,
}

/// Creates individual Algorand transactions.
pub struct TransactionCreator {
    new_group: Arc<dyn Fn() -> Composer>,
}

impl TransactionCreator {
    pub fn new(new_group: impl Fn() -> Composer + 'static) -> Self {
        Self {
            new_group: Arc::new(new_group),
        }
    }

    pub(crate) async fn transaction<F>(
        &self,
        composer_method: F,
    ) -> Result<Transaction, ComposerError>
    where
        F: FnOnce(&mut Composer) -> Result<(), ComposerError>,
    {
        let mut composer = (self.new_group)();
        composer_method(&mut composer)?;
        let built_transactions = composer.build(None).await?;

        built_transactions
            .last()
            .map(|tx_with_signer| tx_with_signer.transaction.clone())
            .ok_or(ComposerError::StateError {
                message: "No transactions were built by the composer".to_string(),
            })
    }

    pub async fn payment(&self, params: PaymentParams) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_payment(params))
            .await
    }

    pub async fn asset_create(
        &self,
        params: AssetCreateParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_create(params))
            .await
    }

    pub async fn asset_transfer(
        &self,
        params: AssetTransferParams,
    ) -> Result<Transaction, ComposerError> {
        // Enhanced parameter validation
        if params.asset_id == 0 {
            return Err(ComposerError::TransactionError {
                message: "Asset ID must be greater than 0".to_string(),
            });
        }
        // Note: amount can be 0 for opt-in transactions, so we don't validate it here

        self.transaction(|composer| composer.add_asset_transfer(params))
            .await
    }

    pub async fn asset_opt_in(
        &self,
        params: AssetOptInParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_opt_in(params))
            .await
    }

    pub async fn asset_opt_out(
        &self,
        params: AssetOptOutParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_opt_out(params))
            .await
    }

    pub async fn asset_config(
        &self,
        params: AssetReconfigureParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_reconfigure(params))
            .await
    }

    pub async fn asset_destroy(
        &self,
        params: AssetDestroyParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_destroy(params))
            .await
    }

    pub async fn asset_freeze(
        &self,
        params: AssetFreezeParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_freeze(params))
            .await
    }

    pub async fn asset_unfreeze(
        &self,
        params: AssetUnfreezeParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_unfreeze(params))
            .await
    }

    pub async fn asset_clawback(
        &self,
        params: AssetClawbackParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_asset_clawback(params))
            .await
    }

    pub async fn application_call(
        &self,
        params: AppCallParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_call(params))
            .await
    }

    pub async fn application_create(
        &self,
        params: AppCreateParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_create(params))
            .await
    }

    pub async fn application_update(
        &self,
        params: AppUpdateParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_update(params))
            .await
    }

    pub async fn application_delete(
        &self,
        params: AppDeleteParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_app_delete(params))
            .await
    }

    pub async fn application_method_call(
        &self,
        params: AppCallMethodCallParams,
    ) -> Result<BuiltTransactions, ComposerError> {
        self.built_transactions(|composer| composer.add_app_call_method_call(params))
            .await
    }

    pub async fn application_create_method_call(
        &self,
        params: AppCreateMethodCallParams,
    ) -> Result<BuiltTransactions, ComposerError> {
        self.built_transactions(|composer| composer.add_app_create_method_call(params))
            .await
    }

    pub async fn application_update_method_call(
        &self,
        params: AppUpdateMethodCallParams,
    ) -> Result<BuiltTransactions, ComposerError> {
        self.built_transactions(|composer| composer.add_app_update_method_call(params))
            .await
    }

    pub async fn application_delete_method_call(
        &self,
        params: AppDeleteMethodCallParams,
    ) -> Result<BuiltTransactions, ComposerError> {
        self.built_transactions(|composer| composer.add_app_delete_method_call(params))
            .await
    }

    pub async fn online_key_registration(
        &self,
        params: OnlineKeyRegistrationParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_online_key_registration(params))
            .await
    }

    pub async fn offline_key_registration(
        &self,
        params: OfflineKeyRegistrationParams,
    ) -> Result<Transaction, ComposerError> {
        self.transaction(|composer| composer.add_offline_key_registration(params))
            .await
    }

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
    ) -> Result<BuiltTransactions, ComposerError>
    where
        F: FnOnce(&mut Composer) -> Result<(), ComposerError>,
    {
        let mut composer = (self.new_group)();
        composer_method(&mut composer)?;
        let transactions_with_signers = composer.build(None).await?;

        let transactions = transactions_with_signers
            .iter()
            .map(|ts| ts.transaction.clone())
            .collect();

        let signers = transactions_with_signers
            .iter()
            .enumerate()
            .map(|(i, ts)| (i, ts.signer.clone()))
            .collect();

        let method_calls = composer.extract_method_calls();

        Ok(BuiltTransactions {
            transactions,
            method_calls,
            signers,
        })
    }
}
