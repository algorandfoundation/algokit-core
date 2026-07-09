//! Obtains a funded, signable account from localnet's default KMD wallet.
//!
//! localnet ships an `unencrypted-default-wallet` pre-loaded with funded accounts. This walks the
//! wallet (list wallets → open a handle → list keys → export a key) to recover a secret key we can
//! sign transactions with, picking the account with the highest balance as the dispenser.

use std::str::FromStr;

use algod_client::AlgodClient;
use algokit_crypto::ed25519::CryptoxideEd25519Keypair;
use algokit_transact::Address;
use algokit_transact::signer::{AddressWithSigners, generate_address_with_signers};
use kmd_client::KmdClient;
use kmd_client::models::{ExportKeyRequest, InitWalletHandleTokenRequest, ListKeysRequest};

/// localnet's default wallet name and (empty) password.
const DEFAULT_WALLET_NAME: &str = "unencrypted-default-wallet";
const DEFAULT_WALLET_PASSWORD: &str = "";

/// A localnet account recovered from KMD, ready to sign and fund transactions.
pub struct KmdAccount {
    /// The account's address, derived from its keypair.
    pub address: Address,
    /// Signer wrapping the account's ed25519 keypair.
    pub signer: AddressWithSigners,
}

/// Recover the highest-balance account from localnet's default KMD wallet.
pub async fn dispenser_account(kmd: &KmdClient, algod: &AlgodClient) -> KmdAccount {
    let handle = default_wallet_handle(kmd).await;

    let addresses = kmd
        .list_keys_in_wallet(ListKeysRequest {
            wallet_handle_token: handle.clone(),
        })
        .await
        .expect("failed to list wallet keys")
        .addresses;

    let richest = highest_balance_address(algod, &addresses).await;

    let private_key = kmd
        .export_key(ExportKeyRequest {
            address: Address::from_str(&richest).expect("kmd returned an invalid address"),
            wallet_handle_token: handle,
            wallet_password: Some(DEFAULT_WALLET_PASSWORD.to_string()),
        })
        .await
        .expect("failed to export account key")
        .private_key;

    account_from_kmd_key(&private_key)
}

/// Open a handle on the default wallet, returning its token.
async fn default_wallet_handle(kmd: &KmdClient) -> String {
    let wallets = kmd
        .list_wallets()
        .await
        .expect("failed to list kmd wallets")
        .wallets;
    let wallet_id = wallets
        .into_iter()
        .find(|w| w.name == DEFAULT_WALLET_NAME)
        .expect("localnet default wallet not found")
        .id;

    kmd.init_wallet_handle(InitWalletHandleTokenRequest {
        wallet_id,
        wallet_password: DEFAULT_WALLET_PASSWORD.to_string(),
    })
    .await
    .expect("failed to open wallet handle")
    .wallet_handle_token
}

/// Pick the address with the largest balance (the dispenser account).
async fn highest_balance_address(algod: &AlgodClient, addresses: &[String]) -> String {
    let mut best: Option<(String, u64)> = None;
    for address in addresses {
        let amount = algod
            .account_information(address, None)
            .await
            .expect("failed to fetch account information")
            .amount;
        if best.as_ref().is_none_or(|(_, b)| amount > *b) {
            best = Some((address.clone(), amount));
        }
    }
    best.expect("wallet has no accounts").0
}

/// Build a signable account from a KMD-exported key (64 bytes: 32-byte seed + 32-byte public key).
fn account_from_kmd_key(kmd_key: &[u8]) -> KmdAccount {
    let seed: [u8; 32] = kmd_key[..32]
        .try_into()
        .expect("kmd private key shorter than 32 bytes");
    let keypair =
        CryptoxideEd25519Keypair::try_generate(Some(seed)).expect("failed to build keypair");
    let signer = generate_address_with_signers(keypair);
    KmdAccount {
        address: signer.addr.clone(),
        signer,
    }
}
