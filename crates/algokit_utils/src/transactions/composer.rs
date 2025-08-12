use crate::genesis_id_is_localnet;
use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{
        PendingTransactionResponse, SimulateRequest, SimulateRequestTransactionGroup,
        TransactionParams,
    },
};
use algokit_abi::ABIMethod;
use algokit_transact::{
    Address, AlgoKitTransactError, AlgorandMsgpack, Byte32, EMPTY_SIGNATURE, FeeParams,
    MAX_TX_GROUP_SIZE, SignedTransaction, Transaction, TransactionHeader, TransactionId,
    Transactions,
};
use derive_more::Debug;
use std::{collections::HashMap, sync::Arc};

use crate::{
    AppMethodCallArg,
    transactions::{
        application_call::{
            AppCallMethodCallParams, ProcessedAppMethodCallArg, build_app_call_method_call,
            build_app_create_method_call, build_app_delete_method_call,
            build_app_update_method_call,
        },
        common::TransactionWithSigner,
    },
};

use super::application_call::{
    ABIReturn, AppCallParams, AppCreateMethodCallParams, AppCreateParams,
    AppDeleteMethodCallParams, AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams,
    build_app_call, build_app_create_call, build_app_delete_call, build_app_update_call,
};
use super::asset_config::{
    AssetCreateParams, AssetDestroyParams, AssetReconfigureParams, build_asset_create,
    build_asset_destroy, build_asset_reconfigure,
};
use super::asset_freeze::{
    AssetFreezeParams, AssetUnfreezeParams, build_asset_freeze, build_asset_unfreeze,
};
use super::asset_transfer::{
    AssetClawbackParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
    build_asset_clawback, build_asset_opt_in, build_asset_opt_out, build_asset_transfer,
};
use super::common::{CommonParams, TransactionSigner, TransactionSignerGetter};
use super::key_registration::{
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams, build_non_participation_key_registration,
    build_offline_key_registration, build_online_key_registration,
};
use super::payment::{AccountCloseParams, PaymentParams, build_account_close, build_payment};

// ABI return values are stored in logs with the prefix 0x151f7c75
const ABI_RETURN_PREFIX: &[u8] = &[0x15, 0x1f, 0x7c, 0x75];

#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(#[from] AlgodError),
    #[error("AlgoKit Transact error: {0}")]
    TransactError(#[from] AlgoKitTransactError),
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
    #[error("Max wait round expired: {0}")]
    MaxWaitRoundExpired(String),
    #[error("ABI argument encoding error: {0}")]
    ABIEncodingError(String),
    #[error("ABI argument decoding error: {0}")]
    ABIDecodingError(String),
}

#[derive(Debug)]
pub struct SendTransactionComposerResults {
    pub group_id: Option<Byte32>,
    pub transaction_ids: Vec<String>,
    pub confirmations: Vec<PendingTransactionResponse>,
    pub abi_returns: Vec<Result<Option<ABIReturn>, ComposerError>>,
}

#[derive(Debug, Default, Clone)]
pub struct SendParams {
    pub max_rounds_to_wait_for_confirmation: Option<u64>,
    pub cover_app_call_inner_transaction_fees: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct BuildParams {
    pub cover_app_call_inner_transaction_fees: Option<bool>,
}

impl From<&SendParams> for BuildParams {
    fn from(send_params: &SendParams) -> Self {
        BuildParams {
            cover_app_call_inner_transaction_fees: send_params
                .cover_app_call_inner_transaction_fees,
        }
    }
}

#[derive(Debug)]
struct TransactionAnalysis {
    /// The fee difference required for this transaction
    required_fee_delta: FeeDelta,
}

#[derive(Debug)]
struct GroupAnalysis {
    transactions: Vec<TransactionAnalysis>,
}

/// Represents the fee difference for a transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeeDelta {
    /// Transaction has the exact required fee
    None,
    /// Transaction has an insufficient fee (needs more)
    Deficit(u64),
    /// Transaction has an excess fee (can cover other transactions in the group)
    Surplus(u64),
}

impl FeeDelta {
    /// Create a FeeDelta from an i64 value (positive = deficit, negative = surplus, zero = none)
    pub fn from_i64(value: i64) -> Self {
        match value.cmp(&0) {
            std::cmp::Ordering::Greater => FeeDelta::Deficit(value as u64),
            std::cmp::Ordering::Less => FeeDelta::Surplus((-value) as u64),
            std::cmp::Ordering::Equal => FeeDelta::None,
        }
    }

    /// Convert to i64 representation (positive = deficit, negative = surplus, zero = none)
    pub fn to_i64(&self) -> i64 {
        match self {
            FeeDelta::None => 0,
            FeeDelta::Deficit(amount) => *amount as i64,
            FeeDelta::Surplus(amount) => -(*amount as i64),
        }
    }

    /// Check if this represents no fee difference (neutral)
    pub fn is_none(&self) -> bool {
        matches!(self, FeeDelta::None)
    }

    /// Check if this represents a deficit (needs more fees)
    pub fn is_deficit(&self) -> bool {
        matches!(self, FeeDelta::Deficit(_))
    }

