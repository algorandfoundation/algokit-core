//! Traits for AlgoKit Core data encoding and transaction identification.
//!
//! This module provides traits for standardized MessagePack encoding/decoding of
//! Algorand data structures and for calculating transaction identifiers.

use crate::error::AlgoKitTransactError;
use crate::utils::sort_msgpack_value;
use crate::Transaction;
use crate::{constants::HASH_BYTES_LENGTH, utils::hash};
use serde::{Deserialize, Serialize};

/// Trait for Algorand MessagePack encoding and decoding.
///
/// This trait defines methods for serializing and deserializing Algorand data structures
/// to and from MessagePack format with the specific requirements of the Algorand protocol,
/// including canonical sorting of map keys and domain separation prefixes.
pub trait AlgorandMsgpack: Serialize + for<'de> Deserialize<'de> {
    /// Domain separation prefix used during encoding.
    ///
    /// This prefix is prepended to the encoded data to distinguish different types of
    /// Algorand objects. For example, transactions use "TX" as their prefix.
    /// An empty prefix means no domain separation is applied.
    const PREFIX: &'static [u8] = b"";

    /// Encodes the object to MessagePack format without any prefix.
    ///
    /// This method performs canonical encoding with sorted map keys and omitted empty fields,
    /// but does not include any domain separation prefix.
    ///
    /// # Returns
    /// The raw encoded bytes or an AlgoKitTransactError if serialization fails.
    fn encode_raw(&self) -> Result<Vec<u8>, AlgoKitTransactError> {
        // First serialize to a temporary buffer to get the map entries
        let mut temp_buf = Vec::new();
        let mut temp_serializer = rmp_serde::Serializer::new(&mut temp_buf)
            .with_struct_map()
            .with_bytes(rmp_serde::config::BytesMode::ForceAll);

        self.serialize(&mut temp_serializer)?;

        // Deserialize into a Value and sort recursively
        let value = rmpv::decode::read_value(&mut temp_buf.as_slice())?;
        let sorted_value = sort_msgpack_value(value);

        // Serialize the sorted value
        let mut final_buf = Vec::new();
        rmpv::encode::write_value(&mut final_buf, &sorted_value)?;

        Ok(final_buf)
    }

    /// Decodes MessagePack bytes into an instance of this same type.
    ///
    /// If the bytes start with the expected PREFIX for this type, the prefix is
    /// automatically removed before decoding.
    ///
    /// # Parameters
    /// * `bytes` - The MessagePack encoded bytes
    ///
    /// # Returns
    /// The decoded instance or an AlgoKitTransactError if the input is empty or
    /// deserialization fails.
    fn decode(bytes: &[u8]) -> Result<Self, AlgoKitTransactError> {
        if bytes.is_empty() {
            return Err(AlgoKitTransactError::InputError(
                "attempted to decode 0 bytes".to_string(),
            ));
        }

        // If there is a PREFIX defined, bytes is longer than the prefix, and the bytes start
        // with the prefix, decode the bytes without the prefix
        if Self::PREFIX.len() > 0
            && bytes.len() > Self::PREFIX.len()
            && &bytes[..Self::PREFIX.len()] == Self::PREFIX
        {
            let without_prefix = &bytes[Self::PREFIX.len()..];
            Ok(rmp_serde::from_slice(&without_prefix)?)
        } else {
            Ok(rmp_serde::from_slice(bytes)?)
        }
    }

    /// Encodes the object to MessagePack format with the appropriate prefix.
    ///
    /// This method performs canonical encoding and prepends the domain separation
    /// prefix defined by the PREFIX constant.
    ///
    /// Use `encode_raw()` if you want to encode without the prefix.
    ///
    /// # Returns
    /// The encoded bytes with prefix or an AlgoKitTransactError if serialization fails.
    fn encode(&self) -> Result<Vec<u8>, AlgoKitTransactError> {
        let encoded: Vec<u8> = self.encode_raw()?;
        if Self::PREFIX.is_empty() {
            return Ok(encoded);
        }

        let mut buf = Vec::with_capacity(encoded.len() + Self::PREFIX.len());
        buf.extend_from_slice(Self::PREFIX);
        buf.extend_from_slice(&encoded);
        Ok(buf)
    }
}

/// Trait for generating transaction identifiers.
///
/// This trait provides methods for calculating and representing transaction IDs,
/// which are Sha512_256 hashes of the encoded transaction data. These IDs are used
/// to uniquely identify transactions in the Algorand blockchain.
pub trait TransactionId: AlgorandMsgpack {
    /// Generates the raw transaction ID as a hash of the transaction data.
    ///
    /// Computes a Sha512_256 hash of the MessagePack-encoded transaction
    /// (including the appropriate domain prefix).
    ///
    /// # Returns
    /// The transaction ID as a 32-byte array or an AlgoKitTransactError if encoding fails.
    fn id_raw(&self) -> Result<[u8; HASH_BYTES_LENGTH], AlgoKitTransactError> {
        Ok(hash(&self.encode()?))
    }

    /// Converts the transaction ID to a base32-encoded string.
    ///
    /// This creates the standard human-readable representation of a transaction ID
    /// as it would appear in block explorers, APIs, and other interfaces.
    ///
    /// # Returns
    /// The base32-encoded transaction ID string or an AlgoKitTransactError if ID calculation fails.
    fn id(&self) -> Result<String, AlgoKitTransactError> {
        let hash = self.id_raw()?;

        Ok(base32::encode(
            base32::Alphabet::Rfc4648 { padding: false },
            &hash,
        ))
    }
}

pub trait EstimateTransactionSize: AlgorandMsgpack {
    fn estimate_size(&self) -> Result<usize, AlgoKitTransactError>;
}

/// Trait for operations which pertain to a collection of transactions.
pub trait Transactions: Sized {
    /// Groups the supplied transactions by calculating and assigning the group to each transaction.
    ///
    /// # Returns
    /// A result containing the transactions with group assign or an error if grouping fails.
    fn assign_group(self) -> Result<Vec<Transaction>, AlgoKitTransactError>;
}
