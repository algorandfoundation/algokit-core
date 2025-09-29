use crate::applications::app_client::error_transformation::LogicErrorContext;
use crate::applications::app_client::{AppClientMethodCallParams, CompilationParams};
use crate::applications::app_deployer::{AppLookup, OnSchemaBreak, OnUpdate};
use crate::applications::{
    AppDeployMetadata, AppDeployParams, AppDeployResult, CreateParams, DeleteParams, UpdateParams,
};
use crate::transactions::{
    TransactionComposerConfig, TransactionSigner, composer::SendParams as ComposerSendParams,
};
use crate::{AlgorandClient, AppClient, AppClientParams, AppSourceMaps, TransactionSenderError};
use algokit_abi::Arc56Contract;
use algokit_abi::arc56_contract::CallOnApplicationComplete;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod compilation;
mod error;
mod params_builder;
mod transaction_builder;
mod transaction_sender;
mod types;
mod utils;

pub use error::AppFactoryError;
pub use params_builder::ParamsBuilder;
pub use transaction_builder::TransactionBuilder;
pub use transaction_sender::TransactionSender;
pub use types::*;

/// ARC-56 factory that compiles an application spec, deploys app instances,
/// and builds [`AppClient`]s for interacting with them.
///
/// Constructed from [`AppFactoryParams`], the factory centralises shared context such
/// as the Algorand client, default sender and signer, and any deploy-time template
/// substitutions.
pub struct AppFactory {
    app_spec: Arc56Contract,
    algorand: Arc<AlgorandClient>,
    app_name: String,
    version: String,
    default_sender: Option<String>,
    default_signer: Option<Arc<dyn TransactionSigner>>,
    approval_source_map: Mutex<Option<serde_json::Value>>,
    clear_source_map: Mutex<Option<serde_json::Value>>,
    compilation_params: Option<CompilationParams>,
    transaction_composer_config: Option<TransactionComposerConfig>,
}

#[derive(Default)]
pub struct DeployArgs {
    pub on_update: Option<OnUpdate>,
    pub on_schema_break: Option<OnSchemaBreak>,
    pub create_params: Option<AppFactoryCreateMethodCallParams>,
    pub update_params: Option<AppClientMethodCallParams>,
    pub delete_params: Option<AppClientMethodCallParams>,
    pub existing_deployments: Option<AppLookup>,
    pub ignore_cache: Option<bool>,
    pub app_name: Option<String>,
    pub send_params: Option<ComposerSendParams>,
}

impl AppFactory {
    pub fn new(params: AppFactoryParams) -> Self {
        let AppFactoryParams {
            algorand,
            app_spec,
            app_name,
            default_sender,
            default_signer,
            version,
            compilation_params,
            source_maps,
            transaction_composer_config,
        } = params;

        let (initial_approval_source_map, initial_clear_source_map) = match source_maps {
            Some(maps) => (maps.approval_source_map, maps.clear_source_map),
            None => (None, None),
        };

        Self {
            app_spec,
            algorand,
            app_name: app_name.unwrap_or_else(|| "<unnamed>".to_string()),
            version: version.unwrap_or_else(|| "1.0".to_string()),
            default_sender,
            default_signer,
            approval_source_map: Mutex::new(initial_approval_source_map),
            clear_source_map: Mutex::new(initial_clear_source_map),
            compilation_params,
            transaction_composer_config,
        }
    }

    /// Returns the application name derived from the app spec or provided override.
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    /// Returns the normalised ARC-56 contract backing this factory.
    pub fn app_spec(&self) -> &Arc56Contract {
        &self.app_spec
    }

