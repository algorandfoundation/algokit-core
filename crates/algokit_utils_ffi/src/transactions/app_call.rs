use crate::abi::{abi_type::ABIType, abi_value::ABIValue};
use crate::create_transaction_params;
use algokit_transact::Address;
use algokit_transact_ffi::Transaction;
use algokit_transact_ffi::transactions::app_call::{
    BoxReference, OnApplicationComplete, StateSchema,
};
use derive_more::Debug;
use std::sync::Arc;

use super::asset_config::{AssetConfigParams, AssetCreateParams, AssetDestroyParams};
use super::asset_freeze::{AssetFreezeParams, AssetUnfreezeParams};
use super::asset_transfer::{
    AssetClawbackParams, AssetOptInParams, AssetOptOutParams, AssetTransferParams,
};
use super::common::{
    FfiTransactionSignerFromRust, RustTransactionSignerFromFfi, TransactionSigner,
    TransactionWithSigner, UtilsError,
};
use super::key_registration::{
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use super::payment::{AccountCloseParams, PaymentParams};

use algokit_utils::transactions::app_call::{
    AppCallParams as RustAppCallParams, AppCreateParams as RustAppCreateParams,
};

use algokit_abi::ABIMethodArg as RustABIMethodArg;
use algokit_abi::ABIMethodArgType as RustABIMethodArgType;
use algokit_abi::ABIReferenceType as RustABIReferenceType;
use algokit_abi::ABITransactionType as RustABITransactionType;
use algokit_abi::abi_method::ABIReferenceValue as RustABIReferenceValue;
use algokit_utils::transactions::app_call::AppCallMethodCallParams as RustAppCallMethodCallParams;
use algokit_utils::transactions::app_call::AppMethodCallArg as RustAppMethodCallArg;
use algokit_utils::transactions::common::TransactionSigner as RustTransactionSigner;

#[derive(uniffi::Enum, Debug)]
pub enum ABIReferenceType {
    /// Reference to an account in the Accounts reference array
    Account,
    /// Reference to an application in the Applications reference array
    Application,
    /// Reference to an asset in the Assets reference array
    Asset,
}

#[derive(uniffi::Enum, Debug)]
pub enum ABITransactionType {
    /// Any transaction type
    Txn,
    /// Payment (algo transfer)
    Payment,
    /// Key registration (configure consensus participation)
    KeyRegistration,
    /// Asset configuration (create, configure, or destroy ASAs)
    AssetConfig,
    /// Asset transfer (ASA transfer)
    AssetTransfer,
    /// Asset freeze (freeze or unfreeze ASAs)
    AssetFreeze,
    /// App call (create, update, delete and call an app)
    AppCall,
}

#[derive(uniffi::Enum, Debug)]
pub enum ABIMethodArgType {
    /// A value that is directly encoded in the app arguments.
    Value(Arc<ABIType>),
    Transaction(ABITransactionType),
    Reference(ABIReferenceType),
}

impl From<ABIMethodArgType> for RustABIMethodArgType {
    fn from(value: ABIMethodArgType) -> Self {
        match value {
            ABIMethodArgType::Value(abi_type) => {
                RustABIMethodArgType::Value(abi_type.abi_type.clone())
            }
            ABIMethodArgType::Transaction(txn_type) => {
                let rust_txn_type = match txn_type {
                    ABITransactionType::Txn => RustABITransactionType::Txn,
                    ABITransactionType::Payment => RustABITransactionType::Payment,
                    ABITransactionType::KeyRegistration => RustABITransactionType::KeyRegistration,
                    ABITransactionType::AssetConfig => RustABITransactionType::AssetConfig,
                    ABITransactionType::AssetTransfer => RustABITransactionType::AssetTransfer,
                    ABITransactionType::AssetFreeze => RustABITransactionType::AssetFreeze,
                    ABITransactionType::AppCall => RustABITransactionType::AppCall,
                };
                RustABIMethodArgType::Transaction(rust_txn_type)
            }
            ABIMethodArgType::Reference(ref_type) => {
                let rust_ref_type = match ref_type {
                    ABIReferenceType::Account => RustABIReferenceType::Account,
                    ABIReferenceType::Application => RustABIReferenceType::Application,
                    ABIReferenceType::Asset => RustABIReferenceType::Asset,
                };
                RustABIMethodArgType::Reference(rust_ref_type)
            }
        }
    }
}

impl From<RustABIMethodArgType> for ABIMethodArgType {
    fn from(value: RustABIMethodArgType) -> Self {
        match value {
            RustABIMethodArgType::Value(abi_type) => {
                ABIMethodArgType::Value(Arc::new(ABIType { abi_type }))
            }
            RustABIMethodArgType::Transaction(txn_type) => {
                let ffi_txn_type = match txn_type {
                    RustABITransactionType::Txn => ABITransactionType::Txn,
                    RustABITransactionType::Payment => ABITransactionType::Payment,
                    RustABITransactionType::KeyRegistration => ABITransactionType::KeyRegistration,
                    RustABITransactionType::AssetConfig => ABITransactionType::AssetConfig,
                    RustABITransactionType::AssetTransfer => ABITransactionType::AssetTransfer,
                    RustABITransactionType::AssetFreeze => ABITransactionType::AssetFreeze,
                    RustABITransactionType::AppCall => ABITransactionType::AppCall,
                };
                ABIMethodArgType::Transaction(ffi_txn_type)
            }
            RustABIMethodArgType::Reference(ref_type) => {
                let ffi_ref_type = match ref_type {
                    RustABIReferenceType::Account => ABIReferenceType::Account,
                    RustABIReferenceType::Application => ABIReferenceType::Application,
                    RustABIReferenceType::Asset => ABIReferenceType::Asset,
                };
                ABIMethodArgType::Reference(ffi_ref_type)
            }
        }
    }
}

#[derive(uniffi::Record, Debug)]
pub struct ABIMethodArg {
    /// The type of the argument.
    pub arg_type: ABIMethodArgType,
    /// An optional name for the argument.
    pub name: Option<String>,
    /// An optional description of the argument.
    pub description: Option<String>,
}

impl From<ABIMethodArg> for RustABIMethodArg {
    fn from(value: ABIMethodArg) -> Self {
        RustABIMethodArg {
            arg_type: value.arg_type.into(),
            name: value.name,
            description: value.description,
            default_value: None, // FFI doesn't support default values yet
        }
    }
}

impl From<RustABIMethodArg> for ABIMethodArg {
    fn from(value: RustABIMethodArg) -> Self {
        ABIMethodArg {
            arg_type: value.arg_type.into(),
            name: value.name,
            description: value.description,
        }
    }
}

use algokit_abi::ABIMethod as RustABIMethod;

#[derive(uniffi::Record, Debug)]
pub struct ABIMethod {
    /// The name of the method.
    pub name: String,
    /// A list of the method's arguments.
    pub args: Vec<ABIMethodArg>,
    /// The return type of the method, or `None` if the method does not return a value.
    pub returns: Option<Arc<ABIType>>,
    /// An optional description of the method.
    pub description: Option<String>,
}

impl From<ABIMethod> for RustABIMethod {
    fn from(value: ABIMethod) -> Self {
        RustABIMethod {
            name: value.name,
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
            returns: value.returns.map(|r| r.abi_type.clone()),
            description: value.description,
        }
    }
}

impl From<RustABIMethod> for ABIMethod {
    fn from(value: RustABIMethod) -> Self {
        ABIMethod {
            name: value.name,
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
            returns: value.returns.map(|r| Arc::new(ABIType { abi_type: r })),
            description: value.description,
        }
    }
}

#[derive(uniffi::Enum, Debug)]
pub enum ABIReferenceValue {
    /// The address to an Algorand account.
    Account(String),
    /// An Algorand asset ID.
    Asset(u64),
    /// An Algorand app ID.
    Application(u64),
}

impl TryFrom<ABIReferenceValue> for RustABIReferenceValue {
    type Error = UtilsError;
    fn try_from(value: ABIReferenceValue) -> Result<Self, Self::Error> {
        Ok(match value {
            ABIReferenceValue::Account(address) => {
                RustABIReferenceValue::Account(address.parse().map_err(|e| {
                    UtilsError::UtilsError {
                        message: format!("Invalid account address: {}", e),
                    }
                })?)
            }
            ABIReferenceValue::Asset(asset_id) => RustABIReferenceValue::Asset(asset_id),
            ABIReferenceValue::Application(app_id) => RustABIReferenceValue::Application(app_id),
        })
    }
}

impl From<RustABIReferenceValue> for ABIReferenceValue {
    fn from(value: RustABIReferenceValue) -> Self {
        match value {
            RustABIReferenceValue::Account(address) => {
                ABIReferenceValue::Account(address.to_string())
            }
            RustABIReferenceValue::Asset(asset_id) => ABIReferenceValue::Asset(asset_id),
            RustABIReferenceValue::Application(app_id) => ABIReferenceValue::Application(app_id),
        }
    }
}

#[derive(uniffi::Enum, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum AppMethodCallArg {
    ABIReference(ABIReferenceValue),
    /// Sentinel to request ARC-56 default resolution for this argument (handled by AppClient params builder)
    DefaultValue,
    /// Placeholder for a transaction-typed argument. Not encoded; satisfied by a transaction
    /// included in the same group (extracted from other method call arguments).
    TransactionPlaceHolder,
    ABIValue(Arc<ABIValue>),
    AppCreateCall(AppCreateParams),
    AppUpdateCall(AppUpdateParams),
    AppDeleteCall(AppDeleteParams),
    AppCallMethodCall(AppCallMethodCallParams),
    AppCreateMethodCall(AppCreateMethodCallParams),
    AppUpdateMethodCall(AppUpdateMethodCallParams),
    AppDeleteMethodCall(AppDeleteMethodCallParams),
    Transaction(Transaction),
    #[debug("TransactionWithSigner")]
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
    OnlineKeyRegistration(OnlineKeyRegistrationParams),
    OfflineKeyRegistration(OfflineKeyRegistrationParams),
    NonParticipationKeyRegistration(NonParticipationKeyRegistrationParams),
}

