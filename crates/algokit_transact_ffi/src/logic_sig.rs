use algokit_transact::Validate;
use ffi_macros::{ffi_func, ffi_record};
use serde::{Deserialize, Serialize};

use crate::{AlgoKitTransactError, MultisigSignature, vec_to_array};

/// Representation of an Algorand logic signature.
#[ffi_record]
pub struct LogicSignature {
    /// The compiled program bytes.
    pub logic: Vec<u8>,
    /// Signature of an account delegating to this program.
    pub signature: Option<Vec<u8>>,
    /// Legacy multisig delegation, rejected by the network since consensus v41.
    pub multisignature: Option<MultisigSignature>,
    /// Multisig delegation, binding the signature to the delegating account.
    pub logic_multisignature: Option<MultisigSignature>,
    /// Arguments made available to the program.
    pub args: Option<Vec<Vec<u8>>>,
}

impl From<algokit_transact::LogicSignature> for LogicSignature {
    fn from(lsig: algokit_transact::LogicSignature) -> Self {
        Self {
            logic: lsig.logic,
            signature: lsig.signature.map(|sig| sig.into()),
            multisignature: lsig.multisignature.map(Into::into),
            logic_multisignature: lsig.logic_multisignature.map(Into::into),
            args: lsig.args,
        }
    }
}

impl TryFrom<LogicSignature> for algokit_transact::LogicSignature {
    type Error = AlgoKitTransactError;

    fn try_from(lsig: LogicSignature) -> Result<Self, Self::Error> {
        Ok(Self {
            logic: lsig.logic,
            signature: lsig
                .signature
                .map(|sig| vec_to_array(&sig, "signature"))
                .transpose()
                .map_err(|e| AlgoKitTransactError::DecodingError {
                    error_msg: format!("Error while decoding a logic signature signature: {}", e),
                })?,
            multisignature: lsig.multisignature.map(TryInto::try_into).transpose()?,
            logic_multisignature: lsig
                .logic_multisignature
                .map(TryInto::try_into)
                .transpose()?,
            args: lsig.args,
        })
    }
}

/// Returns the escrow address a program authorizes as when it is not delegated.
#[ffi_func]
pub fn get_logic_signature_address(logic: Vec<u8>) -> Result<String, AlgoKitTransactError> {
    Ok(algokit_transact::LogicSignature::new(logic)
        .address()
        .to_string())
}

/// Returns the bytes an account signs to delegate a program to itself.
#[ffi_func]
pub fn get_logic_signature_bytes_to_sign(logic: Vec<u8>) -> Result<Vec<u8>, AlgoKitTransactError> {
    Ok(algokit_transact::LogicSignature::new(logic).bytes_to_sign())
}

/// Returns the bytes a multisig participant signs to delegate a program.
#[ffi_func]
pub fn get_logic_signature_bytes_to_sign_for_multisig(
    logic: Vec<u8>,
    multisignature: MultisigSignature,
) -> Result<Vec<u8>, AlgoKitTransactError> {
    let msig: algokit_transact::MultisigSignature = multisignature.try_into()?;
    Ok(algokit_transact::LogicSignature::new(logic).bytes_to_sign_for_multisig(&msig))
}

/// Validates a logic signature, returning the reasons it is not well formed.
#[ffi_func]
pub fn validate_logic_signature(
    logic_signature: LogicSignature,
) -> Result<(), AlgoKitTransactError> {
    let lsig: algokit_transact::LogicSignature = logic_signature.try_into()?;
    lsig.validate()
        .map_err(|errors| AlgoKitTransactError::DecodingError {
            error_msg: errors.join("; "),
        })
}
