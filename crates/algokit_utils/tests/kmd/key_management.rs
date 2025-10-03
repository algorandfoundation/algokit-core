use kmd_client::apis::client::KmdClient;
use kmd_client::models::{
    CreateWalletRequest, GenerateKeyRequest, InitWalletHandleTokenRequest, ListKeysRequest,
    ReleaseWalletHandleTokenRequest,
};
use rand::{Rng, distributions::Alphanumeric};

// Wallet key management flow: create wallet -> init handle token -> list keys -> generate key -> list keys (increment) -> release token
#[tokio::test]
async fn key_management_flow() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = KmdClient::localnet();

    let wallet_name: String = format!(
        "test_wallet_keys_{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>()
            .to_lowercase()
    );

    // Create wallet
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

    let created_wallet = create_response
        .wallet
        .as_ref()
        .expect("Expected created wallet in response");

    let wallet_id = created_wallet
        .id
        .as_ref()
        .expect("Wallet id should be present");
    assert_eq!(created_wallet.name.as_deref(), Some(wallet_name.as_str()));

    // Init wallet handle token
    let init_response = client
        .init_wallet_handle_token(InitWalletHandleTokenRequest {
            wallet_id: Some(wallet_id.clone()),
            wallet_password: Some("testpass".to_string()),
        })
        .await?;
    let wallet_handle_token = init_response
        .wallet_handle_token
        .as_ref()
        .expect("Wallet handle token should be present")
        .clone();

    // Baseline keys list
    let list_before = client
        .list_keys_in_wallet(ListKeysRequest {
            wallet_handle_token: Some(wallet_handle_token.clone()),
        })
        .await?;
    let before_addresses = list_before.addresses.unwrap_or_default();

    // Generate new key
    let _generate_response = client
        .generate_key(GenerateKeyRequest {
            wallet_handle_token: Some(wallet_handle_token.clone()),
            display_mnemonic: Some(false),
        })
        .await?;

    // List after
    let list_after = client
        .list_keys_in_wallet(ListKeysRequest {
            wallet_handle_token: Some(wallet_handle_token.clone()),
        })
        .await?;
    let after_addresses = list_after.addresses.unwrap_or_default();

    assert_eq!(
        after_addresses.len(),
        before_addresses.len() + 1,
        "Expected one additional key after generation"
    );

    // Release handle token
    let _release_response = client
        .release_wallet_handle_token(ReleaseWalletHandleTokenRequest {
            wallet_handle_token: Some(wallet_handle_token.clone()),
        })
        .await?;

    Ok(())
}
