use std::sync::Arc;

use super::account_helpers::{NetworkType, TestAccount, TestAccountConfig, TestAccountManager};
use crate::{AlgoConfig, ClientManager, Composer};
use algod_client::AlgodClient;
use algokit_transact::Transaction;

pub struct AlgorandFixture {
    config: AlgoConfig,
    context: Option<AlgorandTestContext>,
}

pub struct AlgorandTestContext {
    pub algod: Arc<AlgodClient>,

    pub composer: Composer,

    pub test_account: TestAccount,

    pub account_manager: TestAccountManager,
}

#[derive(Debug)]
pub struct TransactionResult {
    pub transaction: Transaction,
    pub tx_id: String,
    pub signed_bytes: Vec<u8>,
}

impl AlgorandFixture {
    pub fn new(config: AlgoConfig) -> Self {
        Self {
            config,
            context: None,
        }
    }

    pub fn context(
        &self,
    ) -> Result<&AlgorandTestContext, Box<dyn std::error::Error + Send + Sync>> {
        self.context
            .as_ref()
            .ok_or_else(|| "Context not initialized; make sure to call fixture.new_scope() before accessing context.".into())
    }

    pub fn new_composer(&mut self) -> Result<Composer, Box<dyn std::error::Error + Send + Sync>> {
        let context = self
            .context
            .as_mut()
            .ok_or("Context not initialized; call new_scope() first")?;

        Ok(Composer::new(
            context.algod.clone(),
            Arc::new(context.test_account.clone()),
        ))
    }

    pub async fn new_scope(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let algod = Arc::new(ClientManager::get_algod_client(&self.config.algod_config));

        let mut account_manager = TestAccountManager::new(algod.clone());

        let test_account_config = TestAccountConfig {
            initial_funds: 10_000_000,
            network_type: NetworkType::LocalNet,
            funding_note: Some("AlgorandFixture test account".to_string()),
        };

        let test_account = account_manager
            .get_test_account(Some(test_account_config))
            .await
            .map_err(|e| format!("Failed to create test account: {}", e))?;

        // Now TestAccount implements TransactionSignerGetter directly, so we can use it without a wrapper
        let composer = Composer::new(algod.clone(), Arc::new(test_account.clone()));

        self.context = Some(AlgorandTestContext {
            algod,
            composer,
            test_account,
            account_manager,
        });

        Ok(())
    }

    pub async fn generate_account(
        &mut self,
        config: Option<TestAccountConfig>,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        let context = self
            .context
            .as_mut()
            .ok_or("Context not initialized; call new_scope() first")?;

        let account = context
            .account_manager
            .get_test_account(config)
            .await
            .map_err(|e| format!("Failed to generate account: {}", e))?;

        Ok(account)
    }
}

pub async fn algorand_fixture() -> Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>
{
    let config = ClientManager::get_config_from_environment_or_localnet();
    Ok(AlgorandFixture::new(config))
}

pub async fn algorand_fixture_with_config(
    config: AlgoConfig,
) -> Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>> {
    Ok(AlgorandFixture::new(config))
}
