use algokit_transact::{
    Address, AssetTransferTransactionFields, KeyRegistrationTransactionFields,
    PaymentTransactionFields, Transaction, TransactionHeader, Transactions,
};
use snafu::ResultExt;

use crate::error::{ComposerError, InvalidAddressSnafu};
use crate::params::{
    AssetOptInParams, AssetTransferParams, CommonTxnParams, OfflineKeyRegParams,
    OnlineKeyRegParams, PaymentParams, TxnParams,
};

/// Network-suggested parameters fetched from algod.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuggestedParams {
    /// Fee in microALGO. Used as-is when `flat_fee` is true, else a per-byte rate.
    pub fee: u64,
    pub flat_fee: bool,
    pub first_round_valid: u64,
    pub last_round_valid: u64,
    /// 32-byte genesis hash that pins the transaction to a specific chain.
    pub genesis_hash: Vec<u8>,
    pub genesis_id: String,
}

/// Composer-level defaults applied to every TxnParams unless an individual entry overrides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerParams {
    pub suggested_params: SuggestedParams,
    /// Number of rounds a transaction is valid by default when an individual TxnParams does not specify a validity window.
    pub default_validity_window: Option<u64>,
}

/// Build transactions from the supplied TxnParams list using the composer-level defaults.
///
/// When the list contains more than one entry, the returned transactions will be atomically
/// grouped (shared group ID assigned). A single-entry list is returned ungrouped.
pub fn compose(
    txn_params: Vec<TxnParams>,
    composer_params: ComposerParams,
) -> Result<Vec<Transaction>, ComposerError> {
    let txns: Vec<Transaction> = txn_params
        .into_iter()
        .map(|p| compose_transaction(p, &composer_params))
        .collect::<Result<Vec<_>, _>>()?;

    if txns.len() > 1 {
        Ok(txns.as_slice().assign_group()?)
    } else {
        Ok(txns)
    }
}

fn compose_transaction(
    p: TxnParams,
    composer: &ComposerParams,
) -> Result<Transaction, ComposerError> {
    match p {
        TxnParams::Payment(payment) => compose_payment(payment, composer),
        TxnParams::AssetTransfer(transfer) => compose_asset_transfer(transfer, composer),
        TxnParams::AssetOptIn(opt_in) => compose_asset_opt_in(opt_in, composer),
        TxnParams::OnlineKeyReg(keyreg) => compose_online_key_reg(keyreg, composer),
        TxnParams::OfflineKeyReg(keyreg) => compose_offline_key_reg(keyreg, composer),
    }
}

fn compose_payment(
    p: PaymentParams,
    composer: &ComposerParams,
) -> Result<Transaction, ComposerError> {
    let header = build_header(&p.common, composer)?;
    let receiver = parse_address(&p.receiver)?;
    let close_remainder_to = p
        .close_remainder_to
        .as_deref()
        .map(parse_address)
        .transpose()?;

    Ok(Transaction::Payment(PaymentTransactionFields {
        header,
        receiver,
        amount: p.amount,
        close_remainder_to,
    }))
}

fn compose_asset_transfer(
    p: AssetTransferParams,
    composer: &ComposerParams,
) -> Result<Transaction, ComposerError> {
    let header = build_header(&p.common, composer)?;
    let receiver = parse_address(&p.receiver)?;
    let asset_sender = p
        .clawback_target
        .as_deref()
        .map(parse_address)
        .transpose()?;
    let close_remainder_to = p.close_asset_to.as_deref().map(parse_address).transpose()?;

    Ok(Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: p.asset_id,
        amount: p.amount,
        receiver,
        asset_sender,
        close_remainder_to,
    }))
}

fn compose_asset_opt_in(
    p: AssetOptInParams,
    composer: &ComposerParams,
) -> Result<Transaction, ComposerError> {
    let header = build_header(&p.common, composer)?;
    let account = header.sender.clone();

    Ok(Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: p.asset_id,
        amount: 0,
        receiver: account,
        asset_sender: None,
        close_remainder_to: None,
    }))
}

fn compose_online_key_reg(
    p: OnlineKeyRegParams,
    composer: &ComposerParams,
) -> Result<Transaction, ComposerError> {
    let header = build_header(&p.common, composer)?;
    let vote_key = bytes_to_fixed::<32>(&p.vote_key, "vote_key")?;
    let selection_key = bytes_to_fixed::<32>(&p.selection_key, "selection_key")?;
    let state_proof_key = bytes_to_fixed::<64>(&p.state_proof_key, "state_proof_key")?;

    Ok(Transaction::KeyRegistration(
        KeyRegistrationTransactionFields {
            header,
            vote_key: Some(vote_key),
            selection_key: Some(selection_key),
            state_proof_key: Some(state_proof_key),
            vote_first: Some(p.vote_first),
            vote_last: Some(p.vote_last),
            vote_key_dilution: Some(p.vote_key_dilution),
            non_participation: None,
        },
    ))
}

