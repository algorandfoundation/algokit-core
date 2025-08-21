use crate::clients::app_manager::AppManager;
use crate::clients::asset_manager::AssetManager;
use crate::clients::client_manager::ClientManager;
use crate::clients::network_client::{AlgoConfig, AlgorandService};
use crate::transactions::{Composer, TransactionCreator, TransactionSender};
use algod_client::models::TransactionParams;
use std::sync::Arc;

pub struct AlgorandClient {
    client_manager: ClientManager,
    asset_manager: AssetManager,
    app_manager: AppManager,
    transaction_sender: TransactionSender,
    transaction_creator: TransactionCreator,
}

impl AlgorandClient {
    fn new(config: AlgoConfig) -> Self {
        let client_manager = ClientManager::new(config);
        let algod_client = client_manager.algod();

        let asset_manager = AssetManager::new(algod_client.clone());
        let app_manager = AppManager::new(algod_client.clone());

        // Create closure for new_group function
        let algod_client_for_sender = algod_client.clone();
        let transaction_sender = TransactionSender::new(
            move || {
                Composer::new(
                    algod_client_for_sender.clone(),
                    // TODO: Replace EmptySigner with dynamic signer resolution once AccountManager
                    // abstraction is implemented. Should resolve default signers from sender addresses
                    // similar to py/ts utils implementation's get signer function.
                    Arc::new(crate::transactions::EmptySigner {}),
                )
            },
            asset_manager.clone(),
            app_manager.clone(),
        );

        // Create closure for TransactionCreator
        let algod_client_for_creator = algod_client.clone();
        let transaction_creator = TransactionCreator::new(move || {
            Composer::new(
                algod_client_for_creator.clone(),
                // TODO: Replace EmptySigner with dynamic signer resolution once AccountManager
                // abstraction is implemented. Should resolve default signers from sender addresses
                // similar to py/ts utils implementation's get signer function.
                Arc::new(crate::transactions::EmptySigner {}),
            )
        });

        Self {
            client_manager,
            asset_manager,
            app_manager,
            transaction_sender,
            transaction_creator,
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

    /// Get access to the AssetManager for asset operations
    pub fn asset(&self) -> &AssetManager {
        &self.asset_manager
    }

    /// Get access to the AppManager for app operations
    pub fn app(&self) -> &AppManager {
        &self.app_manager
    }

    /// Get access to the TransactionSender for sending transactions
    pub fn send(&self) -> &TransactionSender {
        &self.transaction_sender
    }

    /// Get access to the TransactionCreator for building transactions
    pub fn create(&self) -> &TransactionCreator {
        &self.transaction_creator
    }

    /// Create a new transaction composer for building transaction groups
    pub fn new_group(&self) -> Composer {
        // TODO: Replace EmptySigner with dynamic signer resolution once AccountManager
        // abstraction is implemented. Should resolve default signers from sender addresses
        // similar to py/ts utils implementation's get signer function.
        Composer::new(
            self.client_manager.algod().clone(),
            Arc::new(crate::transactions::EmptySigner {}),
        )
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
