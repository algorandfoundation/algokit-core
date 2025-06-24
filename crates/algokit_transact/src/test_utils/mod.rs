use crate::{
    transactions::{
        ApplicationCallTransactionBuilder, AssetTransferTransactionBuilder, OnApplicationComplete,
        PaymentTransactionBuilder, StateSchema,
    },
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

    pub fn application_create() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/L6B56N2BAXE43PUI7IDBXCJN5DEB6NLCH4AAN3ON64CXPSCTJNTA
        ApplicationCallTransactionBuilder::default()
            .header(TransactionHeaderMother::testnet()
                .sender(AddressMother::nfd_testnet())
                .first_valid(21038057)
                .last_valid(21039057)
                .note(
                    BASE64_STANDARD
                        .decode("TkZEIFJlZ2lzdHJ5IENvbnRyYWN0")
                        .unwrap(),
                )
                .build()
                .unwrap())
            .app_id(0)
            .on_complete(OnApplicationComplete::NoOp)
            .approval_program(
                BASE64_STANDARD
                .decode("BiAKAQACCAQGEAoFAyYDBmkuYXBwcwhhZGRfYWRkcgAxFiQMQAKmMRYiCTUAI0ACmjEZIQgSQAKQMRkhBBJAAoIxGCMSQAJ5MRkkEkACcDEZIhJAAb8xGSMSQAABADYaAIADZ2FzEkABqTIEJA80ADgQIhIQNAA4IDIDEhA0ADgJMgMSEDQAOAgyAA80ADgBMgANERA0AIgCgSISEEAAAiNDMRskEjYaACkSEDEWIgk4BzEAEhBAAUsxGyQSNhoAgAtyZW1vdmVfYWRkchIQMRYiCTgHMQASEEABGTIEIQQPNhoAgARtaW50EhAxGyEEEhBAAAEAMRYhCQk4PTUBNAFyCDUENQMxFiIJiAINMRaIAggQFEAA2zEWJAmIAfwxFiQJOAgjEhBAAMI2GgEXNQIxFiQJOAcyChIxFiQJOAg0AhIQMRYiCTgHMgoSEDEWIgk4CIHAmgwSEDEYMggSEDEQIQUSEBRAAIGxIrIQMRYiCTgINAIIsgg0A7IHI7IBtiEFshAjshkxALIcNjAAsjA0AbIYNhoAsho2GgKyGjYaA7IaI7IBtiEFshAjshkxALIcNjAAsjA0AbIYgA5vZmZlcl9mb3Jfc2FsZbIaNhoBshoxFiEJCTkaA7IaI7IBs7gBOgA1yCJC/rYjQyM1AkL/PiNDMQAoIQY2GgEXiAJ1Qv6dMQAoIQY2GgEXiAIyQv6OIkMyBCQPNAA4ECISEDQAOCAyAxIQNAA4CTIDEhA0ADgIMgAPNAA4ATIADREQNACIANYiEhA0ADgHMQASEEAAAiNDMgQkDzEbIhIQNhoAgAZhc3NpZ24SEEAAJzEbJBI2GgApEhAxFiIJOAcxABIQQAABADEAKCEGNhoBF4gBsEL/vzEAgAdpLmFzYWlkMRYkCTvIZjEAgAdpLmFwcGlkMRYhCAk4PRZmIkL/lSJDIkMxADIJEkMjQyNDIzUAQv1aNQ6ACjAxMjM0NTY3ODk0DiJYiTUNNA0jEkAAKDQNIQcKIw1AAA0qNA0hBxiI/9FQQgAUNA0hBwo0DUyI/9VMNQ1C/+OAATCJNQU0BTgAgbirnShwADUHNQY0BiISNAU4CTIDEhA0BTggMgMSEIk1FzUWNRU0FTQWNBdjNRk1GDQZQAAEKkIAAjQYiTURNRA1DzQPMgg0EIj/1BUjEkAAcTQPNBBiNRIjNRM0EzQSFSUKDEAAGTQSFYF4DEAAAiOJNA80EDQSNBEWUGZCAE00EjQTJQtbNRQ0FCMSQAATNBQ0ERJAAAk0EyIINRNC/7siiTQPNBA0EiM0EyULUjQRFlA0EjQTJQslCDQSFVJQZiKJNA80EDQRFmYiiSKJNSE1IDUfNB80IGI1IiM1IzQjNCIVJQoMQQA1NCI0IyULWzQhEkAACTQjIgg1I0L/3zQfNCA0IiM0IyULUiMWUDQiNCMlCyUINCIVUlBmIokjiTULNQo1CTUIIzUMNAw0CgxBAB80CDQJNAyI/ohQNAuI/voiEkAACTQMIgg1DEL/2yKJI4k1HTUcNRs1GiM1HjQeNBwMQQAfNBo0GzQeiP5UUDQdiP9YIhJAAAk0HiIINR5C/9siiSOJ")
                .unwrap()
            )
            .clear_state_program(
                BASE64_STANDARD
                .decode("BoEBQw==")
                .unwrap()
            )
            .extra_program_pages(3)
            .local_state_schema(StateSchema {
                num_uints: 0,
                num_byte_slices: 16,
            })
            .to_owned()
    }

    pub fn application_update() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/NQVNJ5VWEDX42DMJQIQET4QPNUOW27EYIPKZ4SDWKOOEFJQB7PZA
        ApplicationCallTransactionBuilder::default()
            .header(TransactionHeaderMother::testnet()
                .sender(AddressMother::nfd_testnet())
                .first_valid(43679851)
                .last_valid(43679951)
                .note(
                    BASE64_STANDARD
                        .decode("TkZEIFJlZ2lzdHJ5IENvbnRyYWN0OldTT1RSSlhMWUJRWVZNRkxPSUxZVlVTTklXS0JXVUJXM0dOVVdBRktHS0hOS05SWDY2TkVaSVRVTE0=")
                        .unwrap(),
                )
                .build()
                .unwrap())
            .app_id(84366825)
            .on_complete(OnApplicationComplete::UpdateApplication)
            .args(vec![
                BASE64_STANDARD.decode("dGVhbHNjcmlwdC1kdW1teQ==").unwrap(),
            ])
            .approval_program(
                BASE64_STANDARD
                .decode("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")
                .unwrap()
            )
            .clear_state_program(
                BASE64_STANDARD
                .decode("Cg==")
                .unwrap()
            )
            .to_owned()
    }

    pub fn application_call() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/6Y644M5SGTKNBH7ZX6D7QAAHDF6YL6FDJPRAGSUHNZLR4IKGVSPQ
        ApplicationCallTransactionBuilder::default()
            .header(
                TransactionHeaderMother::testnet()
                    .sender("KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE".parse().unwrap())
                    .first_valid(21038300)
                    .last_valid(21039300)
                    .note(BASE64_STANDARD.decode("AAAAAAAPQkA=").unwrap())
                    .fee(5000)
                    .group(BASE64_STANDARD.decode("ktxBY/2UFfqvhwKKxwihS9YhfG+of3hz2I3ErgNZZSo=").unwrap().try_into().unwrap())
                    .build()
                    .unwrap(),
            )
            .app_id(84366825)
            .on_complete(OnApplicationComplete::NoOp)
            .args(vec![
                BASE64_STANDARD.decode("bWludA==").unwrap(),
                BASE64_STANDARD.decode("AAAAAAAPQkA=").unwrap(),
                BASE64_STANDARD.decode("c2VjdXJpdGl6ZS5hbGdv").unwrap(),
                BASE64_STANDARD.decode("dGVtcGxhdGUtaXBmczovL3tpcGZzY2lkOjE6ZGFnLXBiOnJlc2VydmU6c2hhMi0yNTZ9L25mZC5qc29u").unwrap(),
            ])
            .account_references(vec![
                "KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE"
                    .parse()
                    .unwrap(),
                "KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE"
                    .parse()
                    .unwrap(),
            ])
            .asset_references(vec![84366776])
            .to_owned()
    }

    pub fn application_delete() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/mainnet/transaction/XVVC7UDLCPI622KCJZLWK3SEAWWVUEPEXUM5CO3DFLWOBH7NOPDQ
        ApplicationCallTransactionBuilder::default()
            .header(
                TransactionHeaderMother::mainnet()
                    .sender(
                        "H3OQEQIIC35RZTJNU5A75LT4PCTUCF3VKVEQTSXAJMUGNTRUKEKI4QSRW4"
                            .parse()
                            .unwrap(),
                    )
                    .first_valid(39723798)
                    .last_valid(39724798)
                    .build()
                    .unwrap(),
            )
            .app_id(1898586902)
            .on_complete(OnApplicationComplete::DeleteApplication)
            .account_references(vec![
                "MDIVKI64M2HEKCWKH7SOTUXEEW6KNOYSAOBTDTS32KUQOGUT75D43MSP5M"
                    .parse()
                    .unwrap(),
                "H3OQEQIIC35RZTJNU5A75LT4PCTUCF3VKVEQTSXAJMUGNTRUKEKI4QSRW4"
                    .parse()
                    .unwrap(),
                "MDIVKI64M2HEKCWKH7SOTUXEEW6KNOYSAOBTDTS32KUQOGUT75D43MSP5M"
                    .parse()
                    .unwrap(),
            ])
            .asset_references(vec![850924184])
            .to_owned()
    }

    pub fn application_opt_in() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/BNASGY47TXXUTFUZPDAGGPQKK54B4QPEEPDTJIZFDXC64WQH4GOQ
        ApplicationCallTransactionBuilder::default()
            .header(
                TransactionHeaderMother::testnet()
                    .sender(
                        "RAJ6J5E32CAU47LTXYQESPEGNTIE4AE652XZMU4V2AYBNTRVDPOF5DXOQM"
                            .parse()
                            .unwrap(),
                    )
                    .first_valid(21038233)
                    .last_valid(21039233)
                    .fee(1000)
                    .group(
                        BASE64_STANDARD
                            .decode("3T4cJx8PgfSOdwtONtjXVpkUjd46fNik93wUUZMszxY=")
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .app_id(84366825)
            .on_complete(OnApplicationComplete::OptIn)
            .args(vec![BASE64_STANDARD.decode("YXNzaWdu").unwrap()])
            .account_references(vec![
                "KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE"
                    .parse()
                    .unwrap(),
            ])
            .asset_references(vec![84366776])
            .to_owned()
    }

    pub fn application_call_example() -> ApplicationCallTransactionBuilder {
        ApplicationCallTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .app_id(12345)
            .on_complete(OnApplicationComplete::NoOp)
            .to_owned()
    }

    pub fn application_close_out() -> ApplicationCallTransactionBuilder {
        Self::application_call_example()
            .on_complete(OnApplicationComplete::CloseOut)
            .to_owned()
    }

    pub fn application_clear_state() -> ApplicationCallTransactionBuilder {
        Self::application_call_example()
            .on_complete(OnApplicationComplete::ClearState)
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
        let transaction = TransactionMother::application_create().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn application_update() -> TransactionTestData {
        let transaction = TransactionMother::application_update().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn application_delete() -> TransactionTestData {
        let transaction = TransactionMother::application_delete().build().unwrap();
        TransactionTestData::new(transaction, SIGNING_PRIVATE_KEY)
    }

    pub fn application_call() -> TransactionTestData {
        let transaction = TransactionMother::application_call().build().unwrap();
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
}
