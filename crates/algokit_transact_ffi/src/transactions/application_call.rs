use crate::*;

/// Represents an application call transaction that interacts with Algorand Smart Contracts.
///
/// Application call transactions are used to create, update, delete, opt-in to,
/// close out of, or clear state from Algorand applications (smart contracts).
#[ffi_record]
pub struct ApplicationCallTransactionFields {
    /// ID of the application being called.
    ///
    /// Set this to 0 to indicate an application creation call.
    app_id: u64,

    /// Defines what additional actions occur with the transaction.
    on_complete: OnApplicationComplete,

    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for application creation and update transactions.
    approval_program: Option<ByteBuf>,

    /// Logic executed for application call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for application creation and update transactions.
    clear_state_program: Option<ByteBuf>,

    /// Holds the maximum number of global state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    global_state_schema: Option<StateSchema>,

    /// Holds the maximum number of local state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    local_state_schema: Option<StateSchema>,

    /// Number of additional pages allocated to the application's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    extra_program_pages: Option<u64>,

    /// Transaction specific arguments available in the application's
    /// approval program and clear state program.
    args: Option<Vec<ByteBuf>>,

    /// List of accounts in addition to the sender that may be accessed
    /// from the application's approval program and clear state program.
    account_references: Option<Vec<Address>>,

    /// List of applications in addition to the application ID that may be called
    /// from the application's approval program and clear state program.
    app_references: Option<Vec<u64>>,

    /// Lists the assets whose parameters may be accessed by this application's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    asset_references: Option<Vec<u64>>,

    /// The boxes that should be made available for the runtime of the program.
    box_references: Option<Vec<BoxReference>>,
}

