//! Algorand address handling and manipulation.
//!
//! This module provides functionality for working with Algorand addresses,
//! including creation, validation, encoding, and decoding. Algorand addresses
//! are base32-encoded strings that represent a public key with a checksum.

use crate::constants::{
    Byte32, ALGORAND_ADDRESS_LENGTH, ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
};
use crate::error::AlgoKitTransactError;
use crate::utils::pub_key_to_checksum;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an address.
///
/// An Algorand address is a 32-byte public key with a 4-byte checksum,
/// typically represented as a 58-character base32-encoded string.
/// This struct encapsulates the underlying public key and provides
/// methods for creating, validating, and converting human-readable addresses.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(transparent)]
pub struct Address {
    /// The 32-byte Ed25519 public key associated with this address.
    #[serde_as(as = "Bytes")]
    pub pub_key: Byte32,
}

impl Address {
    /// Creates a new Address from a 32-byte public key.
    ///
    /// # Parameters
    /// * `pub_key` - The 32-byte Ed25519 public key
    ///
    /// # Returns
    /// A new Address instance containing the provided public key.
    pub fn from_pubkey(pub_key: &Byte32) -> Self {
        Address { pub_key: *pub_key }
    }

    /// Calculates the 4-byte checksum for this address's public key.
    ///
    /// # Returns
    /// A 4-byte array containing the checksum.
    pub fn checksum(&self) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
        pub_key_to_checksum(&self.pub_key)
    }
}

impl FromStr for Address {
    type Err = AlgoKitTransactError;

    /// Creates a new Address from a base32-encoded string.
    ///
    /// # Parameters
    /// * `s` - A 58-character base32-encoded Algorand address string
    ///
    /// # Returns
    /// The Address or an error if the string is invalid (wrong length, invalid base32,
    /// invalid format, or checksum mismatch, etc.).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ALGORAND_ADDRESS_LENGTH {
            return Err(AlgoKitTransactError::InvalidAddress(
                "address length is not 58".to_string(),
            ));
        }
        let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s)
            .expect("decoded value should exist");

        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] = decoded
            [..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "could not decode address into 32-byte public key".to_string(),
                )
            })?;

        let checksum: [u8; 4] = decoded[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "could not get 4-byte checksum from decoded address".to_string(),
                )
            })?;

        let computed_checksum = pub_key_to_checksum(&pub_key);

        if computed_checksum != checksum {
            return Err(AlgoKitTransactError::InvalidAddress(
                "checksum is invalid".to_string(),
            ));
        }

        Ok(Self { pub_key })
    }
}

impl Display for Address {
    /// Formats the address as a base32-encoded string.
    ///
    /// # Parameters
    /// * `f` - The formatter
    ///
    /// # Returns
    /// A formatting result with a string containing the base32-encoded address or error
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut address_bytes =
            [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH + ALGORAND_CHECKSUM_BYTE_LENGTH];

        address_bytes[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH].copy_from_slice(&self.pub_key);

        let checksum = self.checksum();
        address_bytes[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..].copy_from_slice(&checksum);

        let result = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &address_bytes);
        write!(f, "{}", result)
    }
}
