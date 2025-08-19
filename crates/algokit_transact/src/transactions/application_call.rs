//! Application transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing application transactions,
//! which are used to create, update, delete and call Algorand Smart Contracts (Applications).

use crate::traits::{MsgPackEmpty, Validate};
use crate::transactions::common::{TransactionHeader, TransactionValidationError};
use crate::utils::{is_empty_struct_opt, is_empty_vec_opt, is_zero, is_zero_opt};
use crate::{
    Address, MAX_ACCOUNT_REFERENCES, MAX_APP_ARGS, MAX_APP_REFERENCES, MAX_ARGS_SIZE,
    MAX_ASSET_REFERENCES, MAX_BOX_REFERENCES, MAX_EXTRA_PROGRAM_PAGES, MAX_GLOBAL_STATE_KEYS,
    MAX_LOCAL_STATE_KEYS, MAX_OVERALL_REFERENCES, PROGRAM_PAGE_SIZE, Transaction,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{Bytes, serde_as, skip_serializing_none};

// Field name constants for validation error messages
const FIELD_APPROVAL_PROGRAM: &str = "Approval program";
const FIELD_CLEAR_STATE_PROGRAM: &str = "Clear state program";
const FIELD_GLOBAL_STATE_SCHEMA: &str = "Global state schema";
const FIELD_LOCAL_STATE_SCHEMA: &str = "Local state schema";
const FIELD_EXTRA_PROGRAM_PAGES: &str = "Extra program pages";
const FIELD_APP_ID: &str = "App id";
const FIELD_ARGS: &str = "Args";

/// On-completion actions for application transactions.
///
/// These values define what additional actions occur with the transaction.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
#[derive(Default)]
pub enum OnApplicationComplete {
    /// NoOp indicates that an application transaction will simply call its
    /// approval program without any additional action.
    #[default]
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

/// Schema for application state storage.
///
/// Defines the maximum number of values that may be stored in application
/// key/value storage for both global and local state.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StateSchema {
    /// Maximum number of integer values that may be stored.
    #[serde(rename = "nui")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub num_uints: u64,

    /// Maximum number of byte slice values that may be stored.
    #[serde(rename = "nbs")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub num_byte_slices: u64,
}

impl MsgPackEmpty for StateSchema {
    fn is_empty(&self) -> bool {
        self.num_uints == 0 && self.num_byte_slices == 0
    }
}

/// Box reference for application call transactions.
///
/// References a specific box that should be made available for the runtime
/// of the program.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BoxReference {
    /// Application ID that owns the box.
    /// A value of 0 indicates the current application.
    #[serde(rename = "i")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub app_id: u64,

    /// Name of the box.
    #[serde(rename = "n")]
    #[serde_as(as = "Bytes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub name: Vec<u8>,
}

/// Represents an application call transaction that interacts with Algorand Smart Contracts.
///
/// Application call transactions are used to create, update, delete, opt-in to,
/// close out of, or clear state from Algorand applications (smart contracts).
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = ApplicationCallTransactionBuilder,
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct ApplicationCallTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// ID of the application being called.
    ///
    /// Set this to 0 to indicate an application creation call.
    #[serde(rename = "apid")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub app_id: u64,

    /// Defines what additional actions occur with the transaction.
    #[serde(rename = "apan")]
    #[serde(skip_serializing_if = "is_default_on_complete")]
    #[serde(default)]
    pub on_complete: OnApplicationComplete,

    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for application creation and update transactions.
    #[serde(rename = "apap")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub approval_program: Option<Vec<u8>>,

    /// Logic executed for application call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for application creation and update transactions.
    #[serde(rename = "apsu")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub clear_state_program: Option<Vec<u8>>,

    /// Holds the maximum number of global state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    #[serde(rename = "apgs")]
    #[serde(skip_serializing_if = "is_empty_struct_opt")]
    #[serde(default)]
    #[builder(default)]
    pub global_state_schema: Option<StateSchema>,

    /// Holds the maximum number of local state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    #[serde(rename = "apls")]
    #[serde(skip_serializing_if = "is_empty_struct_opt")]
    #[serde(default)]
    #[builder(default)]
    pub local_state_schema: Option<StateSchema>,

    /// Number of additional pages allocated to the application's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    #[serde(rename = "apep")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub extra_program_pages: Option<u64>,

    /// Transaction specific arguments available in the application's
    /// approval program and clear state program.
    #[serde(rename = "apaa")]
    #[serde_as(as = "Option<Vec<Bytes>>")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub args: Option<Vec<Vec<u8>>>,

    /// List of accounts in addition to the sender that may be accessed
    /// from the application's approval program and clear state program.
    #[serde(rename = "apat")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub account_references: Option<Vec<Address>>,

    /// List of applications in addition to the current application that may be called
    /// from the application's approval program and clear state program.
    #[serde(rename = "apfa")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub app_references: Option<Vec<u64>>,

    /// Lists the assets whose parameters may be accessed by this application's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    #[serde(rename = "apas")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub asset_references: Option<Vec<u64>>,

    /// The boxes that should be made available for the runtime of the program.
    #[serde(rename = "apbx")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[builder(default)]
    pub box_references: Option<Vec<BoxReference>>,
}

