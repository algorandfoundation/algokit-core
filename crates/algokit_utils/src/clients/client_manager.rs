use crate::AlgorandClient;
use crate::applications::app_client::{
    AppClient, AppClientError, AppClientParams, AppSourceMaps, CompilationParams,
};
use crate::applications::app_factory::{AppFactory, AppFactoryParams};
use crate::clients::network_client::{
    AlgoClientConfig, AlgoConfig, AlgorandService, NetworkDetails, TokenHeader,
    genesis_id_is_localnet,
};
use crate::transactions::{TransactionComposerConfig, TransactionSigner};
use algod_client::{AlgodClient, apis::Error as AlgodError};
use algokit_abi::Arc56Contract;
use algokit_http_client::DefaultHttpClient;
use base64::{Engine, engine::general_purpose};
use indexer_client::IndexerClient;
use kmd_client::KmdClient;
use snafu::Snafu;
use std::{env, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Snafu)]
pub enum ClientManagerError {
    #[snafu(display("Environment Error: {message}"))]
    EnvironmentError { message: String },

    #[snafu(display("Http Client Error: {message}"))]
    HttpClientError { message: String },

    #[snafu(display("Indexer Error: {message}"))]
    IndexerError { message: String },

    #[snafu(display("KMD Error: {message}"))]
    KmdError { message: String },

    #[snafu(display("Algod client error: {source}"))]
    AlgodClientError { source: AlgodError },
}

impl From<AlgodError> for ClientManagerError {
    fn from(e: AlgodError) -> Self {
        Self::AlgodClientError { source: e }
    }
}

pub struct ClientManager {
    algod: Arc<AlgodClient>,
    indexer: Option<Arc<IndexerClient>>,
    kmd: Option<Arc<KmdClient>>,
    cached_network_details: RwLock<Option<Arc<NetworkDetails>>>,
}

impl ClientManager {
    pub fn new(config: &AlgoConfig) -> Result<Self, ClientManagerError> {
        Ok(Self {
            algod: Arc::new(Self::get_algod_client(&config.algod_config)?),
            indexer: match config.indexer_config.as_ref() {
                Some(indexer_config) => Some(Arc::new(Self::get_indexer_client(indexer_config)?)),
                None => None,
            },
            kmd: match config.kmd_config.as_ref() {
                Some(kmd_config) => Some(Arc::new(Self::get_kmd_client(kmd_config)?)),
                None => None,
            },
            cached_network_details: RwLock::new(None),
        })
    }

    pub fn algod(&self) -> Arc<AlgodClient> {
        Arc::clone(&self.algod)
    }

    pub fn indexer(&self) -> Result<Arc<IndexerClient>, ClientManagerError> {
        self.indexer
            .as_ref()
            .map(Arc::clone)
            .ok_or(ClientManagerError::IndexerError {
                message: "Indexer client not configured".to_string(),
            })
    }

    pub fn indexer_if_present(&self) -> Option<Arc<IndexerClient>> {
        self.indexer.as_ref().map(Arc::clone)
    }

    pub fn kmd(&self) -> Result<Arc<KmdClient>, ClientManagerError> {
        self.kmd
            .as_ref()
            .map(Arc::clone)
            .ok_or(ClientManagerError::KmdError {
                message: "KMD client not configured".to_string(),
            })
    }

    pub fn kmd_if_present(&self) -> Option<Arc<KmdClient>> {
        self.kmd.as_ref().map(Arc::clone)
    }

    pub async fn network(&self) -> Result<Arc<NetworkDetails>, ClientManagerError> {
        // Fast path: multiple readers can access concurrently
        {
            let cached = self.cached_network_details.read().await;
            if let Some(ref details) = *cached {
                return Ok(Arc::clone(details));
            }
        }

        // Slow path: exclusive write access for initialization
        let mut cached = self.cached_network_details.write().await;

        // Double-check: someone else might have initialized while we waited for write lock
        if let Some(ref details) = *cached {
            return Ok(Arc::clone(details));
        }

        // Initialize - errors are NOT cached, allowing retries for transient failures
        let params = self.algod().transaction_params().await?;
        let network_details = Arc::new(NetworkDetails::new(
            params.genesis_id.clone(),
            general_purpose::STANDARD.encode(&params.genesis_hash),
        ));

        // Cache only on success
        *cached = Some(Arc::clone(&network_details));
        Ok(network_details)
    }

