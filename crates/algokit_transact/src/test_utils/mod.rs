mod application_call;
mod asset_config;

use crate::{
    transactions::{AssetTransferTransactionBuilder, PaymentTransactionBuilder},
    Address, AlgorandMsgpack, Byte32, SignedTransaction, Transaction, TransactionHeaderBuilder,
    TransactionId, ALGORAND_PUBLIC_KEY_BYTE_LENGTH, HASH_BYTES_LENGTH,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use convert_case::{Case, Casing};
use ed25519_dalek::{Signer, SigningKey};
use serde::Serialize;
use serde_json::to_writer_pretty;
use std::vec;
use std::{fs::File, str::FromStr};

pub use application_call::ApplicationCallTransactionMother;
pub use asset_config::AssetConfigTransactionMother;

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
            .sender(AddressMother::address())
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
            .sender(AddressMother::example())
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
                    .sender(AddressMother::neil())
                    .first_valid(51183672)
                    .last_valid(51183872)
                    .build()
                    .unwrap(),
            )
            .asset_id(107686045)
            .amount(1000)
            .receiver(AddressMother::address())
            .to_owned()
    }

    pub fn opt_in_asset_transfer() -> AssetTransferTransactionBuilder {
        Self::simple_asset_transfer()
            .amount(0)
            .receiver(AddressMother::neil())
            .to_owned()
    }
}

pub struct AddressMother {}
impl AddressMother {
    pub fn zero_address() -> Address {
        Address::from_pubkey(&[0; ALGORAND_PUBLIC_KEY_BYTE_LENGTH])
    }

    pub fn address() -> Address {
        "RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q"
            .parse()
            .unwrap()
    }

    pub fn neil() -> Address {
        "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
            .parse()
            .unwrap()
    }

    pub fn nfd_testnet() -> Address {
        "3Y62HTJ4WYSIEKC74XE3F2JFCS7774EN3CYNUHQCEFIN7QBYFAWLKE5MFY"
            .parse()
            .unwrap()
    }

    pub fn example() -> Address {
        "ALGOC4J2BCZ33TCKSSAMV5GAXQBMV3HDCHDBSPRBZRNSR7BM2FFDZRFGXA"
            .parse()
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
            .sender(AddressMother::neil())
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
            .receiver(AddressMother::neil())
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
            .receiver(AddressMother::neil())
            .amount(200000)
            .build()
            .unwrap();

        vec![pay_1, pay_2]
    }

    pub fn group_of(number_of_transactions: usize) -> Vec<Transaction> {
        let header_builder = TransactionHeaderMother::testnet()
            .sender(AddressMother::neil())
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
                .receiver(AddressMother::neil())
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
        };
        let signed_bytes = signed_txn.encode().unwrap();

        let rekeyed_sender_auth_address =
            Address::from_str("BKDYDIDVSZCP75JVCB76P3WBJRY6HWAIFDSEOKYHJY5WMNJ2UWJ65MYETU")
                .unwrap();
        let signer_signed_txn = SignedTransaction {
            transaction: transaction.clone(),
            signature: Some(signature.to_bytes()),
            auth_address: Some(rekeyed_sender_auth_address.clone()),
        };
        let rekeyed_sender_signed_bytes = signer_signed_txn.encode().unwrap();

