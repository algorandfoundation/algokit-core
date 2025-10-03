use super::{localnet, test_account::TestAccount};
use crate::{
    clients::algod_client::AlgodClientTrait,
    transactions::{
        asset_config::AssetCreateParams,
        common::{TransactionSignerGetter, UtilsError},
        composer::ComposerFactory,
        payment::PaymentParams,
    },
};
use std::sync::{Arc, Mutex};

/// Test fixture that provides high-level test operations using foreign traits
/// This enables test orchestration in Rust while delegating I/O to target languages
#[derive(uniffi::Object)]
pub struct TestFixture {
    pub algod_client: Arc<dyn AlgodClientTrait>,
    pub composer_factory: Arc<dyn ComposerFactory>,
    pub signer_getter: Arc<dyn TransactionSignerGetter>,
    dispenser_account: TestAccount,
    test_accounts: Mutex<Vec<TestAccount>>,
}

#[uniffi::export]
impl TestFixture {
    /// Create a new test fixture with foreign trait dependencies
    /// Gets the dispenser account mnemonic from localnet automatically
    #[uniffi::constructor]
    pub async fn new(
        algod_client: Arc<dyn AlgodClientTrait>,
        composer_factory: Arc<dyn ComposerFactory>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<Self, UtilsError> {
        // Get dispenser mnemonic from localnet
        let dispenser_mnemonic = localnet::get_dispenser_mnemonic().await?;
        let dispenser_account = TestAccount::from_mnemonic(dispenser_mnemonic.clone())?;

        // Register dispenser account with signer getter
        signer_getter.register_account(
            dispenser_account.address.clone(),
            dispenser_account.mnemonic.clone(),
        )?;

        Ok(TestFixture {
            algod_client,
            composer_factory,
            signer_getter,
            dispenser_account,
            test_accounts: Mutex::new(Vec::new()),
        })
    }

    /// Generate a new test account and register it with the signer getter
    pub fn generate_account(&self) -> Result<TestAccount, UtilsError> {
        let account = TestAccount::generate()?;

        // Register account with signer getter
        self.signer_getter
            .register_account(account.address.clone(), account.mnemonic.clone())?;

        // Track generated accounts
        self.test_accounts.lock().unwrap().push(account.clone());
        Ok(account)
    }

    /// Fund an account with ALGO from the dispenser
    /// Uses the composer factory to create a fresh composer for this operation
    pub async fn fund_account(
        &self,
        account: TestAccount,
        amount: u64,
    ) -> Result<String, UtilsError> {
        // Create a fresh composer for this operation (via factory)
        let composer = self.composer_factory.create_composer();

        let dispenser_signer = self
            .signer_getter
            .get_signer(self.dispenser_account.address.clone())?;

        // Build payment parameters
        let payment_params = PaymentParams {
            sender: self.dispenser_account.address.clone(),
            receiver: account.address.clone(),
            amount,
            signer: Some(dispenser_signer),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
        };

        // Add payment to composer
        composer.add_payment(payment_params).await?;

        // Build and send transaction
        composer.build().await?;
        let tx_ids = composer.send().await?;

        // Return first transaction ID
        tx_ids
            .first()
            .cloned()
            .ok_or_else(|| UtilsError::UtilsError {
                message: "No transaction ID returned".to_string(),
            })
    }

    /// Create a test asset with optional freeze manager
    /// Returns the asset ID
    pub async fn create_test_asset(
        &self,
        creator: TestAccount,
        freeze_manager: Option<TestAccount>,
    ) -> Result<u64, UtilsError> {
        // Create a fresh composer
        let composer = self.composer_factory.create_composer();

        let creator_signer = self.signer_getter.get_signer(creator.address.clone())?;

        // Build asset creation parameters
        let asset_params = AssetCreateParams {
            sender: creator.address.clone(),
            total: 1000000,
            decimals: Some(0),
            default_frozen: Some(false),
            unit_name: Some("TEST".to_string()),
            asset_name: Some("Test Asset".to_string()),
            url: None,
            metadata_hash: None,
            manager: None,
            reserve: None,
            freeze: freeze_manager.map(|fm| fm.address),
            clawback: None,
            signer: Some(creator_signer),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
        };

        // Add asset create to composer
        composer.add_asset_create(asset_params).await?;

        // Build and send
        composer.build().await?;
        let tx_ids = composer.send().await?;

        // Wait for confirmation to get asset ID
        let tx_id = tx_ids.first().ok_or_else(|| UtilsError::UtilsError {
            message: "No transaction ID returned".to_string(),
        })?;

        let info = self
            .algod_client
            .wait_for_confirmation(tx_id.clone())
            .await?;

        info.asset_id.ok_or_else(|| UtilsError::UtilsError {
            message: "No asset ID in transaction result".to_string(),
        })
    }

    /// Get the dispenser account (useful for tests)
    pub fn dispenser_account(&self) -> TestAccount {
        self.dispenser_account.clone()
    }
}
