use crate::clients::client_manager::ClientManager;
use crate::clients::network_client::{AlgoConfig, AlgorandService};
use algod_client::models::TransactionParams;

pub struct AlgorandClient {
    client_manager: ClientManager,
}

impl AlgorandClient {
    fn new(config: AlgoConfig) -> Self {
        Self {
            client_manager: ClientManager::new(config),
        }
    }

    pub async fn get_suggested_params(
        &self,
    ) -> Result<TransactionParams, Box<dyn std::error::Error>> {
        Ok(self.client_manager.algod().transaction_params().await?)
    }

    pub fn client(&self) -> &ClientManager {
        &self.client_manager
    }

    pub fn default_localnet() -> Self {
        Self::new(AlgoConfig {
            algod_config: ClientManager::get_default_localnet_config(AlgorandService::Algod),
            indexer_config: ClientManager::get_default_localnet_config(AlgorandService::Indexer),
        })
    }

    pub fn testnet() -> Self {
        Self::new(AlgoConfig {
            algod_config: ClientManager::get_algonode_config("testnet", AlgorandService::Algod),
            indexer_config: ClientManager::get_algonode_config("testnet", AlgorandService::Indexer),
        })
    }

    pub fn mainnet() -> Self {
        Self::new(AlgoConfig {
            algod_config: ClientManager::get_algonode_config("mainnet", AlgorandService::Algod),
            indexer_config: ClientManager::get_algonode_config("mainnet", AlgorandService::Indexer),
        })
    }

    pub fn from_environment() -> Self {
        Self::new(ClientManager::get_config_from_environment_or_localnet())
    }

    pub fn from_config(config: AlgoConfig) -> Self {
        Self::new(config)
    }
}