impl TryFrom<AppMethodCallArg> for RustAppMethodCallArg {
    type Error = UtilsError;
    fn try_from(value: AppMethodCallArg) -> Result<Self, Self::Error> {
        Ok(match value {
            AppMethodCallArg::ABIValue(abi_value) => {
                RustAppMethodCallArg::ABIValue(abi_value.rust_value.clone())
            }
            AppMethodCallArg::AppCreateCall(app_create_params) => {
                RustAppMethodCallArg::AppCreateCall(app_create_params.try_into()?)
            }
            AppMethodCallArg::AppUpdateCall(app_update_params) => {
                RustAppMethodCallArg::AppUpdateCall(app_update_params.try_into()?)
            }
            AppMethodCallArg::AppDeleteCall(app_delete_params) => {
                RustAppMethodCallArg::AppDeleteCall(app_delete_params.try_into()?)
            }
            AppMethodCallArg::AppCallMethodCall(app_call_method_params) => {
                RustAppMethodCallArg::AppCallMethodCall(app_call_method_params.try_into()?)
            }
            AppMethodCallArg::AppCreateMethodCall(app_create_method_params) => {
                RustAppMethodCallArg::AppCreateMethodCall(app_create_method_params.try_into()?)
            }
            AppMethodCallArg::AppUpdateMethodCall(app_update_method_params) => {
                RustAppMethodCallArg::AppUpdateMethodCall(app_update_method_params.try_into()?)
            }
            AppMethodCallArg::AppDeleteMethodCall(app_delete_method_params) => {
                RustAppMethodCallArg::AppDeleteMethodCall(app_delete_method_params.try_into()?)
            }
            AppMethodCallArg::Transaction(txn) => {
                RustAppMethodCallArg::Transaction(txn.try_into().map_err(|e| {
                    UtilsError::UtilsError {
                        message: format!("Invalid transaction: {}", e),
                    }
                })?)
            }
            AppMethodCallArg::TransactionWithSigner(txn_with_signer) => {
                RustAppMethodCallArg::TransactionWithSigner(txn_with_signer.try_into()?)
            }
            AppMethodCallArg::Payment(payment_params) => {
                RustAppMethodCallArg::Payment(payment_params.try_into()?)
            }
            AppMethodCallArg::AccountClose(account_close_params) => {
                RustAppMethodCallArg::AccountClose(account_close_params.try_into()?)
            }
            AppMethodCallArg::AssetTransfer(asset_transfer_params) => {
                RustAppMethodCallArg::AssetTransfer(asset_transfer_params.try_into()?)
            }
            AppMethodCallArg::AssetOptIn(asset_opt_in_params) => {
                RustAppMethodCallArg::AssetOptIn(asset_opt_in_params.try_into()?)
            }
            AppMethodCallArg::AssetOptOut(asset_opt_out_params) => {
                RustAppMethodCallArg::AssetOptOut(asset_opt_out_params.try_into()?)
            }
            AppMethodCallArg::AssetClawback(asset_clawback_params) => {
                RustAppMethodCallArg::AssetClawback(asset_clawback_params.try_into()?)
            }
            AppMethodCallArg::AssetCreate(asset_create_params) => {
                RustAppMethodCallArg::AssetCreate(asset_create_params.try_into()?)
            }
            AppMethodCallArg::AssetConfig(asset_config_params) => {
                RustAppMethodCallArg::AssetConfig(asset_config_params.try_into()?)
            }
            AppMethodCallArg::AssetDestroy(asset_destroy_params) => {
                RustAppMethodCallArg::AssetDestroy(asset_destroy_params.try_into()?)
            }
            AppMethodCallArg::AssetFreeze(asset_freeze_params) => {
                RustAppMethodCallArg::AssetFreeze(asset_freeze_params.try_into()?)
            }
            AppMethodCallArg::AssetUnfreeze(asset_unfreeze_params) => {
                RustAppMethodCallArg::AssetUnfreeze(asset_unfreeze_params.try_into()?)
            }
            AppMethodCallArg::AppCall(app_call_params) => {
                RustAppMethodCallArg::AppCall(app_call_params.try_into()?)
            }
            AppMethodCallArg::OnlineKeyRegistration(online_key_registration_params) => {
                RustAppMethodCallArg::OnlineKeyRegistration(
                    online_key_registration_params.try_into()?,
                )
            }
            AppMethodCallArg::OfflineKeyRegistration(offline_key_registration_params) => {
                RustAppMethodCallArg::OfflineKeyRegistration(
                    offline_key_registration_params.try_into()?,
                )
            }
            AppMethodCallArg::NonParticipationKeyRegistration(
                non_participation_key_registration_params,
            ) => RustAppMethodCallArg::NonParticipationKeyRegistration(
                non_participation_key_registration_params.try_into()?,
            ),
            AppMethodCallArg::DefaultValue => RustAppMethodCallArg::DefaultValue,
            AppMethodCallArg::TransactionPlaceHolder => {
                RustAppMethodCallArg::TransactionPlaceholder
            }
            AppMethodCallArg::ABIReference(abi_reference) => {
                RustAppMethodCallArg::ABIReference(abi_reference.try_into()?)
            }
        })
    }
}

