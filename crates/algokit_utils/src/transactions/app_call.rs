use super::composer::ComposerError;
use crate::{
    AccountCloseParams, AssetClawbackParams, AssetConfigParams, AssetCreateParams,
    AssetDestroyParams, AssetFreezeParams, AssetOptInParams, AssetOptOutParams,
    AssetTransferParams, AssetUnfreezeParams, NonParticipationKeyRegistrationParams,
    OfflineKeyRegistrationParams, OnlineKeyRegistrationParams, PaymentParams, TransactionSigner,
    TransactionWithSigner, create_transaction_params,
};
use algokit_abi::{
    ABIMethod, ABIMethodArgType, ABIReferenceValue, ABIType, ABIValue, abi_type::BitSize,
};
use algokit_transact::{
    Address, AppCallTransactionBuilder, AppCallTransactionFields, BoxReference,
    OnApplicationComplete, StateSchema, Transaction, TransactionHeader,
};
use derive_more::Debug;
use num_bigint::BigUint;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum AppMethodCallArg {
    ABIValue(ABIValue),
    ABIReference(ABIReferenceValue),
    /// Sentinel to request ARC-56 default resolution for this argument (handled by AppClient params builder)
    DefaultValue,
    /// Placeholder for a transaction-typed argument. Not encoded; satisfied by a transaction
    /// included in the same group (extracted from other method call arguments).
    TransactionPlaceholder,
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
    AppCallMethodCall(AppCallMethodCallParams),
    AppCreateMethodCall(AppCreateMethodCallParams),
    AppUpdateMethodCall(AppUpdateMethodCallParams),
    AppDeleteMethodCall(AppDeleteMethodCallParams),
    OnlineKeyRegistration(OnlineKeyRegistrationParams),
    OfflineKeyRegistration(OfflineKeyRegistrationParams),
    NonParticipationKeyRegistration(NonParticipationKeyRegistrationParams),
}

#[derive(Debug, Clone)]
pub enum ProcessedAppMethodCallArg {
    ABIValue(ABIValue),
    ABIReference(ABIReferenceValue),
    TransactionPlaceholder,
}

/// This pattern create a trait ValidMethodCallArg
/// that can only implemented by AppMethodCallArg and ProcessedAppMethodCallArg.
mod sealed {
    pub trait ValidMethodCallArgSealed {}
    impl ValidMethodCallArgSealed for super::AppMethodCallArg {}
    impl ValidMethodCallArgSealed for super::ProcessedAppMethodCallArg {}
}
pub trait ValidMethodCallArg: sealed::ValidMethodCallArgSealed {}

impl ValidMethodCallArg for AppMethodCallArg {}
impl ValidMethodCallArg for ProcessedAppMethodCallArg {}

create_transaction_params! {
    /// Parameters for creating an app call transaction.
    #[derive(Clone, Default)]
    pub struct AppCallParams {
        /// ID of the app being called.
        pub app_id: u64,
        /// Defines what additional actions occur with the transaction.
        pub on_complete: OnApplicationComplete,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        pub account_references: Option<Vec<Address>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        pub box_references: Option<Vec<BoxReference>>,
    }
}

create_transaction_params! {
    /// Parameters for creating an app create transaction.
    #[derive(Clone, Default)]
    pub struct AppCreateParams {
        /// Defines what additional actions occur with the transaction.
        pub on_complete: OnApplicationComplete,
        /// Logic executed for every app call transaction, except when
        /// on-completion is set to "clear".
        ///
        /// Approval programs may reject the transaction.
        pub approval_program: Vec<u8>,
        /// Logic executed for app call transactions with on-completion set to "clear".
        ///
        /// Clear state programs cannot reject the transaction.
        pub clear_state_program: Vec<u8>,
        /// Holds the maximum number of global state values.
        ///
        /// This cannot be changed after creation.
        pub global_state_schema: Option<StateSchema>,
        /// Holds the maximum number of local state values.
        ///
        /// This cannot be changed after creation.
        pub local_state_schema: Option<StateSchema>,
        /// Number of additional pages allocated to the app's approval
        /// and clear state programs.
        ///
        /// Each extra program page is 2048 bytes. The sum of approval program
        /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
        /// Currently, the maximum value is 3.
        /// This cannot be changed after creation.
        pub extra_program_pages: Option<u32>,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        pub account_references: Option<Vec<Address>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        pub box_references: Option<Vec<BoxReference>>,
    }
}

