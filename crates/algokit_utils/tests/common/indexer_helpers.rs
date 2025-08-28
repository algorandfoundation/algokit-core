use indexer_client::{IndexerClient, apis::Error as IndexerError};
use snafu::Snafu;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for indexer wait operations
#[derive(Debug, Clone)]
pub struct IndexerWaitConfig {
    /// Maximum number of retry attempts (default: 100)
    pub max_attempts: u32,
    /// Delay between retry attempts (default: 200ms)
    pub retry_delay: Duration,
}

impl Default for IndexerWaitConfig {
    fn default() -> Self {
        Self {
            max_attempts: 100,
            retry_delay: Duration::from_millis(200),
        }
    }
}

/// Error types for indexer wait operations
#[derive(Debug, Snafu)]
pub enum IndexerWaitError {
    #[snafu(display("Indexer operation failed after {attempts} attempts: {last_error}"))]
    MaxAttemptsExceeded { attempts: u32, last_error: String },
    #[snafu(display("Indexer client error: {message}"))]
    ClientError { message: String },
    #[snafu(display("Transaction {tx_id} not found after {attempts} attempts"))]
    TransactionNotFound { tx_id: String, attempts: u32 },
}

/// Runs the given indexer operation until it succeeds or max attempts are reached.
pub async fn wait_for_indexer<F, Fut, T, E>(
    operation: F,
    config: Option<IndexerWaitConfig>,
) -> Result<T, IndexerWaitError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let config = config.unwrap_or_default();
    let mut last_error = String::new();

    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                last_error = format!("{:?}", err);

                // Check if this looks like a 404 error (indexer hasn't caught up)
                let is_not_found = last_error.contains("404")
                    || last_error.contains("not found")
                    || last_error.contains("NotFound");

                // If it's not a 404-like error, fail immediately
                if !is_not_found {
                    return Err(IndexerWaitError::ClientError {
                        message: last_error,
                    });
                }

                // If we've reached max attempts, break out of the loop
                if attempt >= config.max_attempts {
                    break;
                }

                // Wait before next attempt
                sleep(config.retry_delay).await;
            }
        }
    }

    Err(IndexerWaitError::MaxAttemptsExceeded {
        attempts: config.max_attempts,
        last_error,
    })
}

/// Waits for a specific transaction to appear in the indexer.
pub async fn wait_for_indexer_transaction(
    indexer_client: &IndexerClient,
    tx_id: &str,
    config: Option<IndexerWaitConfig>,
) -> Result<(), IndexerWaitError> {
    let config = config.unwrap_or_default();
    let tx_id = tx_id.to_string();

    wait_for_indexer(
        || {
            let client = indexer_client.clone();
            let tx_id = tx_id.clone();

            Box::pin(async move {
                client
                    .search_for_transactions(
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(&tx_id),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .await
                    .and_then(|response| {
                        if response.transactions.is_empty() {
                            // Return a string error that will be treated as "not found"
                            Err(IndexerError::Serde {
                                message: "Transaction not found".to_string(),
                            })
                        } else {
                            Ok(())
                        }
                    })
            })
        },
        Some(config.clone()),
    )
    .await
    .map_err(|err| match err {
        IndexerWaitError::MaxAttemptsExceeded { attempts, .. } => {
            IndexerWaitError::TransactionNotFound { tx_id, attempts }
        }
        other => other,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn succeeds_immediately() {
        let result = wait_for_indexer(|| async { Ok::<(), String>(()) }, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn retries_until_success() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        let config = IndexerWaitConfig {
            max_attempts: 5,
            retry_delay: Duration::from_millis(1),
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = wait_for_indexer(
            move || {
                let count = attempts_clone.fetch_add(1, Ordering::SeqCst);
                async move { if count < 2 { Err("not found") } else { Ok(()) } }
            },
            Some(config),
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fails_after_max_attempts() {
        let config = IndexerWaitConfig {
            max_attempts: 2,
            retry_delay: Duration::from_millis(1),
        };

        let result =
            wait_for_indexer(|| async { Err::<(), &str>("not found") }, Some(config)).await;

        assert!(matches!(
            result,
            Err(IndexerWaitError::MaxAttemptsExceeded { .. })
        ));
    }

    #[tokio::test]
    async fn fails_immediately_on_non_retriable_error() {
        let result = wait_for_indexer(|| async { Err::<(), &str>("server error") }, None).await;

        assert!(matches!(
            result,
            Err(IndexerWaitError::ClientError { message: _ })
        ));
    }
}
