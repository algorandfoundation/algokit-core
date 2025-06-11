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
use std::{fs::File, str::FromStr};

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
}

pub struct TransactionMother {}
impl TransactionMother {
    pub fn simple_payment() -> PaymentTransactionBuilder {
        PaymentTransactionBuilder::default()
            .header(TransactionHeaderMother::simple_testnet().build().unwrap())
            .amount(101000)
            .receiver(
                "VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ"
                    .parse::<Address>()
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

    pub fn opt_in_asset_transfer() -> AssetTransferTransactionBuilder {
        AssetTransferTransactionBuilder::default()
            .header(
                TransactionHeaderMother::simple_testnet()
                    .sender(
                        "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
                            .parse::<Address>()
                            .unwrap(),
                    )
                    .first_valid(51183672)
                    .last_valid(51183872)
                    .build()
                    .unwrap(),
            )
            .asset_id(107686045)
            .amount(0)
            .receiver(
                "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
                    .parse::<Address>()
                    .unwrap(),
            )
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
            .parse::<Address>()
            .unwrap()
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
        let id = transaction.id().unwrap();
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
        let signing_private_key: Byte32 = [
            2, 205, 103, 33, 67, 14, 82, 196, 115, 196, 206, 254, 50, 110, 63, 182, 149, 229, 184,
            216, 93, 11, 13, 99, 69, 213, 218, 165, 134, 118, 47, 44,
        ];
        let transaction = TransactionMother::simple_payment().build().unwrap();
        TransactionTestData::new(transaction, signing_private_key)
    }

    pub fn opt_in_asset_transfer() -> TransactionTestData {
        let signing_private_key: Byte32 = [
            2, 205, 103, 33, 67, 14, 82, 196, 115, 196, 206, 254, 50, 110, 63, 182, 149, 229, 184,
            216, 93, 11, 13, 99, 69, 213, 218, 165, 134, 118, 47, 44,
        ];
        let transaction = TransactionMother::opt_in_asset_transfer().build().unwrap();
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
        }));

        let file = File::create(path).expect("Failed to create export file");
        to_writer_pretty(file, &test_data).expect("Failed to write export JSON");
    }
}

