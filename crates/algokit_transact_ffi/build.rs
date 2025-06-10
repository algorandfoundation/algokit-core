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
        signing_private_key: [u8; ALGORAND_SECRET_KEY_BYTE_LENGTH],
        transaction: Transaction,
        unsigned_bytes: Vec<u8>,
        signed_bytes: Vec<u8>,
        signer: Address,
        signer_signed_bytes: Vec<u8>,
        id: String,
        id_raw: [u8; HASH_BYTES_LENGTH],
    }

    test_utils::TestDataMother::export(
        Path::new("./test_data.json"),
        Some(|d: &test_utils::TransactionTestData| TransactionTestData {
            signing_private_key: d.signing_private_key,
            transaction: d.transaction.clone().try_into().unwrap(),
            unsigned_bytes: d.unsigned_bytes.clone(),
            signed_bytes: d.signed_bytes.clone(),
            signer: d.signer.clone().into(),
            signer_signed_bytes: d.signer_signed_bytes.clone(),
            id: d.id.clone(),
            id_raw: d.id_raw,
        }),
    );
}
