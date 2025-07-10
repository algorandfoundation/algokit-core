//! Algorand single signature account representation and manipulation.
//!
//! This module provides the [`Account`] type, which encapsulates an Algorand single signature
//! account's public key and offers methods for creation, conversion, and display. An account's
//! [`Address`] is derived from its public key and encoded as a 58-character base32 string.

use crate::address::Address;
use crate::constants::Byte32;
use crate::error::AlgoKitTransactError;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents a single signature Algorand account.
///
/// An Algorand account is defined by a 32-byte Ed25519 public key.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(transparent)]
pub struct Account {
    /// The 32-byte Ed25519 public key associated with this account.
    #[serde_as(as = "Bytes")]
    pub pub_key: Byte32,
}

impl Account {
    /// Creates a new [`Account`] from a 32-byte public key.
    ///
    /// # Arguments
    /// * `pub_key` - The 32-byte Ed25519 public key.
    ///
    /// # Returns
    /// A new [`Account`] instance containing the provided public key.
    pub fn from_pubkey(pub_key: &Byte32) -> Self {
        Account { pub_key: *pub_key }
    }

    /// Returns the [`Address`] corresponding to this account's public key.
    ///
    /// # Returns
    /// The [`Address`] derived from the account's public key.
    pub fn address(&self) -> Address {
        Address::from(self.clone())
    }
}

impl From<Address> for Account {
    /// Converts an [`Address`] into an [`Account`] by extracting the underlying public key bytes.
    fn from(addr: Address) -> Self {
        Account::from_pubkey(addr.as_bytes())
    }
}

impl From<Account> for Address {
    /// Converts an [`Account`] into an [`Address`] by wrapping its public key.
    fn from(account: Account) -> Address {
        Address(account.pub_key)
    }
}

impl FromStr for Account {
    type Err = AlgoKitTransactError;

    /// Parses an [`Account`] from a string by first parsing it as an [`Address`].
    ///
    /// # Arguments
    /// * `s` - A string slice representing an Algorand address.
    ///
    /// # Returns
    /// An [`Account`] if the string is a valid address, or an error otherwise.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Address>().map(Into::into)
    }
}

impl Display for Account {
    /// Formats the [`Account`] as a base32-encoded Algorand address string.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
