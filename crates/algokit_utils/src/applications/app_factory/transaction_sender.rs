use super::AppFactory;
use super::utils::{
    build_bare_create_params, build_create_method_call_params, merge_args_with_defaults,
    transform_transaction_error_for_factory,
};
use crate::applications::app_client::AppClient;
use crate::applications::app_client::CompilationParams;
use crate::applications::app_factory::{
    AppFactoryCreateMethodCallParams, AppFactoryCreateMethodCallResult, AppFactoryCreateParams,
};
use crate::transactions::{SendParams, TransactionSenderError};
use algokit_transact::Address;

/// Sends factory-backed create transactions and returns both the client and send results.
pub struct TransactionSender<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

/// Bare transaction helpers for AppFactory create flows.
pub struct BareTransactionSender<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

impl<'app_factory> TransactionSender<'app_factory> {
    /// Returns helpers for bare (non-ABI) create transactions.
    pub fn bare(&self) -> BareTransactionSender<'app_factory> {
        BareTransactionSender {
            factory: self.factory,
        }
    }

    /// Sends an app creation method call and returns the new client with the factory
    /// flavoured result wrapper.
    ///
    /// # Errors
    /// Returns [`TransactionSenderError`] if argument merging, compilation, or the
    /// underlying transaction submission fails.
    pub async fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, AppFactoryCreateMethodCallResult), TransactionSenderError> {
        let merged_args = merge_args_with_defaults(self.factory, &params.method, &params.args)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let (compiled, method, sender) = self
            .factory
            .prepare_compiled_method(&params.method, compilation_params, &params.sender)
            .await
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let approval_bytes = compiled.approval.compiled_base64_to_bytes.clone();
        let clear_bytes = compiled.clear.compiled_base64_to_bytes.clone();

        let create_params = build_create_method_call_params(
            self.factory,
            sender,
            &params,
            method,
            merged_args,
            approval_bytes.clone(),
            clear_bytes.clone(),
        );

        let result = self
            .factory
            .algorand()
            .send()
            .app_create_method_call(create_params, send_params)
            .await
            .map_err(|e| transform_transaction_error_for_factory(self.factory, e, false))
            .map(|mut res| {
                res.compiled_approval = Some(approval_bytes.clone());
                res.compiled_clear = Some(clear_bytes.clone());
                res
            })?;

        let app_client = self.factory.get_app_client_by_id(
            result.app_id,
            Some(self.factory.app_name().to_string()),
            None,
            None,
            None,
        );

        // Extract app ID and construct the factory result
        let app_id = result.app_id;
        let app_address = Address::from_app_id(&app_id);
        let arc56_return = self
            .factory
            .parse_method_return_value(&result.abi_return)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let factory_result = AppFactoryCreateMethodCallResult::new(
            result.common_params.transaction,
            result.common_params.confirmation,
            result.common_params.tx_id,
            result.common_params.group_id,
            result.common_params.tx_ids,
            result.common_params.transactions,
            result.common_params.confirmations,
            app_id,
            app_address,
            Some(approval_bytes),
            Some(clear_bytes),
            result.abi_return,
            arc56_return,
        );

        Ok((app_client, factory_result))
    }
}

impl BareTransactionSender<'_> {
    /// Sends a bare app creation and returns the new client with the send result.
    ///
    /// # Errors
    /// Returns [`TransactionSenderError`] if compilation fails, the sender address is
    /// invalid, or the underlying transaction submission fails.
    pub async fn create(
        &self,
        params: Option<AppFactoryCreateParams>,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, AppFactoryCreateMethodCallResult), TransactionSenderError> {
        let params = params.unwrap_or_default();

        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        let create_params = build_bare_create_params(
            self.factory,
            sender,
            &params,
            compiled.approval.compiled_base64_to_bytes.clone(),
            compiled.clear.compiled_base64_to_bytes.clone(),
        );

        let mut result = self
            .factory
            .algorand()
            .send()
            .app_create(create_params, send_params)
            .await
            .map_err(|e| transform_transaction_error_for_factory(self.factory, e, false))?;

        let app_id = result.app_id;
        let app_address = Address::from_app_id(&app_id);

        let app_client = self.factory.get_app_client_by_id(
            app_id,
            Some(self.factory.app_name().to_string()),
            None,
            None,
            None,
        );

        // Update the result with compiled programs
        result.compiled_approval = Some(compiled.approval.compiled_base64_to_bytes.clone());
        result.compiled_clear = Some(compiled.clear.compiled_base64_to_bytes.clone());

        // Convert to factory result with flattened fields
        let factory_result = AppFactoryCreateMethodCallResult::new(
            result.common_params.transaction,
            result.common_params.confirmation,
            result.common_params.tx_id,
            result.common_params.group_id,
            result.common_params.tx_ids,
            result.common_params.transactions,
            result.common_params.confirmations,
            app_id,
            app_address,
            result.compiled_approval,
            result.compiled_clear,
            result.abi_return,
            None, // No arc56_return for bare calls
        );

        Ok((app_client, factory_result))
    }
}
