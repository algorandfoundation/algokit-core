// Standard library imports
use std::sync::Arc;

// Third-party imports
use async_trait::async_trait;
use tokio::sync::Mutex;

// Crate imports
use crate::transactions::{
    app_call::{
        AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams, AppCreateParams,
        AppDeleteMethodCallParams, AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams,
    },
    asset_config::{AssetConfigParams, AssetCreateParams, AssetDestroyParams},
    asset_freeze::{AssetFreezeParams, AssetUnfreezeParams},
    asset_transfer::{
        AssetClawbackParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
    },
    common::{RustTransactionSignerGetterFromFfi, TransactionSignerGetter, UtilsError},
    payment::PaymentParams,
};

// External crate imports
// algod_client
use algod_client::AlgodClient as RustAlgodClient;

// algokit_http_client
use algokit_http_client::HttpClient;

// algokit_utils
use algokit_utils::transactions::{
    TransactionComposerParams, composer::TransactionComposer as RustComposer,
};

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
            RustComposer::new(TransactionComposerParams {
                algod_client: Arc::new(rust_algod_client.clone()),
                signer_getter: Arc::new(rust_signer_getter),
                composer_config: None,
            })
        };

        Composer {
            inner_composer: Mutex::new(rust_composer),
        }
    }

    pub fn add_payment(&self, params: PaymentParams) -> Result<(), UtilsError> {
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
            transaction_ids: result
                .results
                .iter()
                .map(|r| r.transaction_id.clone())
                .collect(),
            app_ids: result
                .results
                .iter()
                .map(|r| r.confirmation.app_id)
                .collect(),
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

    pub fn add_asset_config(&self, params: AssetConfigParams) -> Result<(), UtilsError> {
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

    pub fn add_app_create(&self, params: AppCreateParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_create(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_call(&self, params: AppCallParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_update(&self, params: AppUpdateParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_update(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_delete(&self, params: AppDeleteParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_delete(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_call_method_call(
        &self,
        params: AppCallMethodCallParams,
    ) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_call_method_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_create_method_call(
        &self,
        params: AppCreateMethodCallParams,
    ) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_create_method_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_update_method_call(
        &self,
        params: AppUpdateMethodCallParams,
    ) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_update_method_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_app_delete_method_call(
        &self,
        params: AppDeleteMethodCallParams,
    ) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_delete_method_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }
}

// Implement ComposerTrait for Composer to keep them in sync
#[async_trait]
impl ComposerTrait for Composer {
    async fn build(&self) -> Result<(), UtilsError> {
        Composer::build(self).await
    }

    async fn send(&self) -> Result<Vec<String>, UtilsError> {
        let response = Composer::send(self).await?;
        Ok(response.transaction_ids)
    }

    async fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), UtilsError> {
        Composer::add_payment(self, params)
    }

    async fn add_asset_create(&self, params: AssetCreateParams) -> Result<(), UtilsError> {
        Composer::add_asset_create(self, params)
    }

    async fn add_asset_reconfigure(&self, params: AssetConfigParams) -> Result<(), UtilsError> {
        Composer::add_asset_config(self, params)
    }

    async fn add_asset_destroy(&self, params: AssetDestroyParams) -> Result<(), UtilsError> {
        Composer::add_asset_destroy(self, params)
    }

    async fn add_asset_freeze(&self, params: AssetFreezeParams) -> Result<(), UtilsError> {
        Composer::add_asset_freeze(self, params)
    }

    async fn add_asset_unfreeze(&self, params: AssetUnfreezeParams) -> Result<(), UtilsError> {
        Composer::add_asset_unfreeze(self, params)
    }

    async fn add_asset_transfer(&self, params: AssetTransferParams) -> Result<(), UtilsError> {
        Composer::add_asset_transfer(self, params)
    }

    async fn add_asset_opt_in(&self, params: AssetOptInParams) -> Result<(), UtilsError> {
        Composer::add_asset_opt_in(self, params)
    }

    async fn add_asset_opt_out(&self, params: AssetOptOutParams) -> Result<(), UtilsError> {
        Composer::add_asset_opt_out(self, params)
    }

    async fn add_asset_clawback(&self, params: AssetClawbackParams) -> Result<(), UtilsError> {
        Composer::add_asset_clawback(self, params)
    }

    async fn add_app_create(
        &self,
        params: super::app_call::AppCreateParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_create(self, params)
    }

    async fn add_app_call(&self, params: super::app_call::AppCallParams) -> Result<(), UtilsError> {
        Composer::add_app_call(self, params)
    }

    async fn add_app_update(
        &self,
        params: super::app_call::AppUpdateParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_update(self, params)
    }

    async fn add_app_delete(
        &self,
        params: super::app_call::AppDeleteParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_delete(self, params)
    }

    async fn add_app_call_method_call(
        &self,
        params: super::app_call::AppCallMethodCallParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_call_method_call(self, params)
    }

    async fn add_app_create_method_call(
        &self,
        params: super::app_call::AppCreateMethodCallParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_create_method_call(self, params)
    }

    async fn add_app_update_method_call(
        &self,
        params: super::app_call::AppUpdateMethodCallParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_update_method_call(self, params)
    }

    async fn add_app_delete_method_call(
        &self,
        params: super::app_call::AppDeleteMethodCallParams,
    ) -> Result<(), UtilsError> {
        Composer::add_app_delete_method_call(self, params)
    }
}

//
// Foreign trait for target language testing
// This trait is implemented by Python to enable async test orchestration
// where Python controls the async context and Rust handles business logic
//
// STATEFUL DESIGN: Implementations store algod_client and signer_getter
// internally, eliminating the need to pass them on every method call.
//
#[uniffi::export(with_foreign)]
#[async_trait]
pub trait ComposerTrait: Send + Sync {
    async fn build(&self) -> Result<(), UtilsError>;
    async fn send(&self) -> Result<Vec<String>, UtilsError>;

    async fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), UtilsError>;

    async fn add_asset_create(&self, params: AssetCreateParams) -> Result<(), UtilsError>;

    async fn add_asset_reconfigure(&self, params: AssetConfigParams) -> Result<(), UtilsError>;

    async fn add_asset_destroy(&self, params: AssetDestroyParams) -> Result<(), UtilsError>;

    async fn add_asset_freeze(&self, params: AssetFreezeParams) -> Result<(), UtilsError>;

    async fn add_asset_unfreeze(&self, params: AssetUnfreezeParams) -> Result<(), UtilsError>;

    async fn add_asset_transfer(&self, params: AssetTransferParams) -> Result<(), UtilsError>;

    async fn add_asset_opt_in(&self, params: AssetOptInParams) -> Result<(), UtilsError>;

    async fn add_asset_opt_out(&self, params: AssetOptOutParams) -> Result<(), UtilsError>;

    async fn add_asset_clawback(&self, params: AssetClawbackParams) -> Result<(), UtilsError>;

    async fn add_app_create(
        &self,
        params: super::app_call::AppCreateParams,
    ) -> Result<(), UtilsError>;

    async fn add_app_call(&self, params: super::app_call::AppCallParams) -> Result<(), UtilsError>;

    async fn add_app_update(
        &self,
        params: super::app_call::AppUpdateParams,
    ) -> Result<(), UtilsError>;

    async fn add_app_delete(
        &self,
        params: super::app_call::AppDeleteParams,
    ) -> Result<(), UtilsError>;

    async fn add_app_call_method_call(
        &self,
        params: super::app_call::AppCallMethodCallParams,
    ) -> Result<(), UtilsError>;

    async fn add_app_create_method_call(
        &self,
        params: super::app_call::AppCreateMethodCallParams,
    ) -> Result<(), UtilsError>;

    async fn add_app_update_method_call(
        &self,
        params: super::app_call::AppUpdateMethodCallParams,
    ) -> Result<(), UtilsError>;

    async fn add_app_delete_method_call(
        &self,
        params: super::app_call::AppDeleteMethodCallParams,
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

//
// Concrete ComposerFactory implementation for Rust-side usage
// Creates fresh Composer instances (the FFI concrete type)
//
#[derive(uniffi::Object)]
pub struct DefaultComposerFactory {
    algod_client: Arc<AlgodClient>,
    signer_getter: Arc<dyn TransactionSignerGetter>,
}

#[uniffi::export]
impl DefaultComposerFactory {
    #[uniffi::constructor]
    pub fn new(
        algod_client: Arc<AlgodClient>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Self {
        DefaultComposerFactory {
            algod_client,
            signer_getter,
        }
    }
}

impl ComposerFactory for DefaultComposerFactory {
    fn create_composer(&self) -> Arc<dyn ComposerTrait> {
        Arc::new(Composer::new(
            self.algod_client.clone(),
            self.signer_getter.clone(),
        ))
    }
}
