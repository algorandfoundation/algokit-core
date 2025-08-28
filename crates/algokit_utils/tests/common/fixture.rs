use std::sync::Arc;

use crate::common::LocalNetDispenser;
use crate::common::logging::init_test_logging;

use super::indexer_helpers::wait_for_indexer_transaction;
use super::test_account::{NetworkType, TestAccount, TestAccountConfig};
use algod_client::AlgodClient;
use algokit_transact::Transaction;
use algokit_utils::{AlgoConfig, AlgorandClient, ClientManager};
use indexer_client::IndexerClient;
use rstest::*;

pub struct AlgorandFixture {
    pub algod: Arc<AlgodClient>,
    pub indexer: Arc<IndexerClient>,
    pub algorand_client: AlgorandClient,
    pub test_account: TestAccount,
}

pub type AlgorandFixtureResult = Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct TransactionResult {
    pub transaction: Transaction,
    pub tx_id: String,
    pub signed_bytes: Vec<u8>,
}

impl AlgorandFixture {
    pub async fn new(
        config: &AlgoConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let algod = Arc::new(ClientManager::get_algod_client(&config.algod_config));
        let indexer = Arc::new(ClientManager::get_indexer_client(&config.indexer_config));

        let mut algorand_client = AlgorandClient::new(config);

        let test_account = Self::generate_account_internal(
            algod.clone(),
            &mut algorand_client,
            Some(TestAccountConfig {
                initial_funds: 10_000_000,
                network_type: NetworkType::LocalNet,
                funding_note: Some("AlgorandFixture test account".to_string()),
            }),
        )
        .await
        .map_err(|e| format!("Failed to create test account: {}", e))?;

        Ok(Self {
            algod,
            indexer,
            algorand_client,
            test_account,
        })
    }

    async fn generate_account_internal(
        algod: Arc<AlgodClient>,
        algorand_client: &mut AlgorandClient,
        config: Option<TestAccountConfig>,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        let config = config.unwrap_or_default();
        let mut dispenser = LocalNetDispenser::new(algod.clone());

        // Generate new account using ed25519_dalek
        let test_account = TestAccount::generate()?;
        let test_account_address = test_account.account().address();

        // Fund the account based on network type
        match config.network_type {
            NetworkType::LocalNet => {
                dispenser
                    .fund_account(&test_account_address.to_string(), config.initial_funds)
                    .await?;
            }
            NetworkType::TestNet => {
                return Err(format!(
                    "⚠ TestNet funding not yet implemented. Please fund manually: {}",
                    test_account_address
                )
                .into());
            }
            NetworkType::MainNet => {
                return Err(format!(
                    "⚠ MainNet detected. Account generated but not funded: {}",
                    test_account_address
                )
                .into());
            }
        }

        algorand_client.set_signer(test_account_address, Arc::new(test_account.clone()));
        Ok(test_account)
    }

    pub async fn generate_account(
        &mut self,
        config: Option<TestAccountConfig>,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        Self::generate_account_internal(self.algod.clone(), &mut self.algorand_client, config).await
    }
}

impl AlgorandFixture {
    /// Waits for a transaction to appear in the indexer
    pub async fn wait_for_indexer_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        wait_for_indexer_transaction(&self.indexer, transaction_id, None).await?;
        Ok(())
    }
}

#[fixture]
pub async fn algorand_fixture() -> Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>
{
    init_test_logging();
    let config = ClientManager::get_config_from_environment_or_localnet();
    AlgorandFixture::new(&config).await
}

pub async fn algorand_fixture_with_config(
    config: AlgoConfig,
) -> Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>> {
    init_test_logging();
    AlgorandFixture::new(&config).await
}
