use crate::*;

/// Represents an app call transaction that interacts with Algorand Smart Contracts.
///
/// App call transactions are used to create, update, delete, opt-in to,
/// close out of, or clear state from Algorand applications (smart contracts).
#[ffi_record]
pub struct AppCallTransactionFields {
    /// ID of the app being called.
    ///
    /// Set this to 0 to indicate an app creation call.
    app_id: u64,

    /// Defines what additional actions occur with the transaction.
    on_complete: OnApplicationComplete,

    /// Logic executed for every app call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for app creation and update transactions.
    approval_program: Option<Vec<u8>>,

    /// Logic executed for app call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for app creation and update transactions.
    clear_state_program: Option<Vec<u8>>,

    /// Holds the maximum number of global state values.
    ///
    /// Only required for app creation transactions.
    /// This cannot be changed after creation.
    global_state_schema: Option<StateSchema>,

    /// Holds the maximum number of local state values.
    ///
    /// Only required for app creation transactions.
    /// This cannot be changed after creation.
    local_state_schema: Option<StateSchema>,

    /// Number of additional pages allocated to the app's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    extra_program_pages: Option<u32>,

    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    args: Option<Vec<Vec<u8>>>,

    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    account_references: Option<Vec<String>>,

    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    app_references: Option<Vec<u64>>,

    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    asset_references: Option<Vec<u64>>,

    /// The boxes that should be made available for the runtime of the program.
    box_references: Option<Vec<BoxReference>>,
}

impl From<algokit_transact::AppCallTransactionFields> for AppCallTransactionFields {
    fn from(tx: algokit_transact::AppCallTransactionFields) -> Self {
        Self {
            app_id: tx.app_id,
            on_complete: tx.on_complete.into(),
            approval_program: tx.approval_program,
            clear_state_program: tx.clear_state_program,
            global_state_schema: tx.global_state_schema.map(Into::into),
            local_state_schema: tx.local_state_schema.map(Into::into),
            extra_program_pages: tx.extra_program_pages,
            args: tx.args.map(|args| args.into_iter().collect()),
            account_references: tx
                .account_references
                .map(|addrs| addrs.into_iter().map(|addr| addr.as_str()).collect()),
            app_references: tx.app_references,
            asset_references: tx.asset_references,
            box_references: tx
                .box_references
                .map(|boxes| boxes.into_iter().map(Into::into).collect()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::AppCallTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::AppCall || tx.app_call.is_none() {
            return Err(Self::Error::DecodingError {
                message: "AppCall call data missing".to_string(),
            });
        }

        let data = tx.clone().app_call.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let transaction_fields = Self {
            header,
            app_id: data.app_id,
            on_complete: data.on_complete.into(),
            approval_program: data.approval_program,
            clear_state_program: data.clear_state_program,
            global_state_schema: data.global_state_schema.map(Into::into),
            local_state_schema: data.local_state_schema.map(Into::into),
            extra_program_pages: data.extra_program_pages,
            args: data.args,
            account_references: data
                .account_references
                .map(|addrs| {
                    addrs
                        .into_iter()
                        .map(|addr| addr.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            app_references: data.app_references,
            asset_references: data.asset_references,
            box_references: data
                .box_references
                .map(|boxes| boxes.into_iter().map(Into::into).collect()),
        };

        transaction_fields
            .validate()
            .map_err(|errors| AlgoKitTransactError::DecodingError {
                message: format!("App call validation failed: {}", errors.join("\n")),
            })?;

        Ok(transaction_fields)
    }
}

/// Box reference for app call transactions.
///
/// References a specific box that should be made available for the runtime
/// of the program.
#[ffi_record]
pub struct BoxReference {
    /// App ID that owns the box.
    /// A value of 0 indicates the current app.
    app_id: u64,

    /// Name of the box.
    name: Vec<u8>,
}

impl From<algokit_transact::BoxReference> for BoxReference {
    fn from(value: algokit_transact::BoxReference) -> Self {
        Self {
            app_id: value.app_id,
            name: value.name,
        }
    }
}

impl From<BoxReference> for algokit_transact::BoxReference {
    fn from(val: BoxReference) -> Self {
        algokit_transact::BoxReference {
            app_id: val.app_id,
            name: val.name,
        }
    }
}

/// On-completion actions for app transactions.
///
/// These values define what additional actions occur with the transaction.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum OnApplicationComplete {
    /// NoOp indicates that an app transaction will simply call its
    /// approval program without any additional action.
    NoOp = 0,

    /// OptIn indicates that an app transaction will allocate some
    /// local state for the app in the sender's account.
    OptIn = 1,

    /// CloseOut indicates that an app transaction will deallocate
    /// some local state for the app from the user's account.
    CloseOut = 2,

    /// ClearState is similar to CloseOut, but may never fail. This
    /// allows users to reclaim their minimum balance from an app
    /// they no longer wish to opt in to.
    ClearState = 3,

    /// UpdateApplication indicates that an app transaction will
    /// update the approval program and clear state program for the app.
    UpdateApplication = 4,

    /// DeleteApplication indicates that an app transaction will
    /// delete the app parameters for the app from the creator's
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

impl From<OnApplicationComplete> for algokit_transact::OnApplicationComplete {
    fn from(val: OnApplicationComplete) -> Self {
        match val {
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

/// Schema for app state storage.
///
/// Defines the maximum number of values that may be stored in app
/// key/value storage for both global and local state.
#[ffi_record]
pub struct StateSchema {
    /// Maximum number of integer values that may be stored.
    num_uints: u32,

    /// Maximum number of byte slice values that may be stored.
    num_byte_slices: u32,
}

impl From<algokit_transact::StateSchema> for StateSchema {
    fn from(value: algokit_transact::StateSchema) -> Self {
        Self {
            num_uints: value.num_uints,
            num_byte_slices: value.num_byte_slices,
        }
    }
}

impl From<StateSchema> for algokit_transact::StateSchema {
    fn from(val: StateSchema) -> Self {
        algokit_transact::StateSchema {
            num_uints: val.num_uints,
            num_byte_slices: val.num_byte_slices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;

    #[test]
    fn test_encode_transaction_validation_integration() {
        // invalid
        let mut tx: Transaction = TestDataMother::app_call().transaction.into();
        tx.app_call.as_mut().unwrap().app_id = 0;
        let result = encode_transaction(tx);
        assert!(result.is_err());

        // valid
        let result = encode_transaction(TestDataMother::app_call().transaction.into());
        assert!(result.is_ok());
    }
}
