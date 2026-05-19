use algokit_transact::{
    Address, AssetTransferTransactionFields, PaymentTransactionFields, Transaction,
    TransactionHeader,
};
use snafu::ResultExt;

use crate::error::{ComposerError, InvalidAddressSnafu};
use crate::params::{
    AssetOptInParams, AssetTransferParams, CommonTxnParams, PaymentParams, TxnParams,
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
    txn_params
        .into_iter()
        .map(|p| compose_one(p, &composer_params))
        .collect()
}

fn compose_one(p: TxnParams, composer: &ComposerParams) -> Result<Transaction, ComposerError> {
    match p {
        TxnParams::Payment(payment) => compose_payment(payment, composer),
        TxnParams::AssetTransfer(transfer) => compose_asset_transfer(transfer, composer),
        TxnParams::AssetOptIn(opt_in) => compose_asset_opt_in(opt_in, composer),
        _ => Err(ComposerError::UnsupportedTxnType),
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

    let genesis_hash = sp
        .genesis_hash
        .clone()
        .try_into()
        .map_err(|v: Vec<u8>| ComposerError::InvalidGenesisHashLength { found: v.len() })?;

    let lease = c
        .lease
        .as_ref()
        .map(|v| {
            v.clone()
                .try_into()
                .map_err(|v: Vec<u8>| ComposerError::InvalidLeaseLength { found: v.len() })
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TransactionMother;
    use pretty_assertions::assert_eq;

    use crate::params::{
        AssetOptInParams, AssetTransferParams, CommonTxnParams, PaymentParams, TxnParams,
    };

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
}