create_transaction_params! {
    /// Parameters for creating an app delete transaction.
    #[derive(Clone, Default)]
    pub struct AppDeleteParams {
        /// ID of the app being deleted.
        pub app_id: u64,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        pub account_references: Option<Vec<Address>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        pub box_references: Option<Vec<BoxReference>>,
    }
}

create_transaction_params! {
    /// Parameters for creating an app update transaction.
    #[derive(Clone, Default)]
    pub struct AppUpdateParams {
        /// ID of the app being updated.
        pub app_id: u64,
        /// Logic executed for every app call transaction, except when
        /// on-completion is set to "clear".
        ///
        /// Approval programs may reject the transaction.
        pub approval_program: Vec<u8>,
        /// Logic executed for app call transactions with on-completion set to "clear".
        ///
        /// Clear state programs cannot reject the transaction.
        pub clear_state_program: Vec<u8>,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        pub account_references: Option<Vec<Address>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        pub box_references: Option<Vec<BoxReference>>,
    }
}

/// Parameters for creating an app method call transaction.
#[derive(Debug, Clone)]
pub struct AppCallMethodCallParams<T = AppMethodCallArg>
where
    T: ValidMethodCallArg,
{
    #[debug(skip)]
    /// A signer used to sign transaction(s); if not specified then
    /// an attempt will be made to find a registered signer for the
    ///  given `sender` or use a default signer (if configured).
    pub signer: Option<std::sync::Arc<dyn TransactionSigner>>,
    /// The address of the account sending the transaction.
    pub sender: algokit_transact::Address,
    /// Change the signing key of the sender to the given address.
    /// **Warning:** Please be careful with this parameter and be sure to read the [official rekey guidance](https://dev.algorand.co/concepts/accounts/rekeying).
    pub rekey_to: Option<algokit_transact::Address>,
    /// Note to attach to the transaction. Max of 1000 bytes.
    pub note: Option<Vec<u8>>,
    /// Prevent multiple transactions with the same lease being included within the validity window.
    ///
    /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
    /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
    pub lease: Option<[u8; 32]>,
    /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
    pub static_fee: Option<u64>,
    /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
    pub extra_fee: Option<u64>,
    /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
    pub max_fee: Option<u64>,
    /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
    pub validity_window: Option<u32>,
    /// Set the first round this transaction is valid.
    /// If left undefined, the value from algod will be used.
    ///
    /// We recommend you only set this when you intentionally want this to be some time in the future.
    pub first_valid_round: Option<u64>,
    /// The last round this transaction is valid. It is recommended to use validity window instead.
    pub last_valid_round: Option<u64>,
    /// ID of the app being called.
    pub app_id: u64,
    /// The ABI method to call.
    pub method: ABIMethod,
    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    pub args: Vec<T>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
}

impl<T> Default for AppCallMethodCallParams<T>
where
    T: ValidMethodCallArg,
{
    fn default() -> Self {
        Self {
            app_id: 0,
            method: ABIMethod::default(),
            args: Vec::new(),
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: OnApplicationComplete::NoOp,
            sender: Address::default(),
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
        }
    }
}

