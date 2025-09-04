mod app_call;
mod asset_config;
mod asset_freeze;
mod key_registration;

use crate::{
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, Address, AlgorandMsgpack, Byte32, EMPTY_SIGNATURE,
    HASH_BYTES_LENGTH, KeyPairAccount, MultisigSignature, MultisigSubsignature, SignedTransaction,
    Transaction, TransactionHeaderBuilder, TransactionId,
    transactions::{AssetTransferTransactionBuilder, PaymentTransactionBuilder},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use convert_case::{Case, Casing};
use ed25519_dalek::{Signer, SigningKey};
use serde::Serialize;
use serde_json::to_writer_pretty;
use std::vec;
use std::{fs::File, str::FromStr};

pub use app_call::AppCallTransactionMother;
pub use asset_config::AssetConfigTransactionMother;
pub use asset_freeze::AssetFreezeTransactionMother;
pub use key_registration::KeyRegistrationTransactionMother;

pub struct TransactionHeaderMother {}
impl TransactionHeaderMother {
    pub fn testnet() -> TransactionHeaderBuilder {
        TransactionHeaderBuilder::default()
            .genesis_id(String::from("testnet-v1.0"))
            .genesis_hash(
                BASE64_STANDARD
                    .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .fee(1000)
            .to_owned()
    }

    pub fn mainnet() -> TransactionHeaderBuilder {
        TransactionHeaderBuilder::default()
            .genesis_id(String::from("mainnet-v1.0"))
            .genesis_hash(
                BASE64_STANDARD
                    .decode("wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .fee(1000)
            .to_owned()
    }

    pub fn simple_testnet() -> TransactionHeaderBuilder {
        Self::testnet()
            .sender(AccountMother::account().address())
            .first_valid(50659540)
            .last_valid(50660540)
            .to_owned()
    }

    pub fn example() -> TransactionHeaderBuilder {
        TransactionHeaderBuilder::default()
            .genesis_id(String::from("example"))
            .genesis_hash(
                BASE64_STANDARD
                    .decode("3r2+nRwL99aTROTiOtPEeUQarv2fATkmNliHqfGxNJA=")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .first_valid(1)
            .last_valid(999)
            .fee(1000)
            .sender(AccountMother::example().address())
            .to_owned()
    }
}

pub struct TransactionMother {}
impl TransactionMother {
    pub fn simple_payment() -> PaymentTransactionBuilder {
        PaymentTransactionBuilder::default()
            .header(TransactionHeaderMother::simple_testnet().build().unwrap())
            .amount(101000)
            .receiver(
                "VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ"
                    .parse()
                    .unwrap(),
            )
            .to_owned()
    }

    pub fn payment_with_note() -> PaymentTransactionBuilder {
        Self::simple_payment()
            .header(
                TransactionHeaderMother::simple_testnet()
                    .note(
                        BASE64_STANDARD
                            .decode("MGFhNTBkMjctYjhmNy00ZDc3LWExZmItNTUxZmQ1NWRmMmJj")
                            .unwrap(),
                    )
                    .to_owned()
                    .build()
                    .unwrap(),
            )
            .to_owned()
    }

    pub fn observed_payment() -> PaymentTransactionBuilder {
        // https://lora.algokit.io/mainnet/transaction/VTADY3NGJGE4DVZ4CKLX43NTEE3C2J4JJANZ5TPBR4OYJ2D4F2CA
        PaymentTransactionBuilder::default()
            .header(
                TransactionHeaderMother::mainnet()
                    .first_valid(51169629)
                    .last_valid(51170629)
                    .sender(
                        "P5IFX3UBXZJPDSLPT4TB4RYACD2XJ74XSNKCF7KMW3P7ZGN4RRE3C2T5WM"
                            .parse()
                            .unwrap(),
                    )
                    .group(
                        BASE64_STANDARD
                            .decode("u8X2MQIAMHmcBUEsoE0ivmGoYxSWU91VbNN8Z+Zb+sk=")
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .amount(53100000)
            .receiver(
                "G6TOB3V7INUMZ5BYFOH52RNMMCZCX3ZCX7JHF3BGIG46PFFZNRPHDCIDIM"
                    .parse()
                    .unwrap(),
            )
            .to_owned()
    }

    pub fn simple_asset_transfer() -> AssetTransferTransactionBuilder {
        AssetTransferTransactionBuilder::default()
            .header(
                TransactionHeaderMother::simple_testnet()
                    .sender(AccountMother::neil().address())
                    .first_valid(51183672)
                    .last_valid(51183872)
                    .build()
                    .unwrap(),
            )
            .asset_id(107686045)
            .amount(1000)
            .receiver(AccountMother::account().address())
            .to_owned()
    }

    pub fn opt_in_asset_transfer() -> AssetTransferTransactionBuilder {
        Self::simple_asset_transfer()
            .amount(0)
            .receiver(AccountMother::neil().address())
            .to_owned()
    }

    pub fn observed_multisig_asset_transfer() -> AssetTransferTransactionBuilder {
        // https://lora.algokit.io/mainnet/transaction/R2FD3AV2NMUIAFAIQKK7YLXMXEF546NTDCUXPBGXQ7MR2DWP2KYQ
        AssetTransferTransactionBuilder::default()
            .header(
                TransactionHeaderMother::mainnet()
                    .sender(
                        "P5VB5V7PE7455UHXZBQ67LR37URRJYPGWNM5GU773FCLJY4EM55YO47QTY"
                            .parse()
                            .unwrap(),
                    )
                    .first_valid(51875497)
                    .last_valid(51876497)
                    .build()
                    .unwrap(),
            )
            .asset_id(849191641)
            .receiver(
                "TR6G2PSAEIWBA7MDQGLKJX3HJBEY3KIH3AD7FXQLKX6FPHFP56ZAVOMUXQ"
                    .parse()
                    .unwrap(),
            )
            .amount(8295000)
            .to_owned()
    }
}

pub struct AccountMother {}
impl AccountMother {
    pub fn zero_address_account() -> KeyPairAccount {
        KeyPairAccount::from_pubkey(&[0; ALGORAND_PUBLIC_KEY_BYTE_LENGTH])
    }

    pub fn account() -> KeyPairAccount {
        "RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q"
            .parse()
            .unwrap()
    }

    pub fn neil() -> KeyPairAccount {
        "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
            .parse()
            .unwrap()
    }

    pub fn nfd_testnet() -> KeyPairAccount {
        "3Y62HTJ4WYSIEKC74XE3F2JFCS7774EN3CYNUHQCEFIN7QBYFAWLKE5MFY"
            .parse()
            .unwrap()
    }

    pub fn example() -> KeyPairAccount {
        "ALGOC4J2BCZ33TCKSSAMV5GAXQBMV3HDCHDBSPRBZRNSR7BM2FFDZRFGXA"
            .parse()
            .unwrap()
    }

    pub fn msig() -> MultisigSignature {
        MultisigSignature::from_participants(
            1,
            2,
            vec![Self::account().into(), Self::example().into()],
        )
        .unwrap()
    }
}

const SIGNING_PRIVATE_KEY: Byte32 = [
    2, 205, 103, 33, 67, 14, 82, 196, 115, 196, 206, 254, 50, 110, 63, 182, 149, 229, 184, 216, 93,
    11, 13, 99, 69, 213, 218, 165, 134, 118, 47, 44,
];

pub struct TransactionGroupMother {}
impl TransactionGroupMother {
    pub fn testnet_payment_group() -> Vec<Transaction> {
        // This is a real TestNet transaction group with two payment transactions.
        let header_builder = TransactionHeaderMother::testnet()
            .sender(AccountMother::neil().address())
            .first_valid(51532821)
            .last_valid(51533021)
            .to_owned();

        let pay_1 = PaymentTransactionBuilder::default()
            .header(
                header_builder
                    .clone()
                    .note(BASE64_STANDARD.decode("VGVzdCAx").unwrap())
                    .build()
                    .unwrap(),
            )
            .receiver(AccountMother::neil().address())
            .amount(1000000)
            .build()
            .unwrap();

        let pay_2: Transaction = PaymentTransactionBuilder::default()
            .header(
                header_builder
                    .clone()
                    .note(BASE64_STANDARD.decode("VGVzdCAy").unwrap())
                    .build()
                    .unwrap(),
            )
            .receiver(AccountMother::neil().address())
            .amount(200000)
            .build()
            .unwrap();

        vec![pay_1, pay_2]
    }

    pub fn group_of(number_of_transactions: usize) -> Vec<Transaction> {
        let header_builder = TransactionHeaderMother::testnet()
            .sender(AccountMother::neil().address())
            .first_valid(51532821)
            .last_valid(51533021)
            .to_owned();

        let mut txs = vec![];
        for i in 0..number_of_transactions {
            let tx: Transaction = PaymentTransactionBuilder::default()
                .header(
                    header_builder
                        .clone()
                        .note(format!("tx:{}", i).as_bytes().to_vec())
                        .build()
                        .unwrap(),
                )
                .receiver(AccountMother::neil().address())
                .amount(200000)
                .build()
                .unwrap();
            txs.push(tx);
        }
        txs
    }
}

#[derive(Serialize)]
pub struct TransactionTestData {
    pub transaction: Transaction,
    pub id: String,
    pub id_raw: Byte32,
    pub unsigned_bytes: Vec<u8>,
    pub signing_private_key: Byte32,
    pub signed_bytes: Vec<u8>,
    pub rekeyed_sender_auth_address: Address,
    pub rekeyed_sender_signed_bytes: Vec<u8>,
    pub multisig_addresses: (Address, Address),
    pub multisig_signed_bytes: Vec<u8>,
}

impl TransactionTestData {
    pub fn new(transaction: Transaction, signing_private_key: Byte32) -> Self {
        let signing_key: SigningKey = SigningKey::from_bytes(&signing_private_key);
        let id: String = transaction.id().unwrap();
        let id_raw: [u8; HASH_BYTES_LENGTH] = transaction.id_raw().unwrap();
        let unsigned_bytes = transaction.encode().unwrap();
        let signature = signing_key.sign(&unsigned_bytes);
        let signed_txn = SignedTransaction {
            transaction: transaction.clone(),
            signature: Some(signature.to_bytes()),
            auth_address: None,
            multisignature: None,
        };
        let signed_bytes = signed_txn.encode().unwrap();

        let rekeyed_sender_auth_address =
            Address::from_str("BKDYDIDVSZCP75JVCB76P3WBJRY6HWAIFDSEOKYHJY5WMNJ2UWJ65MYETU")
                .unwrap();
        let signer_signed_txn = SignedTransaction {
            transaction: transaction.clone(),
            signature: Some(signature.to_bytes()),
            auth_address: Some(rekeyed_sender_auth_address.clone()),
            multisignature: None,
        };
        let rekeyed_sender_signed_bytes = signer_signed_txn.encode().unwrap();

        let multisig_addresses = (
            "424ZV7KBBUJ52DUKP2KLQ6I5GBOHKBXOW7Q7UQIOOYNDWYRM4EKOSMVVRI"
                .parse()
                .unwrap(),
            "NY6DHEEFW73R2NUWY562U2NNKSKBKVYY5OOQFLD3M2II5RUNKRZDEGUGEA"
                .parse()
                .unwrap(),
        );
        let multisig_signature = MultisigSignature {
            version: 1,
            threshold: 2,
            subsignatures: vec![
                MultisigSubsignature {
                    address: multisig_addresses.clone().0,
                    signature: Some(signature.to_bytes()),
                },
                MultisigSubsignature {
                    address: multisig_addresses.clone().1,
                    signature: Some(signature.to_bytes()),
                },
            ],
        };
        let multisig_signed_txn = SignedTransaction {
            transaction: transaction.clone(),
            signature: None,
            auth_address: None,
            multisignature: Some(multisig_signature),
        };
        let multisig_signed_bytes = multisig_signed_txn.encode().unwrap();

        Self {
            transaction,
            id,
            id_raw,
            unsigned_bytes,
            signing_private_key,
            signed_bytes,
            rekeyed_sender_auth_address,
            rekeyed_sender_signed_bytes,
            multisig_addresses,
            multisig_signed_bytes,
        }
    }

    pub fn as_json<F, T>(&self, transform: &Option<F>) -> serde_json::Value
    where
        F: Fn(&Self) -> T,
        T: serde::Serialize,
    {
        match transform {
            Some(f) => serde_json::json!(f(self)),
            None => serde_json::json!(self),
        }
    }
}

pub struct TestDataMother {}

impl TestDataMother {
    pub fn simple_payment() -> TransactionTestData {
        let transaction = TransactionMother::simple_payment().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn simple_asset_transfer() -> TransactionTestData {
        let transaction = TransactionMother::simple_asset_transfer().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn opt_in_asset_transfer() -> TransactionTestData {
        let transaction = TransactionMother::opt_in_asset_transfer().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn app_create() -> TransactionTestData {
        let transaction = AppCallTransactionMother::app_create().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn app_update() -> TransactionTestData {
        let transaction = AppCallTransactionMother::app_update().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn app_delete() -> TransactionTestData {
        let transaction = AppCallTransactionMother::app_delete().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn app_call() -> TransactionTestData {
        let transaction = AppCallTransactionMother::app_call().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn asset_create() -> TransactionTestData {
        let transaction = AssetConfigTransactionMother::asset_create()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn asset_destroy() -> TransactionTestData {
        let transaction = AssetConfigTransactionMother::asset_destroy()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn asset_config() -> TransactionTestData {
        let transaction = AssetConfigTransactionMother::asset_config()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn online_key_registration() -> TransactionTestData {
        let transaction = KeyRegistrationTransactionMother::online_key_registration()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn offline_key_registration() -> TransactionTestData {
        let transaction = KeyRegistrationTransactionMother::offline_key_registration()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn non_participation_key_registration() -> TransactionTestData {
        let transaction = KeyRegistrationTransactionMother::non_participation_key_registration()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn asset_freeze() -> TransactionTestData {
        let signing_private_key: Byte32 = [
            2, 205, 103, 33, 67, 14, 82, 196, 115, 196, 206, 254, 50, 110, 63, 182, 149, 229, 184,
            216, 93, 11, 13, 99, 69, 213, 218, 165, 134, 118, 47, 44,
        ];
        let transaction = AssetFreezeTransactionMother::asset_freeze()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, signing_private_key)
    }

    pub fn asset_unfreeze() -> TransactionTestData {
        let signing_private_key: Byte32 = [
            2, 205, 103, 33, 67, 14, 82, 196, 115, 196, 206, 254, 50, 110, 63, 182, 149, 229, 184,
            216, 93, 11, 13, 99, 69, 213, 218, 165, 134, 118, 47, 44,
        ];
        let transaction = AssetFreezeTransactionMother::asset_unfreeze()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, signing_private_key)
    }

    pub fn export<F, T>(path: &std::path::Path, transform: Option<F>)
    where
        F: Fn(&TransactionTestData) -> T,
        T: serde::Serialize,
    {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create export path directories");
        }

        let test_data = normalise_json(serde_json::json!({
            "simple_payment": Self::simple_payment().as_json(&transform),
            "opt_in_asset_transfer": Self::opt_in_asset_transfer().as_json(&transform),
            "app_create": Self::app_create().as_json(&transform),
            "app_update": Self::app_update().as_json(&transform),
            "app_delete": Self::app_delete().as_json(&transform),
            "app_call": Self::app_call().as_json(&transform),
            "asset_create": Self::asset_create().as_json(&transform),
            "asset_destroy": Self::asset_destroy().as_json(&transform),
            "asset_config": Self::asset_config().as_json(&transform),
            "online_key_registration": Self::online_key_registration().as_json(&transform),
            "offline_key_registration": Self::offline_key_registration().as_json(&transform),
            "non_participation_key_registration": Self::non_participation_key_registration().as_json(&transform),
            "asset_freeze": Self::asset_freeze().as_json(&transform),
            "asset_unfreeze": Self::asset_unfreeze().as_json(&transform),
        }));

        let file = File::create(path).expect("Failed to create export file");
        to_writer_pretty(file, &test_data).expect("Failed to write export JSON");
    }
}

fn normalise_json(value: serde_json::Value) -> serde_json::Value {
    const ZERO_VALUE_EXCLUDED_FIELDS: &[&str] = &[
        "amount",
        "asset_id",
        "app_id",
        "num_byte_slices",
        "num_uints",
    ];

    match value {
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.into_iter()
                .filter(|(k, v)| {
                    !(v.is_null()
                        || v.is_boolean() && v.as_bool() == Some(false)
                        || v.is_number()
                            && v.as_u64() == Some(0)
                            && !ZERO_VALUE_EXCLUDED_FIELDS
                                // Convert to snake case because when building for FFI the field names may be in camelCase
                                .contains(&k.to_case(Case::Snake).as_str()))
                })
                .map(|(k, v)| (k.to_case(Case::Camel), normalise_json(v)))
                .collect(),
        ),
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(normalise_json).collect())
        }
        other => other,
    }
}

pub fn check_transaction_encoding(tx: &Transaction, expected_encoded_len: usize) {
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, *tx);

    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some(EMPTY_SIGNATURE),
        auth_address: None,
        multisignature: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, *tx);

    let raw_encoded = tx.encode_raw().unwrap();
    assert_eq!(encoded[0], b'T');
    assert_eq!(encoded[1], b'X');
    assert_eq!(encoded.len(), raw_encoded.len() + 2);
    assert_eq!(encoded[2..], raw_encoded);
    assert_eq!(encoded.len(), expected_encoded_len);
}

pub fn check_signed_transaction_encoding(
    tx: &Transaction,
    expected_encoded_len: usize,
    auth_account: Option<KeyPairAccount>,
) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some(EMPTY_SIGNATURE),
        auth_address: auth_account.map(|acc| acc.address()),
        multisignature: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), expected_encoded_len);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
}

pub fn check_multisigned_transaction_encoding(tx: &Transaction, expected_encoded_len: usize) {
    let unsigned_multisignature = MultisigSignature::from_participants(
        1,
        2,
        vec![
            AccountMother::account().address(),
            AccountMother::neil().address(),
        ],
    )
    .unwrap();
    let multisignature_0 = unsigned_multisignature
        .apply_subsignature(AccountMother::account().address(), EMPTY_SIGNATURE)
        .unwrap();
    let multisignature_1 = unsigned_multisignature
        .apply_subsignature(AccountMother::neil().address(), EMPTY_SIGNATURE)
        .unwrap();
    let multisignature = Some(multisignature_0.merge(&multisignature_1).unwrap());
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: None,
        auth_address: None,
        multisignature,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), expected_encoded_len);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
}

pub fn check_transaction_id(tx: &Transaction, expected_tx_id: &str) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some(EMPTY_SIGNATURE),
        auth_address: None,
        multisignature: None,
    };

    assert_eq!(tx.id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.id().unwrap(), expected_tx_id);
}
