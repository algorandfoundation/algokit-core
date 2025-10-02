use kmd_client::apis::client::KmdClient;
use kmd_client::models::CreateWalletRequest;
use rand::{Rng, distributions::Alphanumeric};

// Basic wallet lifecycle: create wallet, ensure it appears in list
#[tokio::test]
async fn wallet_lifecycle() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Arrange
    let client = KmdClient::localnet();

    let wallet_name: String = format!(
        "test_wallet_{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>()
            .to_lowercase()
    );

    // Act: create wallet
    let create_response = client
        .create_wallet(CreateWalletRequest {
            wallet_name: Some(wallet_name.clone()),
            wallet_driver_name: Some("sqlite".to_string()),
            wallet_password: Some("testpass".to_string()),
            ..Default::default()
        })
        .await
        .map_err(|e| {
            format!(
                "Failed to create wallet (possible KMD token/availability issue): {}",
                e
            )
        })?;

    // Assert create response basic invariants
    let created_wallet = create_response
        .wallet
        .as_ref()
        .expect("Expected created wallet in response");
    assert_eq!(created_wallet.name.as_deref(), Some(wallet_name.as_str()));

    // List wallets and ensure presence
    let list_response = client.list_wallets().await?;
    let wallets = list_response.wallets.unwrap_or_default();
    let found = wallets
        .iter()
        .any(|w| w.name.as_deref() == Some(wallet_name.as_str()));
    assert!(found, "Created wallet should be present in list of wallets");

    Ok(())
}
