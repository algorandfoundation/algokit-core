use algod_api::AlgodClient;

use algod_api::TransactionParams;
use algokit_transact::Address;
use algokit_transact::FeeParams;
use algokit_transact::PaymentTransactionFields;
use algokit_transact::SignedTransaction;
use algokit_transact::Transaction;
use algokit_transact::TransactionHeader;
use algokit_transact::Transactions;
use base64::{Engine as _, engine::general_purpose};
use derive_more::Debug;
use std::sync::Arc;

use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error(transparent)]
    HttpError(#[from] algokit_http_client::HttpError),
    #[error("Decode Error: {0}")]
    DecodeError(String),
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Signing Error: {0}")]
    SigningError(String),
    #[error("Composer State Error: {0}")]
    StateError(String),
}

#[derive(Debug, Default, Clone)]
pub struct CommonParams {
    pub sender: Address,
    #[debug(skip)]
    pub signer: Option<Arc<dyn TxnSigner>>,
    pub rekey_to: Option<Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Debug)]
pub struct PaymentParams {
    pub common_params: CommonParams,
    pub receiver: Address,
    pub amount: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug)]
pub struct AssetTransferParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset transfers
    /// without additional side effects.
    /// Only in the case where the receiver is equal to the sender and the amount is zero,
    /// this is an asset opt-in transaction.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
}

#[derive(Debug)]
pub struct AssetOptInParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-in
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
}

#[derive(Debug)]
pub struct AssetOptOutParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-out
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug)]
pub struct AssetClawbackParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset clawback
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
    // The address from which ASAs are taken.
    pub clawback_target: Address,
}

// TODO: TransactionWithSigner
#[derive(Debug)]
pub enum ComposerTxn {
    Transaction(Transaction),
    Payment(PaymentParams),
    AssetTransfer(AssetTransferParams),
    AssetOptIn(AssetOptInParams),
    AssetOptOut(AssetOptOutParams),
    AssetClawback(AssetClawbackParams),
}

impl ComposerTxn {
    pub fn common_params(&self) -> CommonParams {
        match self {
            ComposerTxn::Payment(payment_params) => payment_params.common_params.clone(),
            ComposerTxn::AssetTransfer(asset_transfer_params) => {
                asset_transfer_params.common_params.clone()
            }
            ComposerTxn::AssetOptIn(asset_opt_in_params) => {
                asset_opt_in_params.common_params.clone()
            }
            ComposerTxn::AssetOptOut(asset_opt_out_params) => {
                asset_opt_out_params.common_params.clone()
            }
            ComposerTxn::AssetClawback(asset_clawback_params) => {
                asset_clawback_params.common_params.clone()
            }
            _ => CommonParams::default(),
        }
    }
}

#[async_trait]
pub trait TxnSigner: Send + Sync {
    async fn sign_txns(&self, txns: &[Transaction], indices: &[usize]) -> Vec<SignedTransaction>;

    async fn sign_txn(&self, txn: &Transaction) -> SignedTransaction {
        self.sign_txns(&[txn.clone()], &[0]).await[0].clone()
    }
}

#[async_trait]
pub trait TxnSignerGetter: Send + Sync {
    async fn get_signer(&self, address: Address) -> Option<&dyn TxnSigner>;
}

struct DefaultSignerGetter;

#[async_trait]
impl TxnSignerGetter for DefaultSignerGetter {
    async fn get_signer(&self, _address: Address) -> Option<&dyn TxnSigner> {
        None
    }
}

pub struct EmptySigner {}

#[async_trait]
impl TxnSigner for EmptySigner {
    async fn sign_txns(&self, txns: &[Transaction], indices: &[usize]) -> Vec<SignedTransaction> {
        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    SignedTransaction {
                        transaction: txns[idx].clone(),
                        signature: Some([0; 64]),
                        auth_address: None,
                    }
                } else {
                    panic!("Index out of bounds for transactions");
                }
            })
            .collect()
    }
}

#[async_trait]
impl TxnSignerGetter for EmptySigner {
    async fn get_signer(&self, _address: Address) -> Option<&dyn TxnSigner> {
        Some(self)
    }
}

pub struct Composer {
    transactions: Vec<ComposerTxn>,
    algod_client: AlgodClient,
    signer_getter: Arc<dyn TxnSignerGetter>,
    built_group: Option<Vec<Transaction>>,
    signed_group: Option<Vec<SignedTransaction>>,
}

