use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TokenHeader {
    String(String),
    Headers(HashMap<String, String>),
}

/// Represents the different Algorand networks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorandNetwork {
    /// Local development network
    LocalNet,
    /// Algorand TestNet
    TestNet,
    /// Algorand MainNet
    MainNet,
}

impl AlgorandNetwork {
    /// Get the string representation of the network
    pub fn as_str(&self) -> &'static str {
        match self {
            AlgorandNetwork::LocalNet => "localnet",
            AlgorandNetwork::TestNet => "testnet",
            AlgorandNetwork::MainNet => "mainnet",
        }
    }

    /// Check if this network is a local development network
    pub fn is_localnet(&self) -> bool {
        matches!(self, AlgorandNetwork::LocalNet)
    }

    /// Check if this network is TestNet
    pub fn is_testnet(&self) -> bool {
        matches!(self, AlgorandNetwork::TestNet)
    }

    /// Check if this network is MainNet
    pub fn is_mainnet(&self) -> bool {
        matches!(self, AlgorandNetwork::MainNet)
    }

    /// Get the expected genesis ID for this network
    pub fn expected_genesis_id(&self) -> Option<&'static str> {
        match self {
            AlgorandNetwork::LocalNet => None, // LocalNet can have various genesis IDs
            AlgorandNetwork::TestNet => Some("testnet-v1.0"),
            AlgorandNetwork::MainNet => Some("mainnet-v1.0"),
        }
    }
}

/// Represents the different Algorand services
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorandService {
    /// Algorand daemon (algod) - provides access to the blockchain
    Algod,
    /// Algorand indexer - provides historical blockchain data
    Indexer,
    /// Key Management Daemon (kmd) - provides key management functionality
    Kmd,
}

impl AlgorandService {
    /// Get the string representation of the service
    pub fn as_str(&self) -> &'static str {
        match self {
            AlgorandService::Algod => "algod",
            AlgorandService::Indexer => "indexer",
            AlgorandService::Kmd => "kmd",
        }
    }
}

/// Config for an Algorand client.
#[derive(Debug, Clone)]
pub struct AlgoClientConfig {
    /// Base URL of the server e.g. http://localhost, https://testnet-api.algonode.cloud/, etc.
    pub server: String,
    /// Optional port to use e.g. 4001, 443, etc.
    pub port: Option<u16>,
    /// Optional token to use for API authentication
    pub token: Option<TokenHeader>,
}

/// Configuration for algod, indexer and kmd clients.
#[derive(Debug, Clone)]
pub struct AlgoConfig {
    /// Algod client configuration
    pub algod_config: AlgoClientConfig,
    /// Indexer client configuration
    pub indexer_config: Option<AlgoClientConfig>,
    /// KMD client configuration
    pub kmd_config: Option<AlgoClientConfig>,
}

impl AlgoConfig {
    pub fn new(
        algod_config: AlgoClientConfig,
        indexer_config: Option<AlgoClientConfig>,
        kmd_config: Option<AlgoClientConfig>,
    ) -> Self {
        Self {
            algod_config,
            indexer_config,
            kmd_config,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkDetails {
    pub is_testnet: bool,
    pub is_mainnet: bool,
    pub is_localnet: bool,
    pub genesis_id: String,
    pub genesis_hash: String,
}

impl NetworkDetails {
    pub fn new(genesis_id: String, genesis_hash: String) -> Self {
        let is_localnet = genesis_id_is_localnet(&genesis_id);
        let is_testnet = genesis_id == "testnet-v1.0";
        let is_mainnet = genesis_id == "mainnet-v1.0";

        Self {
            is_testnet,
            is_mainnet,
            is_localnet,
            genesis_id,
            genesis_hash,
        }
    }
}

pub fn genesis_id_is_localnet(genesis_id: &str) -> bool {
    genesis_id == "devnet-v1" || genesis_id == "sandnet-v1" || genesis_id == "dockernet-v1"
}
