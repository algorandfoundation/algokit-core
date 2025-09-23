use super::AppFactory;
use super::utils::{
    build_bare_create_params, build_create_method_call_params, merge_args_with_defaults,
};
use crate::applications::app_client::CompilationParams;
use crate::applications::app_factory::{AppFactoryCreateMethodCallParams, AppFactoryCreateParams};
use crate::transactions::composer::ComposerError;
use algokit_transact::Transaction;

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
    /// Returns [`ComposerError`] if compilation fails, method lookup fails, or
    /// transaction construction encounters invalid inputs.
    pub async fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<Vec<Transaction>, ComposerError> {
        // Prepare compiled programs, method and sender in one step
        let (compiled, method, sender) = self
            .factory
            .prepare_compiled_method(&params.method, compilation_params, &params.sender)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let merged_args = merge_args_with_defaults(self.factory, &params.method, &params.args)
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

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
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let params = params.unwrap_or_default();

        // Compile using centralized helper
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

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
            .await
    }
}
