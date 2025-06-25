use std::sync::Arc;

use algokit_http_client::{HttpClient, HttpError};

#[cfg(feature = "default_http_client")]
use algokit_http_client::DefaultHttpClient;

use serde::{Deserialize, Serialize};

pub struct AlgodClient {
    http_client: Arc<dyn HttpClient>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TransactionParams {
    pub consensus_version: String,
    pub fee: u64,
    pub last_round: u64,
    pub genesis_id: String,
    pub genesis_hash: String,
    pub min_fee: u64,
}

/// A temporary AlgodClient until the proper client is generated.
/// The exepectation is that this client will use a HttpClient to make requests to the Algorand API
impl AlgodClient {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        AlgodClient { http_client }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        AlgodClient {
            http_client: Arc::new(DefaultHttpClient::new(
                "https://testnet-api.4160.nodely.dev",
            )),
        }
    }

    pub async fn transaction_params(&self) -> Result<TransactionParams, HttpError> {
        let path = "/v2/transactions/params".to_string();
        let response = self.http_client.get(path).await?;

        serde_json::from_slice(&response).map_err(|e| HttpError::HttpError(e.to_string()))
    }
}
