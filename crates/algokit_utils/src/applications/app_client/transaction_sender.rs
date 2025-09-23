use crate::applications::app_client::utils::transform_transaction_error;
use crate::transactions::SendTransactionResult;
use crate::transactions::composer::SimulateParams;
use crate::{AppClientError, SendAppCallResult, SendAppUpdateResult, SendParams};
use algokit_transact::{MAX_SIMULATE_OPCODE_BUDGET, OnApplicationComplete};

use super::types::{AppClientBareCallParams, AppClientMethodCallParams, CompilationParams};
use super::{AppClient, FundAppAccountParams};

pub struct TransactionSender<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

pub struct BareTransactionSender<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

impl<'app_client> TransactionSender<'app_client> {
    /// Get the bare transaction sender.
    pub fn bare(&self) -> BareTransactionSender<'app_client> {
        BareTransactionSender {
            client: self.client,
        }
    }

    /// Execute an ABI method call with the specified on-complete action.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
        on_complete: Option<OnApplicationComplete>,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCallResult, AppClientError> {
        let arc56_method = self
            .client
            .app_spec
            .get_method(&params.method)
            .map_err(|e| AppClientError::ABIError { source: e })?;

        let mut method_params = self.client.params().call(params, on_complete).await?;

        if method_params.on_complete == OnApplicationComplete::NoOp
            && arc56_method.readonly == Some(true)
        {
            let transaction_composer_config = self.client.transaction_composer_config.clone();

            let mut composer = self
                .client
                .algorand()
                .new_group(transaction_composer_config.clone());

            if transaction_composer_config
                .clone()
                .is_some_and(|c| c.cover_app_call_inner_transaction_fees)
                && method_params.max_fee.is_some()
            {
                method_params.static_fee = method_params.max_fee;
                method_params.extra_fee = None;
            }

            let _ = composer
                .add_app_call_method_call(method_params)
                .map_err(|e| AppClientError::ComposerError { source: e });

            let simulate_params = SimulateParams {
                allow_unnamed_resources: Some(
                    transaction_composer_config
                        .map(|c| c.populate_app_call_resources.is_enabled())
                        .unwrap_or(true),
                ),
                skip_signatures: true,
                extra_opcode_budget: Some(MAX_SIMULATE_OPCODE_BUDGET),
                ..Default::default()
            };

            let transactions_with_signers = composer
                .build()
                .await
                .map_err(|e| AppClientError::ComposerError { source: e })?;
            let transactions = transactions_with_signers
                .iter()
                .map(|tx_with_signer| tx_with_signer.transaction.clone())
                .collect();

            let simulate_results = composer
                .simulate(Some(simulate_params))
                .await
                .map_err(|e| AppClientError::ComposerError { source: e })?;

            let group_id = simulate_results
                .group
                .map(hex::encode)
                .unwrap_or_else(|| "".to_string());
            let abi_returns = simulate_results.abi_returns;
            let last_abi_return = abi_returns.last().cloned();

            let send_transaction_result = SendTransactionResult::new(
                group_id,
                simulate_results.transaction_ids,
                transactions,
                simulate_results.confirmations,
                Some(abi_returns),
            )
            .map_err(|e| AppClientError::TransactionResultError { source: e })?;

            let send_app_call_result =
                SendAppCallResult::new(send_transaction_result, last_abi_return);

            Ok(send_app_call_result)
        } else {
            self.client
                .algorand
                .send()
                .app_call_method_call(method_params, send_params)
                .await
                .map_err(|e| transform_transaction_error(self.client, e, false))
        }
    }

    /// Execute an ABI method call with OptIn on-complete action.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCallResult, AppClientError> {
        let method_params = self.client.params().opt_in(params).await?;

        self.client
            .algorand
            .send()
            .app_call_method_call(method_params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Execute an ABI method call with CloseOut on-complete action.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCallResult, AppClientError> {
        let method_params = self.client.params().close_out(params).await?;

        self.client
            .algorand
            .send()
            .app_call_method_call(method_params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Execute an ABI method call with Delete on-complete action.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendAppCallResult, AppClientError> {
        let delete_params = self.client.params().delete(params).await?;

        self.client
            .algorand
            .send()
            .app_delete_method_call(delete_params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Update the application using an ABI method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
        send_params: Option<SendParams>,
    ) -> Result<SendAppUpdateResult, AppClientError> {
        let (update_params, compiled) = self
            .client
            .params()
            .update(params, compilation_params)
            .await?;

        let mut result = self
            .client
            .algorand()
            .send()
            .app_update_method_call(update_params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))?;

        result.compiled_approval = Some(compiled.approval.compiled_base64_to_bytes.clone());
        result.compiled_clear = Some(compiled.clear.compiled_base64_to_bytes.clone());
        result.approval_source_map = compiled.approval.source_map.clone();
        result.clear_source_map = compiled.clear.source_map.clone();

        Ok(result)
    }

    /// Send payment to fund the application's account.
    pub async fn fund_app_account(
        &self,
        params: FundAppAccountParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let payment = self.client.params().fund_app_account(&params)?;

        self.client
            .algorand
            .send()
            .payment(payment, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }
}

impl BareTransactionSender<'_> {
    /// Execute a bare application call with the specified on-complete action.
    pub async fn call(
        &self,
        params: AppClientBareCallParams,
        on_complete: Option<OnApplicationComplete>,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let params = self.client.params().bare().call(params, on_complete)?;
        self.client
            .algorand
            .send()
            .app_call(params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Execute a bare application call with OptIn on-complete action.
    pub async fn opt_in(
        &self,
        params: AppClientBareCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let app_call = self.client.params().bare().opt_in(params)?;
        self.client
            .algorand
            .send()
            .app_call(app_call, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Execute a bare application call with CloseOut on-complete action.
    pub async fn close_out(
        &self,
        params: AppClientBareCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let app_call = self.client.params().bare().close_out(params)?;
        self.client
            .algorand
            .send()
            .app_call(app_call, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Execute a bare application call with Delete on-complete action.
    pub async fn delete(
        &self,
        params: AppClientBareCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let delete_params = self.client.params().bare().delete(params)?;
        self.client
            .algorand
            .send()
            .app_delete(delete_params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))
    }

    /// Execute a bare application call with ClearState on-complete action.
    pub async fn clear_state(
        &self,
        params: AppClientBareCallParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let app_call = self.client.params().bare().clear_state(params)?;
        self.client
            .algorand
            .send()
            .app_call(app_call, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, true))
    }

    /// Update the application using a bare application call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
        send_params: Option<SendParams>,
    ) -> Result<SendAppUpdateResult, AppClientError> {
        let (update_params, compiled_programs) = self
            .client
            .params()
            .bare()
            .update(params, compilation_params)
            .await?;

        let mut result = self
            .client
            .algorand()
            .send()
            .app_update(update_params, send_params)
            .await
            .map_err(|e| transform_transaction_error(self.client, e, false))?;

        result.compiled_approval =
            Some(compiled_programs.approval.compiled_base64_to_bytes.clone());
        result.compiled_clear = Some(compiled_programs.clear.compiled_base64_to_bytes.clone());
        result.approval_source_map = compiled_programs.approval.source_map.clone();
        result.clear_source_map = compiled_programs.clear.source_map.clone();

        Ok(result)
    }
}
