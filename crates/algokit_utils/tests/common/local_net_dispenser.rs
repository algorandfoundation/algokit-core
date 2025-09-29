use std::{env, sync::Arc};

use algod_client::AlgodClient;
use algokit_transact::{
    AlgorandMsgpack, PaymentTransactionBuilder, Transaction, TransactionHeaderBuilder,
};
use algokit_utils::TransactionSigner;
use kmd_client::KmdClient;
use kmd_client::models::{
    ExportKeyRequest, GenerateKeyRequest, InitWalletHandleTokenRequest, ListKeysRequest,
    ReleaseWalletHandleTokenRequest,
};

use crate::common::TestAccount;

/// LocalNet dispenser for funding test accounts using KMD
pub struct LocalNetDispenser {
    client: Arc<AlgodClient>,
    kmd_client: Arc<KmdClient>,
    kmd_wallet_name: String,
    dispenser_account: Option<TestAccount>,
}

impl LocalNetDispenser {
    /// Create a new LocalNet dispenser
    pub fn new(client: Arc<AlgodClient>, kmd_client: Arc<KmdClient>) -> Self {
        let kmd_wallet_name = env::var("KMD_WALLET_NAME")
            .unwrap_or_else(|_| "unencrypted-default-wallet".to_string());

        Self {
            client,
            kmd_client,
            kmd_wallet_name,
            dispenser_account: None,
        }
    }

    /// Get the LocalNet dispenser account using KMD
    pub async fn get_dispenser_account(
        &mut self,
    ) -> Result<&TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        if self.dispenser_account.is_none() {
            self.dispenser_account = Some(self.fetch_dispenser_from_kmd().await?);
        }

        Ok(self.dispenser_account.as_ref().unwrap())
    }

    /// Fetch the dispenser account using KMD
    async fn fetch_dispenser_from_kmd(
        &self,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        let wallets_response = self
            .kmd_client
            .list_wallets()
            .await
            .map_err(|e| format!("Failed to list KMD wallets: {:?}", e))?;

        let wallets = wallets_response.wallets.unwrap_or_default();

        let selected_wallet = wallets
            .iter()
            .find(|wallet| wallet.name.as_deref() == Some(self.kmd_wallet_name.as_str()))
            .or_else(|| wallets.first())
            .ok_or("No wallets available via KMD")?;

        let wallet_id = selected_wallet
            .id
            .clone()
            .ok_or("KMD wallet is missing an id")?;

        let init_response = self
            .kmd_client
            .init_wallet_handle_token(InitWalletHandleTokenRequest {
                wallet_id: Some(wallet_id.clone()),
                wallet_password: None,
            })
            .await
            .map_err(|e| format!("Failed to initialize KMD wallet handle: {:?}", e))?;

        let wallet_handle_token = init_response
            .wallet_handle_token
            .ok_or("KMD did not return a wallet handle token")?;

        let release_request = ReleaseWalletHandleTokenRequest {
            wallet_handle_token: Some(wallet_handle_token.clone()),
        };

        let result = async {
            let mut addresses = self
                .kmd_client
                .list_keys_in_wallet(ListKeysRequest {
                    wallet_handle_token: Some(wallet_handle_token.clone()),
                })
                .await
                .map_err(|e| format!("Failed to list keys in KMD wallet: {:?}", e))?
                .addresses
                .unwrap_or_default();

            if addresses.is_empty() {
                let generated = self
                    .kmd_client
                    .generate_key(GenerateKeyRequest {
                        display_mnemonic: Some(false),
                        wallet_handle_token: Some(wallet_handle_token.clone()),
                    })
                    .await
                    .map_err(|e| format!("Failed to generate key in KMD wallet: {:?}", e))?;

                if let Some(address) = generated.address {
                    addresses.push(address);
                }
            }

            if addresses.is_empty() {
                return Err("KMD wallet does not contain any keys".into());
            }

            let mut best_address = None;
            let mut highest_balance = 0u64;

            for address in &addresses {
                match self.client.account_information(address, None, None).await {
                    Ok(info) => {
                        if info.amount > highest_balance {
                            highest_balance = info.amount;
                            best_address = Some(address.clone());
                        }
                    }
                    Err(err) => {
                        println!(
                            "Warning: failed to fetch balance for {}: {:?}",
                            address, err
                        );
                    }
                }
            }

            let dispenser_address = best_address.unwrap_or_else(|| addresses[0].clone());

            let export_response = self
                .kmd_client
                .export_key(ExportKeyRequest {
                    address: Some(dispenser_address.clone()),
                    wallet_handle_token: Some(wallet_handle_token.clone()),
                    wallet_password: None,
                })
                .await
                .map_err(|e| format!("Failed to export dispenser key via KMD: {:?}", e))?;

            let private_key = export_response
                .private_key
                .ok_or("KMD export did not return a private key")?;

            let dispenser = TestAccount::from_secret_key(&private_key)?;

            println!(
                "Found LocalNet dispenser account: {} with {} microALGOs",
                dispenser_address, highest_balance
            );

            Ok(dispenser)
        }
        .await;

        if let Err(err) = self
            .kmd_client
            .release_wallet_handle_token(release_request)
            .await
        {
            println!(
                "Warning: failed to release KMD wallet handle token: {:?}",
                err
            );
        }

        result
    }

    /// Fund an account with ALGOs using the dispenser
    pub async fn fund_account(
        &mut self,
        recipient_address: &str,
        amount: u64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Get transaction parameters first (before borrowing self mutably)
        let params = self
            .client
            .transaction_params()
            .await
            .map_err(|e| format!("Failed to get transaction params: {:?}", e))?;

        let dispenser = self.get_dispenser_account().await?;

        // Convert recipient address string to algokit_transact::Address
        let recipient = recipient_address.parse()?;

        // Convert genesis hash Vec<u8> to 32-byte array (already decoded from base64)
        let genesis_hash_bytes: [u8; 32] =
            params.genesis_hash.try_into().map_err(|v: Vec<u8>| {
                format!("Genesis hash must be 32 bytes, got {} bytes", v.len())
            })?;

        // Build funding transaction
        let header = TransactionHeaderBuilder::default()
            .sender(dispenser.account().address())
            .fee(params.min_fee)
            .first_valid(params.last_round)
            .last_valid(params.last_round + 1000)
            .genesis_id(params.genesis_id.clone())
            .genesis_hash(genesis_hash_bytes)
            .note(b"LocalNet test funding".to_vec())
            .build()?;

        let payment_fields = PaymentTransactionBuilder::default()
            .header(header)
            .receiver(recipient)
            .amount(amount)
            .build_fields()?;

        let transaction = Transaction::Payment(payment_fields);
        let signed_transaction = dispenser.sign_transaction(&transaction).await?;
        let signed_bytes = signed_transaction
            .encode()
            .map_err(|e| format!("Failed to encode signed transaction: {:?}", e))?;

        // Submit transaction
        let response = self
            .client
            .raw_transaction(signed_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction: {:?}", e))?;

        println!(
            "✓ Funded account {} with {} microALGOs (txn: {})",
            recipient_address, amount, response.tx_id
        );

        Ok(response.tx_id)
    }
}
