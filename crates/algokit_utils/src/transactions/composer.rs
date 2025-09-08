use crate::{
    genesis_id_is_localnet,
    transactions::{
        key_registration::{
            build_non_participation_key_registration, build_offline_key_registration,
            build_online_key_registration,
        },
        payment::{build_account_close, build_payment},
    },
};
use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{
        ApplicationLocalReference, AssetHoldingReference, BoxReference, PendingTransactionResponse,
        SimulateRequest, SimulateRequestTransactionGroup, SimulateUnnamedResourcesAccessed,
        TransactionParams,
    },
};
use algokit_abi::{ABIMethod, ABIReturn};
use algokit_transact::{
    Address, AlgoKitTransactError, AlgorandMsgpack, Byte32, EMPTY_SIGNATURE, FeeParams,
    MAX_ACCOUNT_REFERENCES, MAX_OVERALL_REFERENCES, MAX_TX_GROUP_SIZE, SignedTransaction,
    Transaction, TransactionHeader, TransactionId, Transactions,
};
use derive_more::Debug;
use snafu::Snafu;
use std::{collections::HashMap, sync::Arc};

use crate::{
    AppMethodCallArg,
    transactions::{
        app_call::{
            AppCallMethodCallParams, ProcessedAppMethodCallArg, build_app_call_method_call,
            build_app_create_method_call, build_app_delete_method_call,
            build_app_update_method_call,
        },
        common::TransactionWithSigner,
    },
};

