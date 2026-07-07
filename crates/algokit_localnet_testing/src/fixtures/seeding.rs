//! Build, sign, submit and confirm transactions against localnet.
//!
//! These helpers turn a [`KmdAccount`] plus algod's suggested params into confirmed on-chain state,
//! so state-dependent endpoint tests have something real to query.

use algod_client::AlgodClient;
use algokit_transact::{
    Address, AlgorandMsgpack, AppCallTransactionBuilder, OnApplicationComplete,
    PaymentTransactionBuilder, SignedTransaction, Transaction, TransactionHeader,
    TransactionHeaderBuilder, TransactionId,
};

use super::kmd_account::KmdAccount;

/// How many rounds to wait for a submitted transaction to confirm before giving up.
const CONFIRMATION_ROUNDS: u64 = 10;

/// Sign a transaction with `account`, returning the signed transaction without submitting it.
pub async fn sign(account: &KmdAccount, transaction: Transaction) -> SignedTransaction {
    account
        .signer
        .signer
        .sign_transactions(&[transaction], &[0])
        .await
        .expect("failed to sign transaction")
        .remove(0)
}

/// Sign a transaction with `account`, submit it, and wait until it confirms. Returns the txid.
pub async fn submit_and_confirm(
    algod: &AlgodClient,
    account: &KmdAccount,
    transaction: Transaction,
) -> String {
    let signed = account
        .signer
        .signer
        .sign_transactions(&[transaction], &[0])
        .await
        .expect("failed to sign transaction");
    let bytes = signed[0]
        .encode()
        .expect("failed to encode signed transaction");
    let txid = signed[0].id().expect("failed to compute txid");

    algod
        .raw_transaction(bytes)
        .await
        .expect("failed to submit transaction");

    wait_for_confirmation(algod, &txid).await;
    txid
}

/// Send `amount` microAlgos from `dispenser` to `receiver`, confirmed. Returns the txid.
pub async fn fund_account(
    algod: &AlgodClient,
    dispenser: &KmdAccount,
    receiver: &Address,
    amount: u64,
) -> String {
    let payment = payment(algod, &dispenser.address, receiver, amount).await;
    submit_and_confirm(algod, dispenser, payment).await
}

/// Build a transaction header for `sender` from algod's current suggested params.
async fn header(algod: &AlgodClient, sender: &Address) -> TransactionHeader {
    let params = algod
        .transaction_params()
        .await
        .expect("failed to fetch suggested params");

    TransactionHeaderBuilder::default()
        .sender(sender.clone())
        .fee(params.min_fee)
        .first_valid(params.last_round + 1)
        .last_valid(params.last_round + 1 + 1000)
        .genesis_hash(params.genesis_hash)
        .genesis_id(params.genesis_id)
        .build()
        .expect("failed to build transaction header")
}

/// Build a payment transaction using algod's current suggested params.
pub async fn payment(
    algod: &AlgodClient,
    sender: &Address,
    receiver: &Address,
    amount: u64,
) -> Transaction {
    PaymentTransactionBuilder::default()
        .header(header(algod, sender).await)
        .receiver(receiver.clone())
        .amount(amount)
        .build()
        .expect("failed to build payment transaction")
}

/// Build a no-op call to `app_id` from `sender` using algod's current suggested params.
pub async fn app_noop_call(algod: &AlgodClient, sender: &Address, app_id: u64) -> Transaction {
    AppCallTransactionBuilder::default()
        .header(header(algod, sender).await)
        .app_id(app_id)
        .on_complete(OnApplicationComplete::NoOp)
        .build()
        .expect("failed to build app call transaction")
}

/// Poll pending-transaction info until the transaction reports a confirmed round.
async fn wait_for_confirmation(algod: &AlgodClient, txid: &str) {
    let start = algod
        .status()
        .await
        .expect("failed to fetch node status")
        .last_round;

    for round in start..start + CONFIRMATION_ROUNDS {
        let pending = algod
            .pending_transaction_information(txid)
            .await
            .expect("failed to fetch pending transaction");
        if pending.confirmed_round.unwrap_or(0) > 0 {
            return;
        }
        algod
            .status_after_block(round)
            .await
            .expect("failed to wait for next round");
    }
    panic!("transaction {txid} not confirmed within {CONFIRMATION_ROUNDS} rounds");
}
