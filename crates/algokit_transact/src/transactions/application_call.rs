//! Application transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing application transactions,
//! which are used to create, update, delete and call Algorand Smart Contracts (Applications).

use crate::address::Address;
use crate::utils::{is_empty_vec_opt, is_zero, is_zero_opt};
use crate::TransactionHeader;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, skip_serializing_none, Bytes};

/// On-completion actions for application transactions.
///
/// These values define what additional actions occur with the transaction.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
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

impl Default for OnApplicationComplete {
    fn default() -> Self {
        OnApplicationComplete::NoOp
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub global_state_schema: Option<StateSchema>,

    /// Holds the maximum number of local state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    #[serde(rename = "apls")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

    /// List of applications in addition to the application ID that may be called
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
