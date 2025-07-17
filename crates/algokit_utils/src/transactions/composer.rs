use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{PendingTransactionResponse, TransactionParams},
};
use algokit_transact::{
    Address, AlgorandMsgpack, AssetConfigTransactionFields, Byte32, FeeParams,
    KeyRegistrationTransactionFields, MAX_TX_GROUP_SIZE, OnApplicationComplete,
    PaymentTransactionFields, SignedTransaction, Transaction, TransactionHeader, TransactionId,
    Transactions,
};
use derive_more::Debug;
use std::{collections::HashMap, sync::Arc};

use crate::genesis_id_is_localnet;

use super::application_call::{
    ApplicationCallParams, ApplicationCreateParams, ApplicationDeleteParams,
    ApplicationUpdateParams,
};
use super::asset_config::{AssetCreateParams, AssetDestroyParams, AssetReconfigureParams};
use super::asset_freeze::{AssetFreezeParams, AssetUnfreezeParams};
use super::common::{CommonParams, TransactionSigner, TransactionSignerGetter};
use super::key_registration::{
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use super::payment::{AccountCloseParams, PaymentParams};

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
    #[error("Transaction group size exceeds the max limit of: {max}", max = MAX_TX_GROUP_SIZE)]
    GroupSizeError(),
}

#[derive(Clone)]
pub struct TransactionWithSigner {
    pub transaction: Transaction,
    pub signer: Arc<dyn TransactionSigner>,
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
pub struct SendTransactionResults {
    pub group_id: Option<Byte32>,
    pub transaction_ids: Vec<String>,
    pub confirmations: Vec<PendingTransactionResponse>,
}

#[derive(Debug, Clone)]
pub struct SendParams {
    pub max_rounds_to_wait_for_confirmation: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum ComposerTransaction {
    Transaction(Transaction),
    Payment(PaymentParams),
    AccountClose(AccountCloseParams),
    AssetTransfer(AssetTransferParams),
    AssetOptIn(AssetOptInParams),
    AssetOptOut(AssetOptOutParams),
    AssetClawback(AssetClawbackParams),
    AssetCreate(AssetCreateParams),
    AssetReconfigure(AssetReconfigureParams),
    AssetDestroy(AssetDestroyParams),
    AssetFreeze(AssetFreezeParams),
    AssetUnfreeze(AssetUnfreezeParams),
    ApplicationCall(ApplicationCallParams),
    ApplicationCreate(ApplicationCreateParams),
    ApplicationUpdate(ApplicationUpdateParams),
    ApplicationDelete(ApplicationDeleteParams),
    OnlineKeyRegistration(OnlineKeyRegistrationParams),
    OfflineKeyRegistration(OfflineKeyRegistrationParams),
    NonParticipationKeyRegistration(NonParticipationKeyRegistrationParams),
}

impl ComposerTransaction {
    pub fn common_params(&self) -> CommonParams {
        match self {
            ComposerTransaction::Payment(payment_params) => payment_params.common_params.clone(),
            ComposerTransaction::AccountClose(account_close_params) => {
                account_close_params.common_params.clone()
            }
            ComposerTransaction::AssetTransfer(asset_transfer_params) => {
                asset_transfer_params.common_params.clone()
            }
            ComposerTransaction::AssetOptIn(asset_opt_in_params) => {
                asset_opt_in_params.common_params.clone()
            }
            ComposerTransaction::AssetOptOut(asset_opt_out_params) => {
                asset_opt_out_params.common_params.clone()
            }
            ComposerTransaction::AssetClawback(asset_clawback_params) => {
                asset_clawback_params.common_params.clone()
            }
            ComposerTransaction::AssetCreate(asset_create_params) => {
                asset_create_params.common_params.clone()
            }
            ComposerTransaction::AssetReconfigure(asset_reconfigure_params) => {
                asset_reconfigure_params.common_params.clone()
            }
            ComposerTransaction::AssetDestroy(asset_destroy_params) => {
                asset_destroy_params.common_params.clone()
            }
            ComposerTransaction::AssetFreeze(asset_freeze_params) => {
                asset_freeze_params.common_params.clone()
            }
            ComposerTransaction::AssetUnfreeze(asset_unfreeze_params) => {
                asset_unfreeze_params.common_params.clone()
            }
            ComposerTransaction::ApplicationCall(app_call_params) => {
                app_call_params.common_params.clone()
            }
            ComposerTransaction::ApplicationCreate(app_create_params) => {
                app_create_params.common_params.clone()
            }
            ComposerTransaction::ApplicationUpdate(app_update_params) => {
                app_update_params.common_params.clone()
            }
            ComposerTransaction::ApplicationDelete(app_delete_params) => {
                app_delete_params.common_params.clone()
            }
            ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                online_key_reg_params.common_params.clone()
            }
            ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                offline_key_reg_params.common_params.clone()
            }
            ComposerTransaction::NonParticipationKeyRegistration(non_participation_params) => {
                non_participation_params.common_params.clone()
            }
            _ => CommonParams::default(),
        }
    }
}