fn is_default_on_complete(on_complete: &OnApplicationComplete) -> bool {
    matches!(on_complete, OnApplicationComplete::NoOp)
}

/// Custom serializer for application call transactions.
///
/// This serializer handles the special case of box references, where app IDs need to be
/// transformed from actual application IDs to positional indices for wire format compatibility.
pub fn application_call_serializer<S>(
    fields: &ApplicationCallTransactionFields,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Transform box references if present
    if let Some(ref box_references) = fields.box_references {
        if !box_references.is_empty() {
            let app_references = fields.app_references.as_deref().unwrap_or(&[]);

            let box_references = box_references
                .iter()
                .map(|box_ref| {
                    let app_id_index = if box_ref.app_id == 0 || box_ref.app_id == fields.app_id {
                        // A 0 value denotes the current app_id,
                        // return 0 when the app_id is for the current application.
                        0
                    } else {
                        // Find position in app_references and add 1 (1-based indexing)
                        app_references
                            .iter()
                            .position(|&id| id == box_ref.app_id)
                            .map(|pos| (pos + 1) as u64) // App references start from index 1; index 0 is the current application ID.
                            .ok_or_else(|| {
                                format!(
                                    "Box reference with app id {} not found in app references.",
                                    box_ref.app_id
                                )
                            })?
                    };

                    Ok(BoxReference {
                        app_id: app_id_index,
                        name: box_ref.name.clone(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()
                .map_err(serde::ser::Error::custom)?;

            let mut fields: ApplicationCallTransactionFields = fields.clone();
            fields.box_references = Some(box_references);

            return fields.serialize(serializer);
        }
    }

    // No transformation needed, serialize directly
    fields.serialize(serializer)
}

/// Custom deserializer for application call transactions.
///
/// This deserializer handles the special case of box references, where app IDs need to be
/// transformed from positional indices back to actual application IDs.
pub fn application_call_deserializer<'de, D>(
    deserializer: D,
) -> Result<ApplicationCallTransactionFields, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut fields = ApplicationCallTransactionFields::deserialize(deserializer)?;

    // Transform box references if present
    if let Some(ref box_references) = fields.box_references {
        if !box_references.is_empty() {
            let app_references = fields.app_references.as_deref().unwrap_or(&[]);

            let box_references = box_references
                .iter()
                .map(|box_ref| {
                    let app_id = if box_ref.app_id == 0 {
                        // The current app_id is always serialized as 0,
                        // return 0 when the app_id is for the current application.
                        0
                    } else {
                        // Convert 1-based index back to the actual app ID
                        let app_reference_index = box_ref.app_id as usize - 1;
                        app_references
                            .get(app_reference_index)
                            .copied()
                            .ok_or_else(|| {
                                format!("Cannot find app reference index {}.", app_reference_index,)
                            })?
                    };

                    Ok(BoxReference {
                        app_id,
                        name: box_ref.name.clone(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()
                .map_err(serde::de::Error::custom)?;

            fields.box_references = Some(box_references);
        }
    }

    Ok(fields)
}

impl ApplicationCallTransactionFields {
    /// Validates that the app ID is not zero.
    fn validate_app_id_not_zero(&self, errors: &mut Vec<TransactionValidationError>) {
        if self.app_id == 0 {
            errors.push(TransactionValidationError::ZeroValueField(
                FIELD_APP_ID.to_string(),
            ));
        }
    }

    /// Validates that immutable fields are not set (used for update, call, and delete operations).
    fn validate_immutable_fields_not_set(&self, errors: &mut Vec<TransactionValidationError>) {
        if self.global_state_schema.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                FIELD_GLOBAL_STATE_SCHEMA.to_string(),
            ));
        }

        if self.local_state_schema.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                FIELD_LOCAL_STATE_SCHEMA.to_string(),
            ));
        }

        if self.extra_program_pages.is_some() {
            errors.push(TransactionValidationError::ImmutableField(
                FIELD_EXTRA_PROGRAM_PAGES.to_string(),
            ));
        }
    }

    /// Validates that both approval and clear state programs are provided and not empty.
    fn validate_programs_required(&self, errors: &mut Vec<TransactionValidationError>) {
        // Approval program is required
        if self.approval_program.is_none() || self.approval_program.as_ref().unwrap().is_empty() {
            errors.push(TransactionValidationError::RequiredField(
                FIELD_APPROVAL_PROGRAM.to_string(),
            ));
        }

        // Clear state program is required
        if self.clear_state_program.is_none()
            || self.clear_state_program.as_ref().unwrap().is_empty()
        {
            errors.push(TransactionValidationError::RequiredField(
                FIELD_CLEAR_STATE_PROGRAM.to_string(),
            ));
        }
    }

    /// Validates fields specific to application creation.
    pub fn validate_for_create(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        self.validate_programs_required(&mut errors);

        // Validate extra program pages
        if let Some(extra_pages) = self.extra_program_pages {
            if extra_pages > MAX_EXTRA_PROGRAM_PAGES {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: FIELD_EXTRA_PROGRAM_PAGES.to_string(),
                    actual: extra_pages as usize,
                    max: MAX_EXTRA_PROGRAM_PAGES as usize,
                    unit: "pages".to_string(),
                });
            }
        }

        let max_program_size = self.calculate_max_program_size();

        // Validate approval program size
        if let Some(ref approval_program) = self.approval_program {
            if approval_program.len() > max_program_size {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: FIELD_APPROVAL_PROGRAM.to_string(),
                    actual: approval_program.len(),
                    max: max_program_size,
                    unit: "bytes".to_string(),
                });
            }
        }

        // Validate clear state program size
        if let Some(ref clear_state_program) = self.clear_state_program {
            if clear_state_program.len() > max_program_size {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: FIELD_CLEAR_STATE_PROGRAM.to_string(),
                    actual: clear_state_program.len(),
                    max: max_program_size,
                    unit: "bytes".to_string(),
                });
            }
        }

        // Validate combined program size
        if let (Some(approval_program), Some(clear_state_program)) =
            (&self.approval_program, &self.clear_state_program)
        {
            let total_size = approval_program.len() + clear_state_program.len();
            if total_size > max_program_size {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Combined approval and clear state programs".to_string(),
                    actual: total_size,
                    max: max_program_size,
                    unit: "bytes".to_string(),
                });
            }
        }

        // Validate state schemas
        if let Some(ref global_schema) = self.global_state_schema {
            if global_schema.num_uints + global_schema.num_byte_slices > MAX_GLOBAL_STATE_KEYS {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: FIELD_GLOBAL_STATE_SCHEMA.to_string(),
                    actual: (global_schema.num_uints + global_schema.num_byte_slices) as usize,
                    max: MAX_GLOBAL_STATE_KEYS as usize,
                    unit: "keys".to_string(),
                });
            }
        }

        if let Some(ref local_schema) = self.local_state_schema {
            if local_schema.num_uints + local_schema.num_byte_slices > MAX_LOCAL_STATE_KEYS {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: FIELD_LOCAL_STATE_SCHEMA.to_string(),
                    actual: (local_schema.num_uints + local_schema.num_byte_slices) as usize,
                    max: MAX_LOCAL_STATE_KEYS as usize,
                    unit: "keys".to_string(),
                });
            }
        }

        self.validate_common_fields(&mut errors);

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }

    /// Validates fields specific to application update.
    pub fn validate_for_update(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        self.validate_app_id_not_zero(&mut errors);
        self.validate_programs_required(&mut errors);
        // We can't validate the extra program pages on update, as we don't know what the initial create value was.
        self.validate_immutable_fields_not_set(&mut errors);
        self.validate_common_fields(&mut errors);

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }

    /// Validates fields for application call (no-op), opt-in, close-out, clear-state operations.
    pub fn validate_for_call(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        self.validate_app_id_not_zero(&mut errors);
        self.validate_immutable_fields_not_set(&mut errors);

        self.validate_common_fields(&mut errors);

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }

    /// Validates fields for application deletion.
    pub fn validate_for_delete(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        self.validate_app_id_not_zero(&mut errors);
        self.validate_immutable_fields_not_set(&mut errors);

        self.validate_common_fields(&mut errors);

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }

    /// Validates common fields that apply to all application call types.
    fn validate_common_fields(&self, errors: &mut Vec<TransactionValidationError>) {
        if let Some(ref args) = self.args {
            // Validate number of args
            if args.len() > MAX_APP_ARGS {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: FIELD_ARGS.to_string(),
                    actual: args.len(),
                    max: MAX_APP_ARGS,
                    unit: "arguments".to_string(),
                });
            }

            // Validate total size of args
            let total_args_size: usize = args.iter().map(|arg| arg.len()).sum();
            if total_args_size > MAX_ARGS_SIZE {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Args total size".to_string(),
                    actual: total_args_size,
                    max: MAX_ARGS_SIZE,
                    unit: "bytes".to_string(),
                });
            }
        }

        // Validate account references
        if let Some(ref account_refs) = self.account_references {
            if account_refs.len() > MAX_ACCOUNT_REFERENCES {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Account references".to_string(),
                    actual: account_refs.len(),
                    max: MAX_ACCOUNT_REFERENCES,
                    unit: "refs".to_string(),
                });
            }
        }

        // Validate application references
        if let Some(ref app_refs) = self.app_references {
            if app_refs.len() > MAX_APP_REFERENCES {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Application references".to_string(),
                    actual: app_refs.len(),
                    max: MAX_APP_REFERENCES,
                    unit: "refs".to_string(),
                });
            }
        }

        // Validate asset references
        if let Some(ref asset_refs) = self.asset_references {
            if asset_refs.len() > MAX_ASSET_REFERENCES {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Asset references".to_string(),
                    actual: asset_refs.len(),
                    max: MAX_ASSET_REFERENCES,
                    unit: "refs".to_string(),
                });
            }
        }

        // Validate box references
        if let Some(ref box_refs) = self.box_references {
            if box_refs.len() > MAX_BOX_REFERENCES {
                errors.push(TransactionValidationError::FieldTooLong {
                    field: "Box references".to_string(),
                    actual: box_refs.len(),
                    max: MAX_BOX_REFERENCES,
                    unit: "refs".to_string(),
                });
            }

            // Validate box reference app IDs are in app_references
            let app_refs = self.app_references.as_deref().unwrap_or(&[]);
            for box_ref in box_refs {
                if box_ref.app_id != 0
                    && box_ref.app_id != self.app_id
                    && !app_refs.contains(&box_ref.app_id)
                {
                    errors.push(TransactionValidationError::ArbitraryConstraint(format!(
                        "Box reference for app ID {} must be in app references",
                        box_ref.app_id
                    )));
                }
            }
        }

        // Validate overall reference count
        let total_references = self.account_references.as_ref().map_or(0, |v| v.len())
            + self.app_references.as_ref().map_or(0, |v| v.len())
            + self.asset_references.as_ref().map_or(0, |v| v.len())
            + self.box_references.as_ref().map_or(0, |v| v.len());

        if total_references > MAX_OVERALL_REFERENCES {
            errors.push(TransactionValidationError::FieldTooLong {
                field: "Total references".to_string(),
                actual: total_references,
                max: MAX_OVERALL_REFERENCES,
                unit: "refs".to_string(),
            });
        }
    }

    /// Calculates the maximum allowed program size based on extra program pages.
    fn calculate_max_program_size(&self) -> usize {
        let extra_pages = self.extra_program_pages.unwrap_or(0) as usize;
        PROGRAM_PAGE_SIZE + (extra_pages * PROGRAM_PAGE_SIZE)
    }
}

