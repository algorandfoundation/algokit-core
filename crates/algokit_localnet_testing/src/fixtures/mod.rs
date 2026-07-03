//! Localnet fixtures: a ready-to-use dispenser account plus clients for seeding on-chain state.

/// Recovering a signable account from localnet's KMD wallet.
pub mod kmd_account;
/// Building, signing, submitting and confirming transactions.
pub mod seeding;

use std::sync::Arc;

use algod_client::AlgodClient;
use kmd_client::KmdClient;

use crate::http_capture::CapturingHttpClient;
use kmd_account::{KmdAccount, dispenser_account};

/// A localnet test context: a capturing algod client, a kmd client, and a funded dispenser account.
pub struct LocalnetFixture {
    /// Records raw response bytes for schema validation.
    pub capture: Arc<CapturingHttpClient>,
    /// Algod client backed by [`Self::capture`].
    pub algod: AlgodClient,
    /// KMD client for wallet operations.
    pub kmd: KmdClient,
    /// Highest-balance account from the default wallet, ready to sign and fund.
    pub dispenser: KmdAccount,
}

impl LocalnetFixture {
    /// Build a fixture against a running localnet.
    pub async fn new() -> Self {
        let capture = CapturingHttpClient::localnet();
        let algod = capture.client();
        let kmd = KmdClient::localnet();
        let dispenser = dispenser_account(&kmd, &algod).await;
        Self {
            capture,
            algod,
            kmd,
            dispenser,
        }
    }
}
