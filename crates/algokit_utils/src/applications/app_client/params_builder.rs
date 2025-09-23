use super::AppClient;
use super::types::{
    AppClientBareCallParams, AppClientMethodCallParams, CompilationParams, FundAppAccountParams,
};
use crate::AppClientError;
use crate::applications::app_client::utils::parse_account_refs_to_addresses;
use crate::clients::app_manager::AppState;
use crate::clients::app_manager::CompiledPrograms;
use crate::transactions::{
    AppCallMethodCallParams, AppCallParams, AppDeleteMethodCallParams, AppDeleteParams,
    AppMethodCallArg, AppUpdateMethodCallParams, AppUpdateParams, PaymentParams,
};
use algokit_abi::abi_method::ABIDefaultValue;
use algokit_abi::{ABIMethod, ABIMethodArgType, ABIType, ABIValue, DefaultValueSource};
use algokit_transact::{Address, OnApplicationComplete};
use base64::Engine;
use std::str::FromStr;

enum StateSource<'app_client> {
    Global,
    Local(&'app_client str),
}

pub struct ParamsBuilder<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

pub struct BareParamsBuilder<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

impl<'app_client> ParamsBuilder<'app_client> {
    /// Get the bare call params builder.
    pub fn bare(&self) -> BareParamsBuilder<'app_client> {
        BareParamsBuilder {
            client: self.client,
        }
    }

    /// Build parameters for an ABI method call with the specified on-complete action.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, on_complete.unwrap_or(OnApplicationComplete::NoOp))
            .await
    }

    /// Build parameters for an ABI method call with OptIn on-complete action.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, OnApplicationComplete::OptIn)
            .await
    }

    /// Build parameters for an ABI method call with CloseOut on-complete action.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, OnApplicationComplete::CloseOut)
            .await
    }

    /// Build parameters for an ABI method call with ClearState on-complete action.
    pub async fn clear_state(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, OnApplicationComplete::ClearState)
            .await
    }

    /// Build parameters for an ABI method call with Delete on-complete action.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppDeleteMethodCallParams, AppClientError> {
        let abi_method = self.get_abi_method(&params.method)?;
        let sender = self.client.get_sender_address(&params.sender)?.as_str();
        let resolved_args = self
            .resolve_args(&abi_method, &params.args, &sender)
            .await?;

        Ok(AppDeleteMethodCallParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: self.client.app_id,
            method: abi_method,
            args: resolved_args,
            account_references: parse_account_refs_to_addresses(&params.account_references)?,
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
        })
    }

    /// Build parameters for updating the application using an ABI method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppUpdateMethodCallParams, CompiledPrograms), AppClientError> {
        // Compile programs (and populate AppManager cache/source maps)
        let compilation_params = compilation_params.unwrap_or_default();
        let compiled_programs = self.client.compile(&compilation_params).await?;

        let abi_method = self.get_abi_method(&params.method)?;
        let sender = self.client.get_sender_address(&params.sender)?.as_str();
        let resolved_args = self
            .resolve_args(&abi_method, &params.args, &sender)
            .await?;

        let update_params = AppUpdateMethodCallParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: self.client.app_id,
            method: abi_method,
            args: resolved_args,
            account_references: parse_account_refs_to_addresses(&params.account_references)?,
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            approval_program: compiled_programs.approval.compiled_base64_to_bytes.clone(),
            clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.clone(),
        };

        Ok((update_params, compiled_programs))
    }

    /// Build parameters for funding the application's account.
    pub fn fund_app_account(
        &self,
        params: &FundAppAccountParams,
    ) -> Result<PaymentParams, AppClientError> {
        let sender = self.client.get_sender_address(&params.sender)?;
        let receiver = self.client.app_address();
        let rekey_to = get_optional_address(&params.rekey_to)?;

        Ok(PaymentParams {
            sender,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            receiver,
            amount: params.amount,
        })
    }

    async fn get_method_call_params(
        &self,
        params: &AppClientMethodCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        let abi_method = self.get_abi_method(&params.method)?;
        let sender = self.client.get_sender_address(&params.sender)?.as_str();
        let resolved_args = self
            .resolve_args(&abi_method, &params.args, &sender)
            .await?;

        Ok(AppCallMethodCallParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: self.client.app_id,
            method: abi_method,
            args: resolved_args,
            account_references: parse_account_refs_to_addresses(&params.account_references)?,
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            on_complete,
        })
    }

    fn get_abi_method(&self, method_name_or_signature: &str) -> Result<ABIMethod, AppClientError> {
        self.client
            .app_spec
            .find_abi_method(method_name_or_signature)
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    async fn resolve_args(
        &self,
        method: &ABIMethod,
        provided: &Vec<AppMethodCallArg>,
        sender: &str,
    ) -> Result<Vec<AppMethodCallArg>, AppClientError> {
        let mut resolved: Vec<AppMethodCallArg> = Vec::with_capacity(method.args.len());

        if method.args.len() != provided.len() {
            return Err(AppClientError::ValidationError {
                message: format!(
                    "The number of provided arguments is {} while the method expects {} arguments",
                    provided.len(),
                    method.args.len()
                ),
            });
        }

        for (index, (method_arg, provided_arg)) in method.args.iter().zip(provided).enumerate() {
            let method_arg_name = method_arg
                .name
                .clone()
                .unwrap_or_else(|| format!("arg{}", index + 1));
            match (&method_arg.arg_type, provided_arg) {
                (ABIMethodArgType::Value(value_type), AppMethodCallArg::DefaultValue) => {
                    let default_value = method_arg.default_value.as_ref().ok_or_else(|| {
                        AppClientError::ParamsBuilderError {
                            message: format!(
                                "No default value defined for argument {} in call to method {}",
                                method_arg_name, method.name
                            ),
                        }
                    })?;

                    let value = self
                        .resolve_default_value(default_value, value_type, sender)
                        .await
                        .map_err(|e| AppClientError::ParamsBuilderError {
                            message: format!(
                                "Failed to resolve default value for arg {}: {:?}",
                                method_arg_name, e
                            ),
                        })?;
                    resolved.push(AppMethodCallArg::ABIValue(value));
                }
                (_, AppMethodCallArg::DefaultValue) => {
                    return Err(AppClientError::ParamsBuilderError {
                        message: format!(
                            "Default value is not supported by argument {} in call to method {}",
                            method_arg_name, method.name
                        ),
                    });
                }
                // Intentionally defer type compatibility and structural validation to ABI
                // encoding/composer (consistent with TS/Py). Here we only enforce arg count and
                // default value handling; encoding will surface any mismatches.
                (_, value) => {
                    resolved.push(value.clone());
                }
            }
        }

        Ok(resolved)
    }

    async fn resolve_state_value(
        &self,
        default: &ABIDefaultValue,
        value_type: &ABIType,
        source: StateSource<'_>,
    ) -> Result<ABIValue, AppClientError> {
        let key = base64::engine::general_purpose::STANDARD
            .decode(&default.data)
            .map_err(|e| AppClientError::ParamsBuilderError {
                message: format!(
                    "Failed to decode {} key: {}",
                    match source {
                        StateSource::Global => "global",
                        StateSource::Local(_) => "local",
                    },
                    e
                ),
            })?;

        let state = match source {
            StateSource::Global => self.client.get_global_state().await?,
            StateSource::Local(sender) => self.client.get_local_state(sender).await?,
        };

        let app_state = state
            .values()
            .find(|value| match value {
                AppState::Uint(uint_value) => uint_value.key_raw == key,
                AppState::Bytes(bytes_value) => bytes_value.key_raw == key,
            })
            .ok_or_else(|| AppClientError::ParamsBuilderError {
                message: format!(
                    "The key {} could not be found in {} storage",
                    default.data,
                    match source {
                        StateSource::Global => "global",
                        StateSource::Local(_) => "local",
                    }
                ),
            })?;

        match app_state {
            AppState::Uint(uint_value) => Ok(ABIValue::from(uint_value.value)),
            AppState::Bytes(bytes_value) => Ok(value_type
                .decode(&bytes_value.value_raw)
                .map_err(|e| AppClientError::ABIError { source: e })?),
        }
    }

    /// Resolve a default value from various sources (method call, literal, state, or box).
    pub async fn resolve_default_value(
        &self,
        default: &ABIDefaultValue,
        value_type: &ABIType,
        sender: &str,
    ) -> Result<ABIValue, AppClientError> {
        let value_type = default.value_type.clone().unwrap_or(value_type.clone());

        match default.source {
            DefaultValueSource::Method => {
                let method_signature = default.data.clone();
                let arc56_method = self
                    .client
                    .app_spec
                    .get_method(&method_signature)
                    .map_err(|e| AppClientError::ABIError { source: e })?;

                let method_call_params = AppClientMethodCallParams {
                    method: method_signature.clone(),
                    args: vec![AppMethodCallArg::DefaultValue; arc56_method.args.len()],
                    sender: Some(sender.to_string()),
                    ..Default::default()
                };

                let app_call_result =
                    Box::pin(self.client.send().call(method_call_params, None, None)).await?;
                let abi_return = app_call_result.abi_return.ok_or_else(|| {
                    AppClientError::ParamsBuilderError {
                        message: "Method call did not return a value".to_string(),
                    }
                })?;

                match abi_return.return_value {
                    None => Err(AppClientError::ParamsBuilderError {
                        message: "Method call did not return a value".to_string(),
                    }),
                    Some(return_value) => Ok(return_value),
                }
            }
            DefaultValueSource::Literal => {
                let value_bytes = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode base64 literal: {}", e),
                    })?;
                Ok(value_type
                    .decode(&value_bytes)
                    .map_err(|e| AppClientError::ABIError { source: e })?)
            }
            DefaultValueSource::Global => {
                self.resolve_state_value(default, &value_type, StateSource::Global)
                    .await
            }
            DefaultValueSource::Local => {
                self.resolve_state_value(default, &value_type, StateSource::Local(sender))
                    .await
            }
            DefaultValueSource::Box => {
                let box_key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode box key: {}", e),
                    })?;
                let box_value = self.client.get_box_value(&box_key).await?;
                Ok(value_type
                    .decode(&box_value)
                    .map_err(|e| AppClientError::ABIError { source: e })?)
            }
        }
    }
}

