use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

use crate::constants::{
    ALGORAND_SIGNATURE_BYTE_LENGTH, MAX_LOGIC_SIG_SIZE, MULTISIG_PROGRAM_DOMAIN_SEPARATOR,
    PROGRAM_DOMAIN_SEPARATOR,
};
use crate::traits::Validate;
use crate::utils::{hash, is_empty_signature_opt, is_empty_vec_opt};
use crate::{Address, MultisigSignature};

/// A logic signature, authorizing a transaction with a program rather than a key.
///
/// The program can authorize on its own behalf, in which case the transaction sender is
/// the program's escrow address, or it can be delegated by an account that signs the
/// program. Exactly one of `signature`, `multisignature` or `logic_multisignature` is
/// present for a delegated logic signature; all three are absent for an escrow.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct LogicSignature {
    /// The compiled program bytes.
    #[serde(rename = "l")]
    #[serde_as(as = "Bytes")]
    pub logic: Vec<u8>,

    /// Signature of an account delegating to this program. All-zero encodes as absent.
    #[serde(rename = "sig")]
    #[serde(skip_serializing_if = "is_empty_signature_opt")]
    #[serde(default)]
    #[serde_as(as = "Option<Bytes>")]
    pub signature: Option<[u8; ALGORAND_SIGNATURE_BYTE_LENGTH]>,

    /// Legacy multisig delegation, rejected since consensus v41. Decoded, never produced.
    #[serde(rename = "msig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multisignature: Option<MultisigSignature>,

    /// Multisig delegation, binding the signature to the delegating account.
    #[serde(rename = "lmsig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logic_multisignature: Option<MultisigSignature>,

    /// Arguments made available to the program. Individual arguments may be empty.
    #[serde(rename = "arg")]
    #[serde(skip_serializing_if = "is_empty_vec_opt")]
    #[serde(default)]
    #[serde_as(as = "Option<Vec<Bytes>>")]
    pub args: Option<Vec<Vec<u8>>>,
}

impl LogicSignature {
    /// Creates a logic signature for a program authorizing on its own behalf.
    pub fn new(logic: Vec<u8>) -> Self {
        Self {
            logic,
            ..Default::default()
        }
    }

    /// The escrow address this program authorizes as when not delegated.
    pub fn address(&self) -> Address {
        Address(hash(&program_preimage(&self.logic)))
    }

    /// The bytes an account signs to delegate this program to itself.
    pub fn bytes_to_sign(&self) -> Vec<u8> {
        program_preimage(&self.logic)
    }

    /// The bytes a multisig participant signs to delegate this program.
    ///
    /// Bound to the multisig account address, so a subsignature cannot be replayed as a
    /// single-key delegation by the member who produced it.
    pub fn bytes_to_sign_for_multisig(&self, multisignature: &MultisigSignature) -> Vec<u8> {
        let account: Address = multisignature.clone().into();
        let mut buffer = Vec::with_capacity(
            MULTISIG_PROGRAM_DOMAIN_SEPARATOR.len() + account.as_bytes().len() + self.logic.len(),
        );
        buffer.extend_from_slice(MULTISIG_PROGRAM_DOMAIN_SEPARATOR.as_bytes());
        buffer.extend_from_slice(account.as_bytes());
        buffer.extend_from_slice(&self.logic);
        buffer
    }
}

fn program_preimage(logic: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(PROGRAM_DOMAIN_SEPARATOR.len() + logic.len());
    buffer.extend_from_slice(PROGRAM_DOMAIN_SEPARATOR.as_bytes());
    buffer.extend_from_slice(logic);
    buffer
}

impl Validate for LogicSignature {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.logic.is_empty() {
            errors.push("LogicSig program cannot be empty".to_string());
        }

        if self.logic.len() > MAX_LOGIC_SIG_SIZE {
            errors.push(format!(
                "LogicSig program of {} bytes exceeds the maximum of {} bytes",
                self.logic.len(),
                MAX_LOGIC_SIG_SIZE
            ));
        }

        let delegations = [
            self.signature.is_some(),
            self.multisignature.is_some(),
            self.logic_multisignature.is_some(),
        ]
        .iter()
        .filter(|present| **present)
        .count();

