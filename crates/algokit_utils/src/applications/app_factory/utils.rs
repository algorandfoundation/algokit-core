use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::CompilationParams;
use crate::applications::app_factory::{AppFactoryCreateMethodCallParams, AppFactoryCreateParams};
use crate::clients::app_manager::CompiledPrograms;
use crate::transactions::{
    AppCreateMethodCallParams, AppCreateParams, AppMethodCallArg, TransactionSenderError,
    TransactionSigner,
};
use algokit_abi::ABIMethod;
use algokit_abi::abi_type::ABIType;
use algokit_abi::arc56_contract::{DefaultValue, DefaultValueSource, MethodArg};
use algokit_transact::{Address, OnApplicationComplete, StateSchema};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as Base64;
use std::str::FromStr;
use std::sync::Arc;

impl AppFactory {
    pub(crate) async fn prepare_compiled_method(
        &self,
        method_sig: &str,
        compilation_params: Option<CompilationParams>,
        sender_opt: &Option<String>,
    ) -> Result<(CompiledPrograms, ABIMethod, Address), AppFactoryError> {
        let compiled = self.compile_programs_with(compilation_params).await?;
        let method = self
            .app_spec()
            .find_abi_method(method_sig)
            .map_err(|e| AppFactoryError::ABIError { source: e })?;
        let sender = self
            .get_sender_address(sender_opt)
            .map_err(|message| AppFactoryError::ValidationError { message })?;
        Ok((compiled, method, sender))
    }
}

/// Merge user-provided ABI method arguments with ARC-56 literal defaults.
/// Only 'literal' default values are supported; others will be ignored and treated as missing.
pub(crate) fn merge_args_with_defaults(
    factory: &AppFactory,
    method_name_or_signature: &str,
    user_args: &Option<Vec<AppMethodCallArg>>,
) -> Result<Vec<AppMethodCallArg>, AppFactoryError> {
    let contract = factory.app_spec();
    let method = contract.get_method(method_name_or_signature).map_err(|e| {
        AppFactoryError::ValidationError {
            message: e.to_string(),
        }
    })?;

    let mut result: Vec<AppMethodCallArg> = Vec::with_capacity(method.args.len());
    let provided = user_args.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

    for (i, arg_def) in method.args.iter().enumerate() {
        let method_arg_name = arg_def
            .name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| format!("arg{}", i + 1));

        if i < provided.len() {
            let provided_arg = &provided[i];

            if matches!(provided_arg, AppMethodCallArg::DefaultValue) {
                let default = arg_def.default_value.as_ref().ok_or_else(|| {
                    AppFactoryError::ParamsBuilderError {
                        message: format!(
                            "No default value defined for argument {} in call to method {}",
                            method_arg_name, method.name
                        ),
                    }
                })?;

                if default.source != DefaultValueSource::Literal {
                    return Err(AppFactoryError::ParamsBuilderError {
                        message: format!(
                            "Default value is not supported by argument {} in call to method {}",
                            method_arg_name, method.name
                        ),
                    });
                }

                let literal =
                    decode_literal_default_value(default, arg_def, &method.name, &method_arg_name)?;
                result.push(AppMethodCallArg::ABIValue(literal));
            } else {
                result.push(provided_arg.clone());
            }

            continue;
        }

        if let Some(default) = &arg_def.default_value {
            if matches!(default.source, DefaultValueSource::Literal) {
                let literal =
                    decode_literal_default_value(default, arg_def, &method.name, &method_arg_name)?;

                result.push(AppMethodCallArg::ABIValue(literal));
                continue;
            }
        }

        return Err(AppFactoryError::ParamsBuilderError {
            message: format!(
                "No value provided for required argument {} in call to method {}",
                method_arg_name, method.name
            ),
        });
    }

    Ok(result)
}

fn decode_literal_default_value(
    default: &DefaultValue,
    arg_def: &MethodArg,
    method_name: &str,
    arg_name: &str,
) -> Result<algokit_abi::ABIValue, AppFactoryError> {
    if !matches!(default.source, DefaultValueSource::Literal) {
        return Err(AppFactoryError::ParamsBuilderError {
            message: format!(
                "Default value for argument {} in call to method {} must be a literal",
                arg_name, method_name
            ),
        });
    }

    let abi_type_str = default.value_type.as_deref().unwrap_or(&arg_def.arg_type);
    let abi_type =
        ABIType::from_str(abi_type_str).map_err(|e| AppFactoryError::ParamsBuilderError {
            message: e.to_string(),
        })?;

    let bytes = Base64
        .decode(&default.data)
        .map_err(|e| AppFactoryError::ParamsBuilderError {
            message: format!(
                "Failed to base64-decode default literal for argument {} in call to method {}: {}",
                arg_name, method_name, e
            ),
        })?;

    let abi_value = abi_type
        .decode(&bytes)
        .map_err(|e| AppFactoryError::ABIError { source: e })?;

    Ok(abi_value)
}