impl From<algokit_transact::ApplicationCallTransactionFields> for ApplicationCallTransactionFields {
    fn from(tx: algokit_transact::ApplicationCallTransactionFields) -> Self {
        Self {
            app_id: tx.app_id,
            on_complete: tx.on_complete.into(),
            approval_program: tx.approval_program.map(Into::into),
            clear_state_program: tx.clear_state_program.map(Into::into),
            global_state_schema: tx.global_state_schema.map(Into::into),
            local_state_schema: tx.local_state_schema.map(Into::into),
            extra_program_pages: tx.extra_program_pages,
            args: tx
                .args
                .map(|args| args.into_iter().map(Into::into).collect()),
            account_references: tx
                .account_references
                .map(|accs| accs.into_iter().map(Into::into).collect()),
            app_references: tx.app_references,
            asset_references: tx.asset_references,
            box_references: tx
                .box_references
                .map(|boxes| boxes.into_iter().map(Into::into).collect()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::ApplicationCallTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::ApplicationCall || tx.application_call.is_none()
        {
            return Err(Self::Error::DecodingError(
                "Application call data missing".to_string(),
            ));
        }

        let data = tx.clone().application_call.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        Ok(Self {
            header,
            app_id: data.app_id,
            on_complete: data.on_complete.into(),
            approval_program: data.approval_program.map(ByteBuf::into_vec),
            clear_state_program: data.clear_state_program.map(ByteBuf::into_vec),
            global_state_schema: data.global_state_schema.map(Into::into),
            local_state_schema: data.local_state_schema.map(Into::into),
            extra_program_pages: data.extra_program_pages,
            args: data
                .args
                .map(|args| args.into_iter().map(ByteBuf::into_vec).collect()),
            account_references: data
                .account_references
                .map(|accs| {
                    accs.into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            app_references: data.app_references,
            asset_references: data.asset_references,
            box_references: data
                .box_references
                .map(|boxes| boxes.into_iter().map(Into::into).collect()),
        })
    }
}

/// Box reference for application call transactions.
///
/// References a specific box that should be made available for the runtime
/// of the program.
#[ffi_record]
pub struct BoxReference {
    /// Application ID that owns the box.
    /// A value of 0 indicates the current application.
    app_id: u64,

    /// Name of the box.
    name: ByteBuf,
}

impl From<algokit_transact::BoxReference> for BoxReference {
    fn from(value: algokit_transact::BoxReference) -> Self {
        Self {
            app_id: value.app_id,
            name: value.name.into(),
        }
    }
}

impl Into<algokit_transact::BoxReference> for BoxReference {
    fn into(self) -> algokit_transact::BoxReference {
        algokit_transact::BoxReference {
            app_id: self.app_id,
            name: self.name.into_vec(),
        }
    }
}

/// On-completion actions for application transactions.
///
/// These values define what additional actions occur with the transaction.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum OnApplicationComplete {
    /// NoOp indicates that an application transaction will simply call its
    /// approval program without any additional action.
    NoOp = 0,

    /// OptIn indicates that an application transaction will allocate some
    /// local state for the application in the sender's account.
    OptIn = 1,

    /// CloseOut indicates that an application transaction will deallocate
    /// some local state for the application from the user's account.
    CloseOut = 2,

    /// ClearState is similar to CloseOut, but may never fail. This
    /// allows users to reclaim their minimum balance from an application
    /// they no longer wish to opt in to.
    ClearState = 3,

    /// UpdateApplication indicates that an application transaction will
    /// update the approval program and clear state program for the application.
    UpdateApplication = 4,

    /// DeleteApplication indicates that an application transaction will
    /// delete the application parameters for the application from the creator's
    /// balance record.
    DeleteApplication = 5,
}

impl From<algokit_transact::OnApplicationComplete> for OnApplicationComplete {
    fn from(value: algokit_transact::OnApplicationComplete) -> Self {
        match value {
            algokit_transact::OnApplicationComplete::NoOp => OnApplicationComplete::NoOp,
            algokit_transact::OnApplicationComplete::OptIn => OnApplicationComplete::OptIn,
            algokit_transact::OnApplicationComplete::CloseOut => OnApplicationComplete::CloseOut,
            algokit_transact::OnApplicationComplete::ClearState => {
                OnApplicationComplete::ClearState
            }
            algokit_transact::OnApplicationComplete::UpdateApplication => {
                OnApplicationComplete::UpdateApplication
            }
            algokit_transact::OnApplicationComplete::DeleteApplication => {
                OnApplicationComplete::DeleteApplication
            }
        }
    }
}

impl Into<algokit_transact::OnApplicationComplete> for OnApplicationComplete {
    fn into(self) -> algokit_transact::OnApplicationComplete {
        match self {
            OnApplicationComplete::NoOp => algokit_transact::OnApplicationComplete::NoOp,
            OnApplicationComplete::OptIn => algokit_transact::OnApplicationComplete::OptIn,
            OnApplicationComplete::CloseOut => algokit_transact::OnApplicationComplete::CloseOut,
            OnApplicationComplete::ClearState => {
                algokit_transact::OnApplicationComplete::ClearState
            }
            OnApplicationComplete::UpdateApplication => {
                algokit_transact::OnApplicationComplete::UpdateApplication
            }
            OnApplicationComplete::DeleteApplication => {
                algokit_transact::OnApplicationComplete::DeleteApplication
            }
        }
    }
}

/// Schema for application state storage.
///
/// Defines the maximum number of values that may be stored in application
/// key/value storage for both global and local state.
#[ffi_record]
pub struct StateSchema {
    /// Maximum number of integer values that may be stored.
    num_uints: u64,

    /// Maximum number of byte slice values that may be stored.
    num_byte_slices: u64,
}

impl From<algokit_transact::StateSchema> for StateSchema {
    fn from(value: algokit_transact::StateSchema) -> Self {
        Self {
            num_uints: value.num_uints,
            num_byte_slices: value.num_byte_slices,
        }
    }
}

impl Into<algokit_transact::StateSchema> for StateSchema {
    fn into(self) -> algokit_transact::StateSchema {
        algokit_transact::StateSchema {
            num_uints: self.num_uints,
            num_byte_slices: self.num_byte_slices,
        }
    }
}
