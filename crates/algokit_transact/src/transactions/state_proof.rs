//! State proof transaction module for AlgoKit Core.
//!
//! This module provides functionality for decoding state proof transactions.

use crate::Transaction;
use crate::transactions::common::TransactionHeader;
use crate::utils::{is_zero, is_zero_opt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(name = "HashFactoryBuilder")]
pub struct HashFactory {
    #[serde(rename = "t")]
    pub hash_type: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MerkleArrayProof {
    #[serde(rename = "pth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde_as(as = "Vec<Bytes>")]
    pub path: Vec<Vec<u8>>,

    #[serde(rename = "hsh")]
    pub hash_factory: HashFactory,

    #[serde(rename = "td")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub tree_depth: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MerkleSignatureVerifier {
    #[serde(rename = "cmt")]
    #[serde_as(as = "Bytes")]
    pub commitment: [u8; 64],

    #[serde(rename = "lf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub key_lifetime: u64,
}

/// A Participant corresponds to an account whose AccountData.Status is Online, and for which the
/// expected sigRound satisfies AccountData.VoteFirstValid <= sigRound <= AccountData.VoteLastValid.
///
/// In the Algorand ledger, it is possible for multiple accounts to have the same PK. Thus, the PK is
/// not necessarily unique among Participants. However, each account will produce a unique Participant
/// struct, to avoid potential DoS attacks where one account claims to have the same VoteID PK as
/// another account.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Participant {
    #[serde(rename = "p")]
    pub verifier: MerkleSignatureVerifier,

    #[serde(rename = "w")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub weight: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FalconVerifier {
    #[serde(rename = "k")]
    #[serde_as(as = "Bytes")]
    pub public_key: Vec<u8>,
}

/// Represents a signature in the merkle signature scheme using falcon signatures
/// as an underlying crypto scheme. It consists of an ephemeral public key, a signature, a merkle
/// verification path and an index. The merkle signature considered valid only if the Signature is
/// verified under the ephemeral public key and the Merkle verification path verifies that the
/// ephemeral public key is located at the given index of the tree (for the root given in the
/// long-term public key). More details can be found on Algorand's spec
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FalconSignatureStruct {
    #[serde(rename = "sig")]
    #[serde_as(as = "Bytes")]
    pub signature: Vec<u8>,

    #[serde(rename = "idx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub vector_commitment_index: u64,

    #[serde(rename = "prf")]
    pub proof: MerkleArrayProof,

    #[serde(rename = "vkey")]
    pub verifying_key: FalconVerifier,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SigslotCommit {
    #[serde(rename = "s")]
    pub sig: FalconSignatureStruct,

    #[serde(rename = "l")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub lower_sig_weight: u64,
}

/// A single array position revealed as part of a state proof. It reveals an element of the
/// signature array and the corresponding element of the participants array.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reveal {
    #[serde(rename = "s")]
    pub sigslot: SigslotCommit,

    #[serde(rename = "p")]
    pub participant: Participant,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StateProof {
    #[serde(rename = "c")]
    #[serde_as(as = "Bytes")]
    pub sig_commit: Vec<u8>,

    #[serde(rename = "w")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub signed_weight: u64,

    #[serde(rename = "S")]
    pub sig_proofs: MerkleArrayProof,

    #[serde(rename = "P")]
    pub part_proofs: MerkleArrayProof,

    #[serde(rename = "v")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub merkle_signature_salt_version: u64,

    /// A sparse map from the position being revealed to the corresponding elements from the
    /// sigs and participants arrays.
    #[serde(rename = "r")]
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub reveals: BTreeMap<u64, Reveal>,

    #[serde(rename = "pr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub positions_to_reveal: Vec<u64>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StateProofMessage {
    #[serde(rename = "b")]
    #[serde_as(as = "Bytes")]
    pub block_headers_commitment: Vec<u8>,

    #[serde(rename = "v")]
    #[serde_as(as = "Bytes")]
    pub voters_commitment: Vec<u8>,

    #[serde(rename = "P")]
    pub ln_proven_weight: u64,

    #[serde(rename = "f")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub first_attested_round: u64,

    #[serde(rename = "l")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub last_attested_round: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "StateProofTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct StateProofTransactionFields {
    #[serde(flatten)]
    pub header: TransactionHeader,

    #[serde(rename = "sptype")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub state_proof_type: Option<u64>,

    #[serde(rename = "sp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub state_proof: Option<StateProof>,

    #[serde(rename = "spmsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub message: Option<StateProofMessage>,
}

impl StateProofTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, StateProofTransactionBuilderError> {
        self.build_fields().map(Transaction::StateProof)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::TestDataMother;

    #[test]
    fn test_state_proof_snapshot() {
        let data = TestDataMother::state_proof();
        assert_eq!(
            data.id,
            String::from("6D3MLKOASKUXHFTTWYUG563UBKZ5RW3FFKN6ZUUWBCY47RZT3HIA")
        );
    }
}
