use crate::applications::AppDeployer;
use crate::clients::app_manager::AppManager;
use crate::clients::asset_manager::AssetManager;
use crate::clients::client_manager::ClientManager;
use crate::clients::network_client::{AlgoConfig, AlgorandService};
use crate::transactions::{
    Composer, ComposerParams, TransactionComposerConfig, TransactionCreator, TransactionSender,
};
use crate::{AccountManager, TransactionSigner};
use algod_client::models::TransactionParams;
use algokit_transact::Address;
use std::sync::{Arc, Mutex};

pub struct AlgorandClient {
    client_manager: ClientManager,
    asset_manager: AssetManager,
    app_manager: AppManager,
    app_deployer: AppDeployer,
    transaction_sender: TransactionSender,
    transaction_creator: TransactionCreator,
    account_manager: Arc<Mutex<AccountManager>>,
    default_composer_config: Option<TransactionComposerConfig>,
}

/// A client that brokers easy access to Algorand functionality.
pub struct AlgorandClientParams {
    pub client_config: AlgoConfig,
    pub composer_config: Option<TransactionComposerConfig>,
}

impl AlgorandClient {
    pub fn new(params: &AlgorandClientParams) -> Self {
        let client_manager = ClientManager::new(&params.client_config).unwrap();
        let algod_client = client_manager.algod();

        let account_manager = Arc::new(Mutex::new(AccountManager::new()));

        let new_group = {
            let algod_client = algod_client.clone();
            let account_manager = account_manager.clone();
            let default_composer_config = params.composer_config.clone();
            move |composer_config: Option<TransactionComposerConfig>| {
                Composer::new(ComposerParams {
                    algod_client: algod_client.clone(),
                    signer_getter: account_manager.clone(),
                    composer_config: composer_config.or_else(|| default_composer_config.clone()),
                })
            }
        };

        let algod_client_for_asset = algod_client.clone();
        let asset_manager = AssetManager::new(algod_client_for_asset.clone(), new_group.clone());
        let app_manager = AppManager::new(algod_client.clone());

        // Create closure for new_group function
        let transaction_sender = TransactionSender::new(
            new_group.clone(),
            asset_manager.clone(),
            app_manager.clone(),
        );
        // Create closure for TransactionCreator
        let transaction_creator = TransactionCreator::new(new_group.clone());

        let app_deployer = AppDeployer::new(
            app_manager.clone(),
            transaction_sender.clone(),
            Some(client_manager.indexer().unwrap()),
        );

        Self {
            client_manager,
            account_manager: account_manager.clone(),
            asset_manager,
            app_manager,
            app_deployer,
            transaction_sender,
            transaction_creator,
            default_composer_config: params.composer_config.clone(),
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
    pub fn new_group(&self, params: Option<TransactionComposerConfig>) -> Composer {
        Composer::new(ComposerParams {
            algod_client: self.client_manager.algod().clone(),
            signer_getter: self.account_manager.clone(),
            composer_config: params.or_else(|| self.default_composer_config.clone()),
        })
    }

    pub fn default_localnet(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: AlgoConfig {
                algod_config: ClientManager::get_default_localnet_config(AlgorandService::Algod),
                indexer_config: Some(ClientManager::get_default_localnet_config(
                    AlgorandService::Indexer,
                )),
                kmd_config: Some(ClientManager::get_default_localnet_config(
                    AlgorandService::Kmd,
                )),
            },
            composer_config: params,
        })
    }

    pub fn testnet(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: AlgoConfig {
                algod_config: ClientManager::get_algonode_config("testnet", AlgorandService::Algod),
                indexer_config: Some(ClientManager::get_algonode_config(
                    "testnet",
                    AlgorandService::Indexer,
                )),
                kmd_config: None,
            },
            composer_config: params,
        })
    }

    pub fn mainnet(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: AlgoConfig {
                algod_config: ClientManager::get_algonode_config("mainnet", AlgorandService::Algod),
                indexer_config: Some(ClientManager::get_algonode_config(
                    "mainnet",
                    AlgorandService::Indexer,
                )),
                kmd_config: None,
            },
            composer_config: params,
        })
    }

    pub fn from_environment(params: Option<TransactionComposerConfig>) -> Self {
        Self::new(&AlgorandClientParams {
            client_config: ClientManager::get_config_from_environment_or_localnet(),
            composer_config: params,
        })
    }

    pub fn set_signer(&mut self, sender: Address, signer: Arc<dyn TransactionSigner>) {
        self.account_manager
            .lock()
            .unwrap()
            .set_signer(sender, signer);
    }

    /// Get a clone of the persistent AppDeployer (shares cache across clones)
    pub fn app_deployer(&self) -> AppDeployer {
        self.app_deployer.clone()
    }
}
