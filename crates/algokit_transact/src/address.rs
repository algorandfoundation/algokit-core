//! Algorand addresses are base32-encoded strings that represent 32 bytes plus a checksum.
//!
//! This module provides the [`Address`] type, which encapsulates the logic for parsing,
//! validating, and displaying Algorand addresses. An address is a 58-character base32 string
//! encoding 32 bytes of data and a 4-byte checksum.

use crate::constants::Byte32;
use crate::error::AlgoKitTransactError;
use crate::utils::{hash, pub_key_to_checksum};
use crate::{
    ALGORAND_ADDRESS_LENGTH, ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
};
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an Algorand address as decoded bytes without the checksum from a 58-character base32 string.
///
/// The [`Address`] type stores the 32 bytes of the address (the public key or hash digest),
/// and provides methods for encoding to and decoding from the standard Algorand base32 string format.
/// The checksum is automatically calculated and validated as part of parsing and formatting.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct Address(#[serde_as(as = "Bytes")] pub Byte32);

impl Address {
    /// Returns the 32 bytes of the address as a byte array reference.
    pub fn as_bytes(&self) -> &Byte32 {
        &self.0
    }

    /// Computes the address from an application ID.
    pub fn from_app_id(app_id: &u64) -> Self {
        let mut to_hash = b"appID".to_vec();
        to_hash.extend_from_slice(&app_id.to_be_bytes());
        Address(hash(&to_hash))
    }

    /// Returns the base32-encoded string representation of the address, including the checksum.
    pub fn as_str(&self) -> String {
        let mut buffer = [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH + ALGORAND_CHECKSUM_BYTE_LENGTH];
        buffer[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH].copy_from_slice(&self.0);

        let checksum = self.checksum();
        buffer[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..].copy_from_slice(&checksum);

        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &buffer)
    }

    /// Computes the 4-byte checksum for the address.
    pub fn checksum(&self) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
        pub_key_to_checksum(&self.0)
    }
}

impl FromStr for Address {
    type Err = AlgoKitTransactError;

    /// Parses a 58-character base32 Algorand address string into an [`Address`] instance.
    ///
    /// Returns an error if the string is not exactly 58 characters, is not valid base32,
    /// or if the checksum does not match.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ALGORAND_ADDRESS_LENGTH {
            return Err(AlgoKitTransactError::InvalidAddress(
                "Algorand address must be exactly 58 characters".into(),
            ));
        }
        let decoded_address = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s)
            .ok_or_else(|| {
                AlgoKitTransactError::InvalidAddress(
                    "Invalid base32 encoding for Algorand address".into(),
                )
            })?;

        // Although this is called public key (and it actually is when the account is a `KeyPairAccount`),
        // it could be the digest of a hash when the address corresponds to a multisignature account or
        // logic signature account.
        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] = decoded_address
            [..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "Could not decode address into 32-byte public key".to_string(),
                )
            })?;
        let checksum: [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] = decoded_address
            [ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "Could not get 4-byte checksum from decoded address".to_string(),
                )
            })?;

        if pub_key_to_checksum(&pub_key) != checksum {
            return Err(AlgoKitTransactError::InvalidAddress(
                "Checksum is invalid".to_string(),
            ));
        }
        Ok(Address(pub_key))
    }
}

impl Display for Address {
    /// Formats the address as a base32-encoded string.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_app_id() {
        let app_id = 123u64;
        let address = Address::from_app_id(&app_id);
        let address_str = address.to_string();
        assert_eq!(
            address_str,
            "WRBMNT66ECE2AOYKM76YVWIJMBW6Z3XCQZOKG5BL7NISAQC2LBGEKTZLRM"
        );
    }
}