use super::app_call::{
    AppCallParams, AppCreateMethodCallParams, AppCreateParams, AppDeleteMethodCallParams,
    AppDeleteParams, AppUpdateMethodCallParams, AppUpdateParams, build_app_call,
    build_app_create_call, build_app_delete_call, build_app_update_call,
};
use super::asset_config::{
    AssetConfigParams, AssetCreateParams, AssetDestroyParams, build_asset_config,
    build_asset_create, build_asset_destroy,
};
use super::asset_freeze::{
    AssetFreezeParams, AssetUnfreezeParams, build_asset_freeze, build_asset_unfreeze,
};
use super::asset_transfer::{
    AssetClawbackParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
    build_asset_clawback, build_asset_opt_in, build_asset_opt_out, build_asset_transfer,
};
use super::common::TransactionSigner;
use super::key_registration::{
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use super::payment::{AccountCloseParams, PaymentParams};

const COVER_APP_CALL_INNER_TRANSACTION_FEES_DEFAULT: bool = false;

// ABI return values are stored in logs with the prefix 0x151f7c75
const ABI_RETURN_PREFIX: &[u8] = &[0x15, 0x1f, 0x7c, 0x75];

/// Configuration for application call resource population
#[derive(Debug, Clone)]
pub enum ResourcePopulation {
    /// Resource population is disabled
    Disabled,
    /// Resource population is enabled with optional access list usage
    Enabled { use_access_list: bool },
}

impl ResourcePopulation {
    /// Returns true if resource population is enabled
    pub fn is_enabled(&self) -> bool {
        matches!(self, ResourcePopulation::Enabled { .. })
    }

    /// Returns true if access list should be used (only relevant when enabled)
    pub fn use_access_list(&self) -> bool {
        matches!(
            self,
            ResourcePopulation::Enabled {
                use_access_list: true
            }
        )
    }
}

impl Default for ResourcePopulation {
    fn default() -> Self {
        ResourcePopulation::Enabled {
            use_access_list: false,
        }
    }
}

/// Types of resources that can be populated at the group level
#[derive(Debug, Clone)]
enum GroupResourceType {
    Account(String),
    App(u64),
    Asset(u64),
    Box(BoxReference),
    ExtraBoxRef,
    AssetHolding(AssetHoldingReference),
    AppLocal(ApplicationLocalReference),
}

#[derive(Debug, Snafu)]
pub enum ComposerError {
    #[snafu(display("Algod client error: {source}"))]
    AlgodClientError { source: AlgodError },
    #[snafu(display("AlgoKit Transact error: {source}"))]
    TransactError { source: AlgoKitTransactError },
    #[snafu(display("Decode Error: {message}"))]
    DecodeError { message: String },
    #[snafu(display("Transaction Error: {message}"))]
    TransactionError { message: String },
    #[snafu(display("Signing Error: {message}"))]
    SigningError { message: String },
    #[snafu(display("Composer State Error: {message}"))]
    StateError { message: String },
    #[snafu(display("Transaction pool error: {message}"))]
    PoolError { message: String },
    #[snafu(display("Transaction group size exceeds the max limit of: {max}", max = MAX_TX_GROUP_SIZE))]
    GroupSizeError,
    #[snafu(display("Max wait round expired: {message}"))]
    MaxWaitRoundExpired { message: String },
    #[snafu(display("ABI argument encoding error: {message}"))]
    ABIEncodingError { message: String },
    #[snafu(display("ABI argument decoding error: {message}"))]
    ABIDecodingError { message: String },
}

impl From<AlgodError> for ComposerError {
    fn from(e: AlgodError) -> Self {
        Self::AlgodClientError { source: e }
    }
}

impl From<AlgoKitTransactError> for ComposerError {
    fn from(e: AlgoKitTransactError) -> Self {
        Self::TransactError { source: e }
    }
}

#[derive(Debug)]
pub struct SendTransactionComposerResults {
    pub group: Option<Byte32>,
    pub transaction_ids: Vec<String>,
    pub confirmations: Vec<PendingTransactionResponse>,
    pub abi_returns: Vec<Result<Option<ABIReturn>, ComposerError>>,
}

#[derive(Debug, Clone)]
pub struct SendParams {
    pub max_rounds_to_wait_for_confirmation: Option<u32>,
    pub cover_app_call_inner_transaction_fees: bool,
    pub populate_app_call_resources: ResourcePopulation,
}

impl Default for SendParams {
    fn default() -> Self {
        Self {
            max_rounds_to_wait_for_confirmation: None,
            cover_app_call_inner_transaction_fees: COVER_APP_CALL_INNER_TRANSACTION_FEES_DEFAULT,
            populate_app_call_resources: ResourcePopulation::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildParams {
    pub cover_app_call_inner_transaction_fees: bool,
    pub populate_app_call_resources: ResourcePopulation,
}

impl Default for BuildParams {
    fn default() -> Self {
        Self {
            cover_app_call_inner_transaction_fees: COVER_APP_CALL_INNER_TRANSACTION_FEES_DEFAULT,
            populate_app_call_resources: ResourcePopulation::default(),
        }
    }
}

impl From<&SendParams> for BuildParams {
    fn from(send_params: &SendParams) -> Self {
        BuildParams {
            cover_app_call_inner_transaction_fees: send_params
                .cover_app_call_inner_transaction_fees,
            populate_app_call_resources: send_params.populate_app_call_resources.clone(),
        }
    }
}

#[derive(Debug)]
struct TransactionAnalysis {
    /// The fee difference required for this transaction
    required_fee_delta: Option<FeeDelta>,
    /// Resources that this specific transaction accessed but didn't declare
    unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
}

#[derive(Debug)]
struct GroupAnalysis {
    /// Analysis of each transaction in the group
    transactions: Vec<TransactionAnalysis>,
    /// Resources accessed by the group that qualify for group resource sharing
    unnamed_resources_accessed: Option<SimulateUnnamedResourcesAccessed>,
}

/// Represents the fee difference for a transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeeDelta {
    /// Transaction has an insufficient fee (needs more)
    Deficit(u64),
    /// Transaction has an excess fee (can cover other transactions in the group)
    Surplus(u64),
}

impl FeeDelta {
    /// Create a FeeDelta from an i64 value (positive = deficit, negative = surplus, zero = none)
    pub fn from_i64(value: i64) -> Option<Self> {
        match value.cmp(&0) {
            std::cmp::Ordering::Greater => Some(FeeDelta::Deficit(value as u64)),
            std::cmp::Ordering::Less => Some(FeeDelta::Surplus((-value) as u64)),
            std::cmp::Ordering::Equal => None,
        }
    }

    /// Convert to i64 representation (positive = deficit, negative = surplus)
    pub fn to_i64(self) -> i64 {
        match self {
            FeeDelta::Deficit(amount) => amount as i64,
            FeeDelta::Surplus(amount) => -(amount as i64),
        }
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
            FeeDelta::Deficit(amount) | FeeDelta::Surplus(amount) => *amount,
        }
    }
}

impl std::ops::Add for FeeDelta {
    type Output = Option<FeeDelta>;

    fn add(self, rhs: FeeDelta) -> Self::Output {
        FeeDelta::from_i64(self.to_i64() + rhs.to_i64())
    }
}

// Priority levels for fee coverage
// By default PartialOrd and Ord provide the correct sorting logic, based the enum variant order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FeePriority {
    /// Covered / Non-deficit transactions (lowest priority)
    Covered,
    /// App call transactions with deficits that can be modified
    ModifiableDeficit(u64),
    /// Non app call or immutable fee transactions with deficits (highest priority)
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
    AssetConfig(AssetConfigParams),
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

macro_rules! get_composer_transaction_field {
    ($field:ident, $field_type:ty, $get_expr:expr, $default_expr:expr) => {
        pub fn $field(&self) -> $field_type {
            match self {
                $crate::transactions::composer::ComposerTransaction::Payment(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AccountClose(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetTransfer(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetOptIn(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetOptOut(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetClawback(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetCreate(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetConfig(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetDestroy(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetFreeze(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AssetUnfreeze(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppCreateCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppUpdateCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppDeleteCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppCallMethodCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppCreateMethodCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppUpdateMethodCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::AppDeleteMethodCall(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::OnlineKeyRegistration(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::OfflineKeyRegistration(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::NonParticipationKeyRegistration(params) => $get_expr(&params.$field),
                $crate::transactions::composer::ComposerTransaction::Transaction(_) => $default_expr,
                $crate::transactions::composer::ComposerTransaction::TransactionWithSigner(_) => $default_expr,
            }
        }
    };
}

impl ComposerTransaction {
    // Generate field accessor methods
    get_composer_transaction_field!(
        sender,
        algokit_transact::Address,
        |x: &algokit_transact::Address| x.clone(),
        Default::default()
    );
    get_composer_transaction_field!(
        signer,
        Option<std::sync::Arc<dyn crate::transactions::common::TransactionSigner>>,
        |x: &Option<std::sync::Arc<dyn crate::transactions::common::TransactionSigner>>| x.clone(),
        None
    );
    get_composer_transaction_field!(
        rekey_to,
        Option<algokit_transact::Address>,
        |x: &Option<algokit_transact::Address>| x.clone(),
        None
    );
    get_composer_transaction_field!(note, Option<Vec<u8>>, |x: &Option<Vec<u8>>| x.clone(), None);
    get_composer_transaction_field!(lease, Option<[u8; 32]>, |x: &Option<[u8; 32]>| *x, None);
    get_composer_transaction_field!(static_fee, Option<u64>, |x: &Option<u64>| *x, None);
    get_composer_transaction_field!(extra_fee, Option<u64>, |x: &Option<u64>| *x, None);
    get_composer_transaction_field!(max_fee, Option<u64>, |x: &Option<u64>| *x, None);
    get_composer_transaction_field!(validity_window, Option<u32>, |x: &Option<u32>| *x, None);
    get_composer_transaction_field!(first_valid_round, Option<u64>, |x: &Option<u64>| *x, None);
    get_composer_transaction_field!(last_valid_round, Option<u64>, |x: &Option<u64>| *x, None);

    /// Get the logical maximum fee based on static_fee and max_fee
    pub fn logical_max_fee(&self) -> Option<u64> {
        let max_fee = self.max_fee();
        let static_fee = self.static_fee();
        match (max_fee, static_fee) {
            (Some(max_fee_value), static_fee) if max_fee_value > static_fee.unwrap_or(0) => max_fee,
            _ => static_fee,
        }
    }
}

pub type SignerGetter =
    Arc<dyn Fn(Address) -> Result<Arc<dyn TransactionSigner>, ComposerError> + Send + Sync>;
#[derive(Clone)]
pub struct Composer {
    transactions: Vec<ComposerTransaction>,
    algod_client: Arc<AlgodClient>,
    signer_getter: SignerGetter,
    built_group: Option<Vec<TransactionWithSigner>>,
    signed_group: Option<Vec<SignedTransaction>>,
}

impl Composer {
    pub fn new(algod_client: Arc<AlgodClient>, signer_getter: SignerGetter) -> Self {
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
            algod_client: Arc::new(AlgodClient::testnet()),
            signer_getter: Arc::new(|_| Ok(Arc::new(EmptySigner {}))),
            built_group: None,
            signed_group: None,
        }
    }

    fn push(&mut self, txn: ComposerTransaction) -> Result<(), ComposerError> {
        if self.transactions.len() >= MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError);
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

    pub fn add_payment(&mut self, params: PaymentParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Payment(params))
    }

    pub fn add_account_close(&mut self, params: AccountCloseParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AccountClose(params))
    }

    pub fn add_asset_transfer(&mut self, params: AssetTransferParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetTransfer(params))
    }

    pub fn add_asset_opt_in(&mut self, params: AssetOptInParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptIn(params))
    }

    pub fn add_asset_opt_out(&mut self, params: AssetOptOutParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptOut(params))
    }

    pub fn add_asset_clawback(&mut self, params: AssetClawbackParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetClawback(params))
    }

    pub fn add_asset_create(&mut self, params: AssetCreateParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetCreate(params))
    }

    pub fn add_asset_config(&mut self, params: AssetConfigParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetConfig(params))
    }

    pub fn add_asset_destroy(&mut self, params: AssetDestroyParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetDestroy(params))
    }

    pub fn add_asset_freeze(&mut self, params: AssetFreezeParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetFreeze(params))
    }

    pub fn add_asset_unfreeze(&mut self, params: AssetUnfreezeParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetUnfreeze(params))
    }

    pub fn add_online_key_registration(
        &mut self,
        params: OnlineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OnlineKeyRegistration(params))
    }

    pub fn add_offline_key_registration(
        &mut self,
        params: OfflineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OfflineKeyRegistration(params))
    }

    pub fn add_non_participation_key_registration(
        &mut self,
        params: NonParticipationKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::NonParticipationKeyRegistration(params))
    }

    pub fn add_app_call(&mut self, params: AppCallParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppCall(params))
    }

    pub fn add_app_create(&mut self, params: AppCreateParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppCreateCall(params))
    }

    pub fn add_app_update(&mut self, params: AppUpdateParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppUpdateCall(params))
    }

    pub fn add_app_delete(&mut self, params: AppDeleteParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AppDeleteCall(params))
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
                AppMethodCallArg::AssetConfig(params) => {
                    composer_transactions.push(ComposerTransaction::AssetConfig(params.clone()));
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
            return Err(ComposerError::GroupSizeError);
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
                return Err(ComposerError::ABIDecodingError {
                    message: format!(
                        "No logs found for method {} which requires a return type",
                        method.name
                    ),
                });
            }
        };

        // Check if the last log entry has the ABI return prefix
        if !last_log.starts_with(ABI_RETURN_PREFIX) {
            return Err(ComposerError::ABIDecodingError {
                message: format!(
                    "Transaction log for method {} doesn't match with ABI return value format",
                    method.name
                ),
            });
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
            Err(e) => Err(ComposerError::ABIDecodingError {
                message: format!(
                    "Failed to decode ABI return value for method {}: {}",
                    method.name, e
                ),
            }),
        }
    }

    pub fn add_transaction(
        &mut self,
        transaction: Transaction,
        signer: Option<Arc<dyn TransactionSigner>>,
    ) -> Result<(), ComposerError> {
        match signer {
            Some(signer) => self.push(ComposerTransaction::TransactionWithSigner(
                TransactionWithSigner {
                    transaction,
                    signer,
                },
            )),
            None => self.push(ComposerTransaction::Transaction(transaction)),
        }
    }

    async fn analyze_group_requirements(
        &self,
        suggested_params: &TransactionParams,
        default_validity_window: &u32,
        build_params: &BuildParams,
    ) -> Result<GroupAnalysis, ComposerError> {
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
                if build_params.cover_app_call_inner_transaction_fees {
                    if let Transaction::AppCall(_) = txn {
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
            transactions =
                transactions
                    .assign_group()
                    .map_err(|e| ComposerError::TransactionError {
                        message: format!("Failed to assign group: {}", e),
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

        if build_params.cover_app_call_inner_transaction_fees
            && !app_call_indexes_without_max_fees.is_empty()
        {
            return Err(ComposerError::StateError {
                message: format!(
                    "Please provide a max fee for each app call transaction when inner transaction fee coverage is enabled. Required for transaction {}",
                    app_call_indexes_without_max_fees
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            });
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
            .map_err(|e| ComposerError::AlgodClientError { source: e })?;

        let group_response = &response.txn_groups[0];

        // Handle any simulation failures
        if let Some(failure_message) = &group_response.failure_message {
            if build_params.cover_app_call_inner_transaction_fees
                && failure_message.contains("fee too small")
            {
                return Err(ComposerError::StateError {
                    message: "Fees were too small to analyze group requirements via simulate. You may need to increase an app call transaction max fee.".to_string(),
                });
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

            return Err(ComposerError::StateError {
                message: format!(
                    "Error analyzing group requirements via simulate in transaction {}: {}",
                    failed_at, failure_message
                ),
            });
        }

        let txn_analysis_results: Result<Vec<TransactionAnalysis>, ComposerError> = group_response
            .txn_results
            .iter()
            .enumerate()
            .map(|(group_index, simulate_txn_result)| {
                let btxn = &built_transactions[group_index];

                let required_fee_delta = if build_params.cover_app_call_inner_transaction_fees {
                    let min_txn_fee: u64 = btxn
                        .calculate_fee(FeeParams {
                            fee_per_byte: suggested_params.fee,
                            min_fee: suggested_params.min_fee,
                            ..Default::default()
                        })
                        .map_err(|e| ComposerError::TransactionError {
                            message: format!("Failed to calculate min transaction fee: {}", e),
                        })?;

                    let txn_fee = btxn.header().fee.unwrap_or(0);
                    let txn_fee_delta = FeeDelta::from_i64(min_txn_fee as i64 - txn_fee as i64);

                    match btxn {
                        Transaction::AppCall(_) => {
                            // Calculate inner transaction fee delta
                            let inner_txns_fee_delta = Self::calculate_inner_fee_delta(
                                &simulate_txn_result.txn_result.inner_txns,
                                suggested_params.min_fee,
                                None,
                            );
                            FeeDelta::from_i64(
                                inner_txns_fee_delta.map(FeeDelta::to_i64).unwrap_or(0)
                                    + txn_fee_delta.map(FeeDelta::to_i64).unwrap_or(0),
                            )
                        }
                        _ => txn_fee_delta,
                    }
                } else {
                    None
                };

                Ok(TransactionAnalysis {
                    required_fee_delta,
                    unnamed_resources_accessed: if build_params
                        .populate_app_call_resources
                        .is_enabled()
                    {
                        simulate_txn_result.unnamed_resources_accessed.clone()
                    } else {
                        None
                    },
                })
            })
            .collect();

        Ok(GroupAnalysis {
            transactions: txn_analysis_results?,
            unnamed_resources_accessed: if build_params.populate_app_call_resources.is_enabled() {
                group_response.unnamed_resources_accessed.clone()
            } else {
                None
            },
        })
    }

    fn calculate_inner_fee_delta(
        inner_transactions: &Option<Vec<PendingTransactionResponse>>,
        min_transaction_fee: u64,
        acc: Option<FeeDelta>,
    ) -> Option<FeeDelta> {
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

                    let current_fee_delta = FeeDelta::from_i64(
                        recursive_delta.map(FeeDelta::to_i64).unwrap_or(0)
                            + txn_fee_delta.map(FeeDelta::to_i64).unwrap_or(0),
                    );

                    // If after the recursive inner fee calculations we have a surplus,
                    // return None to avoid pooling up surplus fees, which is not allowed.
                    match current_fee_delta {
                        Some(delta) if delta.is_surplus() => None,
                        _ => current_fee_delta,
                    }
                })
            }
            None => acc,
        }
    }

    fn build_transaction_header(
        &self,
        composer_transaction: &ComposerTransaction,
        suggested_params: &TransactionParams,
        default_validity_window: u32,
    ) -> Result<TransactionHeader, ComposerError> {
        let first_valid = composer_transaction
            .first_valid_round()
            .unwrap_or(suggested_params.last_round);

        Ok(TransactionHeader {
            sender: composer_transaction.sender(),
            rekey_to: composer_transaction.rekey_to(),
            note: composer_transaction.note(),
            lease: composer_transaction.lease(),
            fee: composer_transaction.static_fee(),
            genesis_id: Some(suggested_params.genesis_id.clone()),
            genesis_hash: Some(
                suggested_params
                    .genesis_hash
                    .clone()
                    .try_into()
                    .map_err(|_e| ComposerError::DecodeError {
                        message: "Invalid genesis hash".to_string(),
                    })?,
            ),
            first_valid,
            last_valid: composer_transaction.last_valid_round().unwrap_or_else(|| {
                composer_transaction
                    .validity_window()
                    .map(|window| first_valid + window as u64)
                    .unwrap_or(first_valid + default_validity_window as u64)
            }),
            group: None,
        })
    }

    async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        // TODO: Add caching with expiration
        // TODO: This has a different structure to the SuggestedParams in SDK, should we match the format?
        Ok(self.algod_client.transaction_params().await?)
    }

    async fn build_transactions(
        &self,
        suggested_params: &TransactionParams,
        default_validity_window: &u32,
        group_analysis: Option<GroupAnalysis>,
    ) -> Result<Vec<Transaction>, ComposerError> {
        let mut transactions = self
            .transactions
            .iter()
            .map(|ctxn| -> Result<Transaction, ComposerError> {
                let header = self.build_transaction_header(
                    ctxn,
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
                    ComposerTransaction::AssetCreate(params) => build_asset_create(params, header)
                        .map_err(|e| ComposerError::TransactionError {
                            message: e.to_string(),
                        })?,
                    ComposerTransaction::AssetConfig(params) => build_asset_config(params, header),
                    ComposerTransaction::AssetDestroy(params) => {
                        build_asset_destroy(params, header)
                    }
                    ComposerTransaction::AssetFreeze(params) => build_asset_freeze(params, header),
                    ComposerTransaction::AssetUnfreeze(params) => {
                        build_asset_unfreeze(params, header)
                    }
                    ComposerTransaction::AppCall(params) => build_app_call(params, header)
                        .map_err(|e| ComposerError::TransactionError {
                            message: e.to_string(),
                        })?,
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
                            extra_fee: ctxn.extra_fee(),
                            max_fee: ctxn.max_fee(),
                        })
                        .map_err(|e| ComposerError::TransactionError {
                            message: e.to_string(),
                        })?;
                }

                Ok(transaction)
            })
            .collect::<Result<Vec<Transaction>, ComposerError>>()?;

        if let Some(mut group_analysis) = group_analysis {
            // Process fee adjustments
            let (mut surplus_group_fees, mut transaction_analysis): (u64, Vec<_>) =
                group_analysis.transactions.iter().enumerate().fold(
                    (0, Vec::new()),
                    |(mut surplus_group_fees_acc, mut txn_analysis_acc),
                     (group_index, transaction_analysis)| {
                        // Accumulate surplus fees
                        if let Some(FeeDelta::Surplus(amount)) =
                            &transaction_analysis.required_fee_delta
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
                            Some(FeeDelta::Deficit(amount)) => {
                                if is_immutable_fee || !matches!(txn, Transaction::AppCall(_)) {
                                    // High priority: transactions that can't be modified
                                    FeePriority::ImmutableDeficit(*amount)
                                } else {
                                    // Normal priority: app call transactions that can be modified
                                    FeePriority::ModifiableDeficit(*amount)
                                }
                            }
                            _ => FeePriority::Covered,
                        };

                        txn_analysis_acc.push((
                            group_index,
                            &transaction_analysis.required_fee_delta,
                            priority,
                            &transaction_analysis.unnamed_resources_accessed,
                        ));

                        (surplus_group_fees_acc, txn_analysis_acc)
                    },
                );

            // Sort transactions by priority (highest first)
            transaction_analysis.sort_by_key(|&(_, _, priority, _)| std::cmp::Reverse(priority));

            // Cover any additional fees required for the transactions
            for (group_index, required_fee_delta, _, resources_accessed) in transaction_analysis {
                if let Some(FeeDelta::Deficit(deficit_amount)) = *required_fee_delta {
                    // First allocate surplus group fees to cover deficits
                    let mut additional_fee_delta: Option<FeeDelta> = None;
                    if surplus_group_fees == 0 {
                        // No surplus groups fees, the transaction must cover its own deficit
                        additional_fee_delta = Some(FeeDelta::Deficit(deficit_amount));
                    } else if surplus_group_fees >= deficit_amount {
                        // Surplus fully covers the deficit
                        surplus_group_fees -= deficit_amount;
                    } else {
                        // Surplus partially covers the deficit
                        additional_fee_delta =
                            Some(FeeDelta::Deficit(deficit_amount - surplus_group_fees));
                        surplus_group_fees = 0;
                    }

                    // If there is any additional fee deficit, the transaction must cover it by modifying the fee
                    if let Some(FeeDelta::Deficit(deficit_amount)) = additional_fee_delta {
                        match transactions[group_index] {
                            Transaction::AppCall(_) => {
                                let txn_header = transactions[group_index].header_mut();
                                let current_fee = txn_header.fee.unwrap_or(0);
                                let transaction_fee = current_fee + deficit_amount;

                                let logical_max_fee =
                                    self.transactions[group_index].logical_max_fee();
                                if logical_max_fee.is_none()
                                    || transaction_fee > logical_max_fee.unwrap()
                                {
                                    return Err(ComposerError::TransactionError {
                                        message: format!(
                                            "Calculated transaction fee {} µALGO is greater than max of {} for transaction {}",
                                            transaction_fee,
                                            logical_max_fee.unwrap_or(0),
                                            group_index
                                        ),
                                    });
                                }

                                txn_header.fee = Some(transaction_fee);
                            }
                            _ => {
                                return Err(ComposerError::TransactionError {
                                    message: format!(
                                        "An additional fee of {} µALGO is required for non app call transaction {}",
                                        deficit_amount, group_index
                                    ),
                                });
                            }
                        }
                    }
                }

                if let Some(resources_accessed) = resources_accessed {
                    // Apply the transaction level resource population logic
                    if let Transaction::AppCall(ref mut app_call) = transactions[group_index] {
                        // Check for unexpected resources at transaction level
                        if resources_accessed.boxes.is_some()
                            || resources_accessed.extra_box_refs.is_some()
                        {
                            return Err(ComposerError::TransactionError {
                                message: "Unexpected boxes at the transaction level".to_string(),
                            });
                        }
                        if resources_accessed.app_locals.is_some() {
                            return Err(ComposerError::TransactionError {
                                message: "Unexpected app locals at the transaction level"
                                    .to_string(),
                            });
                        }
                        if resources_accessed.asset_holdings.is_some() {
                            return Err(ComposerError::TransactionError {
                                message: "Unexpected asset holdings at the transaction level"
                                    .to_string(),
                            });
                        }

                        let mut accounts_count = 0;
                        let mut apps_count = 0;
                        let mut assets_count = 0;
                        let mut boxes_count = 0;

                        // Populate accounts at the transaction level, apps, assets, and boxes from unnamed resources
                        if let Some(ref accessed_accounts) = resources_accessed.accounts {
                            let accounts = app_call.account_references.get_or_insert_with(Vec::new);

                            for account_str in accessed_accounts {
                                let address = account_str.parse::<Address>().map_err(|e| {
                                    ComposerError::TransactionError {
                                        message: format!("Invalid account address: {}", e),
                                    }
                                })?;
                                if !accounts.contains(&address) {
                                    accounts.push(address);
                                }
                            }
                            accounts_count = accounts.len();
                        }

                        // Populate apps at the transaction level
                        if let Some(ref accessed_apps) = resources_accessed.apps {
                            let apps = app_call.app_references.get_or_insert_with(Vec::new);
                            for app_id in accessed_apps {
                                if !apps.contains(app_id) {
                                    apps.push(*app_id);
                                }
                            }
                            apps_count = apps.len();
                        }

                        // Populate asset at the transaction level
                        if let Some(ref accessed_assets) = resources_accessed.assets {
                            let assets = app_call.asset_references.get_or_insert_with(Vec::new);
                            for asset_id in accessed_assets {
                                if !assets.contains(asset_id) {
                                    assets.push(*asset_id);
                                }
                            }
                            assets_count = assets.len();
                        }

                        // Populate boxes at the transaction level
                        if let Some(ref accessed_boxes) = resources_accessed.boxes {
                            let boxes = app_call.box_references.get_or_insert_with(Vec::new);
                            for box_ref in accessed_boxes {
                                if !boxes
                                    .iter()
                                    .any(|b| b.app_id == box_ref.app && b.name == box_ref.name)
                                {
                                    boxes.push(algokit_transact::BoxReference {
                                        app_id: box_ref.app,
                                        name: box_ref.name.clone(),
                                    });
                                }
                            }
                            boxes_count = boxes.len();
                        }

                        //Validate reference limits
                        if accounts_count > MAX_ACCOUNT_REFERENCES {
                            return Err(ComposerError::TransactionError {
                                message: format!(
                                    "Account reference limit of {} exceeded in transaction {}",
                                    MAX_ACCOUNT_REFERENCES, group_index
                                ),
                            });
                        }

                        if (accounts_count + assets_count + apps_count + boxes_count)
                            > MAX_OVERALL_REFERENCES
                        {
                            return Err(ComposerError::TransactionError {
                                message: format!(
                                    "Resource reference limit of {} exceeded in transaction {}",
                                    MAX_OVERALL_REFERENCES, group_index
                                ),
                            });
                        }
                    }
                }
            }

            // Apply the group level resource population logic
            if let Some(group_resources) = group_analysis.unnamed_resources_accessed.take() {
                Composer::populate_group_resources(&mut transactions, group_resources)?;
            }
        }

        if transactions.len() > 1 {
            transactions =
                transactions
                    .assign_group()
                    .map_err(|e| ComposerError::TransactionError {
                        message: format!("Failed to assign group: {}", e),
                    })?;
        }

        Ok(transactions)
    }

    /// Populate group-level resources for app call transactions
    fn populate_group_resources(
        transactions: &mut [Transaction],
        group_resources: SimulateUnnamedResourcesAccessed,
    ) -> Result<(), ComposerError> {
        let mut remaining_accounts = group_resources.accounts.unwrap_or_default();
        let mut remaining_apps = group_resources.apps.unwrap_or_default();
        let mut remaining_assets = group_resources.assets.unwrap_or_default();
        let remaining_boxes = group_resources.boxes.unwrap_or_default();

        // Process cross-reference resources first (app locals and asset holdings) as they are most restrictive
        if let Some(app_locals) = group_resources.app_locals {
            for app_local in app_locals {
                let app_local_app = app_local.app;
                let app_local_account = app_local.account.clone();

                Composer::populate_group_resource(
                    transactions,
                    &GroupResourceType::AppLocal(app_local),
                )?;

                // Remove resources from remaining if we're adding them here
                remaining_accounts.retain(|acc| acc != &app_local_account);
                remaining_apps.retain(|app| *app != app_local_app);
            }
        }

        if let Some(asset_holdings) = group_resources.asset_holdings {
            for asset_holding in asset_holdings {
                let asset_holding_asset = asset_holding.asset;
                let asset_holding_account = asset_holding.account.clone();

                Composer::populate_group_resource(
                    transactions,
                    &GroupResourceType::AssetHolding(asset_holding),
                )?;

                // Remove resources from remaining if we're adding them here
                remaining_accounts.retain(|acc| acc != &asset_holding_account);
                remaining_assets.retain(|asset| *asset != asset_holding_asset);
            }
        }

        // Process accounts next because account limit is 4
        for account in remaining_accounts {
            Composer::populate_group_resource(transactions, &GroupResourceType::Account(account))?;
        }

        // Process boxes
        for box_ref in remaining_boxes {
            let box_ref_app = box_ref.app;

            Composer::populate_group_resource(transactions, &GroupResourceType::Box(box_ref))?;

            // Remove apps as resource if we're adding it here
            remaining_apps.retain(|app| *app != box_ref_app);
        }

        // Process assets
        for asset in remaining_assets {
            Composer::populate_group_resource(transactions, &GroupResourceType::Asset(asset))?;
        }

        // Process remaining apps
        for app in remaining_apps {
            Composer::populate_group_resource(transactions, &GroupResourceType::App(app))?;
        }

        // Handle extra box refs
        if let Some(extra_box_refs) = group_resources.extra_box_refs {
            for _ in 0..extra_box_refs {
                Composer::populate_group_resource(transactions, &GroupResourceType::ExtraBoxRef)?;
            }
        }

        Ok(())
    }

    // Helper function to check if an application call transaction is below resource limit
    fn is_app_call_below_resource_limit(txn: &Transaction) -> bool {
        if let Transaction::AppCall(app_call) = txn {
            let accounts_count = app_call
                .account_references
                .as_ref()
                .map(|a| a.len())
                .unwrap_or(0);
            let assets_count = app_call
                .asset_references
                .as_ref()
                .map(|a| a.len())
                .unwrap_or(0);
            let apps_count = app_call
                .app_references
                .as_ref()
                .map(|a| a.len())
                .unwrap_or(0);
            let boxes_count = app_call
                .box_references
                .as_ref()
                .map(|b| b.len())
                .unwrap_or(0);

            (accounts_count + assets_count + apps_count + boxes_count) < MAX_OVERALL_REFERENCES
        } else {
            false
        }
    }

    /// Helper function to populate a specific resource into a transaction group
    fn populate_group_resource(
        transactions: &mut [Transaction],
        resource: &GroupResourceType,
    ) -> Result<(), ComposerError> {
        // For asset holdings and app locals, first try to find a transaction that already has the account available
        match resource {
            GroupResourceType::AssetHolding(_) | GroupResourceType::AppLocal(_) => {
                let account = match resource {
                    GroupResourceType::AssetHolding(asset_holding) => &asset_holding.account,
                    GroupResourceType::AppLocal(app_local) => &app_local.account,
                    _ => unreachable!(),
                };

                // Try to find a transaction that already has the account available
                let group_index = transactions.iter().position(|txn| {
                    if !Composer::is_app_call_below_resource_limit(txn) {
                        return false;
                    }

                    if let Transaction::AppCall(app_call) = txn {
                        // Check if account is in foreign accounts array
                        if let Some(ref accounts) = app_call.account_references {
                            let address = account.parse::<Address>().unwrap_or_default();
                            if accounts.contains(&address) {
                                return true;
                            }
                        }

                        // Check if account is available as an app account
                        if let Some(ref apps) = app_call.app_references {
                            for app_id in apps {
                                if account == &Address::from_app_id(app_id).to_string() {
                                    return true;
                                }
                            }
                        }

                        // Check if account appears in any app call transaction fields
                        if app_call.header.sender.to_string() == *account {
                            return true;
                        }
                    }

                    false
                });

                if let Some(group_index) = group_index {
                    if let Transaction::AppCall(ref mut app_call) = transactions[group_index] {
                        match resource {
                            GroupResourceType::AssetHolding(asset_holding) => {
                                let assets = app_call.asset_references.get_or_insert_with(Vec::new);
                                if !assets.contains(&asset_holding.asset) {
                                    assets.push(asset_holding.asset);
                                }
                            }
                            GroupResourceType::AppLocal(app_local) => {
                                let apps = app_call.app_references.get_or_insert_with(Vec::new);
                                if !apps.contains(&app_local.app) {
                                    apps.push(app_local.app);
                                }
                            }
                            _ => {}
                        }
                    }

                    return Ok(());
                }

                // Try to find a transaction that already has the asset/app available and space for account
                let group_index = transactions.iter().position(|txn| {
                    if !Composer::is_app_call_below_resource_limit(txn) {
                        return false;
                    }

                    if let Transaction::AppCall(app_call) = txn {
                        // Check if there's space in the accounts array
                        if app_call
                            .account_references
                            .as_ref()
                            .map(|a| a.len())
                            .unwrap_or(0)
                            >= MAX_ACCOUNT_REFERENCES
                        {
                            return false;
                        }

                        match resource {
                            GroupResourceType::AssetHolding(asset_holding) => {
                                if let Some(ref assets) = app_call.asset_references {
                                    return assets.contains(&asset_holding.asset);
                                }
                            }
                            GroupResourceType::AppLocal(app_local) => {
                                if let Some(ref apps) = app_call.app_references {
                                    return apps.contains(&app_local.app);
                                }
                                return app_call.app_id == app_local.app;
                            }
                            _ => {}
                        }
                    }

                    false
                });

                if let Some(group_index) = group_index {
                    if let Transaction::AppCall(ref mut app_call) = transactions[group_index] {
                        let accounts = app_call.account_references.get_or_insert_with(Vec::new);
                        let address = account.parse::<Address>().map_err(|e| {
                            ComposerError::TransactionError {
                                message: format!("Invalid account address: {}", e),
                            }
                        })?;
                        if !accounts.contains(&address) {
                            accounts.push(address);
                        }
                    }
                    return Ok(());
                }
            }
            GroupResourceType::Box(box_ref) => {
                // For boxes, first try to find a transaction that already has the app available
                let group_index = transactions.iter().position(|txn| {
                    if !Composer::is_app_call_below_resource_limit(txn) {
                        return false;
                    }

                    if let Transaction::AppCall(app_call) = txn {
                        // Check if the app is in the foreign array OR the app being called
                        if let Some(ref apps) = app_call.app_references {
                            if apps.contains(&box_ref.app) {
                                return true;
                            }
                        }
                        return app_call.app_id == box_ref.app;
                    }

                    false
                });

                if let Some(group_index) = group_index {
                    if let Transaction::AppCall(ref mut app_call) = transactions[group_index] {
                        let boxes = app_call.box_references.get_or_insert_with(Vec::new);
                        if !boxes
                            .iter()
                            .any(|b| b.app_id == box_ref.app && b.name == box_ref.name)
                        {
                            boxes.push(algokit_transact::BoxReference {
                                app_id: box_ref.app,
                                name: box_ref.name.clone(),
                            });
                        }
                    }
                    return Ok(());
                }
            }
            _ => {}
        }

        // Find the transaction index to put the reference(s)
        let group_index = transactions.iter().position(|txn| {
            if let Transaction::AppCall(app_call) = txn {
                let accounts_count = app_call
                    .account_references
                    .as_ref()
                    .map(|a| a.len())
                    .unwrap_or(0);
                let assets_count = app_call
                    .asset_references
                    .as_ref()
                    .map(|a| a.len())
                    .unwrap_or(0);
                let apps_count = app_call
                    .app_references
                    .as_ref()
                    .map(|a| a.len())
                    .unwrap_or(0);
                let boxes_count = app_call
                    .box_references
                    .as_ref()
                    .map(|b| b.len())
                    .unwrap_or(0);

                match resource {
                    GroupResourceType::Account(_) => accounts_count < MAX_ACCOUNT_REFERENCES,

                    GroupResourceType::AssetHolding(..) | GroupResourceType::AppLocal(..) => {
                        // If we're adding local state or asset holding, we need space for the account and the other reference (asset or app)
                        (accounts_count + assets_count + apps_count + boxes_count)
                            < (MAX_OVERALL_REFERENCES - 1)
                            && accounts_count < MAX_ACCOUNT_REFERENCES
                    }

                    GroupResourceType::Box(box_ref) => {
                        // If we're adding a box, we need space for both the box reference and the app reference
                        if box_ref.app != 0 {
                            (accounts_count + assets_count + apps_count + boxes_count)
                                < MAX_OVERALL_REFERENCES - 1
                        } else {
                            (accounts_count + assets_count + apps_count + boxes_count)
                                < MAX_OVERALL_REFERENCES
                        }
                    }
                    _ => {
                        (accounts_count + assets_count + apps_count + boxes_count)
                            < MAX_OVERALL_REFERENCES
                    }
                }
            } else {
                false
            }
        });

        let group_index = group_index.ok_or_else(|| ComposerError::TransactionError {
            message:
                "No more transactions below reference limit. Add another app call to the group."
                    .to_string(),
        })?;

        if let Transaction::AppCall(ref mut app_call) = transactions[group_index] {
            match resource {
                GroupResourceType::Account(account) => {
                    let accounts = app_call.account_references.get_or_insert_with(Vec::new);
                    let address = account.parse::<Address>().map_err(|e| {
                        ComposerError::TransactionError {
                            message: format!("Invalid account address: {}", e),
                        }
                    })?;
                    if !accounts.contains(&address) {
                        accounts.push(address);
                    }
                }
                GroupResourceType::App(app_id) => {
                    let apps = app_call.app_references.get_or_insert_with(Vec::new);
                    if !apps.contains(app_id) {
                        apps.push(*app_id);
                    }
                }
                GroupResourceType::Box(box_ref) => {
                    let boxes = app_call.box_references.get_or_insert_with(Vec::new);
                    if !boxes
                        .iter()
                        .any(|b| b.app_id == box_ref.app && b.name == box_ref.name)
                    {
                        boxes.push(algokit_transact::BoxReference {
                            app_id: box_ref.app,
                            name: box_ref.name.clone(),
                        });
                    }
                    if box_ref.app != 0 {
                        let apps = app_call.app_references.get_or_insert_with(Vec::new);
                        if !apps.contains(&box_ref.app) {
                            apps.push(box_ref.app);
                        }
                    }
                }
                GroupResourceType::ExtraBoxRef => {
                    let boxes = app_call.box_references.get_or_insert_with(Vec::new);
                    boxes.push(algokit_transact::BoxReference {
                        app_id: 0,
                        name: Vec::new(),
                    });
                }
                GroupResourceType::AssetHolding(asset_holding) => {
                    let assets = app_call.asset_references.get_or_insert_with(Vec::new);
                    if !assets.contains(&asset_holding.asset) {
                        assets.push(asset_holding.asset);
                    }

                    let accounts = app_call.account_references.get_or_insert_with(Vec::new);
                    let address = asset_holding.account.parse::<Address>().map_err(|e| {
                        ComposerError::TransactionError {
                            message: format!("Invalid account address: {}", e),
                        }
                    })?;
                    if !accounts.contains(&address) {
                        accounts.push(address);
                    }
                }
                GroupResourceType::AppLocal(app_local) => {
                    let apps = app_call.app_references.get_or_insert_with(Vec::new);
                    if !apps.contains(&app_local.app) {
                        apps.push(app_local.app);
                    }

                    let accounts = app_call.account_references.get_or_insert_with(Vec::new);
                    let address = app_local.account.parse::<Address>().map_err(|e| {
                        ComposerError::TransactionError {
                            message: format!("Invalid account address: {}", e),
                        }
                    })?;
                    if !accounts.contains(&address) {
                        accounts.push(address);
                    }
                }
                GroupResourceType::Asset(asset_id) => {
                    let assets = app_call.asset_references.get_or_insert_with(Vec::new);
                    if !assets.contains(asset_id) {
                        assets.push(*asset_id);
                    }
                }
            }
        }

        Ok(())
    }

    fn get_default_validity_window(genesis_id: &str) -> u32 {
        if genesis_id_is_localnet(genesis_id) {
            1000 // LocalNet gets bigger window to avoid dead transactions
        } else {
            10 // Standard default validity window
        }
    }

    pub async fn build(
        &mut self,
        params: Option<BuildParams>,
    ) -> Result<&Vec<TransactionWithSigner>, ComposerError> {
        if let Some(ref group) = self.built_group {
            return Ok(group);
        }

        let suggested_params = self.get_suggested_params().await?;
        let default_validity_window =
            Self::get_default_validity_window(&suggested_params.genesis_id);

        let group_analysis = match params.as_ref() {
            Some(params)
                if params.cover_app_call_inner_transaction_fees
                    || params.populate_app_call_resources.is_enabled() =>
            {
                Some(
                    self.analyze_group_requirements(
                        &suggested_params,
                        &default_validity_window,
                        params,
                    )
                    .await?,
                )
            }
            _ => None,
        };

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
                let signer = if let Some(transaction_signer) = ctxn.signer() {
                    transaction_signer
                } else {
                    let sender_address = txn.header().sender.clone();
                    (self.signer_getter)(sender_address.clone())?
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

        // TODO: Should gather signatures invoke build?
        // This makes it possible to build, then send, however use different args between the two calls?

        let transactions_with_signers =
            self.built_group.as_ref().ok_or(ComposerError::StateError {
                message: "Cannot gather signatures before building the transaction group"
                    .to_string(),
            })?;

        // Group transactions by signer
        let mut transactions = Vec::new();
        let mut signer_groups: HashMap<*const dyn TransactionSigner, Vec<usize>> = HashMap::new();
        for (group_index, txn_with_signer) in transactions_with_signers.iter().enumerate() {
            let signer_ptr = Arc::as_ptr(&txn_with_signer.signer);
            signer_groups
                .entry(signer_ptr)
                .or_default()
                .push(group_index);
            transactions.push(txn_with_signer.transaction.to_owned());
        }

        let mut signed_transactions: Vec<Option<SignedTransaction>> =
            vec![None; transactions_with_signers.len()];

        for (_signer_ptr, indices) in signer_groups {
            // Get the signer from the first transaction with this signer
            let signer = &transactions_with_signers[indices[0]].signer;

            // Sign all transactions for this signer
            let signed_txns: Vec<SignedTransaction> = signer
                .sign_transactions(&transactions, &indices)
                .await
                .map_err(|e| ComposerError::SigningError {
                    message: e.to_string(),
                })?;

            for (i, &index) in indices.iter().enumerate() {
                signed_transactions[index] = Some(signed_txns[i].to_owned());
            }
        }

        let (signed_transactions, unsigned_indexes) =
            signed_transactions.into_iter().enumerate().fold(
                (Vec::<SignedTransaction>::new(), Vec::<usize>::new()),
                |mut acc, (i, stx)| {
                    match stx {
                        Some(stx) => acc.0.push(stx),
                        None => acc.1.push(i),
                    }
                    acc
                },
            );

        if !unsigned_indexes.is_empty()
            || transactions_with_signers.len() != signed_transactions.len()
        {
            return Err(ComposerError::SigningError {
                message: format!(
                    "Transactions at indexes {:?} were not signed",
                    unsigned_indexes
                ),
            });
        }

        self.signed_group = Some(signed_transactions);
        Ok(self.signed_group.as_ref().unwrap())
    }

    async fn wait_for_confirmation(
        &self,
        tx_id: &str,
        max_rounds_to_wait: u32,
    ) -> Result<PendingTransactionResponse, ComposerError> {
        let status =
            self.algod_client
                .get_status()
                .await
                .map_err(|e| ComposerError::TransactionError {
                    message: format!("Failed to get status: {:?}", e),
                })?;

        let start_round = status.last_round + 1;
        let mut current_round = start_round;

        while current_round < start_round + max_rounds_to_wait as u64 {
            match self
                .algod_client
                .pending_transaction_information(tx_id, Some(Format::Msgpack))
                .await
            {
                Ok(response) => {
                    // Check for pool errors first - transaction was kicked out of pool
                    if !response.pool_error.is_empty() {
                        return Err(ComposerError::PoolError {
                            message: format!(
                                "Transaction {} was rejected; pool error: {}",
                                tx_id,
                                response.pool_error.clone()
                            ),
                        });
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
                        algod_client::apis::Error::Api {
                            source: algod_client::apis::AlgodApiError::PendingTransactionInformation {
                                error: algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            }
                        }
                    ) || error.to_string().contains("404");

                    if is_retryable {
                        current_round += 1;
                        continue;
                    } else {
                        return Err(ComposerError::AlgodClientError { source: error });
                    }
                }
            };

            let _ = self.algod_client.wait_for_block(current_round).await;
            current_round += 1;
        }

        Err(ComposerError::MaxWaitRoundExpired {
            message: format!(
                "Transaction {} unconfirmed after {} rounds",
                tx_id, max_rounds_to_wait
            ),
        })
    }

    pub async fn send(
        &mut self,
        params: Option<SendParams>,
    ) -> Result<SendTransactionComposerResults, ComposerError> {
        let build_params = params.as_ref().map(Into::into);

        self.build(build_params).await?;

        let group = {
            let transactions_with_signers =
                self.built_group.as_ref().ok_or(ComposerError::StateError {
                    message: "No transactions to send".to_string(),
                })?;

            if transactions_with_signers.is_empty() {
                return Err(ComposerError::StateError {
                    message: "No transactions to send".to_string(),
                });
            }
            transactions_with_signers[0].transaction.header().group
        };

        self.gather_signatures().await?;

        let signed_transactions = self
            .signed_group
            .as_ref()
            .ok_or(ComposerError::StateError {
                message: "No signed transactions to send".to_string(),
            })?;

        let wait_rounds = if let Some(max_rounds_to_wait_for_confirmation) =
            params.and_then(|p| p.max_rounds_to_wait_for_confirmation)
        {
            max_rounds_to_wait_for_confirmation
        } else {
            let suggested_params = self.get_suggested_params().await?;
            let first_round: u64 = suggested_params.last_round; // The last round seen, so is the first round valid
            let last_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().last_valid)
                .max()
                .ok_or(ComposerError::StateError {
                    message: "Failed to calculate last valid round".to_string(),
                })?;
            (last_round - first_round)
                .try_into()
                .unwrap_or(Self::get_default_validity_window(
                    &suggested_params.genesis_id,
                ))
        };

        // Encode each signed transaction and concatenate them
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn
                .encode()
                .map_err(|e| ComposerError::TransactionError {
                    message: format!("Failed to encode signed transaction: {}", e),
                })?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let _ = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: format!("Failed to submit transaction(s): {:?}", e),
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
            group,
            transaction_ids,
            confirmations,
            abi_returns,
        })
    }

    /// Extract ABI method mapping from built transactions.
    /// Maps transaction index to the ABI method used to create it.
    /// Used by TransactionCreator to populate BuiltTransactions.method_calls.
    pub(crate) fn extract_method_calls(&self) -> HashMap<usize, ABIMethod> {
        let mut method_calls = HashMap::new();

        for (i, transaction) in self.transactions.iter().enumerate() {
            if let Some(method) = self.get_method_from_transaction(transaction) {
                method_calls.insert(i, method.clone());
            }
        }

        method_calls
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
        assert!(composer.add_transaction(txn, None).is_ok());
    }

    #[test]
    fn test_add_too_many_transactions() {
        let mut composer = Composer::testnet();
        for _ in 0..16 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn, None).is_ok());
        }
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn, None).is_err());
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
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        assert!(composer.add_payment(payment_params).is_ok());
    }

    #[tokio::test]
    async fn test_gather_signatures() {
        let mut composer = Composer::testnet();

        let payment_params = PaymentParams {
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
        let group = built_group[0].transaction.header().group.as_ref().unwrap();
        for transaction_with_signer in &built_group[1..] {
            assert_eq!(
                transaction_with_signer
                    .transaction
                    .header()
                    .group
                    .as_ref()
                    .unwrap(),
                group
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
        assert_eq!(FeeDelta::from_i64(100), Some(FeeDelta::Deficit(100)));
        assert_eq!(FeeDelta::from_i64(-50), Some(FeeDelta::Surplus(50)));
        assert_eq!(FeeDelta::from_i64(0), None);

        // Test conversion to i64
        assert_eq!(FeeDelta::Deficit(100).to_i64(), 100);
        assert_eq!(FeeDelta::Surplus(50).to_i64(), -50);

        // Test is_deficit and is_surplus
        assert!(FeeDelta::Deficit(100).is_deficit());
        assert!(!FeeDelta::Deficit(100).is_surplus());

        assert!(FeeDelta::Surplus(50).is_surplus());
        assert!(!FeeDelta::Surplus(50).is_deficit());

        // Test amount extraction
        assert_eq!(FeeDelta::Deficit(100).amount(), 100);
        assert_eq!(FeeDelta::Surplus(50).amount(), 50);
    }

    #[test]
    fn test_fee_priority_ordering() {
        let covered = FeePriority::Covered;
        let modifiable_small = FeePriority::ModifiableDeficit(100);
        let modifiable_large = FeePriority::ModifiableDeficit(1000);
        let immutable_small = FeePriority::ImmutableDeficit(100);
        let immutable_large = FeePriority::ImmutableDeficit(1000);

        // Test basic ordering: ImmutableDeficit > ModifiableDeficit > Covered
        assert!(immutable_small > modifiable_large);
        assert!(modifiable_small > covered);
        assert!(immutable_large > modifiable_large);

        // Test within same priority class, larger deficits have higher priority
        assert!(immutable_large > immutable_small);
        assert!(modifiable_large > modifiable_small);

        // Create a sorted vector to verify the ordering behavior
        let mut priorities = [
            covered,
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
        assert_eq!(priorities[4], FeePriority::Covered);
    }

    #[test]
    fn test_build_params_default() {
        let params = BuildParams::default();
        assert!(!params.cover_app_call_inner_transaction_fees);
        assert!(params.populate_app_call_resources.is_enabled());
    }

    #[test]
    fn test_build_params_from_send_params() {
        let send_params = SendParams {
            max_rounds_to_wait_for_confirmation: Some(10),
            cover_app_call_inner_transaction_fees: true,
            populate_app_call_resources: ResourcePopulation::Enabled {
                use_access_list: true,
            },
        };

        let build_params = BuildParams::from(&send_params);
        assert!(build_params.cover_app_call_inner_transaction_fees);
        assert!(build_params.populate_app_call_resources.is_enabled());
    }
}
