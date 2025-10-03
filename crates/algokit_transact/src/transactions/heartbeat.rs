//! Heartbeat transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing heartbeat transactions,
//! which are used to maintain participation in Algorand consensus.

use crate::Address;
use crate::Byte32;
use crate::Transaction;
use crate::transactions::common::TransactionHeader;
use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{Bytes, serde_as};

/// Represents proof information for a heartbeat transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(name = "HeartbeatProofBuilder", build_fn(name = "build"))]
pub struct HeartbeatProof {
    /// Signature (64 bytes).
    #[serde(rename = "s")]
    #[serde_as(as = "Bytes")]
    pub sig: [u8; 64],

    /// Public key (32 bytes).
    #[serde(rename = "p")]
    #[serde_as(as = "Bytes")]
    pub pk: Byte32,

    /// Public key 2 (32 bytes).
    #[serde(rename = "p2")]
    #[serde_as(as = "Bytes")]
    pub pk2: Byte32,

    /// Public key 1 signature (64 bytes).
    #[serde(rename = "p1s")]
    #[serde_as(as = "Bytes")]
    pub pk1_sig: [u8; 64],

    /// Public key 2 signature (64 bytes).
    #[serde(rename = "p2s")]
    #[serde_as(as = "Bytes")]
    pub pk2_sig: [u8; 64],
}

// Only used for serialise/deserialise
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
struct HeartbeatParams {
    /// Heartbeat address.
    #[serde(rename = "a")]
    pub address: Address,

    /// Heartbeat proof.
    #[serde(rename = "prf")]
    pub proof: HeartbeatProof,

    /// Heartbeat seed.
    #[serde(rename = "sd")]
    #[serde_as(as = "Bytes")]
    pub seed: Vec<u8>,

    /// Heartbeat vote ID.
    #[serde(rename = "vid")]
    #[serde_as(as = "Bytes")]
    pub vote_id: Byte32,

    /// Heartbeat key dilution.
    #[serde(rename = "kd")]
    pub key_dilution: u64,
}

/// Represents a heartbeat transaction that maintains participation in Algorand consensus.
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(name = "HeartbeatTransactionBuilder", build_fn(name = "build_fields"))]
pub struct HeartbeatTransactionFields {
    /// Common transaction header fields.
    pub header: TransactionHeader,

    /// Heartbeat address.
    pub address: Address,

    /// Heartbeat proof.
    pub proof: HeartbeatProof,

    /// Heartbeat seed.
    pub seed: Vec<u8>,

    /// Heartbeat vote ID.
    pub vote_id: Byte32,

    /// Heartbeat key dilution.
    pub key_dilution: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct HeartbeatTransactionFieldsSerde {
    #[serde(flatten)]
    header: TransactionHeader,

    #[serde(rename = "hb")]
    heartbeat_params: HeartbeatParams,
}

pub fn heartbeat_serializer<S>(
    fields: &HeartbeatTransactionFields,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let heartbeat_params = HeartbeatParams {
        address: fields.address.clone(),
        proof: fields.proof.clone(),
        seed: fields.seed.clone(),
        vote_id: fields.vote_id,
        key_dilution: fields.key_dilution,
    };

    let serde_struct = HeartbeatTransactionFieldsSerde {
        header: fields.header.clone(),
        heartbeat_params,
    };

    serde_struct.serialize(serializer)
}

pub fn heartbeat_deserializer<'de, D>(
    deserializer: D,
) -> Result<HeartbeatTransactionFields, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialised_fields = HeartbeatTransactionFieldsSerde::deserialize(deserializer)?;

    Ok(HeartbeatTransactionFields {
        header: deserialised_fields.header,
        address: deserialised_fields.heartbeat_params.address,
        proof: deserialised_fields.heartbeat_params.proof,
        seed: deserialised_fields.heartbeat_params.seed,
        vote_id: deserialised_fields.heartbeat_params.vote_id,
        key_dilution: deserialised_fields.heartbeat_params.key_dilution,
    })
}

impl HeartbeatTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, HeartbeatTransactionBuilderError> {
        self.build_fields().map(Transaction::Heartbeat)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::TestDataMother;

    #[test]
    fn test_heartbeat_snapshot() {
        let data = TestDataMother::heartbeat();
        assert_eq!(
            data.id,
            String::from("GCVW7GJTD5OALIXPQ3RGMYKTTYCWUJY3E4RPJTX7WHIWZK4V6NYA")
        );
    }
}