impl TryFrom<RustAppMethodCallArg> for AppMethodCallArg {
    type Error = UtilsError;

    fn try_from(value: RustAppMethodCallArg) -> Result<Self, Self::Error> {
        Ok(match value {
            RustAppMethodCallArg::ABIValue(rust_value) => {
                AppMethodCallArg::ABIValue(Arc::new(ABIValue { rust_value }))
            }
            RustAppMethodCallArg::AppCreateCall(app_create_params) => {
                AppMethodCallArg::AppCreateCall(app_create_params.into())
            }
            RustAppMethodCallArg::AppUpdateCall(app_update_params) => {
                AppMethodCallArg::AppUpdateCall(app_update_params.into())
            }
            RustAppMethodCallArg::AppDeleteCall(app_delete_params) => {
                AppMethodCallArg::AppDeleteCall(app_delete_params.into())
            }
            RustAppMethodCallArg::AppCallMethodCall(app_call_method_params) => {
                AppMethodCallArg::AppCallMethodCall(app_call_method_params.try_into()?)
            }
            RustAppMethodCallArg::AppCreateMethodCall(app_create_method_params) => {
                AppMethodCallArg::AppCreateMethodCall(app_create_method_params.try_into()?)
            }
            RustAppMethodCallArg::AppUpdateMethodCall(app_update_method_params) => {
                AppMethodCallArg::AppUpdateMethodCall(app_update_method_params.try_into()?)
            }
            RustAppMethodCallArg::AppDeleteMethodCall(app_delete_method_params) => {
                AppMethodCallArg::AppDeleteMethodCall(app_delete_method_params.try_into()?)
            }
            RustAppMethodCallArg::Transaction(txn) => AppMethodCallArg::Transaction(txn.into()),
            RustAppMethodCallArg::TransactionWithSigner(txn_with_signer) => {
                AppMethodCallArg::TransactionWithSigner(txn_with_signer.try_into()?)
            }
            RustAppMethodCallArg::Payment(payment_params) => {
                AppMethodCallArg::Payment(payment_params.into())
            }
            RustAppMethodCallArg::AccountClose(account_close_params) => {
                AppMethodCallArg::AccountClose(account_close_params.into())
            }
            RustAppMethodCallArg::AssetTransfer(asset_transfer_params) => {
                AppMethodCallArg::AssetTransfer(asset_transfer_params.into())
            }
            RustAppMethodCallArg::AssetOptIn(asset_opt_in_params) => {
                AppMethodCallArg::AssetOptIn(asset_opt_in_params.into())
            }
            RustAppMethodCallArg::AssetOptOut(asset_opt_out_params) => {
                AppMethodCallArg::AssetOptOut(asset_opt_out_params.into())
            }
            RustAppMethodCallArg::AssetClawback(asset_clawback_params) => {
                AppMethodCallArg::AssetClawback(asset_clawback_params.into())
            }
            RustAppMethodCallArg::AssetCreate(asset_create_params) => {
                AppMethodCallArg::AssetCreate(asset_create_params.into())
            }
            RustAppMethodCallArg::AssetConfig(asset_config_params) => {
                AppMethodCallArg::AssetConfig(asset_config_params.into())
            }
            RustAppMethodCallArg::AssetDestroy(asset_destroy_params) => {
                AppMethodCallArg::AssetDestroy(asset_destroy_params.into())
            }
            RustAppMethodCallArg::AssetFreeze(asset_freeze_params) => {
                AppMethodCallArg::AssetFreeze(asset_freeze_params.into())
            }
            RustAppMethodCallArg::AssetUnfreeze(asset_unfreeze_params) => {
                AppMethodCallArg::AssetUnfreeze(asset_unfreeze_params.into())
            }
            RustAppMethodCallArg::AppCall(app_call_params) => {
                AppMethodCallArg::AppCall(app_call_params.into())
            }
            RustAppMethodCallArg::OnlineKeyRegistration(online_key_registration_params) => {
                AppMethodCallArg::OnlineKeyRegistration(online_key_registration_params.into())
            }
            RustAppMethodCallArg::OfflineKeyRegistration(offline_key_registration_params) => {
                AppMethodCallArg::OfflineKeyRegistration(offline_key_registration_params.into())
            }
            RustAppMethodCallArg::NonParticipationKeyRegistration(
                non_participation_key_registration_params,
            ) => AppMethodCallArg::NonParticipationKeyRegistration(
                non_participation_key_registration_params.into(),
            ),
            RustAppMethodCallArg::DefaultValue => AppMethodCallArg::DefaultValue,
            RustAppMethodCallArg::ABIReference(_) => {
                todo!()
            }
            RustAppMethodCallArg::TransactionPlaceholder => {
                AppMethodCallArg::TransactionPlaceHolder
            }
        })
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCallMethodCallParams {
        /// ID of the app being called.
        pub app_id: u64,
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
        /// Defines what additional actions occur with the transaction.
        pub on_complete: OnApplicationComplete,
    }
}

impl TryFrom<AppCallMethodCallParams> for RustAppCallMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppCallMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppCallMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
            on_complete: value.on_complete.into(),
        })
    }
}

