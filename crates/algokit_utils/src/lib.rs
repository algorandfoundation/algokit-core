use algokit_transact::AlgorandMsgpack;
use algokit_transact::Transaction;
use async_trait::async_trait;

use reqwest;

#[async_trait]
pub trait HTTPClient {
    async fn json(&self, path: &str) -> Result<String, String>;
}

// TODO: Put reqwest and this default client behind a feature flag
struct DefaultHTTPClient {
    host: String,
}

impl DefaultHTTPClient {
    pub fn new(host: &str) -> Self {
        DefaultHTTPClient {
            host: host.to_string(),
        }
    }
}

#[async_trait]
impl HTTPClient for DefaultHTTPClient {
    async fn json(&self, path: &str) -> Result<String, String> {
        let response = reqwest::get(self.host.clone() + path)
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }
}

pub struct Composer {
    transactions: Vec<Transaction>,
    algod_client: Box<dyn HTTPClient>,
}

impl Composer {
    pub fn new(algod_client: Box<dyn HTTPClient>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: algod_client,
        }
    }

    pub fn testnet() -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: Box::new(DefaultHTTPClient::new(
                "https://testnet-api.4160.nodely.dev",
            )),
        }
    }

    pub fn set_http_client(&mut self, client: Box<dyn HTTPClient>) {
        self.algod_client = client;
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

    pub async fn get_suggested_params(&self) -> Result<String, String> {
        let path = "/v2/transactions/params";

        Ok(self.algod_client.json(path).await?)
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
