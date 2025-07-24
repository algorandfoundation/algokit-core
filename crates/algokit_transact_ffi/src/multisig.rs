use crate::*;

/// Representation of an Algorand multisignature signature.
#[ffi_record]
pub struct MultisigSignature {
    /// Multisig version.
    version: u8,
    /// Minimum number of signatures required.
    threshold: u8,
    /// List of subsignatures for each participant.
    subsignatures: Vec<MultisigSubsignature>,
}

/// Representation of a single subsignature in a multisignature transaction.
///
/// Each subsignature contains the participant's address and an optional signature.
#[ffi_record]
pub struct MultisigSubsignature {
    /// Address of the participant.
    address: String,
    /// Optional signature bytes for the participant.
    signature: Option<ByteBuf>,
}

impl From<algokit_transact::MultisigSignature> for MultisigSignature {
    fn from(value: algokit_transact::MultisigSignature) -> Self {
        Self {
            version: value.version,
            threshold: value.threshold,
            subsignatures: value.subsignatures.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<MultisigSignature> for algokit_transact::MultisigSignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: MultisigSignature) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.version,
            value.threshold,
            value
                .subsignatures
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        )?)
    }
}

impl From<algokit_transact::MultisigSubsignature> for MultisigSubsignature {
    fn from(value: algokit_transact::MultisigSubsignature) -> Self {
        Self {
            address: value.address.as_str(),
            signature: value.signature.map(|sig| sig.to_vec().into()),
        }
    }
}

impl TryFrom<MultisigSubsignature> for algokit_transact::MultisigSubsignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: MultisigSubsignature) -> Result<Self, Self::Error> {
        let address = value.address.parse()?;

        Ok(Self {
            address,
            signature: value
                .signature
                .map(|sig| bytebuf_to_bytes(&sig))
                .transpose()
                .map_err(|e| {
                    AlgoKitTransactError::DecodingError(format!(
                        "Error while decoding a subsignature: {}",
                        e
                    ))
                })?,
        })
    }
}

/// Creates an empty multisignature signature from a list of participant addresses.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if any address is invalid or the multisignature parameters are invalid.
#[ffi_func]
pub fn new_multisig_signature(
    version: u8,
    threshold: u8,
    participants: Vec<String>,
) -> Result<MultisigSignature, AlgoKitTransactError> {
    Ok(algokit_transact::MultisigSignature::from_participants(
        version,
        threshold,
        participants
            .into_iter()
            .map(|addr| addr.parse())
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map(Into::into)?)
}

/// Returns the list of participant addresses from a multisignature signature.
///
/// # Errors
/// Returns [`AlgoKitTransactError`] if the multisignature is invalid.
#[ffi_func]
pub fn participants_from_multisig_signature(
    multisig_signature: MultisigSignature,
) -> Result<Vec<String>, AlgoKitTransactError> {
    let multisig: algokit_transact::MultisigSignature = multisig_signature.try_into()?;
    Ok(multisig
        .participants()
        .into_iter()
        .map(|addr| addr.to_string())
        .collect())
}

/// Returns the address of the multisignature account.
///
/// # Errors
/// /// Returns [`AlgoKitTransactError`] if the multisignature signature is invalid or the address cannot be derived.
#[ffi_func]
pub fn address_from_multisig_signature(
    multisig_signature: MultisigSignature,
) -> Result<String, AlgoKitTransactError> {
    let multisig: algokit_transact::MultisigSignature = multisig_signature.try_into()?;
    Ok(multisig.to_string())
}

/// Applies a subsignature for a participant to a multisignature signature, replacing any existing signature.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if the participant address is invalid or not found, or if the signature bytes are invalid.
#[ffi_func]
pub fn apply_multisig_subsignature(
    multisig_signature: MultisigSignature,
    participant: String,
    subsignature: &[u8],
) -> Result<MultisigSignature, AlgoKitTransactError> {
    let multisignature: algokit_transact::MultisigSignature = multisig_signature.try_into()?;
    let partially_signed_multisignature = multisignature.apply_subsignature(
        participant.parse()?,
        subsignature.try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError(format!(
                "signature should be {} bytes",
                ALGORAND_SIGNATURE_BYTE_LENGTH
            ))
        })?,
    )?;
    Ok(partially_signed_multisignature.into())
}

/// Merges two multisignature signatures, replacing signatures in the first with those from the second where present.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if the multisignature parameters or participants do not match.
#[ffi_func]
pub fn merge_multisignatures(
    multisig_signature_a: MultisigSignature,
    multisig_signature_b: MultisigSignature,
) -> Result<MultisigSignature, AlgoKitTransactError> {
    let multisig_a: algokit_transact::MultisigSignature = multisig_signature_a.try_into()?;
    let multisig_b: algokit_transact::MultisigSignature = multisig_signature_b.try_into()?;
    let merged_multisig = multisig_a.merge(&multisig_b)?;
    Ok(merged_multisig.into())
}

#[cfg(test)]
mod tests {
    use algokit_transact::AlgorandMsgpack;
    use algokit_transact::test_utils::TransactionMother;
    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;