        if delegations > 1 {
            errors.push("LogicSig can carry at most one delegation signature".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{AccountMother, PLACEHOLDER_SIGNATURE};
    use crate::traits::AlgorandMsgpack;
    use crate::transactions::SignedTransaction;
    use pretty_assertions::assert_eq;

    /// Program and address from the js-algorand-sdk test suite.
    const PROGRAM: [u8; 5] = [0x01, 0x20, 0x01, 0x01, 0x22];
    const ESCROW_ADDRESS: &str = "6Z3C3LDVWGMX23BMSYMANACQOSINPFIRF77H7N3AWJZYV6OH6GWTJKVMXY";

    fn multisig() -> MultisigSignature {
        MultisigSignature::from_participants(
            1,
            2,
            vec![AccountMother::account(), AccountMother::neil()],
        )
        .unwrap()
    }

    #[test]
    fn escrow_address_matches_known_vector() {
        let lsig = LogicSignature::new(PROGRAM.to_vec());
        assert_eq!(lsig.address().to_string(), ESCROW_ADDRESS);
    }

    #[test]
    fn escrow_address_hashes_the_delegation_preimage() {
        let lsig = LogicSignature::new(PROGRAM.to_vec());
        assert_eq!(hash(&lsig.bytes_to_sign()), lsig.address().0);
    }

    #[test]
    fn multisig_delegation_binds_the_account_address() {
        let lsig = LogicSignature::new(PROGRAM.to_vec());
        let msig = multisig();
        let account: Address = msig.clone().into();

        let preimage = lsig.bytes_to_sign_for_multisig(&msig);

        assert!(preimage.starts_with(MULTISIG_PROGRAM_DOMAIN_SEPARATOR.as_bytes()));
        assert_eq!(
            &preimage[MULTISIG_PROGRAM_DOMAIN_SEPARATOR.len()..][..32],
            account.as_bytes()
        );
        assert!(preimage.ends_with(&PROGRAM));
        assert_ne!(
            preimage,
            lsig.bytes_to_sign(),
            "multisig delegation must not share a preimage with single-key delegation"
        );
    }

    #[test]
    fn encodes_program_only_when_undelegated() {
        let encoded = rmp_serde::to_vec_named(&LogicSignature::new(PROGRAM.to_vec())).unwrap();
        let value: rmpv::Value = rmp_serde::from_slice(&encoded).unwrap();

        let keys: Vec<String> = match value {
            rmpv::Value::Map(entries) => entries
                .iter()
                .filter_map(|(k, _)| k.as_str().map(str::to_string))
                .collect(),
            other => panic!("expected a msgpack map, got {other:?}"),
        };

        assert_eq!(keys, vec!["l".to_string()]);
    }

    #[test]
    fn empty_args_are_omitted_but_empty_elements_are_kept() {
        let mut lsig = LogicSignature::new(PROGRAM.to_vec());
        lsig.args = Some(vec![]);
        let encoded = rmp_serde::to_vec_named(&lsig).unwrap();
        assert!(!String::from_utf8_lossy(&encoded).contains("arg"));

        lsig.args = Some(vec![vec![], vec![1, 2]]);
        let decoded: LogicSignature =
            rmp_serde::from_slice(&rmp_serde::to_vec_named(&lsig).unwrap()).unwrap();
        assert_eq!(decoded.args, Some(vec![vec![], vec![1, 2]]));
    }

    #[test]
    fn round_trips_within_a_signed_transaction() {
        let mut lsig = LogicSignature::new(PROGRAM.to_vec());
        lsig.args = Some(vec![vec![1], vec![2]]);
        lsig.logic_multisignature = Some(
            multisig()
                .apply_subsignature(AccountMother::account(), PLACEHOLDER_SIGNATURE)
                .unwrap(),
        );

        let signed = SignedTransaction {
            transaction: crate::test_utils::TransactionMother::simple_payment()
                .build()
                .unwrap(),
            signature: None,
            auth_address: None,
            multisignature: None,
            logic_signature: Some(lsig.clone()),
        };

        let decoded = SignedTransaction::decode(&signed.encode().unwrap()).unwrap();
        assert_eq!(decoded.logic_signature, Some(lsig));
    }

    #[test]
    fn rejects_an_empty_program() {
        assert!(LogicSignature::new(vec![]).validate().is_err());
    }

    #[test]
    fn rejects_multiple_delegations() {
        let mut lsig = LogicSignature::new(PROGRAM.to_vec());
        lsig.signature = Some(PLACEHOLDER_SIGNATURE);
        lsig.logic_multisignature = Some(multisig());

        assert!(lsig.validate().is_err());
    }
}