fn compose_offline_key_reg(
    p: OfflineKeyRegParams,
    composer: &ComposerParams,
) -> Result<Transaction, ComposerError> {
    let header = build_header(&p.common, composer)?;

    Ok(Transaction::KeyRegistration(
        KeyRegistrationTransactionFields {
            header,
            vote_key: None,
            selection_key: None,
            state_proof_key: None,
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            non_participation: None,
        },
    ))
}

fn build_header(
    c: &CommonTxnParams,
    composer: &ComposerParams,
) -> Result<TransactionHeader, ComposerError> {
    let sp = &composer.suggested_params;

    let first_valid = c.first_valid_round.unwrap_or(sp.first_round_valid);
    let last_valid = c
        .last_valid_round
        .or_else(|| c.validity_window.map(|w| first_valid + w))
        .or_else(|| composer.default_validity_window.map(|w| first_valid + w))
        .unwrap_or(sp.last_round_valid);

    let fee = c.static_fee.or(Some(sp.fee));

    let genesis_hash = bytes_to_fixed::<32>(&sp.genesis_hash, "genesis_hash")?;

    let lease = c
        .lease
        .as_ref()
        .map(|v| bytes_to_fixed::<32>(v, "lease"))
        .transpose()?;

    let rekey_to = c.rekey_to.as_deref().map(parse_address).transpose()?;

    Ok(TransactionHeader {
        sender: parse_address(&c.sender)?,
        fee,
        first_valid,
        last_valid,
        genesis_hash: Some(genesis_hash),
        genesis_id: Some(sp.genesis_id.clone()),
        note: c.note.clone(),
        rekey_to,
        lease,
        group: None,
    })
}

fn parse_address(s: &str) -> Result<Address, ComposerError> {
    s.parse::<Address>().context(InvalidAddressSnafu {
        address: s.to_string(),
    })
}