impl Composer {
    pub fn new(algod_client: AlgodClient, get_signer: Option<Arc<dyn TxnSignerGetter>>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client,
            signer_getter: get_signer.unwrap_or(Arc::new(DefaultSignerGetter)),
            built_group: None,
            signed_group: None,
        }
    }

    pub fn built_group(&self) -> Option<&Vec<Transaction>> {
        self.built_group.as_ref()
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            signer_getter: Arc::new(DefaultSignerGetter),
            built_group: None,
            signed_group: None,
        }
    }

    fn push(&mut self, txn: ComposerTxn) -> Result<(), String> {
        if self.transactions.len() >= 16 {
            return Err("Composer can only hold up to 16 transactions".to_string());
        }
        self.transactions.push(txn);
        Ok(())
    }

    pub fn add_payment(&mut self, payment_params: PaymentParams) -> Result<(), String> {
        self.push(ComposerTxn::Payment(payment_params))
    }

    pub fn add_asset_transfer(
        &mut self,
        asset_transfer_params: AssetTransferParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetTransfer(asset_transfer_params))
    }

    pub fn add_asset_opt_in(
        &mut self,
        asset_opt_in_params: AssetOptInParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetOptIn(asset_opt_in_params))
    }

    pub fn add_asset_opt_out(
        &mut self,
        asset_opt_out_params: AssetOptOutParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetOptOut(asset_opt_out_params))
    }

    pub fn add_asset_clawback(
        &mut self,
        asset_clawback_params: AssetClawbackParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetClawback(asset_clawback_params))
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.push(ComposerTxn::Transaction(transaction))
    }

    pub fn transactions(&self) -> &Vec<ComposerTxn> {
        &self.transactions
    }

    pub async fn get_signer(&self, address: Address) -> Option<&dyn TxnSigner> {
        self.signer_getter.get_signer(address).await
    }

    // TODO: Use Fn defined in ComposerConfig
    pub async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        self.algod_client
            .transaction_params()
            .await
            .map_err(ComposerError::HttpError)
    }

    pub async fn build(&mut self) -> Result<&mut Self, ComposerError> {
        if self.built_group.is_some() {
            return Ok(self);
        }

        let suggested_params = self.get_suggested_params().await?;

        let default_header = TransactionHeader {
            fee: Some(suggested_params.fee),
            genesis_id: Some(suggested_params.genesis_id),
            genesis_hash: Some(
                general_purpose::STANDARD
                    .decode(suggested_params.genesis_hash)
                    .map_err(|e| {
                        ComposerError::DecodeError(format!("Failed to decode genesis hash: {}", e))
                    })?
                    .try_into()
                    .map_err(|e| {
                        ComposerError::DecodeError(format!(
                            "Failed to convert genesis hash: {:?}",
                            e
                        ))
                    })?,
            ),
            // The rest of these fields are set further down per txn
            first_valid: 0,
            last_valid: 0,
            sender: Address::default(),
            rekey_to: None,
            note: None,
            lease: None,
            group: None,
        };

        let txs = self
            .transactions
            .iter()
            .map(|composer_txn| {
                let already_formed_txn = matches!(composer_txn, ComposerTxn::Transaction(_));

                let mut transaction: algokit_transact::Transaction = match composer_txn {
                    ComposerTxn::Transaction(txn) => txn.clone(),
                    ComposerTxn::Payment(pay_params) => {
                        Transaction::Payment(PaymentTransactionFields {
                            header: default_header.clone(),
                            receiver: pay_params.receiver.clone(),
                            amount: pay_params.amount,
                            close_remainder_to: pay_params.close_remainder_to.clone(),
                        })
                    }
                    ComposerTxn::AssetTransfer(asset_transfer_params) => {
                        Transaction::AssetTransfer(
                            algokit_transact::AssetTransferTransactionFields {
                                header: default_header.clone(),
                                asset_id: asset_transfer_params.asset_id,
                                amount: asset_transfer_params.amount,
                                receiver: asset_transfer_params.receiver.clone(),
                                asset_sender: None,
                                close_remainder_to: None,
                            },
                        )
                    }
                    ComposerTxn::AssetOptIn(asset_opt_in_params) => Transaction::AssetTransfer(
                        algokit_transact::AssetTransferTransactionFields {
                            header: default_header.clone(),
                            asset_id: asset_opt_in_params.asset_id,
                            amount: 0,
                            receiver: asset_opt_in_params.common_params.sender.clone(),
                            asset_sender: None,
                            close_remainder_to: None,
                        },
                    ),
                    ComposerTxn::AssetOptOut(asset_opt_out_params) => Transaction::AssetTransfer(
                        algokit_transact::AssetTransferTransactionFields {
                            header: default_header.clone(),
                            asset_id: asset_opt_out_params.asset_id,
                            amount: 0,
                            receiver: asset_opt_out_params.common_params.sender.clone(),
                            asset_sender: None,
                            close_remainder_to: asset_opt_out_params.close_remainder_to.clone(),
                        },
                    ),
                    ComposerTxn::AssetClawback(asset_clawback_params) => {
                        Transaction::AssetTransfer(
                            algokit_transact::AssetTransferTransactionFields {
                                header: default_header.clone(),
                                asset_id: asset_clawback_params.asset_id,
                                amount: asset_clawback_params.amount,
                                receiver: asset_clawback_params.receiver.clone(),
                                asset_sender: Some(asset_clawback_params.clawback_target.clone()),
                                close_remainder_to: None,
                            },
                        )
                    }
                };

                if !already_formed_txn {
                    let common_params = composer_txn.common_params();
                    let header = transaction.header_mut();

                    header.sender = common_params.sender;
                    header.rekey_to = common_params.rekey_to;
                    header.note = common_params.note;
                    header.lease = common_params.lease;

                    transaction
                        .assign_fee(FeeParams {
                            fee_per_byte: suggested_params.fee,
                            min_fee: suggested_params.min_fee,
                            extra_fee: common_params.extra_fee,
                            max_fee: common_params.max_fee,
                        })
                        .map_err(|e| ComposerError::TransactionError(e.to_string()))?;
                }

                Ok(transaction)
            })
            .collect::<Result<Vec<Transaction>, ComposerError>>()?;

        self.built_group = Some(txs.assign_group().map_err(|e| {
            ComposerError::TransactionError(format!("Failed to assign group: {}", e))
        })?);
        Ok(self)
    }

    pub async fn gather_signatures(&mut self) -> Result<&mut Self, ComposerError> {
        let transactions = self.built_group.as_ref().ok_or(ComposerError::StateError(
            "Cannot gather signatures before building the transaction group".to_string(),
        ))?;

        let mut signed_group = Vec::<SignedTransaction>::new();

        for txn in transactions.iter() {
            let signer = self.get_signer(txn.header().sender.clone()).await.ok_or(
                ComposerError::SigningError(format!(
                    "No signer found for address: {}",
                    txn.header().sender
                )),
            )?;

            let signed_txn = signer.sign_txn(txn).await;
            signed_group.push(signed_txn);
        }

        self.signed_group = Some(signed_group);

        Ok(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{AddressMother, TransactionMother};

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

    #[tokio::test]
    async fn test_get_suggested_params() {
        let composer = Composer::testnet();
        let response = composer.get_suggested_params().await.unwrap();

        assert_eq!(
            response.genesis_hash,
            "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI="
        );
    }

    #[test]
    fn test_add_payment() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        assert!(composer.add_payment(payment_params).is_ok());
    }

    #[tokio::test]
    async fn test_build_payment() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        assert!(composer.add_payment(payment_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.built_group().is_some());
    }

    #[tokio::test]
    async fn test_build_asset_transfer() {
        let mut composer = Composer::testnet();
        let asset_transfer_params = AssetTransferParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            asset_id: 12345,
            amount: 1000,
            receiver: AddressMother::address(),
        };
        assert!(composer.add_asset_transfer(asset_transfer_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.built_group().is_some());
    }

    #[tokio::test]
    async fn test_build_asset_opt_in() {
        let mut composer = Composer::testnet();
        let asset_opt_in_params = AssetOptInParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            asset_id: 12345,
        };
        assert!(composer.add_asset_opt_in(asset_opt_in_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.built_group().is_some());
    }

    #[tokio::test]
    async fn test_build_asset_opt_out() {
        let mut composer = Composer::testnet();
        let asset_opt_out_params = AssetOptOutParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            asset_id: 12345,
            close_remainder_to: Some(AddressMother::neil()),
        };
        assert!(composer.add_asset_opt_out(asset_opt_out_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.built_group().is_some());
    }

    #[tokio::test]
    async fn test_build_asset_clawback() {
        let mut composer = Composer::testnet();
        let asset_clawback_params = AssetClawbackParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            asset_id: 12345,
            amount: 1000,
            receiver: AddressMother::address(),
            clawback_target: AddressMother::neil(),
        };
        assert!(composer.add_asset_clawback(asset_clawback_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.built_group().is_some());
    }

    #[tokio::test]
    async fn test_gather_signatures() {
        let mut composer = Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            signer_getter: Arc::new(EmptySigner {}),
            built_group: None,
            signed_group: None,
        };
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        assert!(composer.add_payment(payment_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.gather_signatures().await.is_ok());
        assert!(composer.signed_group.is_some());
    }
}
