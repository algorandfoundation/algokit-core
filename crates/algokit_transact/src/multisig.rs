//! Algorand multisignature account representation and manipulation.
//!
//! This module provides the [`MultisigSignature`] type, which encapsulates an Algorand multisignature
//! account's version, threshold, and participating addresses. The corresponding [`Address`] is derived
//! from the domain separator, version, threshold, and the concatenated addresses, hashed to produce
//! the 32-byte digest used as the address.
//!
//! Contrary to the single signature account, it's not possible to derive a multisignature account
//! from its address, as the "public information" of a multisig account is derived with
//! a cryptographic hash function.

use crate::address::Address;
use crate::utils::hash;
use crate::{
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, MULTISIG_DOMAIN_SEPARATOR,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};

// TODO: The reference implementation uses `Multisig` kind of both as a signature of a transaction, and
//  as an account type. In the reference implementation, everytime we need to sign a new msig transaction,
//  we need a "new" `Multisig` object. We need to evaluate if it's good and convenient to introduce
//  a separate `MultisigAccount` type, like we have for `Account`.
//  This new type would not contain the `sig` field, and it would just be a version, threshold, and list of participants.
//  This new type is not present so we need to be careful introducing new concepts that may not
//  be compatible with the reference implementation.

/// Represents an Algorand multisignature account.
///
/// A multisignature account is defined by a version, a threshold, and a list of participating addresses.
/// The version indicates the multisig protocol version, while the threshold specifies the minimum
/// number of signatures required to authorize a transaction.
/// While technically this accepts [`Address`] types, it is expected that these will be
/// the addresses of [`Account`]s, which are 32-byte Ed25519 public keys.
// TODO: This name deviates from the reference implementation, which uses `Multisig`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSignature {
    /// Multisig version.
    #[serde(rename = "v")]
    pub version: u8,
    /// Minimum number of signatures required.
    #[serde(rename = "thr")]
    pub threshold: u8,
    /// Sub-signatures
    #[serde(rename = "subsig")]
    pub subsignatures: Vec<MultisigSubsignature>,
}

impl MultisigSignature {
    /// Creates a new multisignature account with the specified version, threshold, and participants.
    pub fn new(version: u8, threshold: u8, participants: Vec<Address>) -> Self {
        let subsigs = participants
            .into_iter()
            .map(|address| MultisigSubsignature {
                address,
                signature: None,
            })
            .collect();
        Self {
            version,
            threshold,
            subsignatures: subsigs,
        }
    }

    pub fn participants(&self) -> Vec<Address> {
        self.subsignatures
            .iter()
            .map(|subsig| subsig.address.clone())
            .collect()
    }
}

/// Represents a single subsignature in a multisignature transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSubsignature {
    /// Address of a single signature account that is sub-signing a multisignature transaction.
    #[serde(rename = "pk")]
    pub address: Address,
    /// The signature bytes.
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Bytes>")]
    pub signature: Option<[u8; ALGORAND_SIGNATURE_BYTE_LENGTH]>,
}

impl From<MultisigSignature> for Address {
    /// Converts a [`MultisigSignature`] into an [`Address`] by hashing the domain separator,
    /// version, threshold, and all participating addresses.
    fn from(msig: MultisigSignature) -> Address {
        let mut buffer = Vec::with_capacity(
            MULTISIG_DOMAIN_SEPARATOR.len()
                + 2
                + msig.subsignatures.len() * ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
        );
        buffer.extend_from_slice(MULTISIG_DOMAIN_SEPARATOR.as_bytes());
        buffer.push(msig.version);
        buffer.push(msig.threshold);
        msig.participants()
            .iter()
            .for_each(|addr| buffer.extend_from_slice(addr.as_bytes()));
        let digest = hash(&buffer);

        Address(digest)
    }
}

impl Display for MultisigSignature {
    /// Formats the [`MultisigSignature`] as a base32-encoded Algorand address string.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