impl BareParamsBuilder<'_> {
    /// Build parameters for a bare application call with the specified on-complete action.
    pub fn call(
        &self,
        params: AppClientBareCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, on_complete.unwrap_or(OnApplicationComplete::NoOp))
    }

    /// Build parameters for a bare application call with OptIn on-complete action.
    pub fn opt_in(&self, params: AppClientBareCallParams) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, OnApplicationComplete::OptIn)
    }

    /// Build parameters for a bare application call with CloseOut on-complete action.
    pub fn close_out(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, OnApplicationComplete::CloseOut)
    }

    /// Build parameters for a bare application call with Delete on-complete action.
    pub fn delete(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<AppDeleteParams, AppClientError> {
        Ok(AppDeleteParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: self.client.app_id,
            args: params.args,
            account_references: parse_account_refs_to_addresses(&params.account_references)?,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }

    /// Build parameters for a bare application call with ClearState on-complete action.
    pub fn clear_state(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, OnApplicationComplete::ClearState)
    }

    /// Build parameters for updating the application using a bare application call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppUpdateParams, CompiledPrograms), AppClientError> {
        // Compile programs (and populate AppManager cache/source maps)
        let compilation_params = compilation_params.unwrap_or_default();
        let compiled_programs = self.client.compile(&compilation_params).await?;

        let update_params = AppUpdateParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: self.client.app_id,
            args: params.args,
            account_references: parse_account_refs_to_addresses(&params.account_references)?,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            approval_program: compiled_programs.approval.compiled_base64_to_bytes.clone(),
            clear_state_program: compiled_programs.clear.compiled_base64_to_bytes.clone(),
        };

        Ok((update_params, compiled_programs))
    }

    fn build_bare_app_call_params(
        &self,
        params: AppClientBareCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<AppCallParams, AppClientError> {
        Ok(AppCallParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: self
                .client
                .resolve_signer(params.sender.clone(), params.signer.clone()),
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: self.client.app_id,
            on_complete,
            args: params.args,
            account_references: parse_account_refs_to_addresses(&params.account_references)?,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }
}

fn get_optional_address(value: &Option<String>) -> Result<Option<Address>, AppClientError> {
    match value {
        Some(s) => {
            Ok(Some(Address::from_str(s).map_err(|e| {
                AppClientError::TransactError { source: e }
            })?))
        }
        None => Ok(None),
    }
}
