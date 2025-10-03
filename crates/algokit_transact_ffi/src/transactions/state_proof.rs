use std::collections::BTreeMap;

use crate::*;

#[ffi_record]
pub struct HashFactory {
    hash_type: u64,
}

impl From<algokit_transact::HashFactory> for HashFactory {
    fn from(hf: algokit_transact::HashFactory) -> Self {
        Self {
            hash_type: hf.hash_type,
        }
    }
}

impl From<HashFactory> for algokit_transact::HashFactory {
    fn from(hf: HashFactory) -> Self {
        Self {
            hash_type: hf.hash_type,
        }
    }
}

#[ffi_record]
pub struct MerkleArrayProof {
    path: Vec<Vec<u8>>,
    hash_factory: HashFactory,
    tree_depth: u64,
}

impl From<algokit_transact::MerkleArrayProof> for MerkleArrayProof {
    fn from(proof: algokit_transact::MerkleArrayProof) -> Self {
        Self {
            path: proof.path,
            hash_factory: proof.hash_factory.into(),
            tree_depth: proof.tree_depth,
        }
    }
}

impl From<MerkleArrayProof> for algokit_transact::MerkleArrayProof {
    fn from(proof: MerkleArrayProof) -> Self {
        Self {
            path: proof.path,
            hash_factory: proof.hash_factory.into(),
            tree_depth: proof.tree_depth,
        }
    }
}

#[ffi_record]
pub struct MerkleSignatureVerifier {
    commitment: Vec<u8>,
    key_lifetime: u64,
}

impl From<algokit_transact::MerkleSignatureVerifier> for MerkleSignatureVerifier {
    fn from(verifier: algokit_transact::MerkleSignatureVerifier) -> Self {
        Self {
            commitment: verifier.commitment.to_vec(),
            key_lifetime: verifier.key_lifetime,
        }
    }
}

impl TryFrom<MerkleSignatureVerifier> for algokit_transact::MerkleSignatureVerifier {
    type Error = AlgoKitTransactError;

    fn try_from(verifier: MerkleSignatureVerifier) -> Result<Self, Self::Error> {
        Ok(Self {
            commitment: vec_to_array::<64>(&verifier.commitment, "commitment")?,
            key_lifetime: verifier.key_lifetime,
        })
    }
}

/// A Participant corresponds to an account whose AccountData.Status is Online, and for which the
/// expected sigRound satisfies AccountData.VoteFirstValid <= sigRound <= AccountData.VoteLastValid.
///
/// In the Algorand ledger, it is possible for multiple accounts to have the same PK. Thus, the PK is
/// not necessarily unique among Participants. However, each account will produce a unique Participant
/// struct, to avoid potential DoS attacks where one account claims to have the same VoteID PK as
/// another account.
#[ffi_record]
pub struct Participant {
    verifier: MerkleSignatureVerifier,
    weight: u64,
}

impl From<algokit_transact::Participant> for Participant {
    fn from(participant: algokit_transact::Participant) -> Self {
        Self {
            verifier: participant.verifier.into(),
            weight: participant.weight,
        }
    }
}

impl TryFrom<Participant> for algokit_transact::Participant {
    type Error = AlgoKitTransactError;

    fn try_from(participant: Participant) -> Result<Self, Self::Error> {
        Ok(Self {
            verifier: participant.verifier.try_into()?,
            weight: participant.weight,
        })
    }
}

#[ffi_record]
pub struct FalconVerifier {
    public_key: Vec<u8>,
}

impl From<algokit_transact::FalconVerifier> for FalconVerifier {
    fn from(verifier: algokit_transact::FalconVerifier) -> Self {
        Self {
            public_key: verifier.public_key,
        }
    }
}

impl From<FalconVerifier> for algokit_transact::FalconVerifier {
    fn from(verifier: FalconVerifier) -> Self {
        Self {
            public_key: verifier.public_key,
        }
    }
}

/// Represents a signature in the merkle signature scheme using falcon signatures
/// as an underlying crypto scheme. It consists of an ephemeral public key, a signature, a merkle
/// verification path and an index. The merkle signature considered valid only if the Signature is
/// verified under the ephemeral public key and the Merkle verification path verifies that the
/// ephemeral public key is located at the given index of the tree (for the root given in the
/// long-term public key). More details can be found on Algorand's spec
#[ffi_record]
pub struct FalconSignatureStruct {
    signature: Vec<u8>,
    vector_commitment_index: u64,
    proof: MerkleArrayProof,
    verifying_key: FalconVerifier,
}

impl From<algokit_transact::FalconSignatureStruct> for FalconSignatureStruct {
    fn from(sig: algokit_transact::FalconSignatureStruct) -> Self {
        Self {
            signature: sig.signature,
            vector_commitment_index: sig.vector_commitment_index,
            proof: sig.proof.into(),
            verifying_key: sig.verifying_key.into(),
        }
    }
}

impl From<FalconSignatureStruct> for algokit_transact::FalconSignatureStruct {
    fn from(sig: FalconSignatureStruct) -> Self {
        Self {
            signature: sig.signature,
            vector_commitment_index: sig.vector_commitment_index,
            proof: sig.proof.into(),
            verifying_key: sig.verifying_key.into(),
        }
    }
}

#[ffi_record]
pub struct SigslotCommit {
    sig: FalconSignatureStruct,
    lower_sig_weight: u64,
}

