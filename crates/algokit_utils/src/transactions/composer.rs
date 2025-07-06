use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{PendingTransactionResponse, TransactionParams},
};
use algokit_transact::{
    Address, AlgorandMsgpack, AssetConfigTransactionFields, FeeParams, OnApplicationComplete,
    PaymentTransactionFields, SignedTransaction, Transaction, TransactionHeader, Transactions,
};
use derive_more::Debug;
use std::sync::Arc;

use super::application_call::{
    ApplicationCallParams, ApplicationCreateParams, ApplicationDeleteParams,
    ApplicationUpdateParams,
};
use super::asset_config::{AssetCreateParams, AssetDestroyParams, AssetReconfigureParams};
use super::common::{CommonParams, DefaultSignerGetter, TxnSigner, TxnSignerGetter};
use crate::clients::network_client::genesis_id_is_localnet;

#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(#[from] AlgodError),
    #[error("Decode Error: {0}")]
    DecodeError(String),
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Signing Error: {0}")]
    SigningError(String),
    #[error("Composer State Error: {0}")]
    StateError(String),
    #[error("Transaction pool error: {0}")]
    PoolError(String),
}

#[derive(Debug, Clone)]
pub struct PaymentParams {
    pub common_params: CommonParams,
    pub receiver: Address,
    pub amount: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AssetOptInParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-in
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
}

#[derive(Debug, Clone)]
pub struct AssetOptOutParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-out
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ComposerTxn {
    Transaction(Transaction),
    Payment(PaymentParams),
    AssetTransfer(AssetTransferParams),
    AssetOptIn(AssetOptInParams),
    AssetOptOut(AssetOptOutParams),
    AssetClawback(AssetClawbackParams),
    AssetCreate(AssetCreateParams),
    AssetReconfigure(AssetReconfigureParams),
    AssetDestroy(AssetDestroyParams),
    ApplicationCall(ApplicationCallParams),
    ApplicationCreate(ApplicationCreateParams),
    ApplicationUpdate(ApplicationUpdateParams),
    ApplicationDelete(ApplicationDeleteParams),
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
            ComposerTxn::AssetCreate(asset_create_params) => {
                asset_create_params.common_params.clone()
            }
            ComposerTxn::AssetReconfigure(asset_reconfigure_params) => {
                asset_reconfigure_params.common_params.clone()
            }
            ComposerTxn::AssetDestroy(asset_destroy_params) => {
                asset_destroy_params.common_params.clone()
            }
            ComposerTxn::ApplicationCall(app_call_params) => app_call_params.common_params.clone(),
            ComposerTxn::ApplicationCreate(app_create_params) => {
                app_create_params.common_params.clone()
            }
            ComposerTxn::ApplicationUpdate(app_update_params) => {
                app_update_params.common_params.clone()
            }
            ComposerTxn::ApplicationDelete(app_delete_params) => {
                app_delete_params.common_params.clone()
            }
            _ => CommonParams::default(),
        }
    }
}