impl ApplicationCallTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, ApplicationCallTransactionBuilderError> {
        let fields = self.build_fields()?;
        fields.validate().map_err(|errors| {
            ApplicationCallTransactionBuilderError::ValidationError(format!(
                "Application call validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::ApplicationCall(fields))
    }
}

impl Validate for ApplicationCallTransactionFields {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = match (self.app_id, &self.on_complete) {
            // Application creation (app_id = 0)
            (0, _) => self.validate_for_create(),

            // Application update
            (_, OnApplicationComplete::UpdateApplication) => self.validate_for_update(),

            // Application deletion
            (_, OnApplicationComplete::DeleteApplication) => self.validate_for_delete(),

            // Regular application calls (NoOp, OptIn, CloseOut, ClearState)
            (_, _) => self.validate_for_call(),
        };

        result.map_err(|errors| errors.iter().map(|e| e.to_string()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AlgorandMsgpack;
    use crate::test_utils::{
        AccountMother, ApplicationCallTransactionMother, TestDataMother, TransactionHeaderMother,
    };
    use crate::test_utils::{check_transaction_encoding, check_transaction_id};

    #[test]
    fn test_application_create_transaction_encoding() {
        let application_create_tx = ApplicationCallTransactionMother::application_create()
            .build()
            .unwrap();

        check_transaction_id(
            &application_create_tx,
            "L6B56N2BAXE43PUI7IDBXCJN5DEB6NLCH4AAN3ON64CXPSCTJNTA",
        );
        check_transaction_encoding(&application_create_tx, 1386);
    }

    #[test]
    fn test_application_call_encoding() {
        let application_call_tx = ApplicationCallTransactionMother::application_call()
            .build()
            .unwrap();

        check_transaction_id(
            &application_call_tx,
            "6Y644M5SGTKNBH7ZX6D7QAAHDF6YL6FDJPRAGSUHNZLR4IKGVSPQ",
        );
        check_transaction_encoding(&application_call_tx, 377);
    }

    #[test]
    fn test_application_update_encoding() {
        let application_update_tx = ApplicationCallTransactionMother::application_update()
            .build()
            .unwrap();

        check_transaction_id(
            &application_update_tx,
            "NQVNJ5VWEDX42DMJQIQET4QPNUOW27EYIPKZ4SDWKOOEFJQB7PZA",
        );
        check_transaction_encoding(&application_update_tx, 7069);
    }

    #[test]
    fn test_application_delete_transaction_encoding() {
        let application_delete_tx = ApplicationCallTransactionMother::application_delete()
            .build()
            .unwrap();

        check_transaction_id(
            &application_delete_tx,
            "XVVC7UDLCPI622KCJZLWK3SEAWWVUEPEXUM5CO3DFLWOBH7NOPDQ",
        );
        check_transaction_encoding(&application_delete_tx, 263);
    }

    #[test]
    fn test_application_opt_in_transaction_encoding() {
        let application_opt_in_tx = ApplicationCallTransactionMother::application_opt_in()
            .build()
            .unwrap();

        check_transaction_id(
            &application_opt_in_tx,
            "BNASGY47TXXUTFUZPDAGGPQKK54B4QPEEPDTJIZFDXC64WQH4GOQ",
        );
        check_transaction_encoding(&application_opt_in_tx, 247);
    }

    #[test]
    fn test_application_close_out_transaction_encoding() {
        let application_close_out_tx = ApplicationCallTransactionMother::application_close_out()
            .build()
            .unwrap();

        check_transaction_id(
            &application_close_out_tx,
            "R4LXOUN4KPRIILRLIYKMA2DJ4HKCXWCD5TYWGH76635KUHGFNTUQ",
        );
        check_transaction_encoding(&application_close_out_tx, 131);
    }

    #[test]
    fn test_application_clear_state_transaction_encoding() {
        let application_clear_state_tx =
            ApplicationCallTransactionMother::application_clear_state()
                .build()
                .unwrap();

        check_transaction_id(
            &application_clear_state_tx,
            "XQE2YKONC62QXSXDIRJ7CL6YDWP45JXCQO6N7DAAFQH7DJM6BEKA",
        );
        check_transaction_encoding(&application_clear_state_tx, 131);
    }

    #[test]
    fn test_0_box_ref_application_call_transaction_encoding() {
        let application_call_tx = ApplicationCallTransactionMother::application_call_example()
            .box_references(vec![BoxReference {
                app_id: 0,
                name: "b1".as_bytes().to_vec(),
            }])
            .build()
            .unwrap();

        check_transaction_id(
            &application_call_tx,
            "LXUGSM4264PQ2YSSO3JW535NHGC5JESKLQS6ITONGO2S6ATEWM2A",
        );
        check_transaction_encoding(&application_call_tx, 138);
    }

    #[test]
    fn test_app_id_box_ref_application_call_transaction_encoding() {
        let application_call_tx = ApplicationCallTransactionMother::application_call_example()
            .box_references(vec![BoxReference {
                app_id: 12345,
                name: "b1".as_bytes().to_vec(),
            }])
            .build()
            .unwrap();

        check_transaction_id(
            &application_call_tx,
            "LXUGSM4264PQ2YSSO3JW535NHGC5JESKLQS6ITONGO2S6ATEWM2A",
        );

        let encoded = application_call_tx.encode().unwrap();
        let decoded = Transaction::decode(&encoded).unwrap();

        if let Transaction::ApplicationCall(decoded_app_call) = decoded {
            assert_eq!(
                decoded_app_call.box_references.as_ref().unwrap()[0].app_id,
                0
            );
        } else {
            panic!("Expected ApplicationCall transaction type");
        }
    }

    #[test]
    fn test_external_box_refs_application_call_transaction_encoding() {
        let application_call_tx = ApplicationCallTransactionMother::application_call_example()
            .app_references(vec![54321, 11111, 55555, 22222])
            .box_references(vec![
                BoxReference {
                    app_id: 55555,
                    name: "b1".as_bytes().to_vec(),
                },
                BoxReference {
                    app_id: 54321,
                    name: "b2".as_bytes().to_vec(),
                },
            ])
            .build()
            .unwrap();

        check_transaction_id(
            &application_call_tx,
            "GB4AYDJEHVBLOVSLCBOXG3KASTS3V6QV6GPB6F2BILG7L6J3P4OQ",
        );
        check_transaction_encoding(&application_call_tx, 169);
    }

    #[test]
    fn test_box_ref_missing_app_reference_encode() {
        let application_call_tx_fields =
            ApplicationCallTransactionMother::application_call_example()
                .app_references(vec![54321])
                .box_references(vec![
                    BoxReference {
                        app_id: 55555,
                        name: "b1".as_bytes().to_vec(),
                    },
                    BoxReference {
                        app_id: 54321,
                        name: "b2".as_bytes().to_vec(),
                    },
                ])
                .build_fields() // Skips the builder validation
                .unwrap();

        let application_call_tx = Transaction::ApplicationCall(application_call_tx_fields);

        let result = application_call_tx.encode();

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("Box reference with app id 55555 not found in app references"),
            "Expected missing app reference error, got: {}",
            error_message
        );
    }

    #[test]
    fn test_box_ref_missing_app_reference_decode() {
        let encoded_tx_missing_app_ref = [
            84, 88, 138, 164, 97, 112, 98, 120, 146, 130, 161, 105, 1, 161, 110, 196, 2, 98, 49,
            130, 161, 105, 2, 161, 110, 196, 2, 98, 50, 164, 97, 112, 102, 97, 145, 205, 212, 49,
            164, 97, 112, 105, 100, 205, 48, 57, 163, 102, 101, 101, 205, 3, 232, 162, 102, 118, 1,
            163, 103, 101, 110, 167, 101, 120, 97, 109, 112, 108, 101, 162, 103, 104, 196, 32, 222,
            189, 190, 157, 28, 11, 247, 214, 147, 68, 228, 226, 58, 211, 196, 121, 68, 26, 174,
            253, 159, 1, 57, 38, 54, 88, 135, 169, 241, 177, 52, 144, 162, 108, 118, 205, 3, 231,
            163, 115, 110, 100, 196, 32, 2, 204, 225, 113, 58, 8, 179, 189, 204, 74, 148, 128, 202,
            244, 192, 188, 2, 202, 236, 227, 17, 198, 25, 62, 33, 204, 91, 40, 252, 44, 209, 74,
            164, 116, 121, 112, 101, 164, 97, 112, 112, 108,
        ];

        let result = Transaction::decode(&encoded_tx_missing_app_ref);

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("Cannot find app reference index 1"),
            "Expected missing app reference error, got: {}",
            error_message
        );
    }

    #[test]
    fn test_application_call_empty_value_encoding() {
        let builder = &ApplicationCallTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .app_id(1234)
            .on_complete(OnApplicationComplete::NoOp)
            .to_owned();

        let tx = builder.clone().build().unwrap();
        let tx_with_empties = builder
            .clone()
            .approval_program(vec![])
            .clear_state_program(vec![])
            .args(vec![])
            .account_references(vec![])
            .asset_references(vec![])
            .account_references(vec![])
            .box_references(vec![])
            .build()
            .unwrap();

        let expected_id = "AEAVEJUTYW5MFUWTDX6YPQS57FILUMVGDNYVB6ZGNNWL5Z4D43OA";

        assert_ne!(tx, tx_with_empties);

        // Because id's are a hash of the encoded bytes, we can be sure the encoded bytes are the same
        check_transaction_id(&tx, expected_id);
        check_transaction_id(&tx_with_empties, expected_id);
    }

    #[test]
    fn test_validate_application_create_success() {
        let app_call = ApplicationCallTransactionMother::application_create()
            .build_fields()
            .unwrap();

        assert!(app_call.validate().is_ok());
    }

    #[test]
    fn test_validate_application_create_invalid() {
        let app_call = ApplicationCallTransactionMother::application_create()
            .approval_program(vec![]) // Missing approval program
            .clear_state_program(vec![]) // Missing clear state program
            .extra_program_pages(MAX_EXTRA_PROGRAM_PAGES + 1) // Too many extra pages
            .global_state_schema(StateSchema {
                num_uints: MAX_GLOBAL_STATE_KEYS, // Too many global state keys
                num_byte_slices: 1,
            })
            .local_state_schema(StateSchema {
                num_uints: MAX_LOCAL_STATE_KEYS, // Too many local state keys
                num_byte_slices: 1,
            })
            .build_fields()
            .unwrap();

        let result = app_call.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_APPROVAL_PROGRAM) && e.contains("required"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_CLEAR_STATE_PROGRAM) && e.contains("required"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_EXTRA_PROGRAM_PAGES) && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_GLOBAL_STATE_SCHEMA) && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_LOCAL_STATE_SCHEMA) && e.contains("exceed"))
        );
        assert!(
            errors.len() == 5,
            "Expected 5 validation errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_validate_application_create_programs_too_large() {
        let large_approval_program = vec![0u8; PROGRAM_PAGE_SIZE + 1];
        let large_clear_program = vec![1u8; PROGRAM_PAGE_SIZE + 1];

        let app_call_large_programs = ApplicationCallTransactionMother::application_create()
            .extra_program_pages(0)
            .approval_program(large_approval_program)
            .clear_state_program(large_clear_program)
            .build_fields()
            .unwrap();

        let result = app_call_large_programs.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_APPROVAL_PROGRAM) && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_CLEAR_STATE_PROGRAM) && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Combined approval and clear state programs")
                    && e.contains("exceed"))
        );
    }

    #[test]
    fn test_validate_application_update_success() {
        let app_call = ApplicationCallTransactionMother::application_update()
            .build_fields()
            .unwrap();

        assert!(app_call.validate().is_ok());
    }

    #[test]
    fn test_validate_application_update_invalid() {
        let app_call = ApplicationCallTransactionMother::application_update()
            .app_id(0) // Invalid app ID (must not be zero for update)
            .approval_program(vec![]) // Missing approval program
            .clear_state_program(vec![]) // Missing clear state program
            .global_state_schema(StateSchema {
                num_uints: 2,
                num_byte_slices: 2,
            }) // Immutable field - not allowed on update
            .local_state_schema(StateSchema {
                num_uints: 2,
                num_byte_slices: 3,
            }) // Immutable field - not allowed on update
            .extra_program_pages(1) // Immutable field - not allowed on update
            .build_fields()
            .unwrap();

        let result = app_call.validate_for_update(); // Needs to be explicitly called because the app_id is 0
        assert!(result.is_err());
        let errors: Vec<String> = result.unwrap_err().iter().map(|e| e.to_string()).collect();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_APP_ID) && e.contains("0"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_APPROVAL_PROGRAM) && e.contains("required"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_CLEAR_STATE_PROGRAM) && e.contains("required"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_GLOBAL_STATE_SCHEMA) && e.contains("immutable"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_LOCAL_STATE_SCHEMA) && e.contains("immutable"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_EXTRA_PROGRAM_PAGES) && e.contains("immutable"))
        );
        assert!(
            errors.len() == 6,
            "Expected 6 validation errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_validate_application_delete_success() {
        let app_call = ApplicationCallTransactionMother::application_delete()
            .build_fields()
            .unwrap();

        assert!(app_call.validate().is_ok());
    }

    #[test]
    fn test_validate_application_delete_invalid() {
        let app_call = ApplicationCallTransactionMother::application_delete()
            .app_id(0) // Invalid app ID (must not be zero for delete)
            .global_state_schema(StateSchema {
                num_uints: 2,
                num_byte_slices: 2,
            }) // Immutable field - not allowed on delete
            .local_state_schema(StateSchema {
                num_uints: 2,
                num_byte_slices: 3,
            }) // Immutable field - not allowed on delete
            .extra_program_pages(1) // Immutable field - not allowed on delete
            .build_fields()
            .unwrap();

        let result = app_call.validate_for_delete(); // Needs to be explicitly called because the app_id is 0
        assert!(result.is_err());
        let errors: Vec<String> = result.unwrap_err().iter().map(|e| e.to_string()).collect();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_APP_ID) && e.contains("0"))
        );

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_GLOBAL_STATE_SCHEMA) && e.contains("immutable"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_LOCAL_STATE_SCHEMA) && e.contains("immutable"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_EXTRA_PROGRAM_PAGES) && e.contains("immutable"))
        );

        assert!(
            errors.len() == 4,
            "Expected 4 validation errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_validate_application_call_success() {
        let app_call = ApplicationCallTransactionMother::application_call_example()
            .build_fields()
            .unwrap();

        assert!(app_call.validate().is_ok());
    }

    #[test]
    fn test_validate_application_call_invalid() {
        let app_call = ApplicationCallTransactionMother::application_call_example()
            .app_id(0) // Invalid app ID (must not be zero for delete)
            .global_state_schema(StateSchema {
                num_uints: 2,
                num_byte_slices: 2,
            }) // Immutable field - not allowed on delete
            .local_state_schema(StateSchema {
                num_uints: 2,
                num_byte_slices: 3,
            }) // Immutable field - not allowed on delete
            .extra_program_pages(1) // Immutable field - not allowed on delete
            .build_fields()
            .unwrap();

        let result = app_call.validate_for_call(); // Needs to be explicitly called because the app_id is 0
        assert!(result.is_err());
        let errors: Vec<String> = result.unwrap_err().iter().map(|e| e.to_string()).collect();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_APP_ID) && e.contains("0"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_GLOBAL_STATE_SCHEMA) && e.contains("immutable"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_LOCAL_STATE_SCHEMA) && e.contains("immutable"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_EXTRA_PROGRAM_PAGES) && e.contains("immutable"))
        );

        assert!(
            errors.len() == 4,
            "Expected 4 validation errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_validate_args() {
        // The args are both too many and too large
        let args = (0..=MAX_APP_ARGS).map(|_i| vec![0u8; 700]).collect();
        let app_call = ApplicationCallTransactionMother::application_call_example()
            .args(args)
            .build_fields()
            .unwrap();

        let result = app_call.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains(FIELD_ARGS) && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Args total size") && e.contains("exceed"))
        );

        assert!(
            errors.len() == 2,
            "Expected 2 validation errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_validate_references() {
        // Create vectors that exceed the maximum limits for all reference types
        let excessive_account_refs =
            vec![AccountMother::account().address(); MAX_ACCOUNT_REFERENCES + 1];
        let excessive_app_refs = vec![1; MAX_APP_REFERENCES + 1];
        let excessive_asset_refs = vec![2; MAX_ASSET_REFERENCES + 1];
        let excessive_box_refs = vec![
            BoxReference {
                app_id: 0,
                name: vec![1],
            };
            MAX_BOX_REFERENCES
        ];

        let app_call = ApplicationCallTransactionMother::application_call_example()
            .account_references(excessive_account_refs)
            .app_references(excessive_app_refs)
            .asset_references(excessive_asset_refs)
            .box_references({
                let mut box_refs = excessive_box_refs;
                // Add a box reference with invalid app_id (not in app_references)
                box_refs.push(BoxReference {
                    app_id: 88888,
                    name: vec![1, 2, 3],
                });
                box_refs
            })
            .build_fields()
            .unwrap();

        let result = app_call.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();

        println!("Validation errors ({}): {:?}", errors.len(), errors);

        assert!(
            errors
                .iter()
                .any(|e| e.contains("Account references") && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Application references") && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Asset references") && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Box references") && e.contains("exceed"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Box reference for app ID 88888 must be in app references"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Total references") && e.contains("exceed"))
        );

        assert!(
            errors.len() == 6,
            "Expected 6 validation errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_builder_validation_integration() {
        // invalid
        let result = ApplicationCallTransactionMother::application_call_example()
            .app_id(0)
            .build();
        assert!(result.is_err());

        // valid
        let result = ApplicationCallTransactionMother::application_call_example().build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_application_create_snapshot() {
        let data = TestDataMother::application_create();
        assert_eq!(
            data.id,
            String::from("L6B56N2BAXE43PUI7IDBXCJN5DEB6NLCH4AAN3ON64CXPSCTJNTA")
        );
    }

    #[test]
    fn test_application_call_snapshot() {
        let data = TestDataMother::application_call();
        assert_eq!(
            data.id,
            String::from("6Y644M5SGTKNBH7ZX6D7QAAHDF6YL6FDJPRAGSUHNZLR4IKGVSPQ")
        );
    }

    #[test]
    fn test_application_update_snapshot() {
        let data = TestDataMother::application_update();
        assert_eq!(
            data.id,
            String::from("NQVNJ5VWEDX42DMJQIQET4QPNUOW27EYIPKZ4SDWKOOEFJQB7PZA")
        );
    }

    #[test]
    fn test_application_delete_snapshot() {
        let data = TestDataMother::application_delete();
        assert_eq!(
            data.id,
            String::from("XVVC7UDLCPI622KCJZLWK3SEAWWVUEPEXUM5CO3DFLWOBH7NOPDQ")
        );
    }
}