    /// Returns the shared [`AlgorandClient`] configured for the factory.
    pub fn algorand(&self) -> Arc<AlgorandClient> {
        self.algorand.clone()
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns a [`ParamsBuilder`] that defers transaction construction for create,
    /// update, and delete operations while reusing factory defaults.
    pub fn params(&self) -> ParamsBuilder<'_> {
        ParamsBuilder { factory: self }
    }
    /// Returns a [`TransactionBuilder`] for constructing transactions without submitting
    /// them yet.
    pub fn create_transaction(&self) -> TransactionBuilder<'_> {
        TransactionBuilder { factory: self }
    }
    /// Returns a [`TransactionSender`] that sends transactions immediately and surfaces
    /// their results.
    pub fn send(&self) -> TransactionSender<'_> {
        TransactionSender { factory: self }
    }

    /// Imports compiled source maps so subsequent calls can surface logic errors with
    /// meaningful context.
    pub fn import_source_maps(&self, source_maps: AppSourceMaps) {
        *self.approval_source_map.lock().unwrap() = source_maps.approval_source_map;
        *self.clear_source_map.lock().unwrap() = source_maps.clear_source_map;
    }

    /// Exports the cached source maps, returning an error if they have not been loaded yet.
    pub fn export_source_maps(&self) -> Result<AppSourceMaps, AppFactoryError> {
        let approval = self
            .approval_source_map
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| AppFactoryError::ValidationError {
                message: "Approval source map not loaded".to_string(),
            })?;
        let clear = self
            .clear_source_map
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| AppFactoryError::ValidationError {
                message: "Clear source map not loaded".to_string(),
            })?;
        Ok(AppSourceMaps {
            approval_source_map: Some(approval),
            clear_source_map: Some(clear),
        })
    }

    /// Creates a new [`AppClient`] configured for the provided application ID, with
    /// optional overrides for name, sender, signer, and source maps.
    pub fn get_app_client_by_id(
        &self,
        app_id: u64,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
    ) -> AppClient {
        let resolved_source_maps = source_maps.or_else(|| self.current_source_maps());
        AppClient::new(AppClientParams {
            app_id,
            app_spec: self.app_spec.clone(),
            algorand: self.algorand.clone(),
            app_name: Some(app_name.unwrap_or_else(|| self.app_name.clone())),
            default_sender: default_sender.or_else(|| self.default_sender.clone()),
            default_signer: default_signer.or_else(|| self.default_signer.clone()),
            source_maps: resolved_source_maps,
            transaction_composer_config: self.transaction_composer_config.clone(),
        })
    }

    /// Resolves an application by creator address and name using AlgoKit deployment
    /// semantics and returns a configured [`AppClient`]. Optional overrides control the
    /// resolved name, sender, signer, and whether the lookup cache is bypassed.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the application cannot be resolved or the
    /// resulting client cannot be created.
    pub async fn get_app_client_by_creator_and_name(
        &self,
        creator_address: &str,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        ignore_cache: Option<bool>,
    ) -> Result<AppClient, AppFactoryError> {
        let resolved_app_name = app_name.unwrap_or_else(|| self.app_name.clone());
        let resolved_sender = default_sender.or_else(|| self.default_sender.clone());
        let resolved_signer = default_signer.or_else(|| self.default_signer.clone());

        let client = AppClient::from_creator_and_name(
            creator_address,
            &resolved_app_name,
            self.app_spec.clone(),
            self.algorand.clone(),
            resolved_sender,
            resolved_signer,
            self.current_source_maps(),
            ignore_cache,
            self.transaction_composer_config.clone(),
        )
        .await
        .map_err(|e| AppFactoryError::AppClientError { source: e })?;

        Ok(client)
    }

    pub(crate) fn get_sender_address(
        &self,
        sender: &Option<String>,
    ) -> Result<algokit_transact::Address, String> {
        let sender_str = sender
            .as_ref()
            .or(self.default_sender.as_ref())
            .ok_or_else(|| {
                format!(
                    "No sender provided and no default sender configured for app {}",
                    self.app_name
                )
            })?;
        algokit_transact::Address::from_str(sender_str)
            .map_err(|e| format!("Invalid sender address: {}", e))
    }

    pub(crate) fn update_source_maps(
        &self,
        approval: Option<serde_json::Value>,
        clear: Option<serde_json::Value>,
    ) {
        *self.approval_source_map.lock().unwrap() = approval;
        *self.clear_source_map.lock().unwrap() = clear;
    }

    pub(crate) fn current_source_maps(&self) -> Option<AppSourceMaps> {
        let approval = self.approval_source_map.lock().unwrap().clone();
        let clear = self.clear_source_map.lock().unwrap().clone();

        if approval.is_none() && clear.is_none() {
            None
        } else {
            Some(AppSourceMaps {
                approval_source_map: approval,
                clear_source_map: clear,
            })
        }
    }

    pub(crate) fn detect_deploy_time_control_flag(
        &self,
        template_name: &str,
        on_complete: CallOnApplicationComplete,
    ) -> Option<bool> {
        let source = self.app_spec().source.as_ref()?;
        let approval = source.get_decoded_approval().ok()?;
        if !approval.contains(template_name) {
            return None;
        }

        let bare_allows = self
            .app_spec()
            .bare_actions
            .call
            .iter()
            .any(|action| *action == on_complete);
        let method_allows = self.app_spec().methods.iter().any(|method| {
            method
                .actions
                .call
                .iter()
                .any(|action| *action == on_complete)
        });

        Some(bare_allows || method_allows)
    }

    /// Transform a transaction error using AppClient logic error exposure for factory flows.
    pub(crate) fn handle_transaction_error(
        &self,
        error: TransactionSenderError,
        is_clear_state_program: bool,
    ) -> AppFactoryError {
        let error_str = error.to_string();

        if !(error_str.contains("logic eval error") || error_str.contains("logic error")) {
            return AppFactoryError::TransactionSenderError { source: error };
        }

        let source_maps = self.current_source_maps();
        let context = LogicErrorContext {
            app_id: 0,
            app_spec: &self.app_spec,
            algorand: self.algorand.as_ref(),
            source_maps: source_maps.as_ref(),
        };

        let logic_error = context.expose_logic_error(&error_str, is_clear_state_program);
        AppFactoryError::LogicError {
            message: logic_error.message.clone(),
            logic: Box::new(logic_error),
        }
    }

    /// Idempotently deploys (create, update, or replace) an application using
    /// `AppDeployer` semantics.
    ///
    /// The factory applies deploy-time template substitutions and reuses default
    /// sender/signer settings while coordinating create, update, and optional delete
    /// transactions.
    ///
    /// # Notes
    /// * Inspect `operation_performed` on the returned [`AppFactoryDeployResult`] to
    ///   understand which operation was executed and to access related metadata.
    /// * When `on_schema_break` is `OnSchemaBreak::Replace`, a breaking schema change
    ///   deletes and recreates the application.
    /// * When `on_update` is `OnUpdate::Replace`, differing TEAL sources trigger a
    ///   delete-and-recreate cycle.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if parameter construction fails, compilation fails,
    /// or the deployment encounters an error on chain.
    pub async fn deploy(
        &self,
        args: DeployArgs,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, AppDeployResult), AppFactoryError> {
        let compilation_params = self.resolve_compilation_params(compilation_params);

        let create_deploy_params = match args.create_params {
            Some(cp) => CreateParams::AppCreateMethodCall(self.params().create(cp)?),
            None => CreateParams::AppCreateCall(self.params().bare().create(None)?),
        };

        let update_deploy_params = match args.update_params {
            Some(up) => UpdateParams::AppUpdateMethodCall(self.params().deploy_update(up)?),
            None => UpdateParams::AppUpdateCall(self.params().bare().deploy_update(None)?),
        };

        let delete_deploy_params = match args.delete_params {
            Some(dp) => DeleteParams::AppDeleteMethodCall(self.params().deploy_delete(dp)?),
            None => DeleteParams::AppDeleteCall(self.params().bare().deploy_delete(None)?),
        };

        let metadata = AppDeployMetadata {
            name: args.app_name.unwrap_or_else(|| self.app_name.clone()),
            version: self.version.clone(),
            updatable: compilation_params.updatable,
            deletable: compilation_params.deletable,
        };

        let deploy_params = AppDeployParams {
            metadata,
            deploy_time_params: compilation_params.deploy_time_params,
            on_schema_break: args.on_schema_break,
            on_update: args.on_update,
            create_params: create_deploy_params,
            update_params: update_deploy_params,
            delete_params: delete_deploy_params,
            existing_deployments: args.existing_deployments,
            ignore_cache: args.ignore_cache,
            send_params: args.send_params.unwrap_or_default(),
        };

        let mut app_deployer = self.algorand.as_ref().app_deployer();

        let deploy_result = app_deployer
            .deploy(deploy_params)
            .await
            .map_err(|e| AppFactoryError::AppDeployerError { source: e })?;

        let app_id = match &deploy_result {
            AppDeployResult::Create { app, .. }
            | AppDeployResult::Update { app, .. }
            | AppDeployResult::Replace { app, .. }
            | AppDeployResult::Nothing { app } => app.app_id,
        };

        let app_client = self.get_app_client_by_id(app_id, None, None, None, None);

        // Extract and update source maps from the deploy result
        let (approval_source_map, clear_source_map) = match &deploy_result {
            AppDeployResult::Create { result, .. } => (
                result.compiled_programs.approval.source_map.clone(),
                result.compiled_programs.clear.source_map.clone(),
            ),
            AppDeployResult::Update { result, .. } => (
                result.compiled_programs.approval.source_map.clone(),
                result.compiled_programs.clear.source_map.clone(),
            ),
            AppDeployResult::Replace { result, .. } => (
                result.compiled_programs.approval.source_map.clone(),
                result.compiled_programs.clear.source_map.clone(),
            ),
            AppDeployResult::Nothing { .. } => (None, None),
        };

        self.update_source_maps(approval_source_map, clear_source_map);

        Ok((app_client, deploy_result))
    }
}
