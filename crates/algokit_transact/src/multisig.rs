//! Algorand multisignature signature representation and manipulation.
//!
//! This module provides the [`MultisigSignature`] type, which encapsulates an Algorand multisignature
//! signature's version, threshold, and participating addresses. The corresponding [`Address`] is derived
//! from the domain separator, version, threshold, and the concatenated addresses, hashed to produce
//! the 32-byte digest used as the address.
//!
//! Unlike single-signature addresses, it is not possible to reconstruct the full set of multisignature
//! parameters from an address alone, as the "public information" of a multisig signature is derived with
//! a cryptographic hash function.

use crate::address::Address;
use crate::utils::hash;
use crate::{
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, MULTISIG_DOMAIN_SEPARATOR,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Represents an Algorand multisignature signature.
///
/// A multisignature signature is defined by a version, a threshold, and a list of participating addresses.
/// The version indicates the multisig protocol version, while the threshold specifies the minimum
/// number of signatures required to authorize a transaction.
/// While technically this accepts [`Address`] types, it is expected that these will be
/// the addresses of Ed25519 public keys.
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
    /// Creates a new multisignature signature with the specified version, threshold, and participants.
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
    /// Address of a keypair account participant that is sub-signing a multisignature transaction.
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
