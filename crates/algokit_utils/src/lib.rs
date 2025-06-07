use std::sync::Arc;

use algokit_http_client_trait::{HttpClient, HttpError};
use algokit_transact::AlgorandMsgpack;
use algokit_transact::Transaction;

#[cfg(feature = "default_http_client")]
use async_trait::async_trait;

#[cfg(feature = "default_http_client")]
use reqwest;

#[cfg(feature = "default_http_client")]
struct DefaultHttpClient {
    host: String,
}

#[cfg(feature = "default_http_client")]
impl DefaultHttpClient {
    pub fn new(host: &str) -> Self {
        DefaultHttpClient {
            host: host.to_string(),
        }
    }
}

#[cfg(feature = "default_http_client")]
#[async_trait]
impl HttpClient for DefaultHttpClient {
    async fn json(&self, path: String) -> Result<String, HttpError> {
        let response = reqwest::get(self.host.clone() + &path)
            .await
            .map_err(|e| HttpError::HttpError(e.to_string()))?
            .text()
            .await
            .map_err(|e| HttpError::HttpError(e.to_string()))?;

        Ok(response)
    }
}

pub struct AlgodClient {
    http_client: Arc<dyn HttpClient>,
}

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

    pub async fn get_suggested_params(&self) -> Result<String, HttpError> {
        let path = "/v2/transactions/params".to_string();
        self.http_client.json(path).await
    }
}

pub struct Composer {
    transactions: Vec<Transaction>,
    algod_client: AlgodClient,
}

impl Composer {
    pub fn new(algod_client: AlgodClient) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: algod_client,
        }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if self.transactions.len() >= 16 {
            return Err("Cannot add more than 16 transactions to a Composer".to_string());
        }
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<Vec<u8>>, String> {
        self.transactions
            .iter()
            .map(|tx| tx.encode())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }

    pub fn transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub async fn get_suggested_params(&self) -> Result<String, HttpError> {
        Ok(self.algod_client.get_suggested_params().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TransactionMother;
    use tokio;

    #[test]
    fn test_add_transaction() {
        let mut composer = Composer::testnet();
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_ok());
    }

    #[test]
    fn test_add_too_many_transactions() {
        let mut composer = Composer::testnet();
        for _ in 0..16 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_err());
    }

    #[test]
    fn test_encode_transactions() {
        let mut composer = Composer::testnet();
        for _ in 0..5 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let encoded = composer.encode();
        assert!(encoded.is_ok());
        assert_eq!(encoded.unwrap().len(), 5);
    }

    #[tokio::test]
    async fn test_get_suggested_params() {
        let composer = Composer::testnet();
        let response = composer.get_suggested_params().await.unwrap();

        assert!(
            response.contains(r#""genesis-hash":"SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=""#)
        )
    }
}
