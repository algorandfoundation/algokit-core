use crate::applications::AppDeployer;
use crate::clients::app_manager::{AppState, BoxName};
use crate::clients::network_client::NetworkDetails;
use crate::transactions::{TransactionComposerConfig, TransactionSigner};
use crate::{AlgorandClient, clients::app_manager::BoxIdentifier};
use crate::{SendParams, SendTransactionResult};
use algokit_abi::{ABIType, ABIValue, Arc56Contract};
use algokit_transact::Address;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

/// A box value decoded according to an ABI type
#[derive(Debug, Clone)]
pub struct BoxABIValue {
    pub name: BoxName,
    pub value: ABIValue,
}

/// A box name and its raw value
#[derive(Debug, Clone)]
pub struct BoxValue {
    pub name: BoxName,
    pub value: Vec<u8>,
}
mod compilation;
mod error;
mod error_transformation;
mod params_builder;
mod sender;
mod state_accessor;
mod transaction_builder;
mod types;
mod utils;
pub use error::AppClientError;
use params_builder::ParamsBuilder;
pub use sender::TransactionSender;
pub use state_accessor::StateAccessor;
pub use transaction_builder::TransactionBuilder;
pub use types::{
    AppClientBareCallParams, AppClientMethodCallParams, AppClientParams, AppSourceMaps,
    CompilationParams, FundAppAccountParams,
};

type BoxNameFilter = Box<dyn Fn(&BoxName) -> bool>;

/// A client for interacting with an Algorand smart contract application (ARC-56 focused).
pub struct AppClient {
    app_id: u64,
    app_spec: Arc56Contract,
    algorand: AlgorandClient,
    default_sender: Option<String>,
    default_signer: Option<Arc<dyn TransactionSigner>>,
    source_maps: Option<AppSourceMaps>,
    app_name: Option<String>,
    transaction_composer_config: Option<TransactionComposerConfig>,
}

impl AppClient {
    /// Create a new client from parameters.
    pub fn new(params: AppClientParams) -> Self {
        Self {
            app_id: params.app_id,
            app_spec: params.app_spec,
            algorand: params.algorand,
            default_sender: params.default_sender,
            default_signer: params.default_signer,
            source_maps: params.source_maps,
            app_name: params.app_name,
            transaction_composer_config: params.transaction_composer_config,
        }
    }

    /// Construct from the current network using app_spec.networks mapping.
    ///
    /// Matches on either the network alias ("localnet", "testnet", "mainnet")
    /// or the network's genesis hash present in the node's suggested params.
    pub async fn from_network(
        app_spec: Arc56Contract,
        algorand: AlgorandClient,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
        transaction_composer_config: Option<TransactionComposerConfig>,
    ) -> Result<Self, AppClientError> {
        let network = algorand
            .client()
            .network()
            .await
            .map_err(|e| AppClientError::Network {
                message: e.to_string(),
            })?;

        let candidate_keys = Self::candidate_network_keys(&network);
        let (app_id, available_keys) = match &app_spec.networks {
            Some(nets) => (
                Self::find_app_id_in_networks(&candidate_keys, nets),
                nets.keys().cloned().collect(),
            ),
            None => (None, Vec::new()),
        };

        let app_id = app_id.ok_or_else(|| AppClientError::AppIdNotFound {
            network_names: candidate_keys.clone(),
            available: available_keys,
        })?;

        Ok(Self::new(AppClientParams {
            app_id,
            app_spec,
            algorand,
            app_name,
            default_sender,
            default_signer,
            source_maps,
            transaction_composer_config,
        }))
    }

    /// Construct from creator address and application name via indexer lookup.
    #[allow(clippy::too_many_arguments)]
    pub async fn from_creator_and_name(
        creator_address: &str,
        app_name: &str,
        app_spec: Arc56Contract,
        algorand: AlgorandClient,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
        ignore_cache: Option<bool>,
        transaction_composer_config: Option<TransactionComposerConfig>,
    ) -> Result<Self, AppClientError> {
        let address = Address::from_str(creator_address).map_err(|e| AppClientError::Lookup {
            message: format!("Invalid creator address: {}", e),
        })?;

        let indexer_client = algorand
            .client()
            .indexer()
            .map_err(|e| AppClientError::ClientManagerError { source: e })?;
        let mut app_deployer = AppDeployer::new(
            algorand.app().clone(),
            algorand.send().clone(),
            Some(indexer_client),
        );

        let lookup = app_deployer
            .get_creator_apps_by_name(&address, ignore_cache)
            .await
            .map_err(|e| AppClientError::Lookup {
                message: e.to_string(),
            })?;

        let app_metadata = lookup
            .apps
            .get(app_name)
            .ok_or_else(|| AppClientError::Lookup {
                message: format!(
                    "App not found for creator {} and name {}",
                    creator_address, app_name
                ),
            })?;

        Ok(Self::new(AppClientParams {
            app_id: app_metadata.app_id,
            app_spec,
            algorand,
            app_name: Some(app_name.to_string()),
            default_sender,
            default_signer,
            source_maps,
            transaction_composer_config,
        }))
    }

    fn candidate_network_keys(network: &NetworkDetails) -> Vec<String> {
        let mut names = vec![network.genesis_hash.clone()];
        if network.is_localnet {
            names.push("localnet".to_string());
        }
        if network.is_mainnet {
            names.push("mainnet".to_string());
        }
        if network.is_testnet {
            names.push("testnet".to_string());
        }
        names
    }

