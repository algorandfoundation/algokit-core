use crate::{test_utils::TransactionHeaderMother, KeyRegistrationTransactionBuilder};
use base64::{prelude::BASE64_STANDARD, Engine};

pub struct KeyRegistrationTransactionMother {}

impl KeyRegistrationTransactionMother {
    pub fn online_key_registration() -> KeyRegistrationTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/UCWQQKWB3CMPVK6EU2ML7CN5IDYZJVVSVS3RXYEOLJUURX44SUKQ
        let mut header = TransactionHeaderMother::testnet()
            .sender(
                "PKASUHJJ7HALD6BXBIOLQTRFHAP6HF2TAYQ734E325FGDRB66EE6MYQGTM"
                    .parse()
                    .unwrap(),
            )
            .first_valid(53287880)
            .last_valid(53288880)
            .fee(2000000)
            .build()
            .unwrap();
        header.genesis_id = None;
        KeyRegistrationTransactionBuilder::default()
            .header(
                header
            )
            .vote_key(
                BASE64_STANDARD
                    .decode("jXzwxM2vUp0/wdazgu6be7BesDn9NKCDaEfvwKMmhTE=")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .selection_key(
                BASE64_STANDARD
                    .decode("pi8u2HhXe6qB5IIMTSn2vKiWkDhMCOk1G2G3oyaeSlA=")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .state_proof_key(
                BASE64_STANDARD
                    .decode("+h0VzqDJIOEaYTaCGDZMV0jZKQ4ShsVrhyyObOu+s3yF1+oLp2b4l/WGDFp1+kObVVyoNcCYyuE15OsyAhYZxg==")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .vote_first(53287679)
            .vote_last(56287679)
            .vote_key_dilution(1733)
            .to_owned()
    }

    pub fn offline_key_registration() -> KeyRegistrationTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/WAXJLC44RILOSYX73PJULCAWC43DNBU4AXMWHIRARXK4GO2LHEDQ
        let mut header = TransactionHeaderMother::testnet()
            .sender(
                "W5LKXE4BG7OND7KPGSXPDOOYQDITPNS7NSDU7672TO6I4RDNSEFWXRPISQ"
                    .parse()
                    .unwrap(),
            )
            .first_valid(52556882)
            .last_valid(52557882)
            .fee(1000)
            .build()
            .unwrap();
        header.genesis_id = None;
        KeyRegistrationTransactionBuilder::default()
            .header(header)
            .to_owned()
    }

    pub fn non_participation_key_registration() -> KeyRegistrationTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/ACAP6ZGMGNTLUO3IQ26P22SRKYWTQQO3MF64GX7QO6NICDUFPM5A
        let mut header = TransactionHeaderMother::testnet()
            .sender(
                "4UMX2FKZ636VEL74WR66Z5PSRVDC2QAH6GRPP2DTVSBPPADBDY2JB3PN2U"
                    .parse()
                    .unwrap(),
            )
            .first_valid(3321800)
            .last_valid(3322800)
            .fee(1000)
            .build()
            .unwrap();
        header.genesis_id = None;
        KeyRegistrationTransactionBuilder::default()
            .header(header)
            .non_participation(true)
            .to_owned()
    }
}