fn normalise_json(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k.to_case(Case::Camel), normalise_json(v)))
                .collect(),
        ),
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(|v| normalise_json(v)).collect())
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
        assert_eq!(
            data.id_raw,
            [
                158, 89, 183, 243, 43, 224, 214, 130, 103, 118, 113, 31, 239, 178, 30, 74, 39, 55,
                112, 123, 106, 91, 184, 136, 29, 192, 194, 45, 215, 139, 7, 86
            ]
        );
        assert_eq!(
            data.unsigned_bytes,
            vec![
                84, 88, 137, 163, 97, 109, 116, 206, 0, 1, 138, 136, 163, 102, 101, 101, 205, 3,
                232, 162, 102, 118, 206, 3, 5, 0, 212, 163, 103, 101, 110, 172, 116, 101, 115, 116,
                110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72, 99, 181, 24, 164,
                179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240, 89, 167, 172, 32,
                222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 5, 4, 188, 163, 114,
                99, 118, 196, 32, 173, 207, 218, 63, 201, 93, 52, 35, 35, 15, 161, 115, 204, 245,
                211, 90, 68, 182, 3, 164, 184, 247, 131, 205, 149, 104, 201, 215, 253, 11, 206,
                245, 163, 115, 110, 100, 196, 32, 138, 24, 8, 153, 89, 167, 60, 236, 255, 238, 91,
                198, 115, 190, 137, 254, 3, 35, 198, 98, 195, 33, 65, 123, 138, 200, 132, 194, 74,
                0, 44, 25, 164, 116, 121, 112, 101, 163, 112, 97, 121
            ]
        );
        assert_eq!(
            data.signed_bytes,
            vec![
                130, 163, 115, 105, 103, 196, 64, 198, 56, 196, 15, 176, 92, 85, 96, 205, 178, 248,
                28, 27, 215, 149, 74, 22, 18, 122, 228, 98, 34, 13, 202, 109, 58, 242, 134, 31,
                206, 195, 29, 110, 250, 219, 67, 240, 62, 47, 253, 200, 132, 24, 36, 210, 17, 97,
                97, 165, 32, 154, 49, 102, 252, 16, 157, 51, 135, 216, 86, 41, 198, 47, 15, 163,
                116, 120, 110, 137, 163, 97, 109, 116, 206, 0, 1, 138, 136, 163, 102, 101, 101,
                205, 3, 232, 162, 102, 118, 206, 3, 5, 0, 212, 163, 103, 101, 110, 172, 116, 101,
                115, 116, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72, 99, 181,
                24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240, 89, 167,
                172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 5, 4, 188,
                163, 114, 99, 118, 196, 32, 173, 207, 218, 63, 201, 93, 52, 35, 35, 15, 161, 115,
                204, 245, 211, 90, 68, 182, 3, 164, 184, 247, 131, 205, 149, 104, 201, 215, 253,
                11, 206, 245, 163, 115, 110, 100, 196, 32, 138, 24, 8, 153, 89, 167, 60, 236, 255,
                238, 91, 198, 115, 190, 137, 254, 3, 35, 198, 98, 195, 33, 65, 123, 138, 200, 132,
                194, 74, 0, 44, 25, 164, 116, 121, 112, 101, 163, 112, 97, 121
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
        assert_eq!(
            data.id_raw,
            [
                74, 6, 19, 141, 235, 8, 37, 69, 195, 37, 71, 9, 142, 149, 54, 143, 171, 8, 177,
                239, 44, 141, 128, 168, 154, 186, 173, 238, 183, 116, 130, 110
            ]
        );
        assert_eq!(
            data.unsigned_bytes,
            vec![
                84, 88, 137, 164, 97, 114, 99, 118, 196, 32, 72, 118, 175, 30, 96, 187, 134, 238,
                76, 228, 146, 219, 137, 200, 222, 52, 40, 86, 146, 168, 129, 190, 15, 103, 21, 24,
                5, 31, 88, 27, 201, 123, 163, 102, 101, 101, 205, 3, 232, 162, 102, 118, 206, 3,
                13, 0, 56, 163, 103, 101, 110, 172, 116, 101, 115, 116, 110, 101, 116, 45, 118, 49,
                46, 48, 162, 103, 104, 196, 32, 72, 99, 181, 24, 164, 179, 200, 78, 200, 16, 242,
                45, 79, 16, 129, 203, 15, 113, 240, 89, 167, 172, 32, 222, 198, 47, 127, 112, 229,
                9, 58, 34, 162, 108, 118, 206, 3, 13, 1, 0, 163, 115, 110, 100, 196, 32, 72, 118,
                175, 30, 96, 187, 134, 238, 76, 228, 146, 219, 137, 200, 222, 52, 40, 86, 146, 168,
                129, 190, 15, 103, 21, 24, 5, 31, 88, 27, 201, 123, 164, 116, 121, 112, 101, 165,
                97, 120, 102, 101, 114, 164, 120, 97, 105, 100, 206, 6, 107, 40, 157
            ]
        );
        assert_eq!(
            data.signed_bytes,
            vec![
                130, 163, 115, 105, 103, 196, 64, 108, 27, 242, 197, 141, 1, 233, 137, 108, 190,
                54, 245, 55, 173, 43, 72, 68, 36, 204, 128, 202, 112, 148, 46, 178, 69, 192, 121,
                3, 159, 167, 170, 75, 211, 7, 248, 87, 195, 171, 222, 105, 44, 38, 162, 25, 58,
                154, 189, 182, 48, 252, 167, 101, 145, 73, 180, 101, 107, 181, 191, 37, 57, 211, 1,
                163, 116, 120, 110, 137, 164, 97, 114, 99, 118, 196, 32, 72, 118, 175, 30, 96, 187,
                134, 238, 76, 228, 146, 219, 137, 200, 222, 52, 40, 86, 146, 168, 129, 190, 15,
                103, 21, 24, 5, 31, 88, 27, 201, 123, 163, 102, 101, 101, 205, 3, 232, 162, 102,
                118, 206, 3, 13, 0, 56, 163, 103, 101, 110, 172, 116, 101, 115, 116, 110, 101, 116,
                45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72, 99, 181, 24, 164, 179, 200, 78,
                200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240, 89, 167, 172, 32, 222, 198, 47,
                127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 13, 1, 0, 163, 115, 110, 100, 196,
                32, 72, 118, 175, 30, 96, 187, 134, 238, 76, 228, 146, 219, 137, 200, 222, 52, 40,
                86, 146, 168, 129, 190, 15, 103, 21, 24, 5, 31, 88, 27, 201, 123, 164, 116, 121,
                112, 101, 165, 97, 120, 102, 101, 114, 164, 120, 97, 105, 100, 206, 6, 107, 40,
                157
            ]
        );
    }
}