        Self {
            transaction,
            id,
            id_raw,
            unsigned_bytes,
            signing_private_key,
            signed_bytes,
            rekeyed_sender_auth_address,
            rekeyed_sender_signed_bytes,
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

    pub fn application_create() -> TransactionTestData {
        let transaction = ApplicationCallTransactionMother::application_create()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn application_update() -> TransactionTestData {
        let transaction = ApplicationCallTransactionMother::application_update()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn application_delete() -> TransactionTestData {
        let transaction = ApplicationCallTransactionMother::application_delete()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn application_call() -> TransactionTestData {
        let transaction = ApplicationCallTransactionMother::application_call()
            .build()
            .unwrap();
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

    pub fn asset_reconfigure() -> TransactionTestData {
        let transaction = AssetConfigTransactionMother::asset_reconfigure()
            .build()
            .unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
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
            "application_create": Self::application_create().as_json(&transform),
            "application_update": Self::application_update().as_json(&transform),
            "application_delete": Self::application_delete().as_json(&transform),
            "application_call": Self::application_call().as_json(&transform),
            "asset_create": Self::asset_create().as_json(&transform),
            "asset_destroy": Self::asset_destroy().as_json(&transform),
            "asset_reconfigure": Self::asset_reconfigure().as_json(&transform),
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
                                // Convert to snake case because when building for FFI WASM the field names are in camelCase
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_payment_snapshot() {
        let data = TestDataMother::simple_payment();
        assert_eq!(
            data.id,
            String::from("TZM3P4ZL4DLIEZ3WOEP67MQ6JITTO4D3NJN3RCA5YDBC3V4LA5LA")
        );
    }

    #[test]
    fn test_simple_asset_transfer_snapshot() {
        let data = TestDataMother::simple_asset_transfer();
        assert_eq!(
            data.id,
            String::from("VAHP4FRJH4GRV6ID2BZRK5VYID376EV3VE6T2TKKDFJBBDOXWCCA")
        );
        assert_eq!(
            data.id_raw,
            [
                168, 14, 254, 22, 41, 63, 13, 26, 249, 3, 208, 115, 21, 118, 184, 64, 247, 255, 18,
                187, 169, 61, 61, 77, 74, 25, 82, 16, 141, 215, 176, 132
            ]
        );
        assert_eq!(
            data.unsigned_bytes,
            vec![
                84, 88, 138, 164, 97, 97, 109, 116, 205, 3, 232, 164, 97, 114, 99, 118, 196, 32,
                138, 24, 8, 153, 89, 167, 60, 236, 255, 238, 91, 198, 115, 190, 137, 254, 3, 35,
                198, 98, 195, 33, 65, 123, 138, 200, 132, 194, 74, 0, 44, 25, 163, 102, 101, 101,
                205, 3, 232, 162, 102, 118, 206, 3, 13, 0, 56, 163, 103, 101, 110, 172, 116, 101,
                115, 116, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72, 99, 181,
                24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240, 89, 167,
                172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 13, 1, 0,
                163, 115, 110, 100, 196, 32, 72, 118, 175, 30, 96, 187, 134, 238, 76, 228, 146,
                219, 137, 200, 222, 52, 40, 86, 146, 168, 129, 190, 15, 103, 21, 24, 5, 31, 88, 27,
                201, 123, 164, 116, 121, 112, 101, 165, 97, 120, 102, 101, 114, 164, 120, 97, 105,
                100, 206, 6, 107, 40, 157
            ]
        );
        assert_eq!(
            data.signed_bytes,
            vec![
                130, 163, 115, 105, 103, 196, 64, 115, 182, 105, 213, 69, 248, 151, 218, 20, 41,
                12, 239, 153, 18, 26, 187, 149, 210, 109, 148, 39, 180, 210, 255, 64, 224, 207, 43,
                40, 165, 103, 14, 125, 13, 50, 33, 75, 66, 56, 124, 233, 253, 215, 254, 157, 18, 7,
                244, 15, 55, 31, 76, 190, 117, 201, 189, 5, 26, 44, 249, 196, 219, 73, 0, 163, 116,
                120, 110, 138, 164, 97, 97, 109, 116, 205, 3, 232, 164, 97, 114, 99, 118, 196, 32,
                138, 24, 8, 153, 89, 167, 60, 236, 255, 238, 91, 198, 115, 190, 137, 254, 3, 35,
                198, 98, 195, 33, 65, 123, 138, 200, 132, 194, 74, 0, 44, 25, 163, 102, 101, 101,
                205, 3, 232, 162, 102, 118, 206, 3, 13, 0, 56, 163, 103, 101, 110, 172, 116, 101,
                115, 116, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72, 99, 181,
                24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240, 89, 167,
                172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 13, 1, 0,
                163, 115, 110, 100, 196, 32, 72, 118, 175, 30, 96, 187, 134, 238, 76, 228, 146,
                219, 137, 200, 222, 52, 40, 86, 146, 168, 129, 190, 15, 103, 21, 24, 5, 31, 88, 27,
                201, 123, 164, 116, 121, 112, 101, 165, 97, 120, 102, 101, 114, 164, 120, 97, 105,
                100, 206, 6, 107, 40, 157
            ]
        );
    }

    #[test]
    fn test_opt_in_asset_transfer_snapshot() {
        let data = TestDataMother::opt_in_asset_transfer();
        assert_eq!(
            data.id,
            String::from("JIDBHDPLBASULQZFI4EY5FJWR6VQRMPPFSGYBKE2XKW65N3UQJXA")
        );
    }

    #[test]
    fn test_application_create_snapshot() {
        let data = TestDataMother::application_create();
        assert_eq!(
            data.id,
            String::from("L6B56N2BAXE43PUI7IDBXCJN5DEB6NLCH4AAN3ON64CXPSCTJNTA")
        );
    }

    #[test]
    fn test_application_call_snapshot() {
        let data = TestDataMother::application_call();
        assert_eq!(
            data.id,
            String::from("6Y644M5SGTKNBH7ZX6D7QAAHDF6YL6FDJPRAGSUHNZLR4IKGVSPQ")
        );
    }

    #[test]
    fn test_application_update_snapshot() {
        let data = TestDataMother::application_update();
        assert_eq!(
            data.id,
            String::from("NQVNJ5VWEDX42DMJQIQET4QPNUOW27EYIPKZ4SDWKOOEFJQB7PZA")
        );
    }

    #[test]
    fn test_application_delete_snapshot() {
        let data = TestDataMother::application_delete();
        assert_eq!(
            data.id,
            String::from("XVVC7UDLCPI622KCJZLWK3SEAWWVUEPEXUM5CO3DFLWOBH7NOPDQ")
        );
    }

    #[test]
    fn test_asset_create_snapshot() {
        let data = TestDataMother::asset_create();
        assert_eq!(
            data.id,
            String::from("NXAHS2NA46DJHIULXYPJV2NOJSKKFFNFFXRZP35TA5IDCZNE2MUA")
        );
    }

    #[test]
    fn test_asset_reconfigure_snapshot() {
        let data = TestDataMother::asset_reconfigure();
        assert_eq!(
            data.id,
            String::from("GAMRAG3KCG23U2HOELJF32OQAWAISLIFBB5RLDDDYHUSOZNYN7MQ")
        );
    }

    #[test]
    fn test_asset_destroy_snapshot() {
        let data = TestDataMother::asset_destroy();
        assert_eq!(
            data.id,
            String::from("U4XH6AS5UUYQI4IZ3E5JSUEIU64Y3FGNYKLH26W4HRY7T6PK745A")
        );
    }
}