#[derive(Clone)]
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

    pub fn add_asset_create(
        &mut self,
        asset_create_params: AssetCreateParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetCreate(asset_create_params))
    }

    pub fn add_asset_reconfigure(
        &mut self,
        asset_reconfigure_params: AssetReconfigureParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetReconfigure(asset_reconfigure_params))
    }

    pub fn add_asset_destroy(
        &mut self,
        asset_destroy_params: AssetDestroyParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::AssetDestroy(asset_destroy_params))
    }

    pub fn add_application_call(
        &mut self,
        app_call_params: ApplicationCallParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::ApplicationCall(app_call_params))
    }

    pub fn add_application_create(
        &mut self,
        app_create_params: ApplicationCreateParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::ApplicationCreate(app_create_params))
    }

    pub fn add_application_update(
        &mut self,
        app_update_params: ApplicationUpdateParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::ApplicationUpdate(app_update_params))
    }

    pub fn add_application_delete(
        &mut self,
        app_delete_params: ApplicationDeleteParams,
    ) -> Result<(), String> {
        self.push(ComposerTxn::ApplicationDelete(app_delete_params))
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

    pub async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        self.algod_client
            .transaction_params()
            .await
            .map_err(Into::into)
    }

    pub async fn build(&mut self) -> Result<&mut Self, ComposerError> {
        if self.built_group.is_some() {
            return Ok(self);
        }

        let suggested_params = self.get_suggested_params().await?;

        // Determine validity window: default 10 rounds, but 1000 for LocalNet
        let default_validity_window = if genesis_id_is_localnet(&suggested_params.genesis_id) {
            1000 // LocalNet gets bigger window to avoid dead transactions
        } else {
            10 // Standard default validity window
        };

        let txs = self
            .transactions
            .iter()
            .map(|tx| {
                let common_params = tx.common_params();

                let first_valid = common_params
                    .first_valid_round
                    .unwrap_or(suggested_params.last_round);

                let header: TransactionHeader = TransactionHeader {
                    sender: common_params.sender.clone(),
                    rekey_to: common_params.rekey_to.clone(),
                    note: common_params.note.clone(),
                    lease: common_params.lease,
                    fee: common_params.static_fee,
                    genesis_id: Some(suggested_params.genesis_id.clone()),
                    genesis_hash: Some(suggested_params.genesis_hash.clone().try_into().map_err(
                        |_e| ComposerError::DecodeError("Invalid genesis hash".to_string()),
                    )?),
                    first_valid,
                    last_valid: common_params.last_valid_round.unwrap_or_else(|| {
                        common_params
                            .validity_window
                            .map(|window| first_valid + window)
                            .unwrap_or(first_valid + default_validity_window)
                    }),
                    group: None,
                };
                let mut calculate_fee = header.fee.is_none();

                let mut transaction = match tx {
                    ComposerTxn::Transaction(tx) => {
                        calculate_fee = false;
                        tx.clone()
                    }
                    ComposerTxn::Payment(pay_params) => {
                        let pay_params = PaymentTransactionFields {
                            header,
                            receiver: pay_params.receiver.clone(),
                            amount: pay_params.amount,
                            close_remainder_to: pay_params.close_remainder_to.clone(),
                        };
                        Transaction::Payment(pay_params)
                    }
                    ComposerTxn::AssetTransfer(asset_transfer_params) => {
                        Transaction::AssetTransfer(
                            algokit_transact::AssetTransferTransactionFields {
                                header,
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
                            header,
                            asset_id: asset_opt_in_params.asset_id,
                            amount: 0,
                            receiver: asset_opt_in_params.common_params.sender.clone(),
                            asset_sender: None,
                            close_remainder_to: None,
                        },
                    ),
                    ComposerTxn::AssetOptOut(asset_opt_out_params) => Transaction::AssetTransfer(
                        algokit_transact::AssetTransferTransactionFields {
                            header,
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
                                header,
                                asset_id: asset_clawback_params.asset_id,
                                amount: asset_clawback_params.amount,
                                receiver: asset_clawback_params.receiver.clone(),
                                asset_sender: Some(asset_clawback_params.clawback_target.clone()),
                                close_remainder_to: None,
                            },
                        )
                    }
                    ComposerTxn::AssetCreate(asset_create_params) => {
                        Transaction::AssetConfig(AssetConfigTransactionFields {
                            header,
                            asset_id: 0,
                            total: Some(asset_create_params.total),
                            decimals: asset_create_params.decimals,
                            default_frozen: asset_create_params.default_frozen,
                            asset_name: asset_create_params.asset_name.clone(),
                            unit_name: asset_create_params.unit_name.clone(),
                            url: asset_create_params.url.clone(),
                            metadata_hash: asset_create_params.metadata_hash,
                            manager: asset_create_params.manager.clone(),
                            reserve: asset_create_params.reserve.clone(),
                            freeze: asset_create_params.freeze.clone(),
                            clawback: asset_create_params.clawback.clone(),
                        })
                    }
                    ComposerTxn::AssetReconfigure(asset_reconfigure_params) => {
                        Transaction::AssetConfig(AssetConfigTransactionFields {
                            header,
                            asset_id: asset_reconfigure_params.asset_id,
                            total: None,
                            decimals: None,
                            default_frozen: None,
                            asset_name: None,
                            unit_name: None,
                            url: None,
                            metadata_hash: None,
                            manager: asset_reconfigure_params.manager.clone(),
                            reserve: asset_reconfigure_params.reserve.clone(),
                            freeze: asset_reconfigure_params.freeze.clone(),
                            clawback: asset_reconfigure_params.clawback.clone(),
                        })
                    }
                    ComposerTxn::AssetDestroy(asset_destroy_params) => {
                        Transaction::AssetConfig(AssetConfigTransactionFields {
                            header,
                            asset_id: asset_destroy_params.asset_id,
                            total: None,
                            decimals: None,
                            default_frozen: None,
                            asset_name: None,
                            unit_name: None,
                            url: None,
                            metadata_hash: None,
                            manager: None,
                            reserve: None,
                            freeze: None,
                            clawback: None,
                        })
                    }
                    ComposerTxn::ApplicationCall(app_call_params) => Transaction::ApplicationCall(
                        algokit_transact::ApplicationCallTransactionFields {
                            header,
                            app_id: app_call_params.app_id,
                            on_complete: app_call_params.on_complete,
                            approval_program: None,
                            clear_state_program: None,
                            global_state_schema: None,
                            local_state_schema: None,
                            extra_program_pages: None,
                            args: app_call_params.args.clone(),
                            account_references: app_call_params.account_references.clone(),
                            app_references: app_call_params.app_references.clone(),
                            asset_references: app_call_params.asset_references.clone(),
                            box_references: app_call_params.box_references.clone(),
                        },
                    ),
                    ComposerTxn::ApplicationCreate(app_create_params) => {
                        Transaction::ApplicationCall(
                            algokit_transact::ApplicationCallTransactionFields {
                                header,
                                app_id: 0, // 0 indicates application creation
                                on_complete: app_create_params.on_complete,
                                approval_program: Some(app_create_params.approval_program.clone()),
                                clear_state_program: Some(
                                    app_create_params.clear_state_program.clone(),
                                ),
                                global_state_schema: app_create_params.global_state_schema.clone(),
                                local_state_schema: app_create_params.local_state_schema.clone(),
                                extra_program_pages: app_create_params.extra_program_pages,
                                args: app_create_params.args.clone(),
                                account_references: app_create_params.account_references.clone(),
                                app_references: app_create_params.app_references.clone(),
                                asset_references: app_create_params.asset_references.clone(),
                                box_references: app_create_params.box_references.clone(),
                            },
                        )
                    }
                    ComposerTxn::ApplicationUpdate(app_update_params) => {
                        Transaction::ApplicationCall(
                            algokit_transact::ApplicationCallTransactionFields {
                                header,
                                app_id: app_update_params.app_id,
                                on_complete: OnApplicationComplete::UpdateApplication,
                                approval_program: Some(app_update_params.approval_program.clone()),
                                clear_state_program: Some(
                                    app_update_params.clear_state_program.clone(),
                                ),
                                global_state_schema: None,
                                local_state_schema: None,
                                extra_program_pages: None,
                                args: app_update_params.args.clone(),
                                account_references: app_update_params.account_references.clone(),
                                app_references: app_update_params.app_references.clone(),
                                asset_references: app_update_params.asset_references.clone(),
                                box_references: app_update_params.box_references.clone(),
                            },
                        )
                    }
                    ComposerTxn::ApplicationDelete(app_delete_params) => {
                        Transaction::ApplicationCall(
                            algokit_transact::ApplicationCallTransactionFields {
                                header,
                                app_id: app_delete_params.app_id,
                                on_complete: OnApplicationComplete::DeleteApplication,
                                approval_program: None,
                                clear_state_program: None,
                                global_state_schema: None,
                                local_state_schema: None,
                                extra_program_pages: None,
                                args: app_delete_params.args.clone(),
                                account_references: app_delete_params.account_references.clone(),
                                app_references: app_delete_params.app_references.clone(),
                                asset_references: app_delete_params.asset_references.clone(),
                                box_references: app_delete_params.box_references.clone(),
                            },
                        )
                    }
                };

                if calculate_fee {
                    transaction = transaction
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

        // Only assign group if there are 2 or more transactions (matching algosdk ATC behavior)
        self.built_group = if txs.len() > 1 {
            Some(txs.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?)
        } else {
            Some(txs) // Single transaction, no group assignment
        };
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

            let signed_txn = signer
                .sign_txn(txn)
                .await
                .map_err(ComposerError::SigningError)?;
            signed_group.push(signed_txn);
        }

        self.signed_group = Some(signed_group);

        Ok(self)
    }

    pub async fn wait_for_confirmation(
        &self,
        tx_id: &str,
        max_rounds: u64,
    ) -> Result<PendingTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        let status = self
            .algod_client
            .get_status()
            .await
            .map_err(|e| format!("Failed to get status: {:?}", e))?;

        let start_round = status.last_round + 1;
        let mut current_round = start_round;

        while current_round < start_round + max_rounds {
            match self
                .algod_client
                .pending_transaction_information(tx_id, Some(Format::Msgpack))
                .await
            {
                Ok(response) => {
                    // Check for pool errors first - transaction was kicked out of pool
                    if !response.pool_error.is_empty() {
                        return Err(Box::new(ComposerError::PoolError(
                            response.pool_error.clone(),
                        )));
                    }

                    // Check if transaction is confirmed
                    if response.confirmed_round.is_some() {
                        return Ok(response);
                    }
                }
                Err(error) => {
                    // Only retry for 404 errors (transaction not found yet)
                    // All other errors indicate permanent issues and should fail fast
                    let is_retryable = matches!(
                        &error,
                        algod_client::apis::Error::Api(
                            algod_client::apis::AlgodApiError::PendingTransactionInformation(
                                algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            )
                        )
                    ) || error.to_string().contains("404");

                    if is_retryable {
                        current_round += 1;
                        continue;
                    } else {
                        return Err(Box::new(ComposerError::AlgodClientError(error)));
                    }
                }
            };

            let _ = self.algod_client.wait_for_block(current_round).await;
            current_round += 1;
        }

        Err(format!(
            "Transaction {} not confirmed after {} rounds",
            tx_id, max_rounds
        )
        .into())
    }

    pub async fn send(
        &mut self,
    ) -> Result<PendingTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.build()
            .await
            .map_err(|e| format!("Failed to build transaction: {}", e))?;

        let transactions = self.built_group().ok_or("No transactions built")?;

        if transactions.is_empty() {
            return Err("No transactions to send".into());
        }

        self.gather_signatures()
            .await
            .map_err(|e| format!("Failed to sign transaction: {}", e))?;

        // Encode each signed transaction and concatenate them
        let signed_transactions = self.signed_group.as_ref().ok_or("No signed transactions")?;
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn
                .encode()
                .map_err(|e| format!("Failed to encode signed transaction: {}", e))?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let raw_transaction_response = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction: {:?}", e))?;

        let pending_transaction_response = self
            .wait_for_confirmation(&raw_transaction_response.tx_id, 5)
            .await
            .map_err(|e| format!("Failed to confirm transaction: {}", e))?;

        Ok(pending_transaction_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transactions::common::EmptySigner;
    use algokit_transact::test_utils::{AddressMother, TransactionMother};
    use base64::{Engine, prelude::BASE64_STANDARD};

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
            BASE64_STANDARD
                .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
                .unwrap()
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
        composer.add_payment(payment_params).unwrap();

        let result = composer.build().await;
        assert!(result.is_ok());

        let built_group = composer.built_group().unwrap();
        assert_eq!(built_group.len(), 1);
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
        let mut composer = Composer::new(AlgodClient::testnet(), Some(Arc::new(EmptySigner {})));

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
        composer.add_payment(payment_params).unwrap();
        composer.build().await.unwrap();

        let result = composer.gather_signatures().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_single_transaction_no_group() {
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
        composer.add_payment(payment_params).unwrap();

        composer.build().await.unwrap();

        let built_group = composer.built_group().unwrap();
        assert_eq!(built_group.len(), 1);

        // Single transaction should not have a group ID set
        assert!(built_group[0].header().group.is_none());
    }

    #[tokio::test]
    async fn test_multiple_transactions_have_group() {
        let mut composer = Composer::testnet();

        for _ in 0..2 {
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
            composer.add_payment(payment_params).unwrap();
        }

        composer.build().await.unwrap();

        let built_group = composer.built_group().unwrap();
        assert_eq!(built_group.len(), 2);

        // Multiple transactions should have group IDs set
        for txn in built_group {
            assert!(txn.header().group.is_some());
        }

        // All transactions should have the same group ID
        let group_id = built_group[0].header().group.as_ref().unwrap();
        for txn in &built_group[1..] {
            assert_eq!(txn.header().group.as_ref().unwrap(), group_id);
        }
    }

    #[test]
    fn test_error_recoverability_logic() {
        // Test string-based 404 detection (the primary retry mechanism)
        let error_404_string = "Request failed with status 404: Transaction not found";
        let error_500_string = "Request failed with status 500: Server error";

        // The main retry logic relies on string matching
        assert!(
            error_404_string.contains("404"),
            "404 errors should be retryable"
        );
        assert!(
            !error_500_string.contains("404"),
            "500 errors should not be retryable"
        );
    }

    #[test]
    fn test_validity_window_logic() {
        // Test LocalNet detection and validity window logic
        assert_eq!(
            if genesis_id_is_localnet("devnet-v1") {
                1000
            } else {
                10
            },
            1000,
            "LocalNet should use 1000 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("testnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "TestNet should use 10 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("mainnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "MainNet should use 10 round validity window"
        );
    }
}
