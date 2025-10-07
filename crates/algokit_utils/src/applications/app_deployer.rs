use crate::clients::app_manager::{
    AppInformation, AppManager, AppManagerError, CompiledPrograms, CompiledTeal,
    DeploymentMetadata, TealTemplateParams,
};
use crate::transactions::{TransactionResult, TransactionSender, TransactionSenderError};
use crate::{
    AppCreateMethodCallParams, AppCreateParams, AppDeleteMethodCallParams, AppDeleteParams,
    AppMethodCallArg, AppUpdateMethodCallParams, AppUpdateParams, ComposerError, SendParams,
    create_transaction_params,
};
use algokit_transact::{Address, Byte32, OnApplicationComplete};
use base64::{Engine as _, engine::general_purpose};
use indexer_client::{IndexerClient, apis::Error as IndexerError};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const APP_DEPLOY_NOTE_PREFIX: &str = "ALGOKIT_DEPLOYER";

/// Enum for app program variants - either TEAL source code or compiled bytecode
#[derive(Debug, Clone, PartialEq)]
pub enum AppProgram {
    /// TEAL source code, which will be compiled with template parameters at deploy time
    Teal(String),
    /// Pre-compiled bytecode
    CompiledBytes(Vec<u8>),
}

impl Default for AppProgram {
    fn default() -> Self {
        Self::CompiledBytes(Vec::new())
    }
}

/// What action to perform if a schema break (storage schema or extra pages change) is detected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OnSchemaBreak {
    /// Fail the deployment (throw an error, default)
    Fail,
    /// Delete the old app and create a new one
    Replace,
    /// Deploy a new app and leave the old one as is
    Append,
}

impl Default for OnSchemaBreak {
    fn default() -> Self {
        Self::Fail
    }
}

/// What action to perform if a TEAL code update is detected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OnUpdate {
    /// Fail the deployment (throw an error, default)
    Fail,
    /// Update the app with the new TEAL code
    Update,
    /// Delete the old app and create a new one
    Replace,
    /// Deploy a new app and leave the old one as is
    Append,
}

impl Default for OnUpdate {
    fn default() -> Self {
        Self::Fail
    }
}

/// The deployment metadata for an application
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppDeployMetadata {
    /// The name of the application
    pub name: String,
    /// The version of the application
    pub version: String,
    /// Whether the application is updatable
    pub updatable: Option<bool>,
    /// Whether the application is deletable
    pub deletable: Option<bool>,
}

/// The metadata that can be collected about a deployed app
#[derive(Debug, Clone)]
pub struct AppMetadata {
    /// The id of the app
    pub app_id: u64,
    /// The Algorand address of the account associated with the app
    pub app_address: Address,
    /// The round the app was created
    pub created_round: u64,
    /// The last round that the app was updated
    pub updated_round: u64,
    /// The metadata when the app was created
    pub created_metadata: AppDeployMetadata,
    /// Whether or not the app is deleted
    pub deleted: bool,
    /// The deployment metadata
    pub name: String,
    pub version: String,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
}

/// A lookup of name -> Algorand app for a creator
#[derive(Debug, Clone)]
pub struct AppLookup {
    /// The address of the creator associated with this lookup
    pub creator: Address,
    /// A hash map of app name to app metadata
    pub apps: HashMap<String, AppMetadata>,
}

create_transaction_params! {
    /// Parameters for the create transaction with program variants
    #[derive(Default, Clone)]
    pub struct DeployAppCreateParams {
        pub on_complete: OnApplicationComplete,
        pub approval_program: AppProgram,
        pub clear_state_program: AppProgram,
        pub args: Option<Vec<Vec<u8>>>,
        pub account_references: Option<Vec<Address>>,
        pub app_references: Option<Vec<u64>>,
        pub asset_references: Option<Vec<u64>>,
        pub box_references: Option<Vec<algokit_transact::BoxReference>>,
        pub global_state_schema: Option<algokit_transact::StateSchema>,
        pub local_state_schema: Option<algokit_transact::StateSchema>,
        pub extra_program_pages: Option<u32>,
    }
}

create_transaction_params! {
    /// Parameters for the create method call with program variants
    #[derive(Default, Clone)]
    pub struct DeployAppCreateMethodCallParams {
        pub on_complete: OnApplicationComplete,
        pub approval_program: AppProgram,
        pub clear_state_program: AppProgram,
        pub method: algokit_abi::ABIMethod,
        pub args: Vec<AppMethodCallArg>,
        pub account_references: Option<Vec<Address>>,
        pub app_references: Option<Vec<u64>>,
        pub asset_references: Option<Vec<u64>>,
        pub box_references: Option<Vec<algokit_transact::BoxReference>>,
        pub global_state_schema: Option<algokit_transact::StateSchema>,
        pub local_state_schema: Option<algokit_transact::StateSchema>,
        pub extra_program_pages: Option<u32>,
    }
}

create_transaction_params! {
    #[derive(Default, Clone)]
    pub struct DeployAppUpdateParams {
        pub args: Option<Vec<Vec<u8>>>,
        pub account_references: Option<Vec<Address>>,
        pub app_references: Option<Vec<u64>>,
        pub asset_references: Option<Vec<u64>>,
        pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    }
}

create_transaction_params! {
    /// Parameters for the update method call
    #[derive(Default, Clone)]
    pub struct DeployAppUpdateMethodCallParams {
        pub method: algokit_abi::ABIMethod,
        pub args: Vec<AppMethodCallArg>,
        pub account_references: Option<Vec<Address>>,
        pub app_references: Option<Vec<u64>>,
        pub asset_references: Option<Vec<u64>>,
        pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    }
}