impl From<algokit_transact::SigslotCommit> for SigslotCommit {
    fn from(commit: algokit_transact::SigslotCommit) -> Self {
        Self {
            sig: commit.sig.into(),
            lower_sig_weight: commit.lower_sig_weight,
        }
    }
}

impl From<SigslotCommit> for algokit_transact::SigslotCommit {
    fn from(commit: SigslotCommit) -> Self {
        Self {
            sig: commit.sig.into(),
            lower_sig_weight: commit.lower_sig_weight,
        }
    }
}

/// A single array position revealed as part of a state proof. It reveals an element of the
/// signature array and the corresponding element of the participants array.
#[ffi_record]
pub struct Reveal {
    position: u64,
    sigslot: SigslotCommit,
    participant: Participant,
}

impl TryFrom<Reveal> for (u64, algokit_transact::Reveal) {
    type Error = AlgoKitTransactError;

    fn try_from(reveal: Reveal) -> Result<Self, Self::Error> {
        Ok((
            reveal.position,
            algokit_transact::Reveal {
                sigslot: reveal.sigslot.into(),
                participant: reveal.participant.try_into()?,
            },
        ))
    }
}

#[ffi_record]
pub struct StateProof {
    sig_commit: Vec<u8>,
    signed_weight: u64,
    sig_proofs: MerkleArrayProof,
    part_proofs: MerkleArrayProof,
    merkle_signature_salt_version: u64,
    reveals: Vec<Reveal>,
    positions_to_reveal: Vec<u64>,
}

impl From<algokit_transact::StateProof> for StateProof {
    fn from(proof: algokit_transact::StateProof) -> Self {
        let reveals = proof
            .reveals
            .into_iter()
            .map(|(position, reveal)| Reveal {
                position,
                sigslot: reveal.sigslot.into(),
                participant: reveal.participant.into(),
            })
            .collect();

        Self {
            sig_commit: proof.sig_commit,
            signed_weight: proof.signed_weight,
            sig_proofs: proof.sig_proofs.into(),
            part_proofs: proof.part_proofs.into(),
            merkle_signature_salt_version: proof.merkle_signature_salt_version,
            reveals,
            positions_to_reveal: proof.positions_to_reveal,
        }
    }
}

impl TryFrom<StateProof> for algokit_transact::StateProof {
    type Error = AlgoKitTransactError;

    fn try_from(proof: StateProof) -> Result<Self, Self::Error> {
        let reveals = proof
            .reveals
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<BTreeMap<_, _>, _>>()?;

        Ok(Self {
            sig_commit: proof.sig_commit,
            signed_weight: proof.signed_weight,
            sig_proofs: proof.sig_proofs.into(),
            part_proofs: proof.part_proofs.into(),
            merkle_signature_salt_version: proof.merkle_signature_salt_version,
            reveals,
            positions_to_reveal: proof.positions_to_reveal,
        })
    }
}

#[ffi_record]
pub struct StateProofMessage {
    block_headers_commitment: Vec<u8>,
    voters_commitment: Vec<u8>,
    ln_proven_weight: u64,
    first_attested_round: u64,
    last_attested_round: u64,
}

impl From<algokit_transact::StateProofMessage> for StateProofMessage {
    fn from(msg: algokit_transact::StateProofMessage) -> Self {
        Self {
            block_headers_commitment: msg.block_headers_commitment,
            voters_commitment: msg.voters_commitment,
            ln_proven_weight: msg.ln_proven_weight,
            first_attested_round: msg.first_attested_round,
            last_attested_round: msg.last_attested_round,
        }
    }
}

impl From<StateProofMessage> for algokit_transact::StateProofMessage {
    fn from(msg: StateProofMessage) -> Self {
        Self {
            block_headers_commitment: msg.block_headers_commitment,
            voters_commitment: msg.voters_commitment,
            ln_proven_weight: msg.ln_proven_weight,
            first_attested_round: msg.first_attested_round,
            last_attested_round: msg.last_attested_round,
        }
    }
}

#[ffi_record]
pub struct StateProofTransactionFields {
    state_proof_type: Option<u64>,
    state_proof: Option<StateProof>,
    message: Option<StateProofMessage>,
}

impl From<algokit_transact::StateProofTransactionFields> for StateProofTransactionFields {
    fn from(tx: algokit_transact::StateProofTransactionFields) -> Self {
        Self {
            state_proof_type: tx.state_proof_type,
            state_proof: tx.state_proof.map(|sp| sp.into()),
            message: tx.message.map(|msg| msg.into()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::StateProofTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::StateProof || tx.state_proof.is_none() {
            return Err(Self::Error::DecodingError {
                message: "State proof transaction data missing".to_string(),
            });
        }

        let data = tx.state_proof.clone().unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let state_proof = data.state_proof.map(TryInto::try_into).transpose()?;
        let message = data.message.map(Into::into);

        let transaction_fields = algokit_transact::StateProofTransactionFields {
            header,
            state_proof_type: data.state_proof_type,
            state_proof,
            message,
        };

        Ok(transaction_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;

    #[test]
    fn test_encode_transaction_validation_integration() {
        // Test valid state proof transaction
        let result = encode_transaction(TestDataMother::state_proof().transaction.into());
        assert!(result.is_ok());
    }
}
