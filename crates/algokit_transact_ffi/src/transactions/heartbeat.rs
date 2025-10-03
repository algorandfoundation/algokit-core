use crate::*;

/// Parameters for a heartbeat transaction proof.
#[ffi_record]
pub struct HeartbeatProof {
    /// Signature (64 bytes).
    sig: Vec<u8>,

    /// Public key (32 bytes).
    pk: Vec<u8>,

    /// Public key 2 (32 bytes).
    pk2: Vec<u8>,

    /// Public key 1 signature (64 bytes).
    pk1_sig: Vec<u8>,

    /// Public key 2 signature (64 bytes).
    pk2_sig: Vec<u8>,
}

impl From<algokit_transact::HeartbeatProof> for HeartbeatProof {
    fn from(proof: algokit_transact::HeartbeatProof) -> Self {
        Self {
            sig: proof.sig.to_vec(),
            pk: proof.pk.to_vec(),
            pk2: proof.pk2.to_vec(),
            pk1_sig: proof.pk1_sig.to_vec(),
            pk2_sig: proof.pk2_sig.to_vec(),
        }
    }
}

/// Parameters to define a heartbeat transaction.
///
/// Used to maintain participation in Algorand consensus.
#[ffi_record]
pub struct HeartbeatTransactionFields {
    /// Heartbeat address.
    address: String,

    /// Heartbeat proof.
    proof: HeartbeatProof,

    /// Heartbeat seed.
    seed: Vec<u8>,

    /// Heartbeat vote ID (32 bytes).
    vote_id: Vec<u8>,

    /// Heartbeat key dilution.
    key_dilution: u64,
}

impl From<algokit_transact::HeartbeatTransactionFields> for HeartbeatTransactionFields {
    fn from(tx: algokit_transact::HeartbeatTransactionFields) -> Self {
        Self {
            address: tx.address.as_str(),
            proof: tx.proof.into(),
            seed: tx.seed,
            vote_id: tx.vote_id.to_vec(),
            key_dilution: tx.key_dilution,
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::HeartbeatTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::Heartbeat || tx.heartbeat.is_none() {
            return Err(Self::Error::DecodingError {
                message: "Heartbeat transaction data missing".to_string(),
            });
        }

        let data = tx.heartbeat.clone().unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let sig = vec_to_array::<64>(&data.proof.sig, "heartbeat proof signature")?;
        let pk = vec_to_array::<32>(&data.proof.pk, "heartbeat proof public key")?;
        let pk2 = vec_to_array::<32>(&data.proof.pk2, "heartbeat proof public key 2")?;
        let pk1_sig = vec_to_array::<64>(&data.proof.pk1_sig, "heartbeat proof pk1 signature")?;
        let pk2_sig = vec_to_array::<64>(&data.proof.pk2_sig, "heartbeat proof pk2 signature")?;
        let vote_id = vec_to_array::<32>(&data.vote_id, "heartbeat vote ID")?;

        let proof = algokit_transact::HeartbeatProof {
            sig,
            pk,
            pk2,
            pk1_sig,
            pk2_sig,
        };

        let transaction_fields = algokit_transact::HeartbeatTransactionFields {
            header,
            address: data.address.parse()?,
            proof,
            seed: data.seed,
            vote_id,
            key_dilution: data.key_dilution,
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
        // Test valid heartbeat transaction
        let result = encode_transaction(TestDataMother::heartbeat().transaction.into());
        assert!(result.is_ok());
    }
}