create_transaction_params! {
    /// Parameters for the delete transaction
    #[derive(Clone, Default)]
    pub struct DeployAppDeleteParams {
        pub args: Option<Vec<Vec<u8>>>,
        pub account_references: Option<Vec<Address>>,
        pub app_references: Option<Vec<u64>>,
        pub asset_references: Option<Vec<u64>>,
        pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    }
}

create_transaction_params! {
    /// Parameters for the delete method call
    #[derive(Default, Clone)]
    pub struct DeployAppDeleteMethodCallParams {
        pub method: algokit_abi::ABIMethod,
        pub args: Vec<AppMethodCallArg>,
        pub account_references: Option<Vec<Address>>,
        pub app_references: Option<Vec<u64>>,
        pub asset_references: Option<Vec<u64>>,
        pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum CreateParams {
    AppCreateCall(DeployAppCreateParams),
    AppCreateMethodCall(DeployAppCreateMethodCallParams),
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum UpdateParams {
    AppUpdateCall(DeployAppUpdateParams),
    AppUpdateMethodCall(DeployAppUpdateMethodCallParams),
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum DeleteParams {
    AppDeleteCall(DeployAppDeleteParams),
    AppDeleteMethodCall(DeployAppDeleteMethodCallParams),
}

/// The parameters to idempotently deploy an app
#[derive(Debug, Clone)]
pub struct AppDeployParams {
    /// The deployment metadata
    pub metadata: AppDeployMetadata,
    /// Any deploy-time parameters to replace in the TEAL code before compiling
    pub deploy_time_params: Option<TealTemplateParams>,
    /// What action to perform if a schema break is detected
    pub on_schema_break: Option<OnSchemaBreak>,
    /// What action to perform if a TEAL code update is detected
    pub on_update: Option<OnUpdate>,
    /// Create transaction parameters to use if a create needs to be issued as part of deployment
    pub create_params: CreateParams,
    /// Update transaction parameters to use if an update needs to be issued as part of deployment
    pub update_params: UpdateParams,
    /// Delete transaction parameters to use if a delete needs to be issued as part of deployment
    pub delete_params: DeleteParams,
    /// Optional cached value of the existing apps for the given creator
    pub existing_deployments: Option<AppLookup>,
    /// Whether or not to ignore the app metadata cache and force a lookup
    pub ignore_cache: Option<bool>,
    /// Send transaction parameters
    pub send_params: SendParams,
}

/// The result of an app deployment operation
#[derive(Debug)]
pub enum AppDeployResult {
    /// Application was created
    Create {
        app: AppMetadata,
        /// The result of create transaction
        create_result: TransactionResult,
        /// All transaction results
        group_results: Vec<TransactionResult>,
        /// The group ID for the transaction group (if any)
        group: Option<Byte32>,
        /// The compiled approval and clear programs
        compiled_programs: CompiledPrograms,
    },
    /// Application was updated
    Update {
        app: AppMetadata,
        /// The result of the update transaction
        update_result: TransactionResult,
        /// All transaction results
        group_results: Vec<TransactionResult>,
        /// The group ID for the transaction group (if any)
        group: Option<Byte32>,
        /// The compiled approval and clear programs
        compiled_programs: CompiledPrograms,
    },
    /// Application was replaced (deleted and recreated)
    Replace {
        app: AppMetadata,
        /// The result of the delete transaction
        delete_result: TransactionResult,
        /// The result of the create transaction
        create_result: TransactionResult,
        /// All transaction results
        group_results: Vec<TransactionResult>,
        /// The group ID for the transaction group (if any)
        group: Option<Byte32>,
        /// The compiled approval and clear programs
        compiled_programs: CompiledPrograms,
    },
    /// No operation was performed
    Nothing { app: AppMetadata },
}

/// Errors that can occur during app deployment
#[derive(Debug, Snafu)]
pub enum AppDeployError {
    #[snafu(display("Composer error: {source}"))]
    ComposerError { source: ComposerError },
    #[snafu(display("Indexer client error: {source}"))]
    IndexerError { source: IndexerError },
    #[snafu(display("App manager error: {source}"))]
    AppManagerError { source: AppManagerError },
    #[snafu(display("Transaction sender error: {source}"))]
    TransactionSenderError { source: TransactionSenderError },
    #[snafu(display("Deployment failed: {message}"))]
    DeploymentFailed { message: String },
    #[snafu(display("Deployment lookup failed: {message}"))]
    DeploymentLookupFailed { message: String },
}

/// Allows management of deployment and deployment metadata of applications.
#[derive(Clone)]
pub struct AppDeployer {
    indexer_client: Option<Arc<IndexerClient>>,
    app_manager: AppManager,
    transaction_sender: TransactionSender,
    app_lookups: Arc<Mutex<HashMap<String, AppLookup>>>,
}

impl AppDeployer {
    /// Create a new AppDeployer
    ///
    /// # Arguments
    /// * `app_manager` - An `AppManager` instance
    /// * `transaction_sender` - A `TransactionSender` instance
    /// * `indexer_client` - An optional `IndexerClient` for app metadata lookup
    pub fn new(
        app_manager: AppManager,
        transaction_sender: TransactionSender,
        indexer_client: Option<Arc<IndexerClient>>,
    ) -> Self {
        Self {
            indexer_client,
            app_manager,
            transaction_sender,
            app_lookups: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn deploy(
        &mut self,
        deployment: AppDeployParams,
    ) -> Result<AppDeployResult, AppDeployError> {
        let AppDeployParams {
            metadata,
            deploy_time_params,
            on_schema_break,
            on_update,
            mut create_params,
            mut update_params,
            delete_params,
            existing_deployments,
            ignore_cache,
            send_params,
        } = deployment;

        // Build deployment note using ARC2 format
        let arc2_note = Self::build_deployment_note(&metadata)?;
        match &mut create_params {
            CreateParams::AppCreateCall(params) => {
                params.note = Some(arc2_note.clone());
            }
            CreateParams::AppCreateMethodCall(params) => {
                params.note = Some(arc2_note.clone());
            }
        }
        match &mut update_params {
            UpdateParams::AppUpdateCall(params) => {
                params.note = Some(arc2_note);
            }
            UpdateParams::AppUpdateMethodCall(params) => {
                params.note = Some(arc2_note);
            }
        }

        let sender = match &create_params {
            CreateParams::AppCreateCall(params) => &params.sender,
            CreateParams::AppCreateMethodCall(params) => &params.sender,
        };

        if let Some(ref existing_deployments) = existing_deployments {
            if existing_deployments.creator != *sender {
                return Err(AppDeployError::DeploymentFailed {
                    message: format!(
                        "Invalid existing deployments: received invalid existingDeployments value for creator {} when attempting to deploy for creator {}",
                        existing_deployments.creator, sender
                    ),
                });
            }
        }

        if existing_deployments.is_none() && self.indexer_client.is_none() {
            return Err(AppDeployError::DeploymentFailed {
                message: String::from(
                    "Either indexer client or existing deployments must be provided",
                ),
            });
        }
        // Compile TEAL code if needed and handle template replacement
        let compiled_programs = self
            .compile_app_programs(
                match &create_params {
                    CreateParams::AppCreateCall(params) => &params.approval_program,
                    CreateParams::AppCreateMethodCall(params) => &params.approval_program,
                },
                match &create_params {
                    CreateParams::AppCreateCall(params) => &params.clear_state_program,
                    CreateParams::AppCreateMethodCall(params) => &params.clear_state_program,
                },
                &metadata,
                deploy_time_params.as_ref(),
            )
            .await?;

        info!(
            "Idempotently deploying app \"{}\" from creator {} using {} bytes of approval program and {} bytes of clear state program",
            metadata.name,
            sender,
            compiled_programs.approval.compiled_base64_to_bytes.len(),
            compiled_programs.clear.compiled_base64_to_bytes.len()
        );

        // Get existing apps
        let app_lookup = match existing_deployments {
            Some(apps) => apps,
            None => self.get_creator_apps_by_name(sender, ignore_cache).await?,
        };
        let existing_app_metadata = app_lookup.apps.get(&metadata.name);

        // If app doesn't exist or is deleted, create it
        if existing_app_metadata.is_none() || existing_app_metadata.is_some_and(|app| app.deleted) {
            info!(
                "App {} not found in apps created by {}; deploying app with version {}.",
                metadata.name, sender, metadata.version
            );
            return self
                .create_app(&metadata, &create_params, compiled_programs, &send_params)
                .await;
        }

        let existing_app_metadata = existing_app_metadata.unwrap();
        info!(
            "Existing app {} found by creator {}, with app id {} and version {}.",
            metadata.name, sender, existing_app_metadata.app_id, existing_app_metadata.version
        );

        let existing_app = self
            .app_manager
            .get_by_id(existing_app_metadata.app_id)
            .await
            .map_err(|e| AppDeployError::AppManagerError { source: e })?;

        // Check for changes
        let is_update = self.is_program_different(
            &compiled_programs.approval.compiled_base64_to_bytes,
            &compiled_programs.clear.compiled_base64_to_bytes,
            &existing_app,
        )?;
        let is_schema_break = self.is_schema_break(
            &create_params,
            &existing_app,
            &compiled_programs.approval.compiled_base64_to_bytes,
            &compiled_programs.clear.compiled_base64_to_bytes,
        )?;

        if is_schema_break {
            self.handle_schema_break(
                on_schema_break.unwrap_or_default(),
                existing_app_metadata,
                &metadata,
                &create_params,
                compiled_programs,
                &delete_params,
                &send_params,
            )
            .await
        } else if is_update {
            self.handle_update(
                on_update.unwrap_or_default(),
                existing_app_metadata,
                &metadata,
                &create_params,
                &update_params,
                compiled_programs,
                &delete_params,
                &send_params,
            )
            .await
        } else {
            debug!("No detected changes in app, nothing to do.");
            Ok(AppDeployResult::Nothing {
                app: existing_app_metadata.clone(),
            })
        }
    }

    /// Get apps created by a specific creator address
    pub async fn get_creator_apps_by_name(
        &mut self,
        creator_address: &Address,
        ignore_cache: Option<bool>,
    ) -> Result<AppLookup, AppDeployError> {
        let creator_address_str = creator_address.to_string();
        let ignore_cache = ignore_cache.unwrap_or(false);

        if !ignore_cache {
            {
                let app_lookups = self.app_lookups.lock().unwrap();
                if let Some(cached_lookup) = app_lookups.get(&creator_address_str) {
                    return Ok(cached_lookup.clone());
                }
            }
        }

        let indexer =
            self.indexer_client
                .as_ref()
                .ok_or(AppDeployError::DeploymentLookupFailed {
                    message: String::from(
                        "No indexer client or existing deployments cache provided",
                    ),
                })?;

        // Query indexer for apps created by this address; localnet-only retry to allow catch-up
        let created_apps_response = indexer
            .lookup_account_created_applications(&creator_address_str, None, Some(true), None, None)
            .await
            .map_err(|e| AppDeployError::IndexerError { source: e })?;

        let mut app_lookup = HashMap::new();

        // Sort applications by created_at_round to match TypeScript behavior
        let mut sorted_apps = created_apps_response.applications;
        sorted_apps.sort_by(|a, b| {
            a.created_at_round
                .unwrap_or(0)
                .cmp(&b.created_at_round.unwrap_or(0))
        });

        for app in &sorted_apps {
            if let Some(created_at_round) = app.created_at_round {
                let app_id = app.id;
                // Search for ALL app transactions for this app to find both creation and latest update
                let transactions_response = indexer
                    .search_for_transactions(
                        None,
                        None,
                        Some(&general_purpose::STANDARD.encode(APP_DEPLOY_NOTE_PREFIX)),
                        Some(indexer_client::apis::parameter_enums::TxType::Appl),
                        None,
                        None,
                        None,
                        None,
                        Some(created_at_round),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(&creator_address_str),
                        Some(indexer_client::apis::parameter_enums::AddressRole::Sender),
                        None,
                        None,
                        Some(app_id),
                    )
                    .await
                    .map_err(|e| AppDeployError::IndexerError { source: e })?;

                let mut app_creation_transaction = None;
                let mut latest_app_update_transaction = None;

                // Filter transactions to find creation and latest update
                let mut app_transactions: Vec<_> = transactions_response.transactions;
                // Sort by confirmed round (desc) and intra-round offset (desc) to get latest first
                // In theory this is the order that indexer returns them, but this ensures it
                app_transactions.sort_by(|a, b| {
                    match b
                        .confirmed_round
                        .unwrap_or(0)
                        .cmp(&a.confirmed_round.unwrap_or(0))
                    {
                        std::cmp::Ordering::Equal => b
                            .intra_round_offset
                            .unwrap_or(0)
                            .cmp(&a.intra_round_offset.unwrap_or(0)),
                        other => other,
                    }
                });

                // Find creation transaction and latest update transaction
                for transaction in &app_transactions {
                    if transaction.sender != creator_address_str {
                        continue; // Skip transactions not from the creator
                    }

                    if let Some(app_transaction) = &transaction.application_transaction {
                        if app_transaction.application_id == 0 {
                            // App creation transaction
                            app_creation_transaction = Some(transaction);
                        } else if latest_app_update_transaction.is_none() {
                            // Latest app update transaction (first non-creation we encounter due to sorting)
                            latest_app_update_transaction = Some(transaction);
                        }
                    }
                }

                if let Some(creation_txn) = app_creation_transaction {
                    if let Some(note) = &creation_txn.note {
                        let creation_note = Self::parse_deploy_note(note);
                        let update_note = latest_app_update_transaction
                            .and_then(|t| t.note.as_ref())
                            .and_then(|note| Self::parse_deploy_note(note.as_slice()));

                        if let Some(creation_metadata) = creation_note {
                            // Use update metadata if available, otherwise fall back to creation metadata
                            let current_metadata =
                                update_note.as_ref().unwrap_or(&creation_metadata);

                            let app_metadata = AppMetadata {
                                app_id,
                                app_address: Address::from_app_id(&app_id),
                                created_round: created_at_round,
                                updated_round: latest_app_update_transaction
                                    .and_then(|t| t.confirmed_round)
                                    .unwrap_or(
                                        creation_txn.confirmed_round.unwrap_or(created_at_round),
                                    ),
                                created_metadata: creation_metadata.clone(),
                                deleted: app.deleted.unwrap_or(false),
                                name: current_metadata.name.clone(),
                                version: current_metadata.version.clone(),
                                updatable: current_metadata.updatable,
                                deletable: current_metadata.deletable,
                            };
                            app_lookup.insert(creation_metadata.name, app_metadata);
                        }
                    }
                }
            }
        }

        let lookup = AppLookup {
            creator: creator_address.clone(),
            apps: app_lookup,
        };

        {
            let mut app_lookups = self.app_lookups.lock().unwrap();
            app_lookups.insert(creator_address_str, lookup.clone());
        }
        Ok(lookup)
    }

    fn build_deployment_note(metadata: &AppDeployMetadata) -> Result<Vec<u8>, AppDeployError> {
        let metadata_json =
            serde_json::to_string(&metadata).map_err(|e| AppDeployError::DeploymentFailed {
                message: format!("Failed to serialize metadata: {}", e),
            })?;
        Ok(format!("{}:j{}", APP_DEPLOY_NOTE_PREFIX, metadata_json).into_bytes())
    }

    /// Compile app programs, applying template replacement only for TEAL variant
    async fn compile_app_programs(
        &self,
        approval_program: &AppProgram,
        clear_state_program: &AppProgram,
        deployment_metadata: &AppDeployMetadata,
        deploy_time_params: Option<&TealTemplateParams>,
    ) -> Result<CompiledPrograms, AppDeployError> {
        let approval = match approval_program {
            AppProgram::Teal(code) => {
                let metadata = DeploymentMetadata {
                    updatable: deployment_metadata.updatable,
                    deletable: deployment_metadata.deletable,
                };
                let metadata_opt = if metadata.updatable.is_some() || metadata.deletable.is_some() {
                    Some(&metadata)
                } else {
                    None
                };
                self.app_manager
                    .compile_teal_template(code, deploy_time_params, metadata_opt)
                    .await
                    .map_err(|e| AppDeployError::AppManagerError { source: e })?
            }
            AppProgram::CompiledBytes(bytes) => CompiledTeal {
                teal: String::new(), // Not available for pre-compiled bytes
                compiled: base64::engine::general_purpose::STANDARD.encode(bytes),
                compiled_hash: String::new(), // Not available for pre-compiled bytes
                compiled_base64_to_bytes: bytes.clone(),
                source_map: None,
            },
        };

        let clear = match clear_state_program {
            AppProgram::Teal(code) => self
                .app_manager
                .compile_teal_template(code, deploy_time_params, None)
                .await
                .map_err(|e| AppDeployError::AppManagerError { source: e })?,
            AppProgram::CompiledBytes(bytes) => CompiledTeal {
                teal: String::new(), // Not available for pre-compiled bytes
                compiled: base64::engine::general_purpose::STANDARD.encode(bytes),
                compiled_hash: String::new(), // Not available for pre-compiled bytes
                compiled_base64_to_bytes: bytes.clone(),
                source_map: None,
            },
        };

        Ok(CompiledPrograms { approval, clear })
    }

    fn parse_deploy_note(note: &[u8]) -> Option<AppDeployMetadata> {
        if let Ok(utf8_note) = std::str::from_utf8(note) {
            if utf8_note.starts_with(&format!("{}:j", APP_DEPLOY_NOTE_PREFIX)) {
                let json_part = &utf8_note[APP_DEPLOY_NOTE_PREFIX.len() + 2..];
                return serde_json::from_str::<AppDeployMetadata>(json_part).ok();
            }
        }
        None
    }

    fn is_program_different(
        &self,
        approval_program: &[u8],
        clear_state_program: &[u8],
        existing_app: &AppInformation,
    ) -> Result<bool, AppDeployError> {
        let existing_approval_program = &existing_app.approval_program;
        let existing_clear_state_program = &existing_app.clear_state_program;

        Ok(approval_program != existing_approval_program
            || clear_state_program != existing_clear_state_program)
    }

    fn is_schema_break(
        &self,
        create_params: &CreateParams,
        existing_app: &AppInformation,
        approval_program: &[u8],
        clear_state_program: &[u8],
    ) -> Result<bool, AppDeployError> {
        let (new_global_schema, new_local_schema) = match create_params {
            CreateParams::AppCreateCall(params) => (
                params.global_state_schema.as_ref(),
                params.local_state_schema.as_ref(),
            ),
            CreateParams::AppCreateMethodCall(params) => (
                params.global_state_schema.as_ref(),
                params.local_state_schema.as_ref(),
            ),
        };

        let new_extra_pages =
            Self::calculate_extra_program_pages(approval_program, clear_state_program);
        let global_ints_break =
            new_global_schema.is_some_and(|schema| schema.num_uints > existing_app.global_ints);
        let global_bytes_break = new_global_schema
            .is_some_and(|schema| schema.num_byte_slices > existing_app.global_byte_slices);
        let local_ints_break =
            new_local_schema.is_some_and(|schema| schema.num_uints > existing_app.local_ints);
        let local_bytes_break = new_local_schema
            .is_some_and(|schema| schema.num_byte_slices > existing_app.local_byte_slices);
        let extra_pages_break = new_extra_pages > existing_app.extra_program_pages.unwrap_or(0);

        Ok(global_ints_break
            || global_bytes_break
            || local_ints_break
            || local_bytes_break
            || extra_pages_break)
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_schema_break(
        &mut self,
        on_schema_break: OnSchemaBreak,
        existing_app_metadata: &AppMetadata,
        metadata: &AppDeployMetadata,
        create_params: &CreateParams,
        compiled_programs: CompiledPrograms,
        delete_params: &DeleteParams,
        send_params: &SendParams,
    ) -> Result<AppDeployResult, AppDeployError> {
        warn!(
            "Detected a breaking app schema change in app {}",
            existing_app_metadata.app_id
        );

        match on_schema_break {
            OnSchemaBreak::Fail => Err(AppDeployError::DeploymentFailed {
                message: String::from(
                    "Executing the fail on schema break strategy, stopping deployment. If you want to try deleting and recreating the app then re-run using the replace on schema break strategy",
                ),
            }),
            OnSchemaBreak::Append => {
                info!(
                    "Executing the append on schema break strategy, will attempt to create a new app"
                );
                self.create_app(metadata, create_params, compiled_programs, send_params)
                    .await
            }
            OnSchemaBreak::Replace => {
                if existing_app_metadata.deletable.unwrap_or(false) {
                    info!(
                        "Executing the replace on schema break strategy on deletable app, will attempt to create new app and delete old app"
                    );
                } else {
                    info!(
                        "Executing the replace on schema break strategy on non deletable app, will attempt to delete app, delete will most likely fail"
                    );
                }
                self.replace_app(
                    existing_app_metadata,
                    metadata,
                    create_params,
                    delete_params,
                    compiled_programs,
                    send_params,
                )
                .await
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update(
        &mut self,
        on_update: OnUpdate,
        existing_app_metadata: &AppMetadata,
        metadata: &AppDeployMetadata,
        create_params: &CreateParams,
        update_params: &UpdateParams,
        compiled_programs: CompiledPrograms,
        delete_params: &DeleteParams,
        send_params: &SendParams,
    ) -> Result<AppDeployResult, AppDeployError> {
        info!(
            "Detected an update in app {} for creator",
            existing_app_metadata.app_id
        );

        match on_update {
            OnUpdate::Fail => Err(AppDeployError::DeploymentFailed {
                message: String::from(
                    "Executing the fail on update strategy, stopping deployment. Try a different on update strategy to not fail",
                ),
            }),
            OnUpdate::Append => {
                info!("Executing the append on update strategy, will attempt to create a new app");
                self.create_app(metadata, create_params, compiled_programs, send_params)
                    .await
            }
            OnUpdate::Update => {
                if existing_app_metadata.updatable.unwrap_or(false) {
                    info!("Executing the update on update strategy on updatable app");
                } else {
                    warn!(
                        "Executing the update on update strategy on non updatable app, will attempt to update app, update will most likely fail"
                    );
                }
                self.update_app(
                    existing_app_metadata,
                    metadata,
                    update_params,
                    compiled_programs,
                    send_params,
                )
                .await
            }
            OnUpdate::Replace => {
                if existing_app_metadata.deletable.unwrap_or(false) {
                    warn!(
                        "Executing the replace on update strategy on deletable app, creating new app and deleting old app..."
                    );
                } else {
                    warn!(
                        "Executing the replace on update strategy on non deletable app, will attempt to create new app and delete old app, delete will most likely fail"
                    );
                }
                self.replace_app(
                    existing_app_metadata,
                    metadata,
                    create_params,
                    delete_params,
                    compiled_programs,
                    send_params,
                )
                .await
            }
        }
    }

    async fn create_app(
        &mut self,
        metadata: &AppDeployMetadata,
        create_params: &CreateParams,
        compiled_programs: CompiledPrograms,
        send_params: &SendParams,
    ) -> Result<AppDeployResult, AppDeployError> {
        let mut composer = self.transaction_sender.new_composer(None);

        match create_params {
            CreateParams::AppCreateCall(params) => {
                let computed_extra_pages = Self::calculate_extra_program_pages(
                    &compiled_programs.approval.compiled_base64_to_bytes,
                    &compiled_programs.clear.compiled_base64_to_bytes,
                );
                let app_create_params = AppCreateParams {
                    sender: params.sender.clone(),
                    signer: params.signer.clone(),
                    rekey_to: params.rekey_to.clone(),
                    note: params.note.clone(),
                    lease: params.lease,
                    static_fee: params.static_fee,
                    extra_fee: params.extra_fee,
                    max_fee: params.max_fee,
                    validity_window: params.validity_window,
                    first_valid_round: params.first_valid_round,
                    last_valid_round: params.last_valid_round,
                    on_complete: params.on_complete,
                    approval_program: compiled_programs.approval.compiled_base64_to_bytes.clone(),
                    clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.clone(),
                    global_state_schema: params.global_state_schema.clone(),
                    local_state_schema: params.local_state_schema.clone(),
                    extra_program_pages: params.extra_program_pages.or(Some(computed_extra_pages)),
                    args: params.args.clone(),
                    account_references: params.account_references.clone(),
                    app_references: params.app_references.clone(),
                    asset_references: params.asset_references.clone(),
                    box_references: params.box_references.clone(),
                };
                composer
                    .add_app_create(app_create_params)
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
            CreateParams::AppCreateMethodCall(params) => {
                let computed_extra_pages = Self::calculate_extra_program_pages(
                    &compiled_programs.approval.compiled_base64_to_bytes,
                    &compiled_programs.clear.compiled_base64_to_bytes,
                );
                let app_create_method_params = AppCreateMethodCallParams {
                    sender: params.sender.clone(),
                    signer: params.signer.clone(),
                    rekey_to: params.rekey_to.clone(),
                    note: params.note.clone(),
                    lease: params.lease,
                    static_fee: params.static_fee,
                    extra_fee: params.extra_fee,
                    max_fee: params.max_fee,
                    validity_window: params.validity_window,
                    first_valid_round: params.first_valid_round,
                    last_valid_round: params.last_valid_round,
                    on_complete: params.on_complete,
                    approval_program: compiled_programs.approval.compiled_base64_to_bytes.clone(),
                    clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.clone(),
                    global_state_schema: params.global_state_schema.clone(),
                    local_state_schema: params.local_state_schema.clone(),
                    extra_program_pages: params.extra_program_pages.or(Some(computed_extra_pages)),
                    method: params.method.clone(),
                    args: params.args.clone(),
                    account_references: params.account_references.clone(),
                    app_references: params.app_references.clone(),
                    asset_references: params.asset_references.clone(),
                    box_references: params.box_references.clone(),
                };
                composer
                    .add_app_create_method_call(app_create_method_params)
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
        };

        let composer_result = composer
            .send(Some(send_params.clone()))
            .await
            .map_err(|e| AppDeployError::ComposerError { source: e })?;

        let create_transaction_index = composer_result.results.len() - 1;

        // Extract results from the create transaction
        let create_result = composer_result.results[create_transaction_index].clone();

        let confirmation = create_result.confirmation.clone();
        let app_id = confirmation
            .app_id
            .ok_or_else(|| AppDeployError::DeploymentFailed {
                message: "App creation confirmation missing application-index".to_string(),
            })?;

        let app_address = Address::from_app_id(&app_id);
        let confirmed_round =
            confirmation
                .confirmed_round
                .ok_or_else(|| AppDeployError::DeploymentFailed {
                    message: "App creation confirmation missing confirmed-round".to_string(),
                })?;

        let app_metadata = AppMetadata {
            app_id,
            app_address,
            created_round: confirmed_round,
            updated_round: confirmed_round,
            created_metadata: metadata.clone(),
            deleted: false,
            name: metadata.name.clone(),
            version: metadata.version.clone(),
            updatable: metadata.updatable,
            deletable: metadata.deletable,
        };

        let sender = match create_params {
            CreateParams::AppCreateCall(params) => &params.sender,
            CreateParams::AppCreateMethodCall(params) => &params.sender,
        };

        self.update_app_lookup(sender, &app_metadata);

        Ok(AppDeployResult::Create {
            app: app_metadata,
            create_result,
            group_results: composer_result.results,
            group: composer_result.group,
            compiled_programs,
        })
    }

    async fn update_app(
        &mut self,
        existing_app_metadata: &AppMetadata,
        metadata: &AppDeployMetadata,
        update_params: &UpdateParams,
        compiled_programs: CompiledPrograms,
        send_params: &SendParams,
    ) -> Result<AppDeployResult, AppDeployError> {
        info!(
            "Updating existing {} app to version {}.",
            metadata.name, metadata.version
        );
        let mut composer = self.transaction_sender.new_composer(None);

        match update_params {
            UpdateParams::AppUpdateCall(params) => {
                let app_update_params = AppUpdateParams {
                    sender: params.sender.clone(),
                    signer: params.signer.clone(),
                    rekey_to: params.rekey_to.clone(),
                    note: params.note.clone(),
                    lease: params.lease,
                    static_fee: params.static_fee,
                    extra_fee: params.extra_fee,
                    max_fee: params.max_fee,
                    validity_window: params.validity_window,
                    first_valid_round: params.first_valid_round,
                    last_valid_round: params.last_valid_round,
                    app_id: existing_app_metadata.app_id,
                    approval_program: compiled_programs.approval.compiled_base64_to_bytes.to_vec(),
                    clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.to_vec(),
                    args: params.args.clone(),
                    account_references: params.account_references.clone(),
                    app_references: params.app_references.clone(),
                    asset_references: params.asset_references.clone(),
                    box_references: params.box_references.clone(),
                };
                composer
                    .add_app_update(app_update_params)
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
            UpdateParams::AppUpdateMethodCall(params) => {
                let app_update_method_params = AppUpdateMethodCallParams {
                    sender: params.sender.clone(),
                    signer: params.signer.clone(),
                    rekey_to: params.rekey_to.clone(),
                    note: params.note.clone(),
                    lease: params.lease,
                    static_fee: params.static_fee,
                    extra_fee: params.extra_fee,
                    max_fee: params.max_fee,
                    validity_window: params.validity_window,
                    first_valid_round: params.first_valid_round,
                    last_valid_round: params.last_valid_round,
                    app_id: existing_app_metadata.app_id,
                    approval_program: compiled_programs.approval.compiled_base64_to_bytes.to_vec(),
                    clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.to_vec(),
                    method: params.method.clone(),
                    args: params.args.clone(),
                    account_references: params.account_references.clone(),
                    app_references: params.app_references.clone(),
                    asset_references: params.asset_references.clone(),
                    box_references: params.box_references.clone(),
                };
                composer
                    .add_app_update_method_call(app_update_method_params)
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
        };

        let composer_result = composer
            .send(Some(send_params.clone()))
            .await
            .map_err(|e| AppDeployError::ComposerError { source: e })?;

        let update_transaction_index = composer_result.results.len() - 1;

        // Extract results from the update transaction
        let update_result = composer_result.results[update_transaction_index].clone();

        let confirmed_round = update_result.confirmation.confirmed_round.ok_or_else(|| {
            AppDeployError::DeploymentFailed {
                message: "App update confirmation missing confirmed-round".to_string(),
            }
        })?;

        let app_metadata = AppMetadata {
            app_id: existing_app_metadata.app_id,
            app_address: existing_app_metadata.app_address.clone(),
            created_round: existing_app_metadata.created_round,
            updated_round: confirmed_round,
            created_metadata: existing_app_metadata.created_metadata.clone(),
            deleted: false,
            name: metadata.name.clone(),
            version: metadata.version.clone(),
            updatable: metadata.updatable,
            deletable: metadata.deletable,
        };

        let sender = match update_params {
            UpdateParams::AppUpdateCall(params) => &params.sender,
            UpdateParams::AppUpdateMethodCall(params) => &params.sender,
        };

        self.update_app_lookup(sender, &app_metadata);

        Ok(AppDeployResult::Update {
            app: app_metadata,
            update_result,
            group_results: composer_result.results,
            group: composer_result.group,
            compiled_programs,
        })
    }

    /// Updates the app lookup cache with the given app metadata
    fn update_app_lookup(&mut self, sender: &Address, app_metadata: &AppMetadata) {
        let sender_str = sender.to_string();

        {
            let mut app_lookups = self.app_lookups.lock().unwrap();
            match app_lookups.get_mut(&sender_str) {
                Some(lookup) => {
                    lookup
                        .apps
                        .insert(app_metadata.name.clone(), app_metadata.clone());
                }
                None => {
                    app_lookups.insert(
                        sender_str,
                        AppLookup {
                            creator: sender.clone(),
                            apps: HashMap::from([(
                                app_metadata.name.clone(),
                                app_metadata.clone(),
                            )]),
                        },
                    );
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn replace_app(
        &mut self,
        existing_app_metadata: &AppMetadata,
        metadata: &AppDeployMetadata,
        create_params: &CreateParams,
        delete_params: &DeleteParams,
        compiled_programs: CompiledPrograms,
        send_params: &SendParams,
    ) -> Result<AppDeployResult, AppDeployError> {
        info!(
            "Deploying a new {} app; deploying app with version {}.",
            metadata.name, metadata.version
        );
        warn!(
            "Deleting existing {} app with id {} from account.",
            metadata.name, existing_app_metadata.app_id
        );

        let mut composer = self.transaction_sender.new_composer(None);

        // Add create transaction and track its index
        match create_params {
            CreateParams::AppCreateCall(params) => {
                let computed_extra_pages = Self::calculate_extra_program_pages(
                    &compiled_programs.approval.compiled_base64_to_bytes,
                    &compiled_programs.clear.compiled_base64_to_bytes,
                );
                let app_create_params = AppCreateParams {
                    sender: params.sender.clone(),
                    signer: params.signer.clone(),
                    rekey_to: params.rekey_to.clone(),
                    note: params.note.clone(),
                    lease: params.lease,
                    static_fee: params.static_fee,
                    extra_fee: params.extra_fee,
                    max_fee: params.max_fee,
                    validity_window: params.validity_window,
                    first_valid_round: params.first_valid_round,
                    last_valid_round: params.last_valid_round,
                    on_complete: params.on_complete,
                    approval_program: compiled_programs.approval.compiled_base64_to_bytes.to_vec(),
                    clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.to_vec(),
                    global_state_schema: params.global_state_schema.clone(),
                    local_state_schema: params.local_state_schema.clone(),
                    extra_program_pages: params.extra_program_pages.or(Some(computed_extra_pages)),
                    args: params.args.clone(),
                    account_references: params.account_references.clone(),
                    app_references: params.app_references.clone(),
                    asset_references: params.asset_references.clone(),
                    box_references: params.box_references.clone(),
                };
                composer
                    .add_app_create(app_create_params)
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
            CreateParams::AppCreateMethodCall(params) => {
                let computed_extra_pages = Self::calculate_extra_program_pages(
                    &compiled_programs.approval.compiled_base64_to_bytes,
                    &compiled_programs.clear.compiled_base64_to_bytes,
                );
                let app_create_method_params = AppCreateMethodCallParams {
                    sender: params.sender.clone(),
                    signer: params.signer.clone(),
                    rekey_to: params.rekey_to.clone(),
                    note: params.note.clone(),
                    lease: params.lease,
                    static_fee: params.static_fee,
                    extra_fee: params.extra_fee,
                    max_fee: params.max_fee,
                    validity_window: params.validity_window,
                    first_valid_round: params.first_valid_round,
                    last_valid_round: params.last_valid_round,
                    on_complete: params.on_complete,
                    approval_program: compiled_programs.approval.compiled_base64_to_bytes.to_vec(),
                    clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.to_vec(),
                    global_state_schema: params.global_state_schema.clone(),
                    local_state_schema: params.local_state_schema.clone(),
                    extra_program_pages: params.extra_program_pages.or(Some(computed_extra_pages)),
                    method: params.method.clone(),
                    args: params.args.clone(),
                    account_references: params.account_references.clone(),
                    app_references: params.app_references.clone(),
                    asset_references: params.asset_references.clone(),
                    box_references: params.box_references.clone(),
                };
                composer
                    .add_app_create_method_call(app_create_method_params)
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
        };

        let create_transaction_index = composer.count() - 1;

        // Add delete transaction
        match delete_params {
            DeleteParams::AppDeleteCall(params) => {
                composer
                    .add_app_delete(AppDeleteParams {
                        sender: params.sender.clone(),
                        signer: params.signer.clone(),
                        rekey_to: params.rekey_to.clone(),
                        note: params.note.clone(),
                        lease: params.lease,
                        static_fee: params.static_fee,
                        extra_fee: params.extra_fee,
                        max_fee: params.max_fee,
                        validity_window: params.validity_window,
                        first_valid_round: params.first_valid_round,
                        last_valid_round: params.last_valid_round,
                        app_id: existing_app_metadata.app_id,
                        args: params.args.clone(),
                        account_references: params.account_references.clone(),
                        app_references: params.app_references.clone(),
                        asset_references: params.asset_references.clone(),
                        box_references: params.box_references.clone(),
                    })
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
            DeleteParams::AppDeleteMethodCall(params) => {
                composer
                    .add_app_delete_method_call(AppDeleteMethodCallParams {
                        sender: params.sender.clone(),
                        signer: params.signer.clone(),
                        rekey_to: params.rekey_to.clone(),
                        note: params.note.clone(),
                        lease: params.lease,
                        static_fee: params.static_fee,
                        extra_fee: params.extra_fee,
                        max_fee: params.max_fee,
                        validity_window: params.validity_window,
                        first_valid_round: params.first_valid_round,
                        last_valid_round: params.last_valid_round,
                        app_id: existing_app_metadata.app_id,
                        method: params.method.clone(),
                        args: params.args.clone(),
                        account_references: params.account_references.clone(),
                        app_references: params.app_references.clone(),
                        asset_references: params.asset_references.clone(),
                        box_references: params.box_references.clone(),
                    })
                    .map_err(|e| AppDeployError::ComposerError { source: e })?;
            }
        }

        let result = composer
            .send(Some(send_params.clone()))
            .await
            .map_err(|e| AppDeployError::ComposerError { source: e })?;

        // Extract create and delete results directly
        let delete_transaction_index = result.results.len() - 1;
        let create_result = result.results[create_transaction_index].clone();
        let delete_result = result.results[delete_transaction_index].clone();

        // Get create confirmation from the tracked index
        let create_confirmation = create_result.confirmation.clone();
        let app_id =
            create_confirmation
                .app_id
                .ok_or_else(|| AppDeployError::DeploymentFailed {
                    message: "App creation confirmation missing application-index".to_string(),
                })?;
        let confirmed_round = create_confirmation.confirmed_round.ok_or_else(|| {
            AppDeployError::DeploymentFailed {
                message: "App creation confirmation missing confirmed-round".to_string(),
            }
        })?;
        let app_address = Address::from_app_id(&app_id);

        let app_metadata = AppMetadata {
            app_id,
            app_address,
            created_round: confirmed_round,
            updated_round: confirmed_round,
            created_metadata: metadata.clone(),
            deleted: false,
            name: metadata.name.clone(),
            version: metadata.version.clone(),
            updatable: metadata.updatable,
            deletable: metadata.deletable,
        };

        let sender = match create_params {
            CreateParams::AppCreateCall(params) => &params.sender,
            CreateParams::AppCreateMethodCall(params) => &params.sender,
        };

        self.update_app_lookup(sender, &app_metadata);

        Ok(AppDeployResult::Replace {
            app: app_metadata,
            delete_result,
            create_result,
            group_results: result.results,
            group: result.group,
            compiled_programs,
        })
    }

    /// Calculate minimum number of extra program pages required to fit the programs.
    fn calculate_extra_program_pages(approval: &[u8], clear: &[u8]) -> u32 {
        let total = approval.len().saturating_add(clear.len());
        if total == 0 {
            return 0;
        }
        let page_size = algokit_transact::PROGRAM_PAGE_SIZE;
        let pages = ((total - 1) / page_size) as u32;
        std::cmp::min(pages, algokit_transact::MAX_EXTRA_PROGRAM_PAGES)
    }
}
