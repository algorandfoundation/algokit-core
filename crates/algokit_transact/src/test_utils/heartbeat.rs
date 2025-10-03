use crate::{Byte32, HeartbeatProofBuilder, HeartbeatTransactionBuilder, TransactionHeaderBuilder};
use base64::{Engine, prelude::BASE64_STANDARD};

pub struct HeartbeatTransactionMother {}

impl HeartbeatTransactionMother {
    pub fn heartbeat() -> HeartbeatTransactionBuilder {
        // testnet-GCVW7GJTD5OALIXPQ3RGMYKTTYCWUJY3E4RPJTX7WHIWZK4V6NYA
        let genesis_hash: Byte32 = BASE64_STANDARD
            .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
            .unwrap()
            .try_into()
            .unwrap();

        let header = TransactionHeaderBuilder::default()
            .sender(
                "GAU5WA6DT2EPFS6LKOA333BQP67NXIHZ7JPOOHMZWJDPZRL4XMHDDDUCKA"
                    .parse()
                    .unwrap(),
            )
            .first_valid(48023101)
            .last_valid(48023111)
            .fee(0)
            .genesis_hash(genesis_hash)
            .build()
            .unwrap();

        let proof = HeartbeatProofBuilder::default()
            .sig(
                BASE64_STANDARD
                    .decode("gqUA0TzSTm8hSZpP4zMM+gjp0PxMGDepz1tTvSbjKl0M/wNG42xok/GOd24bdgJx1S4BwOBk24YQ9Kkr4yQHBQ==")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .pk(
                BASE64_STANDARD
                    .decode("Mkw5YAnXm7icofoLlNYyZjlIlhQPZr8/ENYpCkrgIhY=")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .pk2(
                BASE64_STANDARD
                    .decode("gj8CDI6wG/285mSLh9f/2kkHC7K3FaTKuPH7jDnf4N0=")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .pk1_sig(
                BASE64_STANDARD
                    .decode("NJbUaMo7P0K6M1vjLk8b9zcf65MzvtFp+KenZEu1oZuxGSoTWoSjCM+CeLbsOHq/sojXBYssR+ekUgujpKnSAQ==")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .pk2_sig(
                BASE64_STANDARD
                    .decode("i8qqqcWpRfqRYgBXh+8/D2YoTTLhQXvZvhSn49LDM03A+I70mNYGVOql3qDUomxRJ1XYrw5VjLRb0VS9/NrCDQ==")
                    .unwrap()
                    .try_into()
                    .unwrap()
            )
            .build()
            .unwrap();

        HeartbeatTransactionBuilder::default()
            .header(header)
            .address(
                "PQAQM4J776S642O42Y6RDRHOTUAC6DQCXLFJOJVHAOZZQK5U6MQG6O6HFY"
                    .parse()
                    .unwrap(),
            )
            .proof(proof)
            .seed(
                BASE64_STANDARD
                    .decode("4dAbkZxfEtTMdj8gz5pugXqtUamzKEpcKBj6KXWfb/g=")
                    .unwrap(),
            )
            .vote_id(
                BASE64_STANDARD
                    .decode("pNdTh3H82LwT7c0kMwyFox+fweCHBuLR6GBUetfF80c=")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .key_dilution(1419)
            .to_owned()
    }
}
