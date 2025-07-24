//! Algorand multisignature representation and manipulation.
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
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, AlgoKitTransactError,
    MULTISIG_DOMAIN_SEPARATOR,
};
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
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
    ///
    /// # Errors
    ///
    /// Returns [`InvalidMultisigSignature`] if:
    /// - `version` is zero,
    /// - `participants` is empty,
    /// - `threshold` is zero or greater than the number of participants.
    pub fn from_participants(
        version: u8,
        threshold: u8,
        participants: Vec<Address>,
    ) -> Result<Self, AlgoKitTransactError> {
        let subsignatures = participants
            .into_iter()
            .map(|address| MultisigSubsignature {
                address,
                signature: None,
            })
            .collect();
        Self::new(version, threshold, subsignatures)
    }

    /// Creates a new multisignature signature from a vector of subsignatures.
    ///
    /// # Errors
    ///
    /// Returns [`AlgoKitTransactError::InvalidMultisigSignature`] if:
    /// - `version` is zero,
    /// - `subsignatures` is empty,
    /// - `threshold` is zero or greater than the number of subsignatures.
    pub fn new(
        version: u8,
        threshold: u8,
        subsignatures: Vec<MultisigSubsignature>,
    ) -> Result<Self, AlgoKitTransactError> {
        if version == 0 {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Version cannot be zero".to_string(),
            ));
        }
        if subsignatures.is_empty() {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Subsignatures cannot be empty".to_string(),
            ));
        }
        if threshold == 0 || threshold as usize > subsignatures.len() {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Threshold must be greater than zero and less than or equal \
                to the number of sub-signers"
                    .to_string(),
            ));
        }
        Ok(Self {
            version,
            threshold,
            subsignatures,
        })
    }

    /// Returns the list of participant addresses in this multisignature.
    pub fn participants(&self) -> Vec<Address> {
        self.subsignatures
            .iter()
            .map(|subsig| subsig.address.clone())
            .collect()
    }

    /// Applies a subsignature for the given address, replacing all existing signatures for that address.
    ///
    /// Typically, an address appears in a multisignature only once which means that only one
    /// signature for a given address should be in the list of subsignatures.
    /// However, there's a construction of "weighted" multisignatures where the same address
    /// can appear multiple times.
    /// The node checks these subsignatures agnostic to the fact that the same address might be repeated.
    /// This allows a user to effectively give a weight to a particular address.
    /// For this reason, the code is structured to apply the signature to all the instances of address.
    ///
    /// Since ed25519 signatures are deterministic, there's only one valid signature for a given message and public key,
    /// which also means it's OK to apply the same signature multiple times.
    ///
    /// # Disclaimer
    /// This method will overwrite any existing signature for the given address.
    ///
    /// # Errors
    /// Returns [`AlgoKitTransactError::InvalidMultisigSignature`] if the address is not found among the participants.
    pub fn apply_subsignature(
        &self,
        address: Address,
        subsignature: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH],
    ) -> Result<Self, AlgoKitTransactError> {
        let mut subsignatures = self.subsignatures.clone();
        let mut found = false;
        for subsig in subsignatures
            .iter_mut()
            .filter(|subsig| subsig.address == address)
        {
            found = true;
            subsig.signature = Some(subsignature);
        }
        if !found {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Address not found in multisig signature".to_string(),
            ));
        }

        Ok(Self {
            version: self.version,
            threshold: self.threshold,
            subsignatures,
        })
    }

    /// Merges another multisignature into this one, replacing existing signatures with those from `other` where present.
    ///
    /// # Disclaimer
    /// For each participant, the resulting signature will be taken from `other` if present, otherwise from `self`.
    /// This operation does not combine signatures; it replaces them.
    ///
    /// # Errors
    ///
    /// Returns [`AlgoKitTransactError::InvalidMultisigSignature`] if the version, threshold, or participants differ.
    pub fn merge(&self, other: &Self) -> Result<Self, AlgoKitTransactError> {
        if self.version != other.version {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Cannot merge multisig signatures with different versions".to_string(),
            ));
        }
        if self.threshold != other.threshold {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Cannot merge multisig signatures with different thresholds".to_string(),
            ));
        }
        if self.participants() != other.participants() {
            return Err(AlgoKitTransactError::InvalidMultisigSignature(
                "Cannot merge multisig signatures with different participants".to_string(),
            ));
        }

        let subsignatures = self
            .subsignatures
            .iter()
            .zip(&other.subsignatures)
            .map(|(s1, s2)| MultisigSubsignature {
                address: s1.address.clone(),
                signature: s2.signature.or(s1.signature),
            })
            .collect();

        Ok(Self {
            version: self.version,
            threshold: self.threshold,
            subsignatures,
        })
    }
}

/// Represents a single subsignature in a multisignature transaction.
///
/// Each subsignature contains the address of a participant and an optional signature.
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
    fn from(multisig: MultisigSignature) -> Address {
        let mut buffer = Vec::with_capacity(
            MULTISIG_DOMAIN_SEPARATOR.len()
                + 2
                + multisig.subsignatures.len() * ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
        );
        buffer.extend_from_slice(MULTISIG_DOMAIN_SEPARATOR.as_bytes());
        buffer.push(multisig.version);
        buffer.push(multisig.threshold);
        multisig
            .participants()
            .iter()
            .for_each(|addr| buffer.extend_from_slice(addr.as_bytes()));
        let digest = hash(&buffer);

        Address(digest)
    }
}

impl Display for MultisigSignature {
    /// Formats the [`MultisigSignature`] as a base32-encoded Algorand address string.
    ///
    /// This uses the derived address from the multisig parameters and participants.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