impl TryFrom<RustAppCallMethodCallParams> for AppCallMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: RustAppCallMethodCallParams) -> Result<Self, Self::Error> {
        Ok(AppCallMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
            on_complete: value.on_complete.into(),
        })
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCallParams {
        /// ID of the app being called.
        pub app_id: u64,
        /// Defines what additional actions occur with the transaction.
        pub on_complete: OnApplicationComplete,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        #[uniffi(default = None)]
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppCallParams> for RustAppCallParams {
    type Error = UtilsError;

    fn try_from(value: AppCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            on_complete: value.on_complete.into(),
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl From<RustAppCallParams> for AppCallParams {
    fn from(value: RustAppCallParams) -> Self {
        AppCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            on_complete: value.on_complete.into(),
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        }
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
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
        #[uniffi(default = None)]
        pub global_state_schema: Option<StateSchema>,
        /// Holds the maximum number of local state values.
        ///
        /// This cannot be changed after creation.
        #[uniffi(default = None)]
        pub local_state_schema: Option<StateSchema>,
        /// Number of additional pages allocated to the app's approval
        /// and clear state programs.
        ///
        /// Each extra program page is 2048 bytes. The sum of approval program
        /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
        /// Currently, the maximum value is 3.
        /// This cannot be changed after creation.
        #[uniffi(default = None)]
        pub extra_program_pages: Option<u64>,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        #[uniffi(default = None)]
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppCreateParams> for RustAppCreateParams {
    type Error = UtilsError;

    fn try_from(value: AppCreateParams) -> Result<Self, Self::Error> {
        Ok(RustAppCreateParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u32),
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(
                    |e: <Address as std::str::FromStr>::Err| UtilsError::UtilsError {
                        message: e.to_string(),
                    },
                )?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl From<RustAppCreateParams> for AppCreateParams {
    fn from(value: RustAppCreateParams) -> Self {
        AppCreateParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u64),
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        }
    }
}

use algokit_utils::transactions::app_call::{
    AppDeleteParams as RustAppDeleteParams, AppUpdateParams as RustAppUpdateParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppDeleteParams {
        /// ID of the app being deleted.
        pub app_id: u64,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        #[uniffi(default = None)]
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppDeleteParams> for RustAppDeleteParams {
    type Error = UtilsError;

    fn try_from(value: AppDeleteParams) -> Result<Self, Self::Error> {
        Ok(RustAppDeleteParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl From<RustAppDeleteParams> for AppDeleteParams {
    fn from(value: RustAppDeleteParams) -> Self {
        AppDeleteParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        }
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
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
        #[uniffi(default = None)]
        pub args: Option<Vec<Vec<u8>>>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppUpdateParams> for RustAppUpdateParams {
    type Error = UtilsError;

    fn try_from(value: AppUpdateParams) -> Result<Self, Self::Error> {
        Ok(RustAppUpdateParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl From<RustAppUpdateParams> for AppUpdateParams {
    fn from(value: RustAppUpdateParams) -> Self {
        AppUpdateParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        }
    }
}

use algokit_utils::transactions::app_call::{
    AppCreateMethodCallParams as RustAppCreateMethodCallParams,
    AppDeleteMethodCallParams as RustAppDeleteMethodCallParams,
    AppUpdateMethodCallParams as RustAppUpdateMethodCallParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCreateMethodCallParams {
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
        #[uniffi(default = None)]
        pub global_state_schema: Option<StateSchema>,
        /// Holds the maximum number of local state values.
        ///
        /// This cannot be changed after creation.
        #[uniffi(default = None)]
        pub local_state_schema: Option<StateSchema>,
        /// Number of additional pages allocated to the app's approval
        /// and clear state programs.
        ///
        /// Each extra program page is 2048 bytes. The sum of approval program
        /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
        /// Currently, the maximum value is 3.
        /// This cannot be changed after creation.
        #[uniffi(default = None)]
        pub extra_program_pages: Option<u64>,
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppCreateMethodCallParams> for RustAppCreateMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppCreateMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppCreateMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u32),
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,

            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl TryFrom<RustAppCreateMethodCallParams> for AppCreateMethodCallParams {
    type Error = UtilsError;
    fn try_from(value: RustAppCreateMethodCallParams) -> Result<Self, Self::Error> {
        Ok(AppCreateMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u64),
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppUpdateMethodCallParams {
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
        pub args: Vec<AppMethodCallArg>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppUpdateMethodCallParams> for RustAppUpdateMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppUpdateMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppUpdateMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl TryFrom<RustAppUpdateMethodCallParams> for AppUpdateMethodCallParams {
    type Error = UtilsError;
    fn try_from(value: RustAppUpdateMethodCallParams) -> Result<Self, Self::Error> {
        Ok(AppUpdateMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppDeleteMethodCallParams {
        /// ID of the app being deleted.
        pub app_id: u64,
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
        /// List of accounts in addition to the sender that may be accessed
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub account_references: Option<Vec<String>>,
        /// List of apps in addition to the current app that may be called
        /// from the app's approval program and clear state program.
        #[uniffi(default = None)]
        pub app_references: Option<Vec<u64>>,
        /// Lists the assets whose parameters may be accessed by this app's
        /// approval program and clear state program.
        ///
        /// The access is read-only.
        #[uniffi(default = None)]
        pub asset_references: Option<Vec<u64>>,
        /// The boxes that should be made available for the runtime of the program.
        #[uniffi(default = None)]
        pub box_references: Option<Vec<BoxReference>>,
    }
}

impl TryFrom<AppDeleteMethodCallParams> for RustAppDeleteMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppDeleteMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppDeleteMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl TryFrom<RustAppDeleteMethodCallParams> for AppDeleteMethodCallParams {
    type Error = UtilsError;
    fn try_from(value: RustAppDeleteMethodCallParams) -> Result<Self, Self::Error> {
        Ok(AppDeleteMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value
                .args
                .into_iter()
                .map(|arg| arg.try_into())
                .collect::<Result<_, _>>()?,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}
