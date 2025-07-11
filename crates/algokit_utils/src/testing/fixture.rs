use std::sync::Arc;

use super::account_helpers::{NetworkType, TestAccount, TestAccountConfig, TestAccountManager};
use crate::{AlgoConfig, ClientManager, Composer, TransactionSigner, TransactionSignerGetter};
use algod_client::AlgodClient;
use algokit_transact::{Address, AlgorandMsgpack, SignedTransaction, Transaction};
use async_trait::async_trait;

// Implement TransactionSigner for TestAccount directly, eliminating the need for TestAccountSigner wrapper
#[async_trait]
impl TransactionSigner for TestAccount {
    async fn sign_transactions(
        &self,
        txns: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String> {
        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    // Use the TestAccount's sign_transaction method to get signed bytes
                    let signed_bytes = self
                        .sign_transaction(&txns[idx])
                        .map_err(|e| format!("Failed to sign transaction: {}", e))?;

                    // Decode the signed bytes back to SignedTransaction
                    SignedTransaction::decode(&signed_bytes)
                        .map_err(|e| format!("Failed to decode signed transaction: {}", e))
                } else {
                    Err(format!("Index {} out of bounds for transactions", idx))
                }
            })
            .collect()
    }
}

// Implement TransactionSignerGetter for TestAccount as well
#[async_trait]
impl TransactionSignerGetter for TestAccount {
    async fn get_signer(&self, address: Address) -> Option<&dyn TransactionSigner> {
        let test_account_address = self.account().expect("Failed to get test account address");
        if address == test_account_address.address() {
            Some(self)
        } else {
            None
        }
    }
}

pub struct AlgorandFixture {
    config: AlgoConfig,
    context: Option<AlgorandTestContext>,
}

pub struct AlgorandTestContext {
    pub algod: AlgodClient,

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

    pub async fn new_scope(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let algod = ClientManager::get_algod_client(&self.config.algod_config);

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
        let composer = Composer::new(algod.clone(), Some(Arc::new(test_account.clone())));

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