    pub fn genesis_id_is_localnet(genesis_id: &str) -> bool {
        genesis_id_is_localnet(genesis_id)
    }

    pub async fn is_localnet(&self) -> Result<bool, ClientManagerError> {
        Ok(self.network().await?.is_localnet)
    }

    pub async fn is_testnet(&self) -> Result<bool, ClientManagerError> {
        Ok(self.network().await?.is_testnet)
    }

    pub async fn is_mainnet(&self) -> Result<bool, ClientManagerError> {
        Ok(self.network().await?.is_mainnet)
    }

    pub fn get_config_from_environment_or_localnet() -> AlgoConfig {
        match Self::get_algod_config_from_environment() {
            Ok(algod_config) => {
                let kmd_config = if !algod_config.server.contains("mainnet")
                    && !algod_config.server.contains("testnet")
                {
                    Some(AlgoClientConfig {
                        port: env::var("KMD_PORT")
                            .ok()
                            .and_then(|p| p.parse().ok())
                            .or(Some(4002)),
                        ..algod_config.clone()
                    })
                } else {
                    None
                };

                AlgoConfig {
                    algod_config,
                    indexer_config: match Self::get_indexer_config_from_environment() {
                        Ok(indexer_config) => Some(indexer_config),
                        Err(_) => None,
                    },
                    kmd_config,
                }
            }
            Err(_) => AlgoConfig {
                algod_config: Self::get_default_localnet_config(AlgorandService::Algod),
                indexer_config: Some(Self::get_default_localnet_config(AlgorandService::Indexer)),
                kmd_config: Some(Self::get_default_localnet_config(AlgorandService::Kmd)),
            },
        }
    }

    pub fn get_indexer_config_from_environment() -> Result<AlgoClientConfig, ClientManagerError> {
        let server =
            env::var("INDEXER_SERVER").map_err(|_| ClientManagerError::EnvironmentError {
                message: String::from("INDEXER_SERVER environment variable not found"),
            })?;
        let port = env::var("INDEXER_PORT").ok().and_then(|p| p.parse().ok());
        let token = env::var("INDEXER_TOKEN").ok().map(TokenHeader::String);

        Ok(AlgoClientConfig {
            server,
            port,
            token,
        })
    }

    pub fn get_algod_config_from_environment() -> Result<AlgoClientConfig, ClientManagerError> {
        let server =
            env::var("ALGOD_SERVER").map_err(|_| ClientManagerError::EnvironmentError {
                message: String::from("ALGOD_SERVER environment variable not found"),
            })?;
        let port = env::var("ALGOD_PORT").ok().and_then(|p| p.parse().ok());
        let token = env::var("ALGOD_TOKEN").ok().map(TokenHeader::String);

        Ok(AlgoClientConfig {
            server,
            port,
            token,
        })
    }

