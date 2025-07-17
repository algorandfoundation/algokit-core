//! Algorand ed25519 keypair account representation and manipulation.
//!
//! This module provides the [`KeyPairAccount`] type, which encapsulates an Algorand ed25519 keypair
//! account's public key and offers methods for creation, conversion, and display. An account's
//! [`Address`] is derived from its public key and encoded as a 58-character base32 string.

use crate::address::Address;
use crate::constants::Byte32;
use crate::error::AlgoKitTransactError;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an ed25519 keypair Algorand account.
///
/// An Algorand keypair account is defined by a 32-byte Ed25519 public key.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(transparent)]
pub struct KeyPairAccount {
    /// The 32-byte Ed25519 public key associated with this account.
    #[serde_as(as = "Bytes")]
    pub pub_key: Byte32,
}

impl KeyPairAccount {
    /// Creates a new [`KeyPairAccount`] from a 32-byte public key.
    ///
    /// # Arguments
    /// * `pub_key` - The 32-byte Ed25519 public key.
    ///
    /// # Returns
    /// A new [`KeyPairAccount`] instance containing the provided public key.
    pub fn from_pubkey(pub_key: &Byte32) -> Self {
        KeyPairAccount { pub_key: *pub_key }
    }

    /// Returns the [`Address`] corresponding to this account's public key.
    ///
    /// # Returns
    /// The [`Address`] derived from the account's public key.
    pub fn address(&self) -> Address {
        Address::from(self.clone())
    }
}

impl From<Address> for KeyPairAccount {
    /// Converts an [`Address`] into an [`KeyPairAccount`] by extracting the underlying public key bytes.
    fn from(addr: Address) -> Self {
        KeyPairAccount::from_pubkey(addr.as_bytes())
    }
}

impl From<KeyPairAccount> for Address {
    /// Converts an [`KeyPairAccount`] into an [`Address`] by wrapping its public key.
    fn from(account: KeyPairAccount) -> Address {
        Address(account.pub_key)
    }
}

impl FromStr for KeyPairAccount {
    type Err = AlgoKitTransactError;

    /// Parses an [`KeyPairAccount`] from a string by first parsing it as an [`Address`].
    ///
    /// # Arguments
    /// * `s` - A string slice representing an Algorand address.
    ///
    /// # Returns
    /// An [`KeyPairAccount`] if the string is a valid address, or an error otherwise.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Address>().map(Into::into)
    }
}

impl Display for KeyPairAccount {
    /// Formats the [`KeyPairAccount`] as a base32-encoded Algorand address string.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
