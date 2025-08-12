use crate::clients::network_client::{
    AlgoClientConfig, AlgoConfig, AlgorandService, NetworkDetails, TokenHeader,
    genesis_id_is_localnet,
};
use algod_client::AlgodClient;
use algokit_http_client::DefaultHttpClient;
use base64::{Engine, engine::general_purpose};
use indexer_client::IndexerClient;
use std::{env, sync::Arc};
use tokio::sync::RwLock;

pub struct ClientManager {
    algod: AlgodClient,
    indexer: IndexerClient,
    cached_network_details: RwLock<Option<Arc<NetworkDetails>>>,
}

impl ClientManager {
    pub fn new(config: AlgoConfig) -> Self {
        Self {
            algod: Self::get_algod_client(&config.algod_config),
            indexer: Self::get_indexer_client(&config.indexer_config),
            cached_network_details: RwLock::new(None),
        }
    }

    pub fn algod(&self) -> &AlgodClient {
        &self.algod
    }

    pub fn indexer(&self) -> &IndexerClient {
        &self.indexer
    }

    pub async fn network(
        &self,
    ) -> Result<Arc<NetworkDetails>, Box<dyn std::error::Error + Send + Sync>> {
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

    pub async fn is_localnet(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.network().await?.is_localnet)
    }

    pub async fn is_testnet(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.network().await?.is_testnet)
    }

    pub async fn is_mainnet(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.network().await?.is_mainnet)
    }

    pub fn get_config_from_environment_or_localnet() -> AlgoConfig {
        match env::var("ALGOD_SERVER") {
            Ok(_) => AlgoConfig {
                algod_config: Self::get_algod_config_from_environment(),
                indexer_config: Self::get_indexer_config_from_environment(),
            },
            Err(_) => AlgoConfig {
                algod_config: Self::get_default_localnet_config(AlgorandService::Algod),
                indexer_config: Self::get_default_localnet_config(AlgorandService::Indexer),
            },
        }
    }

    pub fn get_indexer_config_from_environment() -> AlgoClientConfig {
        let server = env::var("INDEXER_SERVER").unwrap_or_else(|_| "http://localhost".to_string());
        let port = env::var("INDEXER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .or(Some(8980));
        let token = env::var("INDEXER_TOKEN").ok().map(TokenHeader::String);

        AlgoClientConfig {
            server,
            port,
            token,
        }
    }

    pub fn get_algod_config_from_environment() -> AlgoClientConfig {
        let server =
            env::var("ALGOD_SERVER").expect("ALGOD_SERVER environment variable must be defined");
        let port = env::var("ALGOD_PORT").ok().and_then(|p| p.parse().ok());
        let token = env::var("ALGOD_TOKEN").ok().map(TokenHeader::String);

        AlgoClientConfig {
            server,
            port,
            token,
        }
    }

    pub fn get_algonode_config(network: &str, service: AlgorandService) -> AlgoClientConfig {
        let subdomain = match service {
            AlgorandService::Algod => "api",
            AlgorandService::Indexer => "idx",
            AlgorandService::Kmd => panic!("KMD is not available on algonode"),
        };

        AlgoClientConfig {
            server: format!("https://{}-{}.algonode.cloud/", network, subdomain),
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

    pub fn get_algod_client(config: &AlgoClientConfig) -> AlgodClient {
        let base_url = if let Some(port) = config.port {
            format!("{}:{}", config.server, port)
        } else {
            config.server.clone()
        };

        let http_client = match &config.token {
            Some(TokenHeader::String(token)) => Arc::new(
                DefaultHttpClient::with_header(&base_url, "X-Algo-API-Token", token)
                    .expect("Failed to create HTTP client with token header"),
            ),
            Some(TokenHeader::Headers(headers)) => {
                let (header_name, header_value) = headers
                    .iter()
                    .next()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .unwrap_or(("X-Algo-API-Token", ""));
                Arc::new(
                    DefaultHttpClient::with_header(&base_url, header_name, header_value)
                        .expect("Failed to create HTTP client with custom header"),
                )
            }
            None => Arc::new(DefaultHttpClient::new(&base_url)),
        };

        AlgodClient::new(http_client)
    }

    pub fn get_indexer_client(config: &AlgoClientConfig) -> IndexerClient {
        let base_url = if let Some(port) = config.port {
            format!("{}:{}", config.server, port)
        } else {
            config.server.clone()
        };

        let http_client = match &config.token {
            Some(TokenHeader::String(token)) => Arc::new(
                DefaultHttpClient::with_header(&base_url, "X-Indexer-API-Token", token)
                    .expect("Failed to create HTTP client with token header"),
            ),
            Some(TokenHeader::Headers(headers)) => {
                let (header_name, header_value) = headers
                    .iter()
                    .next()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .unwrap_or(("X-Indexer-API-Token", ""));
                Arc::new(
                    DefaultHttpClient::with_header(&base_url, header_name, header_value)
                        .expect("Failed to create HTTP client with custom header"),
                )
            }
            None => Arc::new(DefaultHttpClient::new(&base_url)),
        };

        IndexerClient::new(http_client)
    }

    pub fn get_indexer_client_from_environment() -> IndexerClient {
        Self::get_indexer_client(&Self::get_indexer_config_from_environment())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::network_client::AlgorandService;

    #[test]
    fn test_cache_initially_empty() {
        let config = AlgoConfig {
            algod_config: AlgoClientConfig {
                server: "http://localhost:4001".to_string(),
                port: None,
                token: None,
            },
            indexer_config: AlgoClientConfig {
                server: "http://localhost:8980".to_string(),
                port: None,
                token: None,
            },
        };
        let manager = ClientManager::new(config);

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
            indexer_config: AlgoClientConfig {
                server: "http://invalid-host:65535".to_string(),
                port: Some(65535),
                token: None,
            },
        };
        let manager = ClientManager::new(config);

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

    #[test]
    fn test_algonode_config() {
        let config = ClientManager::get_algonode_config("testnet", AlgorandService::Algod);
        assert_eq!(config.server, "https://testnet-api.algonode.cloud/");
        assert_eq!(config.port, Some(443));
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
}