    #[test]
    fn test_multisig_transaction_matches_observed_transaction() {
        let observed_txn = TransactionMother::observed_multisig_asset_transfer()
            .build()
            .unwrap();

        let observed_signed_txn = algokit_transact::SignedTransaction {
            transaction: observed_txn.clone(),
            signature: None,
            auth_address: None,
            multisignature: Some(
                algokit_transact::MultisigSignature::new(
                    1,
                    2,
                    vec![
                        algokit_transact::MultisigSubsignature {
                            address: "AXJVIQR43APV5HZ6F3J4MYNYR3GRRFHU56WTRFLJXFNNUJHDAX5SCGF3SQ"
                                .parse()
                                .unwrap(),
                            signature: Some(BASE64_STANDARD.decode("H0W1kLRR68uDwacLk0N7qPuvm4NP09AmiaG+X6HPdsZOCJ5YV5ytc+jCvonAEz2sg+0k388T9ZAbqSZGag93Cg==").unwrap().try_into().unwrap()),
                        },
                        algokit_transact::MultisigSubsignature {
                            address: "QKR2CYWG4MQQAYCAF4LQARVQLLUF2JIDQO42OQ5YN2E7CHTLDURSJGNQRU"
                                .parse()
                                .unwrap(),
                            signature: Some(BASE64_STANDARD.decode("UzvbTgDEfdG6w/HzaiwMePmNLiIk5z+hK4EZoCLR9ghgYMxy0IdS7iTCvPVFmVTDYM+r/W8Lox+lE6m4N/OvCw==").unwrap().try_into().unwrap()),
                        },
                    ],
                )
                    .unwrap(),
            ),
        };
        assert_eq!(
            observed_signed_txn.encode().unwrap(),
            [
                130, 164, 109, 115, 105, 103, 131, 166, 115, 117, 98, 115, 105, 103, 146, 130, 162,
                112, 107, 196, 32, 5, 211, 84, 66, 60, 216, 31, 94, 159, 62, 46, 211, 198, 97, 184,
                142, 205, 24, 148, 244, 239, 173, 56, 149, 105, 185, 90, 218, 36, 227, 5, 251, 161,
                115, 196, 64, 31, 69, 181, 144, 180, 81, 235, 203, 131, 193, 167, 11, 147, 67, 123,
                168, 251, 175, 155, 131, 79, 211, 208, 38, 137, 161, 190, 95, 161, 207, 118, 198,
                78, 8, 158, 88, 87, 156, 173, 115, 232, 194, 190, 137, 192, 19, 61, 172, 131, 237,
                36, 223, 207, 19, 245, 144, 27, 169, 38, 70, 106, 15, 119, 10, 130, 162, 112, 107,
                196, 32, 130, 163, 161, 98, 198, 227, 33, 0, 96, 64, 47, 23, 0, 70, 176, 90, 232,
                93, 37, 3, 131, 185, 167, 67, 184, 110, 137, 241, 30, 107, 29, 35, 161, 115, 196,
                64, 83, 59, 219, 78, 0, 196, 125, 209, 186, 195, 241, 243, 106, 44, 12, 120, 249,
                141, 46, 34, 36, 231, 63, 161, 43, 129, 25, 160, 34, 209, 246, 8, 96, 96, 204, 114,
                208, 135, 82, 238, 36, 194, 188, 245, 69, 153, 84, 195, 96, 207, 171, 253, 111, 11,
                163, 31, 165, 19, 169, 184, 55, 243, 175, 11, 163, 116, 104, 114, 2, 161, 118, 1,
                163, 116, 120, 110, 138, 164, 97, 97, 109, 116, 206, 0, 126, 146, 88, 164, 97, 114,
                99, 118, 196, 32, 156, 124, 109, 62, 64, 34, 44, 16, 125, 131, 129, 150, 164, 223,
                103, 72, 73, 141, 169, 7, 216, 7, 242, 222, 11, 85, 252, 87, 156, 175, 239, 178,
                163, 102, 101, 101, 205, 3, 232, 162, 102, 118, 206, 3, 23, 142, 169, 163, 103,
                101, 110, 172, 109, 97, 105, 110, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103,
                104, 196, 32, 192, 97, 196, 216, 252, 29, 189, 222, 210, 215, 96, 75, 228, 86, 142,
                63, 109, 4, 25, 135, 172, 55, 189, 228, 182, 32, 181, 171, 57, 36, 138, 223, 162,
                108, 118, 206, 3, 23, 146, 145, 163, 115, 110, 100, 196, 32, 127, 106, 30, 215,
                239, 39, 249, 222, 208, 247, 200, 97, 239, 174, 59, 253, 35, 20, 225, 230, 179, 89,
                211, 83, 255, 217, 68, 180, 227, 132, 103, 123, 164, 116, 121, 112, 101, 165, 97,
                120, 102, 101, 114, 164, 120, 97, 105, 100, 206, 50, 157, 162, 217
            ]
        );
    }
}
