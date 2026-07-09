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
use serde::de::{Deserializer, Error as DeError, SeqAccess, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an Algorand address as decoded bytes without the checksum from a 58-character base32 string.
///
/// The [`Address`] type stores the 32 bytes of the address (the public key or hash digest),
/// and provides methods for encoding to and decoding from the standard Algorand base32 string format.
/// The checksum is automatically calculated and validated as part of parsing and formatting.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Address(pub(crate) Byte32);

// JSON encodes addresses as base32 strings, msgpack as raw 32 bytes. Serialization uses
// `is_human_readable()`; deserialization dispatches on the arriving value because that flag is
// unreliable inside serde's buffered internally-tagged enum decoding. (e.g. as done in Transaction type)
impl Serialize for Address {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.as_str())
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AddressVisitor;

        impl<'de> Visitor<'de> for AddressVisitor {
            type Value = Address;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a base32 Algorand address string or 32 address bytes")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Address, E> {
                Address::from_str(v).map_err(DeError::custom)
            }

            fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Address, E> {
                let bytes: Byte32 = v
                    .try_into()
                    .map_err(|_| DeError::invalid_length(v.len(), &"32 address bytes"))?;
                Ok(Address(bytes))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Address, A::Error> {
                let mut bytes = [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH];
                for (i, slot) in bytes.iter_mut().enumerate() {
                    *slot = seq
                        .next_element()?
                        .ok_or_else(|| DeError::invalid_length(i, &"32 address bytes"))?;
                }
                Ok(Address(bytes))
            }
        }

        deserializer.deserialize_any(AddressVisitor)
    }
}

impl Address {
    /// Creates a new Address from a 32-byte public key or hash digest.
    pub const fn new(bytes: Byte32) -> Self {
        Self(bytes)
    }
    /// Returns the 32 bytes of the address as a byte array reference.
    pub fn as_bytes(&self) -> &Byte32 {
        &self.0
    }

    /// Computes the escrow address from an application ID.
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
            return Err(AlgoKitTransactError::InvalidAddress {
                err_msg: "Algorand address must be exactly 58 characters".into(),
            });
        }
        let decoded_address = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s)
            .ok_or_else(|| AlgoKitTransactError::InvalidAddress {
                err_msg: "Invalid base32 encoding for Algorand address".into(),
            })?;

        // Although this is called public key it could be the digest of a hash when the address
        // corresponds to a multisignature account or logic signature account.
        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] = decoded_address
            [..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| AlgoKitTransactError::InvalidAddress {
                err_msg: "Could not decode address into 32-byte public key".to_string(),
            })?;
        let checksum: [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] = decoded_address
            [ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| AlgoKitTransactError::InvalidAddress {
                err_msg: "Could not get 4-byte checksum from decoded address".to_string(),
            })?;

        if pub_key_to_checksum(&pub_key) != checksum {
            return Err(AlgoKitTransactError::InvalidAddress {
                err_msg: "Checksum is invalid".to_string(),
            });
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

    const SAMPLE: &str = "WRBMNT66ECE2AOYKM76YVWIJMBW6Z3XCQZOKG5BL7NISAQC2LBGEKTZLRM";

    /// JSON encodes an address as the base32 string and decodes it back.
    #[test]
    fn json_serdes_as_base32_string() {
        let address = Address::from_str(SAMPLE).unwrap();
        let json = serde_json::to_string(&address).unwrap();
        assert_eq!(json, format!("\"{SAMPLE}\""));
        assert_eq!(serde_json::from_str::<Address>(&json).unwrap(), address);
    }

    /// msgpack encodes an address as 32 raw bytes and decodes it back.
    #[test]
    fn msgpack_serdes_as_bytes() {
        let address = Address::from_str(SAMPLE).unwrap();
        let bytes = rmp_serde::to_vec(&address).unwrap();
        // `c4 20` is the msgpack marker for a 32-byte binary string.
        assert_eq!(&bytes[..2], &[0xc4, 0x20]);
        assert_eq!(rmp_serde::from_slice::<Address>(&bytes).unwrap(), address);
    }
}
