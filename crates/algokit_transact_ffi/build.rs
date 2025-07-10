use algokit_transact::{ALGORAND_SECRET_KEY_BYTE_LENGTH, HASH_BYTES_LENGTH};

include!("src/lib.rs");

fn main() {
    generate_test_data()
}

fn generate_test_data() {
    use algokit_transact::test_utils;
    use serde::Serialize;
    use std::path::Path;

    #[derive(Serialize)]
    struct TransactionTestData {
        transaction: Transaction,
        id: String,
        id_raw: [u8; HASH_BYTES_LENGTH],
        unsigned_bytes: Vec<u8>,
        signing_private_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH],
        signed_bytes: Vec<u8>,
        rekeyed_sender_auth_address: String,
        rekeyed_sender_signed_bytes: Vec<u8>,
        multisig_addresses: (String, String),
        multisig_signed_bytes: Vec<u8>,
    }

    test_utils::TestDataMother::export(
        Path::new("./test_data.json"),
        Some(|d: &test_utils::TransactionTestData| TransactionTestData {
            transaction: d.transaction.clone().try_into().unwrap(),
            id: d.id.clone(),
            id_raw: d.id_raw,
            unsigned_bytes: d.unsigned_bytes.clone(),
            signing_private_key: d.signing_private_key,
            signed_bytes: d.signed_bytes.clone(),
            rekeyed_sender_auth_address: d.rekeyed_sender_auth_address.as_str(),
            rekeyed_sender_signed_bytes: d.rekeyed_sender_signed_bytes.clone(),
            multisig_addresses: (
                d.multisig_addresses.clone().0.as_str(),
                d.multisig_addresses.clone().1.as_str(),
            ),
            multisig_signed_bytes: d.multisig_signed_bytes.clone(),
        }),
    );
}
