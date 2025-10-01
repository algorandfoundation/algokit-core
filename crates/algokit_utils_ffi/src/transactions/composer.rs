use std::sync::Arc;

use crate::{
    clients::algod_client::AlgodClientTrait,
    transactions::{
        asset_config::{AssetCreateParams, AssetDestroyParams, AssetReconfigureParams},
        asset_freeze::{AssetFreezeParams, AssetUnfreezeParams},
        asset_transfer::{
            AssetClawbackParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
        },
        common::{RustTransactionSignerGetterFromFfi, TransactionSignerGetter, UtilsError},
    },
};
use algod_client::AlgodClient as RustAlgodClient;
use algokit_http_client::HttpClient;
use algokit_utils::transactions::{ComposerParams, composer::Composer as RustComposer};
use async_trait::async_trait;
use tokio::sync::Mutex;

#[derive(uniffi::Object)]
pub struct AlgodClient {
    inner_algod_client: Mutex<RustAlgodClient>,
}

#[uniffi::export]
impl AlgodClient {
    #[uniffi::constructor]
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        let algod_client = RustAlgodClient::new(http_client);
        AlgodClient {
            inner_algod_client: Mutex::new(algod_client),
        }
    }
}

// NOTE: This struct is a temporary placeholder until we have a proper algod_api_ffi crate with the fully typed response
#[derive(uniffi::Record)]
pub struct TempSendResponse {
    pub transaction_ids: Vec<String>,
    pub app_ids: Vec<Option<u64>>,
}

#[derive(uniffi::Object)]
pub struct Composer {
    inner_composer: Mutex<RustComposer>,
}

#[uniffi::export]
impl Composer {
    #[uniffi::constructor]
    pub fn new(
        algod_client: Arc<AlgodClient>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Self {
        let rust_signer_getter = RustTransactionSignerGetterFromFfi {
            ffi_signer_getter: signer_getter.clone(),
        };

        let rust_composer = {
            let rust_algod_client = algod_client.inner_algod_client.blocking_lock();
            RustComposer::new(ComposerParams {
                algod_client: Arc::new(rust_algod_client.clone()),
                signer_getter: Arc::new(rust_signer_getter),
                composer_config: None,
            })
        };

        Composer {
            inner_composer: Mutex::new(rust_composer),
        }
    }

    pub fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_payment(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub async fn send(&self) -> Result<TempSendResponse, UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        let result = composer
            .send(None)
            .await
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })?;
        Ok(TempSendResponse {
            transaction_ids: result.transaction_ids,
            app_ids: result.confirmations.iter().map(|c| c.app_id).collect(),
        })
    }

    pub async fn build(&self) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer.build().await.map_err(|e| UtilsError::UtilsError {
            message: e.to_string(),
        })?;

        Ok(())
    }

    pub fn add_asset_create(&self, params: AssetCreateParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_create(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_reconfigure(&self, params: AssetReconfigureParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_config(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_destroy(&self, params: AssetDestroyParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_destroy(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_freeze(&self, params: AssetFreezeParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_freeze(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_unfreeze(&self, params: AssetUnfreezeParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_unfreeze(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_transfer(&self, params: AssetTransferParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_transfer(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_opt_in(&self, params: AssetOptInParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_opt_in(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_opt_out(&self, params: AssetOptOutParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_opt_out(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_clawback(&self, params: AssetClawbackParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_clawback(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }
}

// Implement ComposerTrait for Composer to keep them in sync
#[async_trait]
impl ComposerTrait for Composer {
    async fn build(&self, _algod_client: Arc<dyn AlgodClientTrait>) -> Result<(), UtilsError> {
        Composer::build(self).await
    }

    async fn send(
        &self,
        _algod_client: Arc<dyn AlgodClientTrait>,
    ) -> Result<Vec<String>, UtilsError> {
        let response = Composer::send(self).await?;
        Ok(response.transaction_ids)
    }

    async fn add_payment(
        &self,
        params: super::payment::PaymentParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_payment(self, params)
    }

    async fn add_asset_create(
        &self,
        params: AssetCreateParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_create(self, params)
    }

    async fn add_asset_reconfigure(
        &self,
        params: AssetReconfigureParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_reconfigure(self, params)
    }

    async fn add_asset_destroy(
        &self,
        params: AssetDestroyParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_destroy(self, params)
    }

    async fn add_asset_freeze(
        &self,
        params: AssetFreezeParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_freeze(self, params)
    }

    async fn add_asset_unfreeze(
        &self,
        params: AssetUnfreezeParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_unfreeze(self, params)
    }

    async fn add_asset_transfer(
        &self,
        params: AssetTransferParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_transfer(self, params)
    }

    async fn add_asset_opt_in(
        &self,
        params: AssetOptInParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_opt_in(self, params)
    }

    async fn add_asset_opt_out(
        &self,
        params: AssetOptOutParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_opt_out(self, params)
    }

    async fn add_asset_clawback(
        &self,
        params: AssetClawbackParams,
        _algod_client: Arc<dyn AlgodClientTrait>,
        _signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError> {
        Composer::add_asset_clawback(self, params)
    }
}

//
// Foreign trait for target language testing
// This trait is implemented by Python to enable async test orchestration
// where Python controls the async context and Rust handles business logic
//
#[uniffi::export(with_foreign)]
#[async_trait]
pub trait ComposerTrait: Send + Sync {
    async fn build(&self, algod_client: Arc<dyn AlgodClientTrait>) -> Result<(), UtilsError>;
    async fn send(
        &self,
        algod_client: Arc<dyn AlgodClientTrait>,
    ) -> Result<Vec<String>, UtilsError>;

    async fn add_payment(
        &self,
        params: super::payment::PaymentParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_create(
        &self,
        params: AssetCreateParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_reconfigure(
        &self,
        params: AssetReconfigureParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_destroy(
        &self,
        params: AssetDestroyParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_freeze(
        &self,
        params: AssetFreezeParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_unfreeze(
        &self,
        params: AssetUnfreezeParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_transfer(
        &self,
        params: AssetTransferParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_opt_in(
        &self,
        params: AssetOptInParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_opt_out(
        &self,
        params: AssetOptOutParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;

    async fn add_asset_clawback(
        &self,
        params: AssetClawbackParams,
        algod_client: Arc<dyn AlgodClientTrait>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<(), UtilsError>;
}

//
// Foreign trait for creating fresh composer instances
// This enables proper composer lifecycle management for multi-operation tests
//
#[uniffi::export(with_foreign)]
pub trait ComposerFactory: Send + Sync {
    fn create_composer(&self) -> Arc<dyn ComposerTrait>;
}