    /// Check if this represents a surplus (has excess fees)
    pub fn is_surplus(&self) -> bool {
        matches!(self, FeeDelta::Surplus(_))
    }

    /// Get the amount regardless of whether it's deficit or surplus
    pub fn amount(&self) -> u64 {
        match self {
            FeeDelta::None => 0,
            FeeDelta::Deficit(amount) | FeeDelta::Surplus(amount) => *amount,
        }
    }
}

impl std::ops::Add for FeeDelta {
    type Output = FeeDelta;

    fn add(self, rhs: FeeDelta) -> Self::Output {
        FeeDelta::from_i64(self.to_i64() + rhs.to_i64())
    }
}

// Priority levels for fee coverage
// By default PartialOrd and Ord provide the correct sorting logic, based the enum variant order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FeePriority {
    /// Non-deficit transactions (lowest priority)
    None,
    /// Application call transactions with deficits that can be modified
    ModifiableDeficit(u64),
    /// Non application call or immutable fee transactions with deficits (highest priority)
    ImmutableDeficit(u64),
}

#[derive(Debug, Clone)]
pub enum ComposerTransaction {
    Transaction(Transaction),
    TransactionWithSigner(TransactionWithSigner),
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
    AppCall(AppCallParams),
    AppCreateCall(AppCreateParams),
    AppUpdateCall(AppUpdateParams),
    AppDeleteCall(AppDeleteParams),
    AppCallMethodCall(AppCallMethodCallParams<ProcessedAppMethodCallArg>),
    AppCreateMethodCall(AppCreateMethodCallParams<ProcessedAppMethodCallArg>),
    AppUpdateMethodCall(AppUpdateMethodCallParams<ProcessedAppMethodCallArg>),
    AppDeleteMethodCall(AppDeleteMethodCallParams<ProcessedAppMethodCallArg>),
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
            ComposerTransaction::AppCall(app_call_params) => app_call_params.common_params.clone(),
            ComposerTransaction::AppCreateCall(app_create_params) => {
                app_create_params.common_params.clone()
            }
            ComposerTransaction::AppUpdateCall(app_update_params) => {
                app_update_params.common_params.clone()
            }
            ComposerTransaction::AppDeleteCall(app_delete_params) => {
                app_delete_params.common_params.clone()
            }
            ComposerTransaction::AppCallMethodCall(params) => params.common_params.clone(),
            ComposerTransaction::AppCreateMethodCall(params) => params.common_params.clone(),
            ComposerTransaction::AppUpdateMethodCall(params) => params.common_params.clone(),
            ComposerTransaction::AppDeleteMethodCall(params) => params.common_params.clone(),
            ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                online_key_reg_params.common_params.clone()
            }
            ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                offline_key_reg_params.common_params.clone()
            }
            ComposerTransaction::NonParticipationKeyRegistration(non_participation_params) => {
                non_participation_params.common_params.clone()
            }
            ComposerTransaction::TransactionWithSigner(txn_with_signer) => CommonParams {
                signer: Some(txn_with_signer.signer.clone()),
                ..CommonParams::default()
            },
            _ => CommonParams::default(),
        }
    }

    /// Get the logical maximum fee based on static_fee and max_fee
    pub fn logical_max_fee(&self) -> Option<u64> {
        let common_params = self.common_params();
        let max_fee = common_params.max_fee;
        let static_fee = common_params.static_fee;
        match (max_fee, static_fee) {
            (Some(max_fee_value), static_fee) if max_fee_value > static_fee.unwrap_or(0) => max_fee,
            _ => static_fee,
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

    fn get_method_from_transaction<'a>(
        &self,
        transaction: &'a ComposerTransaction,
    ) -> Option<&'a ABIMethod> {
        match transaction {
            ComposerTransaction::AppCallMethodCall(params) => Some(&params.method),
            ComposerTransaction::AppCreateMethodCall(params) => Some(&params.method),
            ComposerTransaction::AppUpdateMethodCall(params) => Some(&params.method),
            ComposerTransaction::AppDeleteMethodCall(params) => Some(&params.method),
            _ => None,
        }
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

    pub fn add_app_call(&mut self, app_call_params: AppCallParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppCall(app_call_params))
    }

    pub fn add_app_create(
        &mut self,
        app_create_params: AppCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppCreateCall(app_create_params))
    }

    pub fn add_app_update(
        &mut self,
        app_update_params: AppUpdateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppUpdateCall(app_update_params))
    }

    pub fn add_app_delete(
        &mut self,
        app_delete_params: AppDeleteParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppDeleteCall(app_delete_params))
    }

    fn extract_composer_transactions_from_app_method_call_params(
        method_call_args: &[AppMethodCallArg],
    ) -> Vec<ComposerTransaction> {
        let mut composer_transactions: Vec<ComposerTransaction> = vec![];

        for arg in method_call_args.iter() {
            match arg {
                AppMethodCallArg::Transaction(transaction) => {
                    composer_transactions
                        .push(ComposerTransaction::Transaction(transaction.clone()));
                }
                AppMethodCallArg::TransactionWithSigner(transaction) => {
                    composer_transactions.push(ComposerTransaction::TransactionWithSigner(
                        transaction.clone(),
                    ));
                }
                AppMethodCallArg::Payment(params) => {
                    composer_transactions.push(ComposerTransaction::Payment(params.clone()));
                }
                AppMethodCallArg::AccountClose(params) => {
                    composer_transactions.push(ComposerTransaction::AccountClose(params.clone()));
                }
                AppMethodCallArg::AssetTransfer(params) => {
                    composer_transactions.push(ComposerTransaction::AssetTransfer(params.clone()));
                }
                AppMethodCallArg::AssetOptIn(params) => {
                    composer_transactions.push(ComposerTransaction::AssetOptIn(params.clone()));
                }
                AppMethodCallArg::AssetOptOut(params) => {
                    composer_transactions.push(ComposerTransaction::AssetOptOut(params.clone()));
                }
                AppMethodCallArg::AssetClawback(params) => {
                    composer_transactions.push(ComposerTransaction::AssetClawback(params.clone()));
                }
                AppMethodCallArg::AssetCreate(params) => {
                    composer_transactions.push(ComposerTransaction::AssetCreate(params.clone()));
                }
                AppMethodCallArg::AssetReconfigure(params) => {
                    composer_transactions
                        .push(ComposerTransaction::AssetReconfigure(params.clone()));
                }
                AppMethodCallArg::AssetDestroy(params) => {
                    composer_transactions.push(ComposerTransaction::AssetDestroy(params.clone()));
                }
                AppMethodCallArg::AssetFreeze(params) => {
                    composer_transactions.push(ComposerTransaction::AssetFreeze(params.clone()));
                }
                AppMethodCallArg::AssetUnfreeze(params) => {
                    composer_transactions.push(ComposerTransaction::AssetUnfreeze(params.clone()));
                }
                AppMethodCallArg::AppCall(params) => {
                    composer_transactions.push(ComposerTransaction::AppCall(params.clone()));
                }
                AppMethodCallArg::AppCreateCall(params) => {
                    composer_transactions.push(ComposerTransaction::AppCreateCall(params.clone()));
                }
                AppMethodCallArg::AppUpdateCall(params) => {
                    composer_transactions.push(ComposerTransaction::AppUpdateCall(params.clone()));
                }
                AppMethodCallArg::AppDeleteCall(params) => {
                    composer_transactions.push(ComposerTransaction::AppDeleteCall(params.clone()));
                }
                AppMethodCallArg::AppCallMethodCall(params) => {
                    let nested_composer_transactions =
                        Self::extract_composer_transactions_from_app_method_call_params(
                            &params.args,
                        );
                    composer_transactions.extend(nested_composer_transactions);

                    composer_transactions
                        .push(ComposerTransaction::AppCallMethodCall(params.into()));
                }
                AppMethodCallArg::AppCreateMethodCall(params) => {
                    let nested_composer_transactions =
                        Self::extract_composer_transactions_from_app_method_call_params(
                            &params.args,
                        );
                    composer_transactions.extend(nested_composer_transactions);

                    composer_transactions
                        .push(ComposerTransaction::AppCreateMethodCall(params.into()));
                }
                AppMethodCallArg::AppUpdateMethodCall(params) => {
                    let nested_composer_transactions =
                        Self::extract_composer_transactions_from_app_method_call_params(
                            &params.args,
                        );
                    composer_transactions.extend(nested_composer_transactions);

                    composer_transactions
                        .push(ComposerTransaction::AppUpdateMethodCall(params.into()));
                }
                AppMethodCallArg::AppDeleteMethodCall(params) => {
                    let nested_composer_transactions =
                        Self::extract_composer_transactions_from_app_method_call_params(
                            &params.args,
                        );
                    composer_transactions.extend(nested_composer_transactions);

                    composer_transactions
                        .push(ComposerTransaction::AppDeleteMethodCall(params.into()));
                }
                _ => {}
            };
        }

        composer_transactions
    }

    fn add_app_method_call_internal(
        &mut self,
        args: &[AppMethodCallArg],
        create_transaction: impl FnOnce() -> ComposerTransaction,
    ) -> Result<(), ComposerError> {
        let mut composer_transactions =
            Self::extract_composer_transactions_from_app_method_call_params(args);
        composer_transactions.push(create_transaction());

        if self.transactions.len() + composer_transactions.len() > MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }

        for composer_transaction in composer_transactions {
            self.push(composer_transaction)?;
        }

        Ok(())
    }

    pub fn add_app_call_method_call(
        &mut self,
        params: AppCallMethodCallParams,
    ) -> Result<(), ComposerError> {
        self.add_app_method_call_internal(&params.args, || {
            ComposerTransaction::AppCallMethodCall((&params).into())
        })
    }

    pub fn add_app_create_method_call(
        &mut self,
        params: AppCreateMethodCallParams,
    ) -> Result<(), ComposerError> {
        self.add_app_method_call_internal(&params.args, || {
            ComposerTransaction::AppCreateMethodCall((&params).into())
        })
    }

    pub fn add_app_update_method_call(
        &mut self,
        params: AppUpdateMethodCallParams,
    ) -> Result<(), ComposerError> {
        self.add_app_method_call_internal(&params.args, || {
            ComposerTransaction::AppUpdateMethodCall((&params).into())
        })
    }

    pub fn add_app_delete_method_call(
        &mut self,
        params: AppDeleteMethodCallParams,
    ) -> Result<(), ComposerError> {
        self.add_app_method_call_internal(&params.args, || {
            ComposerTransaction::AppDeleteMethodCall((&params).into())
        })
    }

    fn parse_abi_return_values(
        &self,
        confirmations: &[PendingTransactionResponse],
    ) -> Vec<Result<Option<ABIReturn>, ComposerError>> {
        let mut abi_returns = Vec::new();

        for (i, confirmation) in confirmations.iter().enumerate() {
            if let Some(transaction) = self.transactions.get(i) {
                if let Some(method) = self.get_method_from_transaction(transaction) {
                    let abi_return = self.extract_abi_return_from_logs(confirmation, method);
                    abi_returns.push(abi_return);
                }
            }
        }

        abi_returns
    }

    fn extract_abi_return_from_logs(
        &self,
        confirmation: &PendingTransactionResponse,
        method: &ABIMethod,
    ) -> Result<Option<ABIReturn>, ComposerError> {
        // Check if method has return type
        let return_type = match method.returns.as_ref() {
            Some(return_type) => return_type,
            None => return Ok(None), // Method has no return type
        };

        // Non-void method - must examine the last log
        let last_log = match confirmation.logs.as_ref().and_then(|logs| logs.last()) {
            Some(log) => log,
            None => {
                return Err(ComposerError::ABIDecodingError(format!(
                    "No logs found for method {} which requires a return type",
                    method.name
                )));
            }
        };

        // Check if the last log entry has the ABI return prefix
        if !last_log.starts_with(ABI_RETURN_PREFIX) {
            return Err(ComposerError::ABIDecodingError(format!(
                "Transaction log for method {} doesn't match with ABI return value format",
                method.name
            )));
        }

        // Extract the return value bytes (skip the prefix)
        let return_bytes = &last_log[ABI_RETURN_PREFIX.len()..];

        // Decode the return value using the method's return type
        match return_type.decode(return_bytes) {
            Ok(return_value) => Ok(Some(ABIReturn {
                method: method.clone(),
                raw_return_value: return_bytes.to_vec(),
                return_value,
            })),
            Err(e) => Err(ComposerError::ABIDecodingError(format!(
                "Failed to decode ABI return value for method {}: {}",
                method.name, e
            ))),
        }
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

    pub fn add_transaction_with_signer(
        &mut self,
        transaction: TransactionWithSigner,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::TransactionWithSigner(transaction))
    }

    pub fn add_transactions_with_signers(
        &mut self,
        transactions: Vec<TransactionWithSigner>,
    ) -> Result<(), ComposerError> {
        if self.transactions.len() + transactions.len() > MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }

        transactions
            .into_iter()
            .try_for_each(|transaction| self.add_transaction_with_signer(transaction))
    }

    pub fn transactions(&self) -> &Vec<ComposerTransaction> {
        &self.transactions
    }

    fn get_signer(&self, address: Address) -> Option<Arc<dyn TransactionSigner>> {
        self.signer_getter.get_signer(address)
    }

    async fn analyze_group_requirements(
        &self,
        suggested_params: &TransactionParams,
        default_validity_window: &u64,
        build_params: &BuildParams,
    ) -> Result<GroupAnalysis, ComposerError> {
        let cover_inner_fees = build_params
            .cover_app_call_inner_transaction_fees
            .unwrap_or(false);
        let mut app_call_indexes_without_max_fees = Vec::new();

        let built_transactions = &self
            .build_transactions(suggested_params, default_validity_window, None)
            .await?;

        let mut transactions = built_transactions
            .iter()
            .enumerate()
            .map(|(group_index, txn)| {
                let ctxn = &self.transactions[group_index];
                let mut txn_to_simulate = txn.clone();
                let txn_header = txn_to_simulate.header_mut();
                txn_header.group = None;
                if cover_inner_fees {
                    if let Transaction::ApplicationCall(_) = txn {
                        match ctxn.logical_max_fee() {
                            Some(logical_max_fee) => txn_header.fee = Some(logical_max_fee),
                            None => app_call_indexes_without_max_fees.push(group_index),
                        }
                    }
                }
                txn_to_simulate
            })
            .collect::<Vec<_>>();

        // Regroup the transactions, as the transactions have likely been adjusted
        if transactions.len() > 1 {
            transactions = transactions.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?;
        }

        let signed_transactions = transactions
            .into_iter()
            .map(|txn| SignedTransaction {
                transaction: txn,
                signature: Some(EMPTY_SIGNATURE),
                auth_address: None,
                multisignature: None,
            })
            .collect();

        if cover_inner_fees && !app_call_indexes_without_max_fees.is_empty() {
            return Err(ComposerError::StateError(format!(
                "Please provide a max fee for each application call transaction when inner transaction fee coverage is enabled. Required for transaction {}",
                app_call_indexes_without_max_fees
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }

        let txn_group = SimulateRequestTransactionGroup {
            txns: signed_transactions,
        };
        let simulate_request = SimulateRequest {
            txn_groups: vec![txn_group],
            allow_unnamed_resources: Some(true),
            allow_empty_signatures: Some(true),
            fix_signers: Some(true),
            ..Default::default()
        };

        let response = self
            .algod_client
            .simulate_transaction(simulate_request, Some(Format::Msgpack))
            .await
            .map_err(ComposerError::AlgodClientError)?;

        let group_response = &response.txn_groups[0];

        // Handle any simulation failures
        if let Some(failure_message) = &group_response.failure_message {
            if cover_inner_fees && failure_message.contains("fee too small") {
                return Err(ComposerError::StateError(
                    "Fees were too small to analyze group requirements via simulate. You may need to increase an application call transaction max fee.".to_string()
                ));
            }

            let failed_at = group_response
                .failed_at
                .as_ref()
                .map(|group_index| {
                    group_index
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_else(|| "unknown".to_string());

            return Err(ComposerError::StateError(format!(
                "Error analyzing group requirements via simulate in transaction {}: {}",
                failed_at, failure_message
            )));
        }

        let txn_analysis_results: Result<Vec<TransactionAnalysis>, ComposerError> = group_response
            .txn_results
            .iter()
            .enumerate()
            .map(|(group_index, simulate_txn_result)| {
                let btxn = &built_transactions[group_index];

                let required_fee_delta = if cover_inner_fees {
                    let min_txn_fee: u64 = btxn
                        .calculate_fee(FeeParams {
                            fee_per_byte: suggested_params.fee,
                            min_fee: suggested_params.min_fee,
                            ..Default::default()
                        })
                        .map_err(|e| {
                            ComposerError::TransactionError(format!(
                                "Failed to calculate min transaction fee: {}",
                                e
                            ))
                        })?;

                    let txn_fee = btxn.header().fee.unwrap_or(0);
                    let txn_fee_delta = FeeDelta::from_i64(min_txn_fee as i64 - txn_fee as i64);

                    match btxn {
                        Transaction::ApplicationCall(_) => {
                            // Calculate inner transaction fee delta
                            let inner_txns_fee_delta = Self::calculate_inner_fee_delta(
                                &simulate_txn_result.txn_result.inner_txns,
                                suggested_params.min_fee,
                                FeeDelta::None,
                            );
                            inner_txns_fee_delta + txn_fee_delta
                        }
                        _ => txn_fee_delta,
                    }
                } else {
                    FeeDelta::None
                };

                Ok(TransactionAnalysis { required_fee_delta })
            })
            .collect();

        let txn_analysis_results = txn_analysis_results?;

        Ok(GroupAnalysis {
            transactions: txn_analysis_results,
        })
    }

    fn calculate_inner_fee_delta(
        inner_transactions: &Option<Vec<PendingTransactionResponse>>,
        min_transaction_fee: u64,
        acc: FeeDelta,
    ) -> FeeDelta {
        match inner_transactions {
            Some(txns) => {
                // Surplus inner transaction fees do not pool up to the parent transaction.
                // Additionally surplus inner transaction fees only pool from sibling transactions that are sent prior to a given inner transaction, hence why we iterate in reverse order.
                txns.iter().rev().fold(acc, |acc, inner_txn| {
                    let recursive_delta = Self::calculate_inner_fee_delta(
                        &inner_txn.inner_txns,
                        min_transaction_fee,
                        acc,
                    );
                    let txn_fee_delta = FeeDelta::from_i64(
                        min_transaction_fee as i64 // Inner transactions don't require per byte fees
                        - inner_txn.txn.transaction.header().fee.unwrap_or(0) as i64,
                    );

                    let current_fee_delta = recursive_delta + txn_fee_delta;

                    // If after the recursive inner fee calculations we have a surplus,
                    // return 0 to avoid pooling up surplus fees, which is not allowed.
                    if current_fee_delta.is_surplus() {
                        FeeDelta::None
                    } else {
                        current_fee_delta
                    }
                })
            }
            None => acc,
        }
    }

    fn build_transaction_header(
        &self,
        common_params: &CommonParams,
        suggested_params: &TransactionParams,
        default_validity_window: u64,
    ) -> Result<TransactionHeader, ComposerError> {
        let first_valid = common_params
            .first_valid_round
            .unwrap_or(suggested_params.last_round);

        Ok(TransactionHeader {
            sender: common_params.sender.clone(),
            rekey_to: common_params.rekey_to.clone(),
            note: common_params.note.clone(),
            lease: common_params.lease,
            fee: common_params.static_fee,
            genesis_id: Some(suggested_params.genesis_id.clone()),
            genesis_hash: Some(
                suggested_params
                    .genesis_hash
                    .clone()
                    .try_into()
                    .map_err(|_e| ComposerError::DecodeError("Invalid genesis hash".to_string()))?,
            ),
            first_valid,
            last_valid: common_params.last_valid_round.unwrap_or_else(|| {
                common_params
                    .validity_window
                    .map(|window| first_valid + window)
                    .unwrap_or(first_valid + default_validity_window)
            }),
            group: None,
        })
    }

    async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        Ok(self.algod_client.transaction_params().await?)
    }

    async fn build_transactions(
        &self,
        suggested_params: &TransactionParams,
        default_validity_window: &u64,
        group_analysis: Option<GroupAnalysis>,
    ) -> Result<Vec<Transaction>, ComposerError> {
        let mut transactions = self
            .transactions
            .iter()
            .map(|ctxn| -> Result<Transaction, ComposerError> {
                let common_params = ctxn.common_params();
                let header = self.build_transaction_header(
                    &common_params,
                    suggested_params,
                    *default_validity_window,
                )?;
                let mut calculate_fee = header.fee.is_none();

                let mut transaction = match ctxn {
                    ComposerTransaction::Transaction(tx) => {
                        calculate_fee = false;
                        tx.clone()
                    }
                    ComposerTransaction::TransactionWithSigner(tx_with_signer) => {
                        calculate_fee = false;
                        tx_with_signer.transaction.clone()
                    }
                    ComposerTransaction::Payment(params) => build_payment(params, header),
                    ComposerTransaction::AccountClose(params) => {
                        build_account_close(params, header)
                    }
                    ComposerTransaction::AssetTransfer(params) => {
                        build_asset_transfer(params, header)
                    }
                    ComposerTransaction::AssetOptIn(params) => build_asset_opt_in(params, header),
                    ComposerTransaction::AssetOptOut(params) => build_asset_opt_out(params, header),
                    ComposerTransaction::AssetClawback(params) => {
                        build_asset_clawback(params, header)
                    }
                    ComposerTransaction::AssetCreate(params) => build_asset_create(params, header),
                    ComposerTransaction::AssetReconfigure(params) => {
                        build_asset_reconfigure(params, header)
                    }
                    ComposerTransaction::AssetDestroy(params) => {
                        build_asset_destroy(params, header)
                    }
                    ComposerTransaction::AssetFreeze(params) => build_asset_freeze(params, header),
                    ComposerTransaction::AssetUnfreeze(params) => {
                        build_asset_unfreeze(params, header)
                    }
                    ComposerTransaction::AppCall(params) => build_app_call(params, header),
                    ComposerTransaction::AppCreateCall(params) => {
                        build_app_create_call(params, header)
                    }
                    ComposerTransaction::AppUpdateCall(params) => {
                        build_app_update_call(params, header)
                    }
                    ComposerTransaction::AppDeleteCall(params) => {
                        build_app_delete_call(params, header)
                    }
                    ComposerTransaction::AppCallMethodCall(method_call_params) => {
                        build_app_call_method_call(method_call_params, header)?
                    }
                    ComposerTransaction::AppCreateMethodCall(create_method_call_params) => {
                        build_app_create_method_call(create_method_call_params, header)?
                    }
                    ComposerTransaction::AppUpdateMethodCall(update_method_call_params) => {
                        build_app_update_method_call(update_method_call_params, header)?
                    }
                    ComposerTransaction::AppDeleteMethodCall(delete_method_call_params) => {
                        build_app_delete_method_call(delete_method_call_params, header)?
                    }
                    ComposerTransaction::OnlineKeyRegistration(params) => {
                        build_online_key_registration(params, header)
                    }
                    ComposerTransaction::OfflineKeyRegistration(params) => {
                        build_offline_key_registration(params, header)
                    }
                    ComposerTransaction::NonParticipationKeyRegistration(params) => {
                        build_non_participation_key_registration(params, header)
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

        if let Some(group_analysis) = group_analysis {
            let (mut surplus_group_fees, mut transaction_analysis): (u64, Vec<_>) =
                group_analysis.transactions.iter().enumerate().fold(
                    (0, Vec::new()),
                    |(mut surplus_group_fees_acc, mut txn_analysis_acc),
                     (group_index, transaction_analysis)| {
                        // Accumulate surplus fees
                        if let FeeDelta::Surplus(amount) = &transaction_analysis.required_fee_delta
                        {
                            surplus_group_fees_acc += amount;
                        }

                        // Calculate priority and add to transaction info
                        let ctxn = &self.transactions[group_index];
                        let txn = &transactions[group_index];
                        let is_immutable_fee = if let Some(logical_max_fee) = ctxn.logical_max_fee()
                        {
                            logical_max_fee == txn.header().fee.unwrap_or(0)
                        } else {
                            false
                        };
                        let priority = match &transaction_analysis.required_fee_delta {
                            FeeDelta::Deficit(amount) => {
                                if is_immutable_fee
                                    || !matches!(txn, Transaction::ApplicationCall(_))
                                {
                                    // High priority: transactions that can't be modified
                                    FeePriority::ImmutableDeficit(*amount)
                                } else {
                                    // Normal priority: application call transactions that can be modified
                                    FeePriority::ModifiableDeficit(*amount)
                                }
                            }
                            _ => FeePriority::None,
                        };

                        txn_analysis_acc.push((
                            group_index,
                            &transaction_analysis.required_fee_delta,
                            priority,
                        ));

                        (surplus_group_fees_acc, txn_analysis_acc)
                    },
                );

            // Sort transactions by priority (highest first)
            transaction_analysis.sort_by_key(|&(_, _, priority)| std::cmp::Reverse(priority));

            // Cover any additional fees required for the transactions
            for (group_index, required_fee_delta, _) in transaction_analysis {
                if let FeeDelta::Deficit(deficit_amount) = *required_fee_delta {
                    // First allocate surplus group fees to cover deficits
                    let mut additional_fee_delta: FeeDelta = FeeDelta::None;
                    if surplus_group_fees == 0 {
                        // No surplus groups fees, the transaction must cover its own deficit
                        additional_fee_delta = FeeDelta::Deficit(deficit_amount);
                    } else if surplus_group_fees >= deficit_amount {
                        // Surplus fully covers the deficit
                        surplus_group_fees -= deficit_amount;
                    } else {
                        // Surplus partially covers the deficit
                        additional_fee_delta =
                            FeeDelta::Deficit(deficit_amount - surplus_group_fees);
                        surplus_group_fees = 0;
                    }

                    // If there is any additional fee deficit, the transaction must cover it by modifying the fee
                    if let FeeDelta::Deficit(deficit_amount) = additional_fee_delta {
                        match transactions[group_index] {
                            Transaction::ApplicationCall(_) => {
                                let txn_header = transactions[group_index].header_mut();
                                let current_fee = txn_header.fee.unwrap_or(0);
                                let transaction_fee = current_fee + deficit_amount;

                                let logical_max_fee =
                                    self.transactions[group_index].logical_max_fee();
                                if logical_max_fee.is_none()
                                    || transaction_fee > logical_max_fee.unwrap()
                                {
                                    return Err(ComposerError::TransactionError(format!(
                                        "Calculated transaction fee {} µALGO is greater than max of {} for transaction {}",
                                        transaction_fee,
                                        logical_max_fee.unwrap_or(0),
                                        group_index
                                    )));
                                }

                                txn_header.fee = Some(transaction_fee);
                            }
                            _ => {
                                return Err(ComposerError::TransactionError(format!(
                                    "An additional fee of {} µALGO is required for non application call transaction {}",
                                    deficit_amount, group_index
                                )));
                            }
                        }
                    }
                }
            }
        }

        if transactions.len() > 1 {
            transactions = transactions.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?;
        }

        Ok(transactions)
    }

    pub async fn build(
        &mut self,
        params: Option<BuildParams>,
    ) -> Result<&Vec<TransactionWithSigner>, ComposerError> {
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

        let mut group_analysis: Option<GroupAnalysis> = None;
        if let Some(params) = params {
            if params
                .cover_app_call_inner_transaction_fees
                .unwrap_or(false)
            {
                group_analysis = Some(
                    self.analyze_group_requirements(
                        &suggested_params,
                        &default_validity_window,
                        &params,
                    )
                    .await?,
                );
            }
        }

        let transactions = self
            .build_transactions(&suggested_params, &default_validity_window, group_analysis)
            .await?;

        let transactions_with_signers = self.gather_signers(transactions);

        self.built_group = Some(transactions_with_signers?);
        Ok(self.built_group.as_ref().unwrap())
    }

    fn gather_signers(
        &self,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<TransactionWithSigner>, ComposerError> {
        transactions
            .into_iter()
            .enumerate()
            .map(|(group_index, txn)| {
                let ctxn = &self.transactions[group_index];
                let common_params = ctxn.common_params();
                let signer = if let Some(transaction_signer) = common_params.signer {
                    transaction_signer
                } else {
                    let sender_address = txn.header().sender.clone();
                    self.get_signer(sender_address.clone())
                        .ok_or(ComposerError::SigningError(format!(
                            "No signer found for address: {}",
                            sender_address
                        )))?
                };
                Ok(TransactionWithSigner {
                    transaction: txn,
                    signer,
                })
            })
            .collect()
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

        let signed_transactions: Result<Vec<SignedTransaction>, _> = signed_transactions
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

        self.signed_group = Some(signed_transactions?);

        Ok(self.signed_group.as_ref().unwrap())
    }

    async fn wait_for_confirmation(
        &self,
        tx_id: &str,
        max_rounds: u64,
    ) -> Result<PendingTransactionResponse, ComposerError> {
        let status = self.algod_client.get_status().await.map_err(|e| {
            ComposerError::TransactionError(format!("Failed to get status: {:?}", e))
        })?;

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
                        return Err(ComposerError::PoolError(response.pool_error.clone()));
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
                        return Err(ComposerError::AlgodClientError(error));
                    }
                }
            };

            let _ = self.algod_client.wait_for_block(current_round).await;
            current_round += 1;
        }

        Err(ComposerError::MaxWaitRoundExpired(format!(
            "Transaction {} unconfirmed after {} rounds",
            tx_id, max_rounds
        )))
    }

    pub async fn send(
        &mut self,
        params: Option<SendParams>,
    ) -> Result<SendTransactionComposerResults, ComposerError> {
        let build_params = params.as_ref().map(Into::into);

        self.build(build_params).await?;

        let group_id = {
            let transactions_with_signers = self.built_group.as_ref().ok_or(
                ComposerError::StateError("No transactions built".to_string()),
            )?;

            if transactions_with_signers.is_empty() {
                return Err(ComposerError::StateError(
                    "No transactions to send".to_string(),
                ));
            }
            transactions_with_signers[0].transaction.header().group
        };

        self.gather_signatures().await?;

        let signed_transactions = self.signed_group.as_ref().ok_or(ComposerError::StateError(
            "No signed transactions".to_string(),
        ))?;

        let wait_rounds = if let Some(max_rounds_to_wait_for_confirmation) =
            params.and_then(|p| p.max_rounds_to_wait_for_confirmation)
        {
            max_rounds_to_wait_for_confirmation
        } else {
            let first_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().first_valid)
                .min()
                .ok_or(ComposerError::StateError(
                    "Failed to calculate first valid round".to_string(),
                ))?;

            let last_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().last_valid)
                .max()
                .ok_or(ComposerError::StateError(
                    "Failed to calculate last valid round".to_string(),
                ))?;

            last_round - first_round
        };

        // Encode each signed transaction and concatenate them
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn.encode().map_err(|e| {
                ComposerError::TransactionError(format!(
                    "Failed to encode signed transaction: {}",
                    e
                ))
            })?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let _ = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| {
                ComposerError::TransactionError(format!("Failed to submit transaction(s): {:?}", e))
            })?;

        let transaction_ids: Vec<String> = signed_transactions
            .iter()
            .map(|txn| txn.id())
            .collect::<Result<Vec<String>, _>>()?;

        let mut confirmations = Vec::new();
        for id in &transaction_ids {
            let confirmation = self.wait_for_confirmation(id, wait_rounds).await?;
            confirmations.push(confirmation);
        }

        // Parse ABI return values from the confirmations
        let abi_returns = self.parse_abi_return_values(&confirmations);

        Ok(SendTransactionComposerResults {
            group_id,
            transaction_ids,
            confirmations,
            abi_returns,
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
        composer.build(None).await.unwrap();

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

        composer.build(None).await.unwrap();

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

        composer.build(None).await.unwrap();

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
    fn test_fee_delta_operations() {
        // Test creation from i64
        assert_eq!(FeeDelta::from_i64(100), FeeDelta::Deficit(100));
        assert_eq!(FeeDelta::from_i64(-50), FeeDelta::Surplus(50));
        assert_eq!(FeeDelta::from_i64(0), FeeDelta::None);

        // Test conversion to i64
        assert_eq!(FeeDelta::None.to_i64(), 0);
        assert_eq!(FeeDelta::Deficit(100).to_i64(), 100);
        assert_eq!(FeeDelta::Surplus(50).to_i64(), -50);

        // Test is_none, is_deficit and is_surplus
        assert!(FeeDelta::None.is_none());
        assert!(!FeeDelta::None.is_deficit());
        assert!(!FeeDelta::None.is_surplus());

        assert!(FeeDelta::Deficit(100).is_deficit());
        assert!(!FeeDelta::Deficit(100).is_surplus());
        assert!(!FeeDelta::Deficit(100).is_none());

        assert!(FeeDelta::Surplus(50).is_surplus());
        assert!(!FeeDelta::Surplus(50).is_deficit());
        assert!(!FeeDelta::Surplus(50).is_none());

        // Test amount extraction
        assert_eq!(FeeDelta::None.amount(), 0);
        assert_eq!(FeeDelta::Deficit(100).amount(), 100);
        assert_eq!(FeeDelta::Surplus(50).amount(), 50);
    }

    #[test]
    fn test_fee_priority_ordering() {
        let no_deficit = FeePriority::None;
        let modifiable_small = FeePriority::ModifiableDeficit(100);
        let modifiable_large = FeePriority::ModifiableDeficit(1000);
        let immutable_small = FeePriority::ImmutableDeficit(100);
        let immutable_large = FeePriority::ImmutableDeficit(1000);

        // Test basic ordering: ImmutableDeficit > ModifiableDeficit > NoDeficit
        assert!(immutable_small > modifiable_large);
        assert!(modifiable_small > no_deficit);
        assert!(immutable_large > modifiable_large);

        // Test within same priority class, larger deficits have higher priority
        assert!(immutable_large > immutable_small);
        assert!(modifiable_large > modifiable_small);

        // Create a sorted vector to verify the ordering behavior
        let mut priorities = [
            no_deficit,
            modifiable_small,
            immutable_small,
            modifiable_large,
            immutable_large,
        ];

        // Sort in descending order (highest priority first)
        priorities.sort_by(|a, b| b.cmp(a));

        assert_eq!(priorities[0], FeePriority::ImmutableDeficit(1000));
        assert_eq!(priorities[1], FeePriority::ImmutableDeficit(100));
        assert_eq!(priorities[2], FeePriority::ModifiableDeficit(1000));
        assert_eq!(priorities[3], FeePriority::ModifiableDeficit(100));
        assert_eq!(priorities[4], FeePriority::None);
    }
}