/// Parameters for creating an app create method call transaction.
#[derive(Debug, Default, Clone)]
pub struct AppCreateMethodCallParams<T = AppMethodCallArg>
where
    T: ValidMethodCallArg,
{
    #[debug(skip)]
    /// A signer used to sign transaction(s); if not specified then
    /// an attempt will be made to find a registered signer for the
    ///  given `sender` or use a default signer (if configured).
    pub signer: Option<std::sync::Arc<dyn TransactionSigner>>,
    /// The address of the account sending the transaction.
    pub sender: algokit_transact::Address,
    /// Change the signing key of the sender to the given address.
    /// **Warning:** Please be careful with this parameter and be sure to read the [official rekey guidance](https://dev.algorand.co/concepts/accounts/rekeying).
    pub rekey_to: Option<algokit_transact::Address>,
    /// Note to attach to the transaction. Max of 1000 bytes.
    pub note: Option<Vec<u8>>,
    /// Prevent multiple transactions with the same lease being included within the validity window.
    ///
    /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
    /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
    pub lease: Option<[u8; 32]>,
    /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
    pub static_fee: Option<u64>,
    /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
    pub extra_fee: Option<u64>,
    /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
    pub max_fee: Option<u64>,
    /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
    pub validity_window: Option<u32>,
    /// Set the first round this transaction is valid.
    /// If left undefined, the value from algod will be used.
    ///
    /// We recommend you only set this when you intentionally want this to be some time in the future.
    pub first_valid_round: Option<u64>,
    /// The last round this transaction is valid. It is recommended to use validity window instead.
    pub last_valid_round: Option<u64>,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Logic executed for every app call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    pub approval_program: Vec<u8>,
    /// Logic executed for app call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    pub clear_state_program: Vec<u8>,
    /// Holds the maximum number of global state values.
    ///
    /// This cannot be changed after creation.
    pub global_state_schema: Option<StateSchema>,
    /// Holds the maximum number of local state values.
    ///
    /// This cannot be changed after creation.
    pub local_state_schema: Option<StateSchema>,
    /// Number of additional pages allocated to the app's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    pub extra_program_pages: Option<u32>,
    /// The ABI method to call.
    pub method: ABIMethod,
    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    pub args: Vec<T>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for creating an app update method call transaction.
#[derive(Debug, Default, Clone)]
pub struct AppUpdateMethodCallParams<T = AppMethodCallArg>
where
    T: ValidMethodCallArg,
{
    #[debug(skip)]
    /// A signer used to sign transaction(s); if not specified then
    /// an attempt will be made to find a registered signer for the
    ///  given `sender` or use a default signer (if configured).
    pub signer: Option<std::sync::Arc<dyn TransactionSigner>>,
    /// The address of the account sending the transaction.
    pub sender: algokit_transact::Address,
    /// Change the signing key of the sender to the given address.
    /// **Warning:** Please be careful with this parameter and be sure to read the [official rekey guidance](https://dev.algorand.co/concepts/accounts/rekeying).
    pub rekey_to: Option<algokit_transact::Address>,
    /// Note to attach to the transaction. Max of 1000 bytes.
    pub note: Option<Vec<u8>>,
    /// Prevent multiple transactions with the same lease being included within the validity window.
    ///
    /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
    /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
    pub lease: Option<[u8; 32]>,
    /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
    pub static_fee: Option<u64>,
    /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
    pub extra_fee: Option<u64>,
    /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
    pub max_fee: Option<u64>,
    /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
    pub validity_window: Option<u32>,
    /// Set the first round this transaction is valid.
    /// If left undefined, the value from algod will be used.
    ///
    /// We recommend you only set this when you intentionally want this to be some time in the future.
    pub first_valid_round: Option<u64>,
    /// The last round this transaction is valid. It is recommended to use validity window instead.
    pub last_valid_round: Option<u64>,
    /// ID of the app being updated.
    pub app_id: u64,
    /// Logic executed for every app call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    pub approval_program: Vec<u8>,
    /// Logic executed for app call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    pub clear_state_program: Vec<u8>,
    /// The ABI method to call.
    pub method: ABIMethod,
    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    pub args: Vec<T>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for creating an app delete method call transaction.
#[derive(Debug, Default, Clone)]
pub struct AppDeleteMethodCallParams<T = AppMethodCallArg>
where
    T: ValidMethodCallArg,
{
    #[debug(skip)]
    /// A signer used to sign transaction(s); if not specified then
    /// an attempt will be made to find a registered signer for the
    ///  given `sender` or use a default signer (if configured).
    pub signer: Option<std::sync::Arc<dyn TransactionSigner>>,
    /// The address of the account sending the transaction.
    pub sender: algokit_transact::Address,
    /// Change the signing key of the sender to the given address.
    /// **Warning:** Please be careful with this parameter and be sure to read the [official rekey guidance](https://dev.algorand.co/concepts/accounts/rekeying).
    pub rekey_to: Option<algokit_transact::Address>,
    /// Note to attach to the transaction. Max of 1000 bytes.
    pub note: Option<Vec<u8>>,
    /// Prevent multiple transactions with the same lease being included within the validity window.
    ///
    /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
    /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
    pub lease: Option<[u8; 32]>,
    /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
    pub static_fee: Option<u64>,
    /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
    pub extra_fee: Option<u64>,
    /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
    pub max_fee: Option<u64>,
    /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
    pub validity_window: Option<u32>,
    /// Set the first round this transaction is valid.
    /// If left undefined, the value from algod will be used.
    ///
    /// We recommend you only set this when you intentionally want this to be some time in the future.
    pub first_valid_round: Option<u64>,
    /// The last round this transaction is valid. It is recommended to use validity window instead.
    pub last_valid_round: Option<u64>,
    /// ID of the app being deleted.
    pub app_id: u64,
    /// The ABI method to call.
    pub method: ABIMethod,
    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    pub args: Vec<T>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}

const ARGS_TUPLE_PACKING_THRESHOLD: usize = 14; // 14+ args trigger tuple packing, excluding the method selector  (arg 0)

fn process_app_method_call_args(args: &[AppMethodCallArg]) -> Vec<ProcessedAppMethodCallArg> {
    args.iter()
        .map(|arg| match arg {
            AppMethodCallArg::ABIValue(value) => ProcessedAppMethodCallArg::ABIValue(value.clone()),
            AppMethodCallArg::ABIReference(value) => {
                ProcessedAppMethodCallArg::ABIReference(value.clone())
            }
            _ => ProcessedAppMethodCallArg::TransactionPlaceholder,
        })
        .collect()
}

impl AppMethodCallCommonParams for AppCallMethodCallParams<ProcessedAppMethodCallArg> {
    fn app_id(&self) -> u64 {
        self.app_id
    }

    fn method(&self) -> &ABIMethod {
        &self.method
    }

    fn args(&self) -> &[ProcessedAppMethodCallArg] {
        &self.args
    }

    fn account_references(&self) -> Option<&Vec<Address>> {
        self.account_references.as_ref()
    }

    fn app_references(&self) -> Option<&Vec<u64>> {
        self.app_references.as_ref()
    }

    fn asset_references(&self) -> Option<&Vec<u64>> {
        self.asset_references.as_ref()
    }
}

impl AppMethodCallCommonParams for AppCreateMethodCallParams<ProcessedAppMethodCallArg> {
    fn app_id(&self) -> u64 {
        0 // Always 0 for creation
    }

    fn method(&self) -> &ABIMethod {
        &self.method
    }

    fn args(&self) -> &[ProcessedAppMethodCallArg] {
        &self.args
    }

    fn account_references(&self) -> Option<&Vec<Address>> {
        self.account_references.as_ref()
    }

    fn app_references(&self) -> Option<&Vec<u64>> {
        self.app_references.as_ref()
    }

    fn asset_references(&self) -> Option<&Vec<u64>> {
        self.asset_references.as_ref()
    }
}

impl AppMethodCallCommonParams for AppUpdateMethodCallParams<ProcessedAppMethodCallArg> {
    fn app_id(&self) -> u64 {
        self.app_id
    }

    fn method(&self) -> &ABIMethod {
        &self.method
    }

    fn args(&self) -> &[ProcessedAppMethodCallArg] {
        &self.args
    }

    fn account_references(&self) -> Option<&Vec<Address>> {
        self.account_references.as_ref()
    }

    fn app_references(&self) -> Option<&Vec<u64>> {
        self.app_references.as_ref()
    }

    fn asset_references(&self) -> Option<&Vec<u64>> {
        self.asset_references.as_ref()
    }
}

impl AppMethodCallCommonParams for AppDeleteMethodCallParams<ProcessedAppMethodCallArg> {
    fn app_id(&self) -> u64 {
        self.app_id
    }

    fn method(&self) -> &ABIMethod {
        &self.method
    }

    fn args(&self) -> &[ProcessedAppMethodCallArg] {
        &self.args
    }

    fn account_references(&self) -> Option<&Vec<Address>> {
        self.account_references.as_ref()
    }

    fn app_references(&self) -> Option<&Vec<u64>> {
        self.app_references.as_ref()
    }

    fn asset_references(&self) -> Option<&Vec<u64>> {
        self.asset_references.as_ref()
    }
}

impl From<&AppCallMethodCallParams> for AppCallMethodCallParams<ProcessedAppMethodCallArg> {
    fn from(params: &AppCallMethodCallParams) -> Self {
        let processed_args = process_app_method_call_args(&params.args);

        Self {
            sender: params.sender.clone(),
            signer: params.signer.clone(),
            rekey_to: params.rekey_to.clone(),
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: params.app_id,
            method: params.method.clone(),
            args: processed_args,
            account_references: params.account_references.clone(),
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            on_complete: params.on_complete,
        }
    }
}

impl From<&AppCreateMethodCallParams> for AppCreateMethodCallParams<ProcessedAppMethodCallArg> {
    fn from(params: &AppCreateMethodCallParams) -> Self {
        let processed_args = process_app_method_call_args(&params.args);

        Self {
            sender: params.sender.clone(),
            signer: params.signer.clone(),
            rekey_to: params.rekey_to.clone(),
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            method: params.method.clone(),
            args: processed_args,
            account_references: params.account_references.clone(),
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            on_complete: params.on_complete,
            approval_program: params.approval_program.clone(),
            clear_state_program: params.clear_state_program.clone(),
            global_state_schema: params.global_state_schema.clone(),
            local_state_schema: params.local_state_schema.clone(),
            extra_program_pages: params.extra_program_pages,
        }
    }
}

impl From<&AppUpdateMethodCallParams> for AppUpdateMethodCallParams<ProcessedAppMethodCallArg> {
    fn from(params: &AppUpdateMethodCallParams) -> Self {
        let processed_args = process_app_method_call_args(&params.args);

        Self {
            sender: params.sender.clone(),
            signer: params.signer.clone(),
            rekey_to: params.rekey_to.clone(),
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: params.app_id,
            method: params.method.clone(),
            args: processed_args,
            account_references: params.account_references.clone(),
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            approval_program: params.approval_program.clone(),
            clear_state_program: params.clear_state_program.clone(),
        }
    }
}

impl From<&AppDeleteMethodCallParams> for AppDeleteMethodCallParams<ProcessedAppMethodCallArg> {
    fn from(params: &AppDeleteMethodCallParams) -> Self {
        let processed_args = process_app_method_call_args(&params.args);

        Self {
            sender: params.sender.clone(),
            signer: params.signer.clone(),
            rekey_to: params.rekey_to.clone(),
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: params.app_id,
            method: params.method.clone(),
            args: processed_args,
            account_references: params.account_references.clone(),
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
        }
    }
}

trait AppMethodCallCommonParams {
    fn app_id(&self) -> u64;
    fn method(&self) -> &ABIMethod;
    fn args(&self) -> &[ProcessedAppMethodCallArg];
    fn account_references(&self) -> Option<&Vec<Address>>;
    fn app_references(&self) -> Option<&Vec<u64>>;
    fn asset_references(&self) -> Option<&Vec<u64>>;
}

fn populate_method_args_into_reference_arrays(
    sender: &Address,
    app_id: u64,
    method_args: &[ProcessedAppMethodCallArg],
    account_references: &mut Vec<Address>,
    app_references: &mut Vec<u64>,
    asset_references: &mut Vec<u64>,
) -> Result<(), ComposerError> {
    for method_arg in method_args.iter() {
        if let ProcessedAppMethodCallArg::ABIReference(value) = method_arg {
            match value {
                ABIReferenceValue::Account(addr_str) => {
                    let address = Address::from_str(addr_str).map_err(|_e| {
                        ComposerError::TransactionError {
                            message: format!("Invalid address {}", addr_str),
                        }
                    })?;

                    if address != *sender && !account_references.contains(&address) {
                        account_references.push(address);
                    }
                }
                ABIReferenceValue::Asset(asset_id) => {
                    if !asset_references.contains(asset_id) {
                        asset_references.push(*asset_id);
                    }
                }
                ABIReferenceValue::Application(app_id_ref) => {
                    if *app_id_ref != app_id && !app_references.contains(app_id_ref) {
                        app_references.push(*app_id_ref);
                    }
                }
            }
        }
    }

    Ok(())
}

fn calculate_method_arg_reference_array_index(
    ref_value: &ABIReferenceValue,
    sender: &Address,
    app_id: u64,
    account_references: &[Address],
    app_references: &[u64],
    asset_references: &[u64],
) -> Result<u8, ComposerError> {
    match ref_value {
        ABIReferenceValue::Account(addr_str) => {
            let address =
                Address::from_str(addr_str).map_err(|_e| ComposerError::TransactionError {
                    message: format!("Invalid address {}", addr_str),
                })?;

            if address == *sender {
                // If address is the same as sender, use index 0
                Ok(0)
            } else if let Some(existing_index) = account_references
                .iter()
                .position(|ref_addr| *ref_addr == address)
            {
                // If address already exists in account_references, use existing index + 1
                Ok((existing_index + 1) as u8)
            } else {
                Err(ComposerError::ABIEncodingError {
                    message: format!("Account {} not found in reference array", addr_str),
                })
            }
        }
        ABIReferenceValue::Asset(asset_id) => {
            if let Some(existing_index) = asset_references
                .iter()
                .position(|&ref_id| ref_id == *asset_id)
            {
                // If asset already exists in asset_references, use existing index
                Ok(existing_index as u8)
            } else {
                Err(ComposerError::ABIEncodingError {
                    message: format!("Asset {} not found in reference array", asset_id),
                })
            }
        }
        ABIReferenceValue::Application(app_id_ref) => {
            if *app_id_ref == app_id {
                // If app ID is the same as the current app, use index 0
                Ok(0)
            } else if let Some(existing_index) = app_references
                .iter()
                .position(|&ref_id| ref_id == *app_id_ref)
            {
                // If app already exists in app_references, use existing index + 1
                Ok((existing_index + 1) as u8)
            } else {
                Err(ComposerError::ABIEncodingError {
                    message: format!("Application {} not found in reference array", app_id_ref),
                })
            }
        }
    }
}

fn encode_method_arguments(
    method: &ABIMethod,
    args: &[ProcessedAppMethodCallArg],
    sender: &Address,
    app_id: u64,
    account_references: &[Address],
    app_references: &[u64],
    asset_references: &[u64],
) -> Result<Vec<Vec<u8>>, ComposerError> {
    let mut encoded_args = Vec::<Vec<u8>>::new();

    // Insert method selector at the front
    let method_selector = method
        .selector()
        .map_err(|e| ComposerError::ABIEncodingError {
            message: format!("Failed to get method selector: {}", e),
        })?;
    encoded_args.push(method_selector);

    let abi_types = method
        .args
        .iter()
        .filter_map(|arg| {
            match &arg.arg_type {
                ABIMethodArgType::Value(abi_type) => Some(abi_type.clone()),
                // Reference and transaction types encoded as uint8 indexes
                ABIMethodArgType::Reference(_) => Some(ABIType::Uint(
                    BitSize::new(8).expect("8 should always be a valid BitSize"),
                )),
                ABIMethodArgType::Transaction(_) => None,
            }
        })
        .collect::<Vec<_>>();

    let abi_values: Vec<ABIValue> = args
        .iter()
        .filter_map(|arg_value| -> Option<Result<ABIValue, ComposerError>> {
            match arg_value {
                ProcessedAppMethodCallArg::ABIReference(value) => {
                    let foreign_index = calculate_method_arg_reference_array_index(
                        value,
                        sender,
                        app_id,
                        account_references,
                        app_references,
                        asset_references,
                    );
                    Some(foreign_index.map(|index| ABIValue::Uint(BigUint::from(index))))
                }
                ProcessedAppMethodCallArg::ABIValue(value) => Some(Ok(value.clone())),
                ProcessedAppMethodCallArg::TransactionPlaceholder => None,
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    if abi_values.len() != abi_types.len() {
        return Err(ComposerError::ABIEncodingError {
            message: "Mismatch in length of non-transaction arguments".to_string(),
        });
    }

    // Apply ARC-4 tuple packing for methods with more than 14 arguments
    // 14 instead of 15 in the ARC-4 because the first argument (method selector) is added later on
    if abi_types.len() > ARGS_TUPLE_PACKING_THRESHOLD {
        encoded_args.extend(encode_args_with_tuple_packing(&abi_types, &abi_values)?);
    } else {
        encoded_args.extend(encode_args_individually(&abi_types, &abi_values)?);
    }

    Ok(encoded_args)
}

fn encode_args_with_tuple_packing(
    abi_types: &[ABIType],
    abi_values: &[ABIValue],
) -> Result<Vec<Vec<u8>>, ComposerError> {
    // Encode first 14 arguments individually
    let first_14_abi_types = &abi_types[..ARGS_TUPLE_PACKING_THRESHOLD];
    let first_14_abi_values = &abi_values[..ARGS_TUPLE_PACKING_THRESHOLD];
    let encoded_args: &mut Vec<Vec<u8>> =
        &mut encode_args_individually(first_14_abi_types, first_14_abi_values)?;

    // Pack remaining arguments into tuple at position 14
    let remaining_abi_types = &abi_types[ARGS_TUPLE_PACKING_THRESHOLD..];
    let remaining_abi_values = &abi_values[ARGS_TUPLE_PACKING_THRESHOLD..];
    let tuple_type = ABIType::Tuple(remaining_abi_types.to_vec());
    let tuple_value = ABIValue::Array(remaining_abi_values.to_vec());
    let tuple_encoded =
        tuple_type
            .encode(&tuple_value)
            .map_err(|e| ComposerError::ABIEncodingError {
                message: format!("Failed to encode ABI value: {}", e),
            })?;

    encoded_args.push(tuple_encoded);

    Ok(encoded_args.to_vec())
}

fn encode_args_individually(
    abi_types: &[ABIType],
    abi_values: &[ABIValue],
) -> Result<Vec<Vec<u8>>, ComposerError> {
    let encoded_args: &mut Vec<Vec<u8>> = &mut Vec::new();

    for (abi_value, abi_type) in abi_values.iter().zip(abi_types.iter()) {
        let encoded = abi_type
            .encode(abi_value)
            .map_err(|e| ComposerError::ABIEncodingError {
                message: format!("Failed to encode ABI value: {}", e),
            })?;
        encoded_args.push(encoded);
    }

    Ok(encoded_args.to_vec())
}

pub fn build_app_call(
    params: &AppCallParams,
    header: TransactionHeader,
) -> Result<Transaction, String> {
    let mut builder = AppCallTransactionBuilder::default();
    builder
        .header(header)
        .app_id(params.app_id)
        .on_complete(params.on_complete);

    if let Some(ref args) = params.args {
        builder.args(args.clone());
    }

    if let Some(ref account_references) = params.account_references {
        builder.account_references(account_references.clone());
    }

    if let Some(ref app_references) = params.app_references {
        builder.app_references(app_references.clone());
    }

    if let Some(ref asset_references) = params.asset_references {
        builder.asset_references(asset_references.clone());
    }

    if let Some(ref box_references) = params.box_references {
        builder.box_references(box_references.clone());
    }

    builder.build().map_err(|e| e.to_string())
}

pub fn build_app_create_call(params: &AppCreateParams, header: TransactionHeader) -> Transaction {
    Transaction::AppCall(AppCallTransactionFields {
        header,
        app_id: 0,
        on_complete: params.on_complete,
        approval_program: Some(params.approval_program.clone()),
        clear_state_program: Some(params.clear_state_program.clone()),
        global_state_schema: params.global_state_schema.clone(),
        local_state_schema: params.local_state_schema.clone(),
        extra_program_pages: params.extra_program_pages,
        args: params.args.clone(),
        account_references: params.account_references.clone(),
        app_references: params.app_references.clone(),
        asset_references: params.asset_references.clone(),
        box_references: params.box_references.clone(),
    })
}

pub fn build_app_update_call(params: &AppUpdateParams, header: TransactionHeader) -> Transaction {
    Transaction::AppCall(AppCallTransactionFields {
        header,
        app_id: params.app_id,
        on_complete: OnApplicationComplete::UpdateApplication,
        approval_program: Some(params.approval_program.clone()),
        clear_state_program: Some(params.clear_state_program.clone()),
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        args: params.args.clone(),
        account_references: params.account_references.clone(),
        app_references: params.app_references.clone(),
        asset_references: params.asset_references.clone(),
        box_references: params.box_references.clone(),
    })
}

pub fn build_app_delete_call(params: &AppDeleteParams, header: TransactionHeader) -> Transaction {
    Transaction::AppCall(AppCallTransactionFields {
        header,
        app_id: params.app_id,
        on_complete: OnApplicationComplete::DeleteApplication,
        approval_program: None,
        clear_state_program: None,
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        args: params.args.clone(),
        account_references: params.account_references.clone(),
        app_references: params.app_references.clone(),
        asset_references: params.asset_references.clone(),
        box_references: params.box_references.clone(),
    })
}

fn build_method_call_common<T, F>(
    header: TransactionHeader,
    params: &T,
    transaction_builder: F,
) -> Result<Transaction, ComposerError>
where
    T: AppMethodCallCommonParams,
    F: FnOnce(TransactionHeader, Vec<Address>, Vec<u64>, Vec<u64>, Vec<Vec<u8>>) -> Transaction,
{
    let mut account_references = params.account_references().cloned().unwrap_or_default();
    let mut app_references = params.app_references().cloned().unwrap_or_default();
    let mut asset_references = params.asset_references().cloned().unwrap_or_default();

    populate_method_args_into_reference_arrays(
        &header.sender,
        params.app_id(),
        params.args(),
        &mut account_references,
        &mut app_references,
        &mut asset_references,
    )?;

    let encoded_args = encode_method_arguments(
        params.method(),
        params.args(),
        &header.sender,
        params.app_id(),
        &account_references,
        &app_references,
        &asset_references,
    )?;

    Ok(transaction_builder(
        header,
        account_references,
        app_references,
        asset_references,
        encoded_args,
    ))
}

pub fn build_app_call_method_call(
    params: &AppCallMethodCallParams<ProcessedAppMethodCallArg>,
    header: TransactionHeader,
) -> Result<Transaction, ComposerError> {
    build_method_call_common(
        header.clone(),
        params,
        |header, account_refs, app_refs, asset_refs, encoded_args| {
            Transaction::AppCall(algokit_transact::AppCallTransactionFields {
                header,
                app_id: params.app_id,
                on_complete: params.on_complete,
                approval_program: None,
                clear_state_program: None,
                global_state_schema: None,
                local_state_schema: None,
                extra_program_pages: None,
                args: Some(encoded_args),
                account_references: Some(account_refs),
                app_references: Some(app_refs),
                asset_references: Some(asset_refs),
                box_references: params.box_references.clone(),
            })
        },
    )
}

pub fn build_app_create_method_call(
    params: &AppCreateMethodCallParams<ProcessedAppMethodCallArg>,
    header: TransactionHeader,
) -> Result<Transaction, ComposerError> {
    build_method_call_common(
        header.clone(),
        params,
        |header, account_refs, app_refs, asset_refs, encoded_args| {
            Transaction::AppCall(algokit_transact::AppCallTransactionFields {
                header,
                app_id: 0, // 0 indicates app creation
                on_complete: params.on_complete,
                approval_program: Some(params.approval_program.clone()),
                clear_state_program: Some(params.clear_state_program.clone()),
                global_state_schema: params.global_state_schema.clone(),
                local_state_schema: params.local_state_schema.clone(),
                extra_program_pages: params.extra_program_pages,
                args: Some(encoded_args),
                account_references: Some(account_refs),
                app_references: Some(app_refs),
                asset_references: Some(asset_refs),
                box_references: params.box_references.clone(),
            })
        },
    )
}

pub fn build_app_update_method_call(
    params: &AppUpdateMethodCallParams<ProcessedAppMethodCallArg>,
    header: TransactionHeader,
) -> Result<Transaction, ComposerError> {
    build_method_call_common(
        header.clone(),
        params,
        |header, account_refs, app_refs, asset_refs, encoded_args| {
            Transaction::AppCall(algokit_transact::AppCallTransactionFields {
                header,
                app_id: params.app_id,
                on_complete: OnApplicationComplete::UpdateApplication,
                approval_program: Some(params.approval_program.clone()),
                clear_state_program: Some(params.clear_state_program.clone()),
                global_state_schema: None,
                local_state_schema: None,
                extra_program_pages: None,
                args: Some(encoded_args),
                account_references: Some(account_refs),
                app_references: Some(app_refs),
                asset_references: Some(asset_refs),
                box_references: params.box_references.clone(),
            })
        },
    )
}

pub fn build_app_delete_method_call(
    params: &AppDeleteMethodCallParams<ProcessedAppMethodCallArg>,
    header: TransactionHeader,
) -> Result<Transaction, ComposerError> {
    build_method_call_common(
        header.clone(),
        params,
        |header, account_refs, app_refs, asset_refs, encoded_args| {
            Transaction::AppCall(algokit_transact::AppCallTransactionFields {
                header,
                app_id: params.app_id,
                on_complete: OnApplicationComplete::DeleteApplication,
                approval_program: None,
                clear_state_program: None,
                global_state_schema: None,
                local_state_schema: None,
                extra_program_pages: None,
                args: Some(encoded_args),
                account_references: Some(account_refs),
                app_references: Some(app_refs),
                asset_references: Some(asset_refs),
                box_references: params.box_references.clone(),
            })
        },
    )
}