/// Transform a transaction error using AppClient logic error exposure for factory flows.
pub(crate) fn transform_transaction_error_for_factory(
    factory: &AppFactory,
    err: TransactionSenderError,
    is_clear: bool,
) -> TransactionSenderError {
    let err_str = err.to_string();
    if let Some(logic_message) = factory.logic_error_for(&err_str, is_clear) {
        TransactionSenderError::ValidationError {
            message: logic_message,
        }
    } else {
        err
    }
}

/// Resolve signer: prefer explicit signer; otherwise use factory default signer when
/// sender is unspecified or equals the factory default sender.
pub(crate) fn resolve_signer(
    factory: &AppFactory,
    sender: &Option<String>,
    signer: Option<Arc<dyn TransactionSigner>>,
) -> Option<Arc<dyn TransactionSigner>> {
    signer.or_else(
        || match (sender.as_deref(), factory.default_sender.as_deref()) {
            (None, _) => factory.default_signer.clone(),
            (Some(s), Some(d)) if s == d => factory.default_signer.clone(),
            _ => None,
        },
    )
}

/// Returns the provided schemas or falls back to those declared in the contract spec.
pub(crate) fn default_schemas(
    factory: &AppFactory,
    global: Option<StateSchema>,
    local: Option<StateSchema>,
) -> (Option<StateSchema>, Option<StateSchema>) {
    let g = global.or_else(|| {
        let s = &factory.app_spec().state.schema.global_state;
        Some(StateSchema {
            num_uints: s.ints,
            num_byte_slices: s.bytes,
        })
    });
    let l = local.or_else(|| {
        let s = &factory.app_spec().state.schema.local_state;
        Some(StateSchema {
            num_uints: s.ints,
            num_byte_slices: s.bytes,
        })
    });
    (g, l)
}

pub(crate) fn build_create_method_call_params(
    factory: &AppFactory,
    sender: Address,
    base: &AppFactoryCreateMethodCallParams,
    method: ABIMethod,
    args: Vec<AppMethodCallArg>,
    approval_program: Vec<u8>,
    clear_state_program: Vec<u8>,
) -> AppCreateMethodCallParams {
    let (global_state_schema, local_state_schema) = default_schemas(
        factory,
        base.global_state_schema.clone(),
        base.local_state_schema.clone(),
    );

    AppCreateMethodCallParams {
        sender,
        signer: resolve_signer(factory, &base.sender, base.signer.clone()),
        rekey_to: base.rekey_to.clone(),
        note: base.note.clone(),
        lease: base.lease,
        static_fee: base.static_fee,
        extra_fee: base.extra_fee,
        max_fee: base.max_fee,
        validity_window: base.validity_window,
        first_valid_round: base.first_valid_round,
        last_valid_round: base.last_valid_round,
        on_complete: base.on_complete.unwrap_or(OnApplicationComplete::NoOp),
        approval_program,
        clear_state_program,
        method,
        args,
        account_references: base.account_references.clone(),
        app_references: base.app_references.clone(),
        asset_references: base.asset_references.clone(),
        box_references: base.box_references.clone(),
        global_state_schema,
        local_state_schema,
        extra_program_pages: base.extra_program_pages,
    }
}

pub(crate) fn build_bare_create_params(
    factory: &AppFactory,
    sender: Address,
    base: &AppFactoryCreateParams,
    approval_program: Vec<u8>,
    clear_state_program: Vec<u8>,
) -> AppCreateParams {
    let (global_state_schema, local_state_schema) = default_schemas(
        factory,
        base.global_state_schema.clone(),
        base.local_state_schema.clone(),
    );

    AppCreateParams {
        sender,
        signer: resolve_signer(factory, &base.sender, base.signer.clone()),
        rekey_to: base.rekey_to.clone(),
        note: base.note.clone(),
        lease: base.lease,
        static_fee: base.static_fee,
        extra_fee: base.extra_fee,
        max_fee: base.max_fee,
        validity_window: base.validity_window,
        first_valid_round: base.first_valid_round,
        last_valid_round: base.last_valid_round,
        on_complete: base.on_complete.unwrap_or(OnApplicationComplete::NoOp),
        approval_program,
        clear_state_program,
        args: base.args.clone(),
        account_references: base.account_references.clone(),
        app_references: base.app_references.clone(),
        asset_references: base.asset_references.clone(),
        box_references: base.box_references.clone(),
        global_state_schema,
        local_state_schema,
        extra_program_pages: base.extra_program_pages,
    }
}
