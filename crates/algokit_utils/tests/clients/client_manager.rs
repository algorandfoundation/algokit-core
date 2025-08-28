use algokit_utils::ClientManager;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::time::timeout;

use crate::common::logging::init_test_logging;

#[tokio::test]
async fn test_network_caching_with_localnet() {
    init_test_logging();

    let config = ClientManager::get_config_from_environment_or_localnet();
    let manager = ClientManager::new(&config);

    let first = manager.network().await.unwrap();
    let second = manager.network().await.unwrap();
    let third = manager.network().await.unwrap();

    // All should be the same Arc instance
    assert!(Arc::ptr_eq(&first, &second));
    assert!(Arc::ptr_eq(&second, &third));

    // Content validation
    assert!(!first.genesis_id.is_empty());
    assert!(!first.genesis_hash.is_empty());
    assert!(first.is_localnet);
}

#[tokio::test]
async fn test_concurrent_network_calls() {
    init_test_logging();

    let config = ClientManager::get_config_from_environment_or_localnet();
    let manager = Arc::new(ClientManager::new(&config));

    // Spawn 10 concurrent tasks
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            let manager = Arc::clone(&manager);
            tokio::spawn(async move {
                // Add timing variation
                if i % 2 == 0 {
                    tokio::time::sleep(Duration::from_millis(i * 5)).await;
                }
                manager.network().await
            })
        })
        .collect();

    let results: Vec<_> = timeout(Duration::from_secs(30), async {
        futures::future::try_join_all(tasks).await.unwrap()
    })
    .await
    .expect("Concurrent calls should complete within timeout");

    let successful: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
    assert!(!successful.is_empty(), "At least some calls should succeed");

    // All successful results should be the same Arc instance
    let unique_ptrs: HashSet<_> = successful.iter().map(Arc::as_ptr).collect();
    assert_eq!(
        unique_ptrs.len(),
        1,
        "All calls should return same cached instance"
    );
}

#[tokio::test]
async fn test_convenience_methods_with_cache() {
    init_test_logging();

    let config = ClientManager::get_config_from_environment_or_localnet();
    let manager = ClientManager::new(&config);

    let network_details = manager.network().await.unwrap();

    // Call convenience methods
    let is_localnet = manager.is_localnet().await.unwrap();
    let is_testnet = manager.is_testnet().await.unwrap();
    let is_mainnet = manager.is_mainnet().await.unwrap();

    // Should match cached values
    assert_eq!(is_localnet, network_details.is_localnet);
    assert_eq!(is_testnet, network_details.is_testnet);
    assert_eq!(is_mainnet, network_details.is_mainnet);

    // Verify cache consistency
    let cached = manager.network().await.unwrap();
    assert!(Arc::ptr_eq(&network_details, &cached));
}

#[tokio::test]
async fn test_network_details_localnet() {
    init_test_logging();

    let config = ClientManager::get_config_from_environment_or_localnet();
    let manager = ClientManager::new(&config);

    let details = manager.network().await.unwrap();

    // Verify structure
    assert!(!details.genesis_id.is_empty());
    assert!(!details.genesis_hash.is_empty());

    // Verify exactly one network type is detected
    let network_flags = [details.is_localnet, details.is_testnet, details.is_mainnet];
    assert_eq!(network_flags.iter().filter(|&&x| x).count(), 1);

    // Should detect as localnet
    assert!(
        details.is_localnet,
        "Should detect localnet for local config"
    );
}