    pub fn get_kmd_config_from_environment() -> Result<AlgoClientConfig, ClientManagerError> {
        let server = env::var("KMD_SERVER")
            .or_else(|_| env::var("ALGOD_SERVER"))
            .map_err(|_| ClientManagerError::EnvironmentError {
                message: String::from("KMD_SERVER environment variable not found"),
            })?;

        let port = env::var("KMD_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .or_else(|| env::var("ALGOD_PORT").ok().and_then(|p| p.parse().ok()))
            .or(Some(4002));

        let token = env::var("KMD_TOKEN")
            .ok()
            .map(TokenHeader::String)
            .or_else(|| env::var("ALGOD_TOKEN").ok().map(TokenHeader::String));

        Ok(AlgoClientConfig {
            server,
            port,
            token,
        })
    }

    pub fn get_algonode_config(network: &str, service: AlgorandService) -> AlgoClientConfig {
        let subdomain = match service {
            AlgorandService::Algod => "api",
            AlgorandService::Indexer => "idx",
            AlgorandService::Kmd => panic!("KMD is not available on algonode"),
        };

        AlgoClientConfig {
            server: format!("https://{}-{}.4160.nodely.dev", network, subdomain),
            port: Some(443),
            token: None,
        }
    }

    pub fn get_default_localnet_config(service: AlgorandService) -> AlgoClientConfig {
        let port = match service {
            AlgorandService::Algod => 4001,
            AlgorandService::Indexer => 8980,
            AlgorandService::Kmd => 4002,
        };

        AlgoClientConfig {
            server: "http://localhost".to_string(),
            port: Some(port),
            token: Some(TokenHeader::String(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            )),
        }
    }

    pub fn get_algod_client(config: &AlgoClientConfig) -> Result<AlgodClient, ClientManagerError> {
        let base_url = if let Some(port) = config.port {
            format!("{}:{}", config.server, port)
        } else {
            config.server.clone()
        };

        let http_client = match &config.token {
            Some(TokenHeader::String(token)) => Arc::new(
                DefaultHttpClient::with_header(&base_url, "X-Algo-API-Token", token).map_err(
                    |e| ClientManagerError::HttpClientError {
                        message: format!("Failed to create HTTP client with token header: {:?}", e),
                    },
                )?,
            ),
            Some(TokenHeader::Headers(headers)) => {
                let (header_name, header_value) = headers
                    .iter()
                    .next()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .unwrap_or(("X-Algo-API-Token", ""));
                Arc::new(
                    DefaultHttpClient::with_header(&base_url, header_name, header_value).map_err(
                        |e| ClientManagerError::HttpClientError {
                            message: format!(
                                "Failed to create HTTP client with custom header: {:?}",
                                e
                            ),
                        },
                    )?,
                )
            }
            None => Arc::new(DefaultHttpClient::new(&base_url)),
        };

        Ok(AlgodClient::new(http_client))
    }

    pub fn get_algod_client_from_environment() -> Result<AlgodClient, ClientManagerError> {
        let config = Self::get_algod_config_from_environment()?;
        Self::get_algod_client(&config)
    }

    pub fn get_indexer_client(
        config: &AlgoClientConfig,
    ) -> Result<IndexerClient, ClientManagerError> {
        let base_url = if let Some(port) = config.port {
            format!("{}:{}", config.server, port)
        } else {
            config.server.clone()
        };

        let http_client = match &config.token {
            Some(TokenHeader::String(token)) => Arc::new(
                DefaultHttpClient::with_header(&base_url, "X-Indexer-API-Token", token).map_err(
                    |e| ClientManagerError::HttpClientError {
                        message: format!("Failed to create HTTP client with token header: {:?}", e),
                    },
                )?,
            ),
            Some(TokenHeader::Headers(headers)) => {
                let (header_name, header_value) = headers
                    .iter()
                    .next()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .unwrap_or(("X-Indexer-API-Token", ""));
                Arc::new(
                    DefaultHttpClient::with_header(&base_url, header_name, header_value).map_err(
                        |e| ClientManagerError::HttpClientError {
                            message: format!(
                                "Failed to create HTTP client with custom header: {:?}",
                                e
                            ),
                        },
                    )?,
                )
            }
            None => Arc::new(DefaultHttpClient::new(&base_url)),
        };

        Ok(IndexerClient::new(http_client))
    }

    pub fn get_indexer_client_from_environment() -> Result<IndexerClient, ClientManagerError> {
        let config = Self::get_indexer_config_from_environment()?;
        Self::get_indexer_client(&config)
    }

    pub fn get_kmd_client(config: &AlgoClientConfig) -> Result<KmdClient, ClientManagerError> {
        let base_url = if let Some(port) = config.port {
            format!("{}:{}", config.server, port)
        } else {
            config.server.clone()
        };

        let token_value = match &config.token {
            Some(TokenHeader::String(token)) => token.clone(),
            Some(TokenHeader::Headers(headers)) => {
                headers.values().next().cloned().unwrap_or_default()
            }
            None => String::new(),
        };

        let http_client = if token_value.is_empty() {
            Arc::new(DefaultHttpClient::new(&base_url))
        } else {
            Arc::new(
                DefaultHttpClient::with_header(&base_url, "X-KMD-API-Token", &token_value)
                    .map_err(|e| ClientManagerError::HttpClientError {
                        message: format!(
                            "Failed to create HTTP client with KMD token header: {:?}",
                            e
                        ),
                    })?,
            )
        };

        Ok(KmdClient::new(http_client))
    }

    pub fn get_kmd_client_from_environment() -> Result<KmdClient, ClientManagerError> {
        let config = Self::get_kmd_config_from_environment()?;
        Self::get_kmd_client(&config)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_app_factory(
        &self,
        algorand: Arc<AlgorandClient>,
        app_spec: Arc56Contract,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        version: Option<String>,
        compilation_params: Option<CompilationParams>,
        source_maps: Option<AppSourceMaps>,
        transaction_composer_config: Option<TransactionComposerConfig>,
    ) -> AppFactory {
        AppFactory::new(AppFactoryParams {
            algorand,
            app_spec,
            app_name,
            default_sender,
            default_signer,
            version,
            compilation_params,
            source_maps,
            transaction_composer_config,
        })
    }

    /// Returns an AppClient resolved by creator address and name using indexer lookup.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_app_client_by_creator_and_name(
        &self,
        algorand: Arc<AlgorandClient>,
        creator_address: &str,
        app_name: &str,
        app_spec: Arc56Contract,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
        ignore_cache: Option<bool>,
        transaction_composer_config: Option<TransactionComposerConfig>,
    ) -> Result<AppClient, AppClientError> {
        AppClient::from_creator_and_name(
            creator_address,
            app_name,
            app_spec,
            algorand,
            default_sender,
            default_signer,
            source_maps,
            ignore_cache,
            transaction_composer_config,
        )
        .await
    }

    /// Returns an AppClient for an existing application by ID.
    #[allow(clippy::too_many_arguments)]
    pub fn get_app_client_by_id(
        &self,
        algorand: Arc<AlgorandClient>,
        app_spec: Arc56Contract,
        app_id: u64,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
        transaction_composer_config: Option<TransactionComposerConfig>,
    ) -> AppClient {
        AppClient::new(AppClientParams {
            app_id,
            app_spec,
            algorand,
            app_name,
            default_sender,
            default_signer,
            source_maps,
            transaction_composer_config,
        })
    }

    /// Returns an AppClient resolved by network using app spec networks mapping.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_app_client_by_network(
        &self,
        algorand: Arc<AlgorandClient>,
        app_spec: Arc56Contract,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
        transaction_composer_config: Option<TransactionComposerConfig>,
    ) -> Result<AppClient, AppClientError> {
        AppClient::from_network(
            app_spec,
            algorand,
            app_name,
            default_sender,
            default_signer,
            source_maps,
            transaction_composer_config,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::network_client::AlgorandService;
    use rstest::rstest;

    #[test]
    fn test_cache_initially_empty() {
        let config = AlgoConfig {
            algod_config: AlgoClientConfig {
                server: "http://localhost:4001".to_string(),
                port: None,
                token: None,
            },
            indexer_config: Some(AlgoClientConfig {
                server: "http://localhost:8980".to_string(),
                port: None,
                token: None,
            }),
            kmd_config: None,
        };
        let manager = ClientManager::new(&config).unwrap();

        // Cache should be initially empty
        let cache = manager.cached_network_details.try_read().unwrap();
        assert!(cache.is_none());
    }

    #[tokio::test]
    async fn test_error_not_cached() {
        let config = AlgoConfig {
            algod_config: AlgoClientConfig {
                server: "http://invalid-host:65534".to_string(),
                port: Some(65534),
                token: None,
            },
            indexer_config: Some(AlgoClientConfig {
                server: "http://invalid-host:65535".to_string(),
                port: Some(65535),
                token: None,
            }),
            kmd_config: None,
        };
        let manager = ClientManager::new(&config).unwrap();

        // Both calls should fail
        assert!(manager.network().await.is_err());
        assert!(manager.network().await.is_err());

        // Cache should remain empty after errors
        let cache = manager.cached_network_details.read().await;
        assert!(cache.is_none());
    }

    #[test]
    fn test_client_config_builder() {
        let config = AlgoClientConfig {
            server: "http://localhost".to_string(),
            port: Some(4001),
            token: Some(TokenHeader::String("test-token".to_string())),
        };

        assert_eq!(config.server, "http://localhost");
        assert_eq!(config.port, Some(4001));
        assert!(matches!(config.token, Some(TokenHeader::String(_))));
    }

    #[rstest]
    #[case(
        "mainnet",
        AlgorandService::Algod,
        "https://mainnet-api.4160.nodely.dev"
    )]
    #[case(
        "mainnet",
        AlgorandService::Indexer,
        "https://mainnet-idx.4160.nodely.dev"
    )]
    #[case(
        "testnet",
        AlgorandService::Algod,
        "https://testnet-api.4160.nodely.dev"
    )]
    #[case(
        "testnet",
        AlgorandService::Indexer,
        "https://testnet-idx.4160.nodely.dev"
    )]
    fn test_algonode_config_variations(
        #[case] network: &str,
        #[case] service: AlgorandService,
        #[case] expected_server: &str,
    ) {
        let config = ClientManager::get_algonode_config(network, service);

        assert_eq!(config.server, expected_server);
        assert_eq!(config.port, Some(443));
        assert!(config.token.is_none());
    }

    #[rstest]
    #[case("mainnet")]
    #[case("testnet")]
    #[should_panic(expected = "KMD is not available on algonode")]
    fn test_algonode_config_panics_for_kmd(#[case] network: &str) {
        ClientManager::get_algonode_config(network, AlgorandService::Kmd);
    }

    #[test]
    fn test_localnet_config() {
        let config = ClientManager::get_default_localnet_config(AlgorandService::Algod);
        assert_eq!(config.server, "http://localhost");
        assert_eq!(config.port, Some(4001));
        assert!(config.token.is_some());
    }

    #[test]
    fn test_genesis_id_localnet_detection() {
        assert!(ClientManager::genesis_id_is_localnet("devnet-v1"));
        assert!(ClientManager::genesis_id_is_localnet("sandnet-v1"));
        assert!(ClientManager::genesis_id_is_localnet("dockernet-v1"));
        assert!(!ClientManager::genesis_id_is_localnet("testnet-v1.0"));
        assert!(!ClientManager::genesis_id_is_localnet("mainnet-v1.0"));
    }

    #[test]
    fn test_kmd_optional_accessors_when_configured() {
        let config = AlgoConfig {
            algod_config: AlgoClientConfig {
                server: "http://localhost".to_string(),
                port: Some(4001),
                token: None,
            },
            indexer_config: None,
            kmd_config: Some(AlgoClientConfig {
                server: "http://localhost".to_string(),
                port: Some(4002),
                token: Some(TokenHeader::String("kmd-token".to_string())),
            }),
        };

        let manager = ClientManager::new(&config).unwrap();
        assert!(manager.kmd_if_present().is_some());
        assert!(manager.kmd().is_ok());
    }

    #[test]
    fn test_kmd_optional_accessors_when_missing() {
        let config = AlgoConfig {
            algod_config: AlgoClientConfig {
                server: "http://localhost".to_string(),
                port: Some(4001),
                token: None,
            },
            indexer_config: None,
            kmd_config: None,
        };

        let manager = ClientManager::new(&config).unwrap();
        assert!(matches!(
            manager.kmd(),
            Err(ClientManagerError::KmdError { .. })
        ));
        assert!(manager.kmd_if_present().is_none());
    }
}