    fn find_app_id_in_networks(
        candidate_keys: &[String],
        networks: &HashMap<String, algokit_abi::arc56_contract::Network>,
    ) -> Option<u64> {
        for key in candidate_keys {
            if let Some(net) = networks.get(key) {
                return Some(net.app_id);
            }
        }
        None
    }

    /// Get the application ID.
    pub fn app_id(&self) -> u64 {
        self.app_id
    }
    /// Get the ARC-56 application specification.
    pub fn app_spec(&self) -> &Arc56Contract {
        &self.app_spec
    }
    /// Get the Algorand client instance.
    pub fn algorand(&self) -> &AlgorandClient {
        &self.algorand
    }
    /// Get the application name if configured.
    pub fn app_name(&self) -> Option<&String> {
        self.app_name.as_ref()
    }
    /// Get the default sender address if configured.
    pub fn default_sender(&self) -> Option<&String> {
        self.default_sender.as_ref()
    }

    /// Get the application's account address.
    pub fn app_address(&self) -> Address {
        Address::from_app_id(&self.app_id)
    }

    fn get_sender_address(&self, sender: &Option<String>) -> Result<Address, AppClientError> {
        let sender_str = sender
            .as_ref()
            .or(self.default_sender.as_ref())
            .ok_or_else(|| AppClientError::ValidationError {
                message: format!(
                    "No sender provided and no default sender configured for app {}",
                    self.app_name.as_deref().unwrap_or("<unknown>")
                ),
            })?;
        Address::from_str(sender_str).map_err(|e| AppClientError::ValidationError {
            message: format!("Invalid sender address: {}", e),
        })
    }

    /// Resolve the signer for a transaction based on the sender and default configuration.
    /// Returns the provided signer, or the default_signer if sender matches default_sender.
    pub(crate) fn resolve_signer(
        &self,
        sender: Option<String>,
        signer: Option<Arc<dyn TransactionSigner>>,
    ) -> Option<Arc<dyn TransactionSigner>> {
        signer.or_else(|| {
            let should_use_default = sender.is_none() || sender == self.default_sender;

            should_use_default
                .then(|| self.default_signer.clone())
                .flatten()
        })
    }

    /// Fund the application's account with Algos.
    pub async fn fund_app_account(
        &self,
        params: FundAppAccountParams,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResult, AppClientError> {
        self.send().fund_app_account(params, send_params).await
    }

    /// Get the application's global state.
    pub async fn get_global_state(&self) -> Result<HashMap<Vec<u8>, AppState>, AppClientError> {
        self.algorand
            .app()
            .get_global_state(self.app_id)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })
    }

    /// Get the application's local state for a specific account.
    pub async fn get_local_state(
        &self,
        address: &str,
    ) -> Result<std::collections::HashMap<Vec<u8>, AppState>, AppClientError> {
        self.algorand
            .app()
            .get_local_state(self.app_id, address)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })
    }

    /// Get all box names for the application.
    pub async fn get_box_names(&self) -> Result<Vec<BoxName>, AppClientError> {
        self.algorand
            .app()
            .get_box_names(self.app_id)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })
    }

    /// Get all box values (names and contents) for the application.
    pub async fn get_box_values(&self) -> Result<Vec<BoxValue>, AppClientError> {
        let names = self.get_box_names().await?;
        let mut values = Vec::new();
        for name in names {
            let value = self.get_box_value(&name.name_raw).await?;
            values.push(BoxValue { name, value });
        }
        Ok(values)
    }

    /// Get the raw value of a specific box.
    pub async fn get_box_value(&self, name: &BoxIdentifier) -> Result<Vec<u8>, AppClientError> {
        self.algorand
            .app()
            .get_box_value(self.app_id, name)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })
    }

    /// Get a single box value decoded according to an ABI type.
    pub async fn get_box_value_from_abi_type(
        &self,
        name: &BoxIdentifier,
        abi_type: &ABIType,
    ) -> Result<ABIValue, AppClientError> {
        self.algorand
            .app()
            .get_box_value_from_abi_type(self.app_id, name, abi_type)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })
    }

    /// Get multiple box values decoded according to an ABI type.
    pub async fn get_box_values_from_abi_type(
        &self,
        abi_type: &ABIType,
        filter_func: Option<BoxNameFilter>,
    ) -> Result<Vec<BoxABIValue>, AppClientError> {
        let names = self.get_box_names().await?;
        let filtered_names = if let Some(filter) = filter_func {
            names.into_iter().filter(|name| filter(name)).collect()
        } else {
            names
        };

        let box_names: Vec<BoxIdentifier> = filtered_names
            .iter()
            .map(|name| name.name_raw.clone())
            .collect();

        let values = self
            .algorand
            .app()
            .get_box_values_from_abi_type(self.app_id, &box_names, abi_type)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })?;

        Ok(filtered_names
            .into_iter()
            .zip(values.into_iter())
            .map(|(name, value)| BoxABIValue { name, value })
            .collect())
    }

    /// Get a parameter builder for creating transaction parameters.
    pub fn params(&self) -> ParamsBuilder<'_> {
        ParamsBuilder { client: self }
    }
    /// Get a transaction builder for creating unsigned transactions.
    pub fn create_transaction(&self) -> TransactionBuilder<'_> {
        TransactionBuilder { client: self }
    }
    /// Get a transaction sender for executing transactions.
    pub fn send(&self) -> TransactionSender<'_> {
        TransactionSender { client: self }
    }
    /// Get a state accessor for reading application state with ABI decoding.
    pub fn state(&self) -> StateAccessor<'_> {
        StateAccessor::new(self)
    }
}