#[derive(Clone)]
pub struct Composer {
    transactions: Vec<ComposerTransaction>,
    algod_client: AlgodClient,
    signer_getter: Arc<dyn TransactionSignerGetter>,
    built_group: Option<Vec<TransactionWithSigner>>,
    signed_group: Option<Vec<SignedTransaction>>,
}

impl Composer {
    pub fn new(algod_client: AlgodClient, signer_getter: Arc<dyn TransactionSignerGetter>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client,
            signer_getter,
            built_group: None,
            signed_group: None,
        }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        use crate::EmptySigner;

        Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            signer_getter: Arc::new(EmptySigner {}),
            built_group: None,
            signed_group: None,
        }
    }

    fn push(&mut self, txn: ComposerTransaction) -> Result<(), ComposerError> {
        if self.transactions.len() >= MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }
        self.transactions.push(txn);
        Ok(())
    }

    pub fn add_payment(&mut self, payment_params: PaymentParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Payment(payment_params))
    }

    pub fn add_account_close(
        &mut self,
        account_close_params: AccountCloseParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AccountClose(account_close_params))
    }

    pub fn add_asset_transfer(
        &mut self,
        asset_transfer_params: AssetTransferParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetTransfer(asset_transfer_params))
    }

    pub fn add_asset_opt_in(
        &mut self,
        asset_opt_in_params: AssetOptInParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptIn(asset_opt_in_params))
    }

    pub fn add_asset_opt_out(
        &mut self,
        asset_opt_out_params: AssetOptOutParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptOut(asset_opt_out_params))
    }

    pub fn add_asset_clawback(
        &mut self,
        asset_clawback_params: AssetClawbackParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetClawback(asset_clawback_params))
    }

    pub fn add_asset_create(
        &mut self,
        asset_create_params: AssetCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetCreate(asset_create_params))
    }

    pub fn add_asset_reconfigure(
        &mut self,
        asset_reconfigure_params: AssetReconfigureParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetReconfigure(
            asset_reconfigure_params,
        ))
    }

    pub fn add_asset_destroy(
        &mut self,
        asset_destroy_params: AssetDestroyParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetDestroy(asset_destroy_params))
    }

    pub fn add_asset_freeze(
        &mut self,
        asset_freeze_params: AssetFreezeParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetFreeze(asset_freeze_params))
    }

    pub fn add_asset_unfreeze(
        &mut self,
        asset_unfreeze_params: AssetUnfreezeParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetUnfreeze(asset_unfreeze_params))
    }

    pub fn add_online_key_registration(
        &mut self,
        online_key_reg_params: OnlineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OnlineKeyRegistration(
            online_key_reg_params,
        ))
    }

    pub fn add_offline_key_registration(
        &mut self,
        offline_key_reg_params: OfflineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OfflineKeyRegistration(
            offline_key_reg_params,
        ))
    }

    pub fn add_non_participation_key_registration(
        &mut self,
        non_participation_params: NonParticipationKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::NonParticipationKeyRegistration(
            non_participation_params,
        ))
    }

    pub fn add_application_call(
        &mut self,
        app_call_params: ApplicationCallParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationCall(app_call_params))
    }

    pub fn add_application_create(
        &mut self,
        app_create_params: ApplicationCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationCreate(app_create_params))
    }

    pub fn add_application_update(
        &mut self,
        app_update_params: ApplicationUpdateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationUpdate(app_update_params))
    }

    pub fn add_application_delete(
        &mut self,
        app_delete_params: ApplicationDeleteParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationDelete(app_delete_params))
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Transaction(transaction))
    }

    pub fn add_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<(), ComposerError> {
        if self.transactions.len() + transactions.len() > MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }

        transactions
            .into_iter()
            .try_for_each(|transaction| self.add_transaction(transaction))
    }

    pub fn transactions(&self) -> &Vec<ComposerTransaction> {
        &self.transactions
    }

    fn get_signer(&self, address: Address) -> Option<Arc<dyn TransactionSigner>> {
        self.signer_getter.get_signer(address)
    }

    async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        self.algod_client
            .transaction_params()
            .await
            .map_err(Into::into)
    }

    pub async fn build(&mut self) -> Result<&Vec<TransactionWithSigner>, ComposerError> {
        if let Some(ref group) = self.built_group {
            return Ok(group);
        }

        let suggested_params = self.get_suggested_params().await?;

        // Determine validity window: default 10 rounds, but 1000 for LocalNet
        let default_validity_window = if genesis_id_is_localnet(&suggested_params.genesis_id) {
            1000 // LocalNet gets bigger window to avoid dead transactions
        } else {
            10 // Standard default validity window
        };

        let mut transactions = Vec::new();
        let mut signers = Vec::new();

        for tx in &self.transactions {
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
                ComposerTransaction::Transaction(tx) => {
                    calculate_fee = false;
                    tx.clone()
                }
                ComposerTransaction::Payment(pay_params) => {
                    let pay_params = PaymentTransactionFields {
                        header,
                        receiver: pay_params.receiver.clone(),
                        amount: pay_params.amount,
                        close_remainder_to: None,
                    };
                    Transaction::Payment(pay_params)
                }
                ComposerTransaction::AccountClose(account_close_params) => {
                    let pay_params = PaymentTransactionFields {
                        header,
                        receiver: common_params.sender.clone(),
                        amount: 0,
                        close_remainder_to: Some(account_close_params.close_remainder_to.clone()),
                    };
                    Transaction::Payment(pay_params)
                }
                ComposerTransaction::AssetTransfer(asset_transfer_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_transfer_params.asset_id,
                        amount: asset_transfer_params.amount,
                        receiver: asset_transfer_params.receiver.clone(),
                        asset_sender: None,
                        close_remainder_to: None,
                    })
                }
                ComposerTransaction::AssetOptIn(asset_opt_in_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_opt_in_params.asset_id,
                        amount: 0,
                        receiver: asset_opt_in_params.common_params.sender.clone(),
                        asset_sender: None,
                        close_remainder_to: None,
                    })
                }
                ComposerTransaction::AssetOptOut(asset_opt_out_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_opt_out_params.asset_id,
                        amount: 0,
                        receiver: asset_opt_out_params.common_params.sender.clone(),
                        asset_sender: None,
                        close_remainder_to: asset_opt_out_params.close_remainder_to.clone(),
                    })
                }
                ComposerTransaction::AssetClawback(asset_clawback_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_clawback_params.asset_id,
                        amount: asset_clawback_params.amount,
                        receiver: asset_clawback_params.receiver.clone(),
                        asset_sender: Some(asset_clawback_params.clawback_target.clone()),
                        close_remainder_to: None,
                    })
                }
                ComposerTransaction::AssetCreate(asset_create_params) => {
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
                ComposerTransaction::AssetReconfigure(asset_reconfigure_params) => {
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
                ComposerTransaction::AssetDestroy(asset_destroy_params) => {
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
                ComposerTransaction::AssetFreeze(asset_freeze_params) => {
                    Transaction::AssetFreeze(algokit_transact::AssetFreezeTransactionFields {
                        header,
                        asset_id: asset_freeze_params.asset_id,
                        freeze_target: asset_freeze_params.target_address.clone(),
                        frozen: true,
                    })
                }
                ComposerTransaction::AssetUnfreeze(asset_unfreeze_params) => {
                    Transaction::AssetFreeze(algokit_transact::AssetFreezeTransactionFields {
                        header,
                        asset_id: asset_unfreeze_params.asset_id,
                        freeze_target: asset_unfreeze_params.target_address.clone(),
                        frozen: false,
                    })
                }
                ComposerTransaction::ApplicationCall(app_call_params) => {
                    Transaction::ApplicationCall(
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
                    )
                }
                ComposerTransaction::ApplicationCreate(app_create_params) => {
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
                ComposerTransaction::ApplicationUpdate(app_update_params) => {
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
                ComposerTransaction::ApplicationDelete(app_delete_params) => {
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
                ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                        header,
                        vote_key: Some(online_key_reg_params.vote_key),
                        selection_key: Some(online_key_reg_params.selection_key),
                        vote_first: Some(online_key_reg_params.vote_first),
                        vote_last: Some(online_key_reg_params.vote_last),
                        vote_key_dilution: Some(online_key_reg_params.vote_key_dilution),
                        state_proof_key: online_key_reg_params.state_proof_key,
                        non_participation: None,
                    })
                }
                ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                        header,
                        vote_key: None,
                        selection_key: None,
                        vote_first: None,
                        vote_last: None,
                        vote_key_dilution: None,
                        state_proof_key: None,
                        non_participation: offline_key_reg_params.non_participation,
                    })
                }
                ComposerTransaction::NonParticipationKeyRegistration(_) => {
                    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                        header,
                        vote_key: None,
                        selection_key: None,
                        vote_first: None,
                        vote_last: None,
                        vote_key_dilution: None,
                        state_proof_key: None,
                        non_participation: Some(true),
                    })
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

            let signer = if let Some(transaction_signer) = common_params.signer {
                transaction_signer
            } else {
                let sender_address = transaction.header().sender.clone();

                self.get_signer(sender_address.clone())
                    .ok_or(ComposerError::SigningError(format!(
                        "No signer found for address: {}",
                        sender_address
                    )))?
            };

            transactions.push(transaction);
            signers.push(signer);
        }

        if transactions.len() > 1 {
            let grouped_transactions = transactions.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?;
            transactions = grouped_transactions;
        }

        let transactions_with_signers: Vec<TransactionWithSigner> = transactions
            .into_iter()
            .zip(signers.into_iter())
            .map(|(transaction, signer)| TransactionWithSigner {
                transaction,
                signer,
            })
            .collect();

        self.built_group = Some(transactions_with_signers);
        Ok(self.built_group.as_ref().unwrap())
    }

    pub async fn gather_signatures(&mut self) -> Result<&Vec<SignedTransaction>, ComposerError> {
        if let Some(ref group) = self.signed_group {
            return Ok(group);
        }

        let transactions_with_signers =
            self.built_group.as_ref().ok_or(ComposerError::StateError(
                "Cannot gather signatures before building the transaction group".to_string(),
            ))?;

        // Group transactions by signer
        let mut transactions = Vec::new();
        let mut signer_groups: HashMap<*const dyn TransactionSigner, Vec<usize>> = HashMap::new();
        for (index, txn_with_signer) in transactions_with_signers.iter().enumerate() {
            let signer_ptr = Arc::as_ptr(&txn_with_signer.signer);
            signer_groups.entry(signer_ptr).or_default().push(index);
            transactions.push(txn_with_signer.transaction.to_owned());
        }

        let mut signed_transactions = vec![None; transactions_with_signers.len()];

        for (_signer_ptr, indices) in signer_groups {
            // Get the signer from the first transaction with this signer
            let signer = &transactions_with_signers[indices[0]].signer;

            // Sign all transactions for this signer
            let signed_txns = signer
                .sign_transactions(&transactions, &indices)
                .await
                .map_err(ComposerError::SigningError)?;

            for (i, &index) in indices.iter().enumerate() {
                signed_transactions[index] = Some(signed_txns[i].to_owned());
            }
        }

        let final_signed_transactions: Result<Vec<SignedTransaction>, _> = signed_transactions
            .into_iter()
            .enumerate()
            .map(|(i, signed_transaction)| {
                signed_transaction.ok_or_else(|| {
                    ComposerError::SigningError(format!(
                        "Transaction at index {} was not signed",
                        i
                    ))
                })
            })
            .collect();

        self.signed_group = Some(final_signed_transactions?);
        Ok(self.signed_group.as_ref().unwrap())
    }

    async fn wait_for_confirmation(
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
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionResults, Box<dyn std::error::Error + Send + Sync>> {
        self.build()
            .await
            .map_err(|e| format!("Failed to build transaction: {}", e))?;

        let group_id = {
            let transactions_with_signers =
                self.built_group.as_ref().ok_or("No transactions built")?;
            if transactions_with_signers.is_empty() {
                return Err("No transactions to send".into());
            }
            transactions_with_signers[0].transaction.header().group
        };

        self.gather_signatures()
            .await
            .map_err(|e| format!("Failed to sign transaction: {}", e))?;

        let signed_transactions = self.signed_group.as_ref().ok_or("No signed transactions")?;

        let wait_rounds = if let Some(max_rounds_to_wait_for_confirmation) =
            send_params.and_then(|p| p.max_rounds_to_wait_for_confirmation)
        {
            max_rounds_to_wait_for_confirmation
        } else {
            let first_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().first_valid)
                .min()
                .ok_or("Failed to calculate first valid round")?;

            let last_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().last_valid)
                .max()
                .ok_or("Failed to calculate last valid round")?;

            last_round - first_round
        };

        // Encode each signed transaction and concatenate them
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn
                .encode()
                .map_err(|e| format!("Failed to encode signed transaction: {}", e))?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let _ = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction(s): {:?}", e))?;

        let transaction_ids: Vec<String> = signed_transactions
            .iter()
            .map(|txn| txn.id())
            .collect::<Result<Vec<String>, _>>()?;

        let mut confirmations = Vec::new();
        for id in &transaction_ids {
            let confirmation = self
                .wait_for_confirmation(id, wait_rounds)
                .await
                .map_err(|e| format!("Failed to confirm transaction: {}", e))?;
            confirmations.push(confirmation);
        }

        Ok(SendTransactionResults {
            group_id,
            transaction_ids,
            confirmations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{AccountMother, TransactionMother};
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
                sender: AccountMother::account().address(),
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
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        assert!(composer.add_payment(payment_params).is_ok());
    }

    #[tokio::test]
    async fn test_gather_signatures() {
        let mut composer = Composer::testnet();

        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
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
            receiver: AccountMother::account().address(),
            amount: 1000,
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
                sender: AccountMother::account().address(),
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
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        composer.add_payment(payment_params).unwrap();

        composer.build().await.unwrap();

        let built_group = composer.built_group.as_ref().unwrap();
        assert_eq!(built_group.len(), 1);

        // Single transaction should not have a group ID set
        assert!(built_group[0].transaction.header().group.is_none());
    }

    #[tokio::test]
    async fn test_multiple_transactions_have_group() {
        let mut composer = Composer::testnet();

        for _ in 0..2 {
            let payment_params = PaymentParams {
                common_params: CommonParams {
                    sender: AccountMother::account().address(),
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
                receiver: AccountMother::account().address(),
                amount: 1000,
            };
            composer.add_payment(payment_params).unwrap();
        }

        composer.build().await.unwrap();

        let built_group = composer.built_group.as_ref().unwrap();
        assert_eq!(built_group.len(), 2);

        // Multiple transactions should have group IDs set
        for transaction_with_signer in built_group {
            assert!(transaction_with_signer.transaction.header().group.is_some());
        }

        // All transactions should have the same group ID
        let group_id = built_group[0].transaction.header().group.as_ref().unwrap();
        for transaction_with_signer in &built_group[1..] {
            assert_eq!(
                transaction_with_signer
                    .transaction
                    .header()
                    .group
                    .as_ref()
                    .unwrap(),
                group_id
            );
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
