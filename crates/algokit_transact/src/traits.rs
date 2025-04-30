//! Traits for Algokit Core data encoding and transaction identification.
//!
//! This module provides traits for standardized MessagePack encoding/decoding of
//! Algorand data structures and for calculating transaction identifiers.

use crate::constants::HASH_BYTES_LENGTH;
use crate::error::AlgoKitTransactError;
use crate::utils::sort_msgpack_value;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512_256};

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
    const PREFIX: &'static [u8] = b"TX";

    /// Encodes the object to MessagePack format without any prefix.
    ///
    /// This method performs canonical encoding with sorted map keys and omitted empty fields,
    /// but does not include any domain separation prefix.
    ///
    /// # Returns
    /// A Result containing the raw encoded bytes.
    ///
    /// # Errors
    /// Returns an error if serialization fails.
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

    /// Decode the bytes into Self. `PREFIX` is ignored if present
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

    /// msgpack encoding of the transaction with keys sorted and empty fields omitted
    /// To get the raw bytes without any domain separator (such as "TX" for transactions), use
    /// `encode_raw`
    fn encode(&self) -> Result<Vec<u8>, AlgoKitTransactError> {
        let encoded = self.encode_raw()?;
        if Self::PREFIX.is_empty() {
            return Ok(encoded);
        }

        let mut buf = Vec::with_capacity(encoded.len() + Self::PREFIX.len());
        buf.extend_from_slice(Self::PREFIX);
        buf.extend_from_slice(&encoded);
        Ok(buf)
    }
}

pub trait TransactionId: AlgorandMsgpack {
    fn raw_id(&self) -> Result<[u8; HASH_BYTES_LENGTH], AlgoKitTransactError> {
        let mut hasher = Sha512_256::new();
        hasher.update(self.encode()?);

        let mut hash = [0u8; HASH_BYTES_LENGTH];
        hash.copy_from_slice(&hasher.finalize()[..HASH_BYTES_LENGTH]);
        Ok(hash)
    }

    fn id(&self) -> Result<String, AlgoKitTransactError> {
        let hash = self.raw_id()?;

        Ok(base32::encode(
            base32::Alphabet::Rfc4648 { padding: false },
            &hash,
        ))
    }
}
