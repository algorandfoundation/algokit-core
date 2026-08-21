use algokit_crypto::ed25519::{CryptoxideEd25519Keypair, Ed25519Signer};
use algokit_transact::{AlgorandMsgpack, SignedTransaction, Transaction};
use zeroize::Zeroizing;

use crate::error::ComposerError;

/// Sign each transaction with the corresponding 32-byte Ed25519 secret key.
/// `secret_keys[i]` signs `txns[i]`; the two vectors must have equal length.
pub fn sign_transactions(
    txns: Vec<Transaction>,
    secret_keys: Vec<Vec<u8>>,
) -> Result<Vec<SignedTransaction>, ComposerError> {
    if txns.len() != secret_keys.len() {
        return Err(ComposerError::SignerCountMismatch {
            txns: txns.len(),
            keys: secret_keys.len(),
        });
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| ComposerError::Signing {
            message: format!("failed to build tokio runtime: {e}"),
        })?;

    txns.into_iter()
        .zip(secret_keys.into_iter().map(Zeroizing::new))
        .map(|(txn, sk)| sign_transaction(&rt, txn, sk))
        .collect()
}

fn sign_transaction(
    rt: &tokio::runtime::Runtime,
    txn: Transaction,
    secret_key: Zeroizing<Vec<u8>>,
) -> Result<SignedTransaction, ComposerError> {
    let secret_bytes: Zeroizing<[u8; 32]> =
        Zeroizing::new(secret_key.as_slice().try_into().map_err(|_| {
            ComposerError::InvalidByteLength {
                field: "secret_key",
                expected: 32,
                found: secret_key.len(),
            }
        })?);

    let keypair = CryptoxideEd25519Keypair::try_generate(Some(*secret_bytes)).map_err(|e| {
        ComposerError::Signing {
            message: format!("failed to derive keypair from secret key: {e}"),
        }
    })?;

    let encoded = txn.encode()?;
    let signature = rt
        .block_on(keypair.try_sign(&encoded))
        .map_err(|e| ComposerError::Signing { message: e })?;

    Ok(SignedTransaction {
        transaction: txn,
        signature: Some(signature),
        auth_address: None,
        multisignature: None,
        logic_signature: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;
    use pretty_assertions::assert_eq;

    #[test]
    fn sign_transactions_matches_fixture() {
        let fixture = TestDataMother::simple_payment();
        let signed = sign_transactions(
            vec![fixture.transaction.clone()],
            vec![fixture.signing_private_key.to_vec()],
        )
        .unwrap();

        assert_eq!(signed.len(), 1);
        let encoded = signed[0].encode().unwrap();
        assert_eq!(encoded, fixture.signed_bytes);
    }

    #[test]
    fn sign_transactions_signs_each_with_paired_key() {
        let payment = TestDataMother::simple_payment();
        let opt_in = TestDataMother::opt_in_asset_transfer();

        let signed = sign_transactions(
            vec![payment.transaction.clone(), opt_in.transaction.clone()],
            vec![
                payment.signing_private_key.to_vec(),
                opt_in.signing_private_key.to_vec(),
            ],
        )
        .unwrap();

        assert_eq!(signed.len(), 2);
        assert_eq!(signed[0].encode().unwrap(), payment.signed_bytes);
        assert_eq!(signed[1].encode().unwrap(), opt_in.signed_bytes);
    }

    #[test]
    fn sign_transactions_rejects_length_mismatch() {
        let fixture = TestDataMother::simple_payment();
        let err = sign_transactions(vec![fixture.transaction.clone()], vec![]).unwrap_err();
        assert!(matches!(
            err,
            ComposerError::SignerCountMismatch { txns: 1, keys: 0 }
        ));
    }
}
