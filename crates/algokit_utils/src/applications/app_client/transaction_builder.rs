use crate::AppClientError;
use algokit_transact::OnApplicationComplete;
use futures::TryFutureExt;

use super::types::{AppClientBareCallParams, AppClientMethodCallParams, CompilationParams};
use super::{AppClient, FundAppAccountParams};

pub struct TransactionBuilder<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

pub struct BareTransactionBuilder<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

impl TransactionBuilder<'_> {
    /// Get the bare transaction builder.
    pub fn bare(&self) -> BareTransactionBuilder<'_> {
        BareTransactionBuilder {
            client: self.client,
        }
    }

    /// Create an unsigned ABI method call transaction with the specified on-complete action.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().call(params, on_complete).await?;
        let trasactions = self
            .client
            .algorand
            .create()
            .app_call_method_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await?;
        Ok(trasactions[0].clone())
    }

    /// Create an unsigned ABI method call transaction with OptIn on-complete action.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().opt_in(params).await?;
        let trasactions = self
            .client
            .algorand
            .create()
            .app_call_method_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await?;
        Ok(trasactions[0].clone())
    }

    /// Create an unsigned ABI method call transaction with CloseOut on-complete action.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().close_out(params).await?;
        let trasactions = self
            .client
            .algorand
            .create()
            .app_call_method_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await?;
        Ok(trasactions[0].clone())
    }

    /// Create an unsigned ABI method call transaction with ClearState on-complete action.
    pub async fn clear_state(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().clear_state(params).await?;
        let trasactions = self
            .client
            .algorand
            .create()
            .app_call_method_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await?;
        Ok(trasactions[0].clone())
    }

    /// Create an unsigned ABI method call transaction with Delete on-complete action.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().delete(params).await?;
        let trasactions = self
            .client
            .algorand
            .create()
            .app_delete_method_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await?;
        Ok(trasactions[0].clone())
    }

    /// Create an unsigned application update transaction using an ABI method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let (params, _compiled) = self
            .client
            .params()
            .update(params, compilation_params)
            .await?;
        let trasactions = self
            .client
            .algorand()
            .create()
            .app_update_method_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await?;
        Ok(trasactions[0].clone())
    }

    /// Create an unsigned payment transaction to fund the application's account.
    pub async fn fund_app_account(
        &self,
        params: FundAppAccountParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().fund_app_account(&params)?;
        self.client
            .algorand
            .create()
            .payment(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }
}

impl BareTransactionBuilder<'_> {
    /// Create an unsigned bare application call transaction with the specified on-complete action.
    pub async fn call(
        &self,
        params: AppClientBareCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().bare().call(params, on_complete)?;
        self.client
            .algorand
            .create()
            .app_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }

    /// Create an unsigned bare application call transaction with OptIn on-complete action.
    pub async fn opt_in(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().bare().opt_in(params)?;
        self.client
            .algorand
            .create()
            .app_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }

    /// Create an unsigned bare application call transaction with CloseOut on-complete action.
    pub async fn close_out(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().bare().close_out(params)?;
        self.client
            .algorand
            .create()
            .app_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }

    /// Create an unsigned bare application call transaction with Delete on-complete action.
    pub async fn delete(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().bare().delete(params)?;
        self.client
            .algorand
            .create()
            .app_delete(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }

    /// Create an unsigned bare application call transaction with ClearState on-complete action.
    pub async fn clear_state(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let params = self.client.params().bare().clear_state(params)?;
        self.client
            .algorand
            .create()
            .app_call(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }

    /// Create an unsigned application update transaction using a bare application call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<algokit_transact::Transaction, AppClientError> {
        let (params, _compiled) = self
            .client
            .params()
            .bare()
            .update(params, compilation_params)
            .await?;
        self.client
            .algorand()
            .create()
            .app_update(params)
            .map_err(|e| AppClientError::ComposerError { source: e })
            .await
    }
}
