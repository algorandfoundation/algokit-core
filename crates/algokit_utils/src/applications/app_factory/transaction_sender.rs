use super::AppFactory;
use super::utils::{
    build_bare_create_params, build_create_method_call_params, merge_args_with_defaults,
};
use crate::applications::app_client::AppClient;
use crate::applications::app_client::CompilationParams;
use crate::applications::app_factory::{
    AppFactoryCreateMethodCallParams, AppFactoryCreateMethodCallResult, AppFactoryCreateParams,
    AppFactoryCreateResult, AppFactoryError,
};
use crate::transactions::SendParams;
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
    /// Returns [`AppFactoryError`] if argument merging, compilation, or the
    /// underlying transaction submission fails.
    pub async fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, AppFactoryCreateMethodCallResult), AppFactoryError> {
        let compiled = self.factory.compile(compilation_params).await?;

        let method = self
            .factory
            .app_spec()
            .find_abi_method(&params.method)
            .map_err(|e| AppFactoryError::ABIError { source: e })?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        let merged_args = merge_args_with_defaults(self.factory, &params.method, &params.args)?;

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
            .map_err(|e| self.factory.handle_transaction_error(e, false))?;

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

        let factory_result = AppFactoryCreateMethodCallResult {
            transaction: result.transaction,
            confirmation: result.confirmation,
            transaction_id: result.transaction_id,
            group: result.group,
            abi_return: result.abi_return,
            transaction_ids: result.transaction_ids,
            transactions: result.transactions,
            confirmations: result.confirmations,
            app_id,
            app_address,
            compiled_programs: compiled,
            abi_returns: result.abi_returns,
        };

        Ok((app_client, factory_result))
    }
}

impl BareTransactionSender<'_> {
    /// Sends a bare app creation and returns the new client with the send result.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if compilation fails, the sender address is
    /// invalid, or the underlying transaction submission fails.
    pub async fn create(
        &self,
        params: Option<AppFactoryCreateParams>,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, AppFactoryCreateResult), AppFactoryError> {
        let params = params.unwrap_or_default();

        let compiled = self
            .factory
            .compile(compilation_params)
            .await
            .map_err(|e| AppFactoryError::ValidationError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| AppFactoryError::ValidationError { message: e })?;

        let create_params = build_bare_create_params(
            self.factory,
            sender,
            &params,
            compiled.approval.compiled_base64_to_bytes.clone(),
            compiled.clear.compiled_base64_to_bytes.clone(),
        );

        let result = self
            .factory
            .algorand()
            .send()
            .app_create(create_params, send_params)
            .await
            .map_err(|e| self.factory.handle_transaction_error(e, false))?;

        let app_id = result.app_id;
        let app_address = Address::from_app_id(&app_id);

        let app_client = self.factory.get_app_client_by_id(
            app_id,
            Some(self.factory.app_name().to_string()),
            None,
            None,
            None,
        );

        // Convert to factory result with flattened fields
        let factory_result = AppFactoryCreateResult {
            transaction: result.transaction,
            confirmation: result.confirmation,
            transaction_id: result.transaction_id,
            app_id,
            app_address,
            compiled_programs: compiled,
        };

        Ok((app_client, factory_result))
    }
}
