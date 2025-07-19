mod application_call;
mod asset_config;
mod asset_freeze;
mod key_registration;

use crate::{
    transactions::{AssetTransferTransactionBuilder, PaymentTransactionBuilder},
    Address, AlgorandMsgpack, Byte32, KeyPairAccount, MultisigSignature, MultisigSubsignature,
    SignedTransaction, Transaction, TransactionHeaderBuilder, TransactionId,
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, HASH_BYTES_LENGTH,
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
            "application_create": Self::application_create().as_json(&transform),
            "application_update": Self::application_update().as_json(&transform),
            "application_delete": Self::application_delete().as_json(&transform),
            "application_call": Self::application_call().as_json(&transform),
            "asset_create": Self::asset_create().as_json(&transform),
            "asset_destroy": Self::asset_destroy().as_json(&transform),
            "asset_reconfigure": Self::asset_reconfigure().as_json(&transform),
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
        print!("{:?}", data.multisig_signed_bytes);
        assert_eq!(
            data.multisig_signed_bytes,
            [
                130, 164, 109, 115, 105, 103, 131, 166, 115, 117, 98, 115, 105, 103, 146, 130, 162,
                112, 107, 196, 32, 230, 185, 154, 253, 65, 13, 19, 221, 14, 138, 126, 148, 184,
                121, 29, 48, 92, 117, 6, 238, 183, 225, 250, 65, 14, 118, 26, 59, 98, 44, 225, 20,
                161, 115, 196, 64, 198, 56, 196, 15, 176, 92, 85, 96, 205, 178, 248, 28, 27, 215,
                149, 74, 22, 18, 122, 228, 98, 34, 13, 202, 109, 58, 242, 134, 31, 206, 195, 29,
                110, 250, 219, 67, 240, 62, 47, 253, 200, 132, 24, 36, 210, 17, 97, 97, 165, 32,
                154, 49, 102, 252, 16, 157, 51, 135, 216, 86, 41, 198, 47, 15, 130, 162, 112, 107,
                196, 32, 110, 60, 51, 144, 133, 183, 247, 29, 54, 150, 199, 125, 170, 105, 173, 84,
                148, 21, 87, 24, 235, 157, 2, 172, 123, 102, 144, 142, 198, 141, 84, 114, 161, 115,
                196, 64, 198, 56, 196, 15, 176, 92, 85, 96, 205, 178, 248, 28, 27, 215, 149, 74,
                22, 18, 122, 228, 98, 34, 13, 202, 109, 58, 242, 134, 31, 206, 195, 29, 110, 250,
                219, 67, 240, 62, 47, 253, 200, 132, 24, 36, 210, 17, 97, 97, 165, 32, 154, 49,
                102, 252, 16, 157, 51, 135, 216, 86, 41, 198, 47, 15, 163, 116, 104, 114, 2, 161,
                118, 1, 163, 116, 120, 110, 137, 163, 97, 109, 116, 206, 0, 1, 138, 136, 163, 102,
                101, 101, 205, 3, 232, 162, 102, 118, 206, 3, 5, 0, 212, 163, 103, 101, 110, 172,
                116, 101, 115, 116, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72,
                99, 181, 24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240,
                89, 167, 172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 5,
                4, 188, 163, 114, 99, 118, 196, 32, 173, 207, 218, 63, 201, 93, 52, 35, 35, 15,
                161, 115, 204, 245, 211, 90, 68, 182, 3, 164, 184, 247, 131, 205, 149, 104, 201,
                215, 253, 11, 206, 245, 163, 115, 110, 100, 196, 32, 138, 24, 8, 153, 89, 167, 60,
                236, 255, 238, 91, 198, 115, 190, 137, 254, 3, 35, 198, 98, 195, 33, 65, 123, 138,
                200, 132, 194, 74, 0, 44, 25, 164, 116, 121, 112, 101, 163, 112, 97, 121
            ]
        )
    }

    #[test]
    fn test_simple_asset_transfer_snapshot() {
        let data = TestDataMother::simple_asset_transfer();
        assert_eq!(
            data.id,
            String::from("VAHP4FRJH4GRV6ID2BZRK5VYID376EV3VE6T2TKKDFJBBDOXWCCA")
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

    #[test]
    fn test_online_key_registration_snapshot() {
        let data = TestDataMother::online_key_registration();
        assert_eq!(
            data.id,
            String::from("UCWQQKWB3CMPVK6EU2ML7CN5IDYZJVVSVS3RXYEOLJUURX44SUKQ")
        );
    }
    #[test]
    fn test_offline_key_registration_snapshot() {
        let data = TestDataMother::offline_key_registration();
        assert_eq!(
            data.id,
            String::from("WAXJLC44RILOSYX73PJULCAWC43DNBU4AXMWHIRARXK4GO2LHEDQ")
        );
    }
    #[test]
    fn test_non_participation_key_registration_snapshot() {
        let data = TestDataMother::non_participation_key_registration();
        assert_eq!(
            data.id,
            String::from("ACAP6ZGMGNTLUO3IQ26P22SRKYWTQQO3MF64GX7QO6NICDUFPM5A")
        );
    }

    #[test]
    fn test_asset_freeze_snapshot() {
        let data = TestDataMother::asset_freeze();
        assert_eq!(
            data.id,
            String::from("2XFGVOHMFYLAWBHOSIOI67PBT5LDRHBTD3VLX5EYBDTFNVKMCJIA")
        );
    }

    #[test]
    fn test_asset_unfreeze_snapshot() {
        let data = TestDataMother::asset_unfreeze();
        assert_eq!(
            data.id,
            String::from("LZ2ODDAT4ATAVJUEQW34DIKMPCMBXCCHOSIYKMWGBPEVNHLSEV2A")
        );
    }
}
