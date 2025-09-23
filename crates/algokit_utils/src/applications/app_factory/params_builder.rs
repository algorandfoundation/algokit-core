use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::{AppClientBareCallParams, AppClientMethodCallParams};
use crate::applications::app_deployer::{
    AppProgram, DeployAppCreateMethodCallParams, DeployAppCreateParams,
    DeployAppDeleteMethodCallParams, DeployAppDeleteParams, DeployAppUpdateMethodCallParams,
    DeployAppUpdateParams,
};
use crate::applications::app_factory::utils::merge_args_with_defaults;
use crate::applications::app_factory::{AppFactoryCreateMethodCallParams, AppFactoryCreateParams};
use algokit_transact::OnApplicationComplete;
use algokit_transact::StateSchema as TxStateSchema;
use std::str::FromStr;

use super::utils::resolve_signer;

/// Builds method-call deploy parameters using the factory's configuration.
pub struct ParamsBuilder<'a> {
    pub(crate) factory: &'a AppFactory,
}

/// Builds bare (non-ABI) deploy parameters backed by factory defaults.
pub struct BareParamsBuilder<'a> {
    pub(crate) factory: &'a AppFactory,
}

impl<'a> ParamsBuilder<'a> {
    /// Returns the bare parameter builder for constructing non-ABI transactions.
    pub fn bare(&self) -> BareParamsBuilder<'a> {
        BareParamsBuilder {
            factory: self.factory,
        }
    }

    /// Builds [`DeployAppCreateMethodCallParams`] using the supplied inputs and the
    /// factory's compiled programs.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the spec cannot be compiled, the method cannot be
    /// located, or the sender address is invalid.
    pub fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
    ) -> Result<DeployAppCreateMethodCallParams, AppFactoryError> {
        let (approval_teal, clear_teal) = self.factory.app_spec().decoded_teal().map_err(|e| {
            AppFactoryError::CompilationError {
                message: e.to_string(),
            }
        })?;
        let method = self
            .factory
            .app_spec()
            .find_abi_method(&params.method)
            .map_err(|e| AppFactoryError::ABIError { source: e })?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        // Merge user args with ARC-56 literal defaults for create-time ABI
        let merged_args = merge_args_with_defaults(self.factory, &params.method, &params.args)?;

        Ok(DeployAppCreateMethodCallParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            on_complete: params.on_complete.unwrap_or(OnApplicationComplete::NoOp),
            approval_program: AppProgram::Teal(approval_teal),
            clear_state_program: AppProgram::Teal(clear_teal),
            method,
            args: merged_args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: params
                .global_state_schema
                .or_else(|| Some(default_global_schema(self.factory))),
            local_state_schema: params
                .local_state_schema
                .or_else(|| Some(default_local_schema(self.factory))),
            extra_program_pages: params.extra_program_pages,
        })
    }

    /// Builds [`DeployAppUpdateMethodCallParams`] for an update call, merging default
    /// arguments defined in the ARC-56 contract.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the method cannot be resolved, default arguments
    /// cannot be merged, or the sender address is invalid.
    pub fn deploy_update(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<DeployAppUpdateMethodCallParams, AppFactoryError> {
        let method = self
            .factory
            .app_spec()
            .find_abi_method(&params.method)
            .map_err(|e| AppFactoryError::ABIError { source: e })?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        let merged_args =
            merge_args_with_defaults(self.factory, &params.method, &Some(params.args.clone()))?;

        Ok(DeployAppUpdateMethodCallParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            method,
            args: merged_args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }

    /// Builds [`DeployAppDeleteMethodCallParams`] for a delete call, merging default
    /// arguments defined in the ARC-56 contract.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the method cannot be resolved, default arguments
    /// cannot be merged, or the sender address is invalid.
    pub fn deploy_delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<DeployAppDeleteMethodCallParams, AppFactoryError> {
        let method = self
            .factory
            .app_spec()
            .find_abi_method(&params.method)
            .map_err(|e| AppFactoryError::ABIError { source: e })?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        let merged_args =
            merge_args_with_defaults(self.factory, &params.method, &Some(params.args.clone()))?;

        Ok(DeployAppDeleteMethodCallParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            method,
            args: merged_args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }
}

impl BareParamsBuilder<'_> {
    /// Builds [`DeployAppCreateParams`] using factory defaults and compiled programs.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the spec cannot be compiled or the sender address
    /// is invalid.
    pub fn create(
        &self,
        params: Option<AppFactoryCreateParams>,
    ) -> Result<DeployAppCreateParams, AppFactoryError> {
        let params = params.unwrap_or_default();
        let (approval_teal, clear_teal) = self.factory.app_spec().decoded_teal().map_err(|e| {
            AppFactoryError::CompilationError {
                message: e.to_string(),
            }
        })?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        Ok(DeployAppCreateParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            on_complete: params.on_complete.unwrap_or(OnApplicationComplete::NoOp),
            approval_program: AppProgram::Teal(approval_teal),
            clear_state_program: AppProgram::Teal(clear_teal),
            args: params.args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: params
                .global_state_schema
                .or_else(|| Some(default_global_schema(self.factory))),
            local_state_schema: params
                .local_state_schema
                .or_else(|| Some(default_local_schema(self.factory))),
            extra_program_pages: params.extra_program_pages,
        })
    }

    /// Builds [`DeployAppUpdateParams`] for a bare update transaction using factory
    /// defaults.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the sender address is invalid.
    pub fn deploy_update(
        &self,
        params: Option<AppClientBareCallParams>,
    ) -> Result<DeployAppUpdateParams, AppFactoryError> {
        let params = params.unwrap_or_default();
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        Ok(DeployAppUpdateParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            args: params.args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }

    /// Builds [`DeployAppDeleteParams`] for a bare delete transaction using factory
    /// defaults.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if the sender address is invalid.
    pub fn deploy_delete(
        &self,
        params: Option<AppClientBareCallParams>,
    ) -> Result<DeployAppDeleteParams, AppFactoryError> {
        let params = params.unwrap_or_default();
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        Ok(DeployAppDeleteParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            args: params.args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }
}

fn default_global_schema(factory: &AppFactory) -> TxStateSchema {
    let s = &factory.app_spec().state.schema.global_state;
    TxStateSchema {
        num_uints: s.ints,
        num_byte_slices: s.bytes,
    }
}

fn default_local_schema(factory: &AppFactory) -> TxStateSchema {
    let s = &factory.app_spec().state.schema.local_state;
    TxStateSchema {
        num_uints: s.ints,
        num_byte_slices: s.bytes,
    }
}