fn bytes_to_fixed<const N: usize>(
    bytes: &[u8],
    field: &'static str,
) -> Result<[u8; N], ComposerError> {
    bytes
        .try_into()
        .map_err(|_| ComposerError::InvalidByteLength {
            field,
            expected: N,
            found: bytes.len(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{TransactionGroupMother, TransactionMother};
    use pretty_assertions::assert_eq;

    use crate::params::{
        AssetOptInParams, AssetTransferParams, CommonTxnParams, OfflineKeyRegParams,
        OnlineKeyRegParams, PaymentParams, TxnParams,
    };

    const TESTNET_GENESIS_ID: &str = "testnet-v1.0";

    fn common_from(header: &TransactionHeader) -> CommonTxnParams {
        CommonTxnParams {
            sender: header.sender.to_string(),
            note: header.note.clone(),
            lease: None,
            rekey_to: None,
            static_fee: header.fee,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
        }
    }

    fn composer_params_from(header: &TransactionHeader) -> ComposerParams {
        ComposerParams {
            suggested_params: SuggestedParams {
                fee: header.fee.unwrap_or(0),
                flat_fee: true,
                first_round_valid: header.first_valid,
                last_round_valid: header.last_valid,
                genesis_hash: header.genesis_hash.unwrap().to_vec(),
                genesis_id: header.genesis_id.clone().unwrap(),
            },
            default_validity_window: None,
        }
    }

    #[test]
    fn compose_payment_matches_fixture() {
        let expected = TransactionMother::simple_payment().build().unwrap();
        let payment = match &expected {
            Transaction::Payment(p) => p.clone(),
            _ => panic!("expected Payment fixture"),
        };

        let composer_params = composer_params_from(&payment.header);
        let params = TxnParams::Payment(PaymentParams {
            common: common_from(&payment.header),
            receiver: payment.receiver.to_string(),
            amount: payment.amount,
            close_remainder_to: None,
        });

        let composed = compose(vec![params], composer_params).unwrap();
        assert_eq!(composed, vec![expected]);
    }

    #[test]
    fn compose_asset_transfer_matches_fixture() {
        let expected = TransactionMother::simple_asset_transfer().build().unwrap();
        let transfer = match &expected {
            Transaction::AssetTransfer(t) => t.clone(),
            _ => panic!("expected AssetTransfer fixture"),
        };

        let composer_params = composer_params_from(&transfer.header);
        let params = TxnParams::AssetTransfer(AssetTransferParams {
            common: common_from(&transfer.header),
            asset_id: transfer.asset_id,
            receiver: transfer.receiver.to_string(),
            amount: transfer.amount,
            clawback_target: None,
            close_asset_to: None,
        });

        let composed = compose(vec![params], composer_params).unwrap();
        assert_eq!(composed, vec![expected]);
    }

    #[test]
    fn compose_asset_opt_in_matches_fixture() {
        let expected = TransactionMother::opt_in_asset_transfer().build().unwrap();
        let transfer = match &expected {
            Transaction::AssetTransfer(t) => t.clone(),
            _ => panic!("expected AssetTransfer fixture"),
        };

        let composer_params = composer_params_from(&transfer.header);
        let params = TxnParams::AssetOptIn(AssetOptInParams {
            common: common_from(&transfer.header),
            asset_id: transfer.asset_id,
        });

        let composed = compose(vec![params], composer_params).unwrap();
        assert_eq!(composed, vec![expected]);
    }

    #[test]
    fn compose_online_key_reg_matches_fixture() {
        // The keyreg fixtures strip genesis_id as a byte-optimization; the composer
        // always populates it from SuggestedParams, so reinstate it on the expected
        // value before comparing.
        let mut expected = algokit_transact::test_utils::KeyRegistrationTransactionMother::online_key_registration().build().unwrap();
        expected.header_mut().genesis_id = Some(TESTNET_GENESIS_ID.to_string());
        let keyreg = match &expected {
            Transaction::KeyRegistration(k) => k.clone(),
            _ => panic!("expected KeyRegistration fixture"),
        };

        let mut composer_params = composer_params_from(&keyreg.header);
        composer_params.suggested_params.genesis_id = TESTNET_GENESIS_ID.to_string();

        let params = TxnParams::OnlineKeyReg(OnlineKeyRegParams {
            common: common_from(&keyreg.header),
            vote_key: keyreg.vote_key.unwrap().to_vec(),
            selection_key: keyreg.selection_key.unwrap().to_vec(),
            state_proof_key: keyreg.state_proof_key.unwrap().to_vec(),
            vote_first: keyreg.vote_first.unwrap(),
            vote_last: keyreg.vote_last.unwrap(),
            vote_key_dilution: keyreg.vote_key_dilution.unwrap(),
        });

        let composed = compose(vec![params], composer_params).unwrap();
        assert_eq!(composed, vec![expected]);
    }

    #[test]
    fn compose_offline_key_reg_matches_fixture() {
        let mut expected = algokit_transact::test_utils::KeyRegistrationTransactionMother::offline_key_registration().build().unwrap();
        expected.header_mut().genesis_id = Some(TESTNET_GENESIS_ID.to_string());
        let keyreg = match &expected {
            Transaction::KeyRegistration(k) => k.clone(),
            _ => panic!("expected KeyRegistration fixture"),
        };

        let mut composer_params = composer_params_from(&keyreg.header);
        composer_params.suggested_params.genesis_id = TESTNET_GENESIS_ID.to_string();

        let params = TxnParams::OfflineKeyReg(OfflineKeyRegParams {
            common: common_from(&keyreg.header),
        });

        let composed = compose(vec![params], composer_params).unwrap();
        assert_eq!(composed, vec![expected]);
    }

    #[test]
    fn compose_multi_entry_matches_grouped_fixture() {
        // TransactionGroupMother gives an ungrouped pair; calling assign_group on
        // its slice produces the canonical grouped output that compose() must match.
        let ungrouped = TransactionGroupMother::testnet_payment_group();
        let expected: Vec<Transaction> = ungrouped.as_slice().assign_group().unwrap();

        let composer_params = composer_params_from(expected[0].header());

        let txn_params: Vec<TxnParams> = expected
            .iter()
            .map(|tx| match tx {
                Transaction::Payment(p) => TxnParams::Payment(PaymentParams {
                    common: common_from(&p.header),
                    receiver: p.receiver.to_string(),
                    amount: p.amount,
                    close_remainder_to: None,
                }),
                _ => panic!("expected only Payment entries in the fixture group"),
            })
            .collect();

        let composed = compose(txn_params, composer_params).unwrap();
        assert_eq!(composed, expected);

        // Both entries share the same non-empty group bytes.
        let group = composed[0].header().group;
        assert!(
            group.is_some(),
            "group must be assigned on multi-entry compose"
        );
        assert_eq!(group, composed[1].header().group);
    }
}
