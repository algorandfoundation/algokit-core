use super::AppFactory;
use super::utils::{
    build_bare_create_params, build_create_method_call_params, merge_args_with_defaults,
};
use crate::applications::app_client::CompilationParams;
use crate::applications::app_factory::{
    AppFactoryCreateMethodCallParams, AppFactoryCreateParams, AppFactoryError,
};
use algokit_transact::Transaction;
use futures::TryFutureExt;

/// Builds transactions for AppFactory create flows without immediately submitting them.
pub struct TransactionBuilder<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

/// Builds bare create transactions ready for manual submission.
pub struct BareTransactionBuilder<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

impl<'app_factory> TransactionBuilder<'app_factory> {
    /// Returns helpers for building bare (non-ABI) transactions.
    pub fn bare(&self) -> BareTransactionBuilder<'app_factory> {
        BareTransactionBuilder {
            factory: self.factory,
        }
    }

    /// Builds transactions for an app creation method call without sending them.
    ///
    /// # Errors
    /// Returns [`AppFactoryError`] if compilation fails, method lookup fails, or
    /// transaction construction encounters invalid inputs.
    pub async fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<Vec<Transaction>, AppFactoryError> {
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

        let create_params = build_create_method_call_params(
            self.factory,
            sender,
            &params,
            method,
            merged_args,
            compiled.approval.compiled_base64_to_bytes,
            compiled.clear.compiled_base64_to_bytes,
        );

        self.factory
            .algorand()
            .create()
            .app_create_method_call(create_params)
            .map_err(|e| AppFactoryError::ComposerError { source: e })
            .await
    }
}

impl BareTransactionBuilder<'_> {
    /// Builds a bare app creation transaction without sending it.
    ///
    /// # Errors
    /// Returns [`ComposerError`] if compilation fails or the sender address is invalid.
    pub async fn create(
        &self,
        params: Option<AppFactoryCreateParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<algokit_transact::Transaction, AppFactoryError> {
        let params = params.unwrap_or_default();

        let compiled = self.factory.compile(compilation_params).await?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|message| AppFactoryError::ValidationError { message })?;

        let create_params = build_bare_create_params(
            self.factory,
            sender,
            &params,
            compiled.approval.compiled_base64_to_bytes,
            compiled.clear.compiled_base64_to_bytes,
        );

        self.factory
            .algorand()
            .create()
            .app_create(create_params)
            .map_err(|e| AppFactoryError::ComposerError { source: e })
            .await
    }
}
