use crate::{
    transactions::PaymentTransactionBuilder, Address, TransactionHeaderBuilder, TransactionType,
};
use base64::{prelude::BASE64_STANDARD, Engine};

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
            .to_owned()
    }

    pub fn simple_testnet_payment() -> TransactionHeaderBuilder {
        Self::testnet()
            .transaction_type(TransactionType::Payment)
            .sender(
                Address::from_string("RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q")
                    .unwrap(),
            )
            .fee(1000)
            .first_valid(50659540)
            .last_valid(50660540)
            .to_owned()
    }
}

pub struct TransactionMother {}
impl TransactionMother {
    pub fn simple_payment() -> PaymentTransactionBuilder {
        PaymentTransactionBuilder::default()
            .header(
                TransactionHeaderMother::simple_testnet_payment()
                    .build()
                    .unwrap(),
            )
            .amount(101000)
            .receiver(
                Address::from_string("VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ")
                    .unwrap(),
            )
            .to_owned()
    }

    pub fn payment_with_note() -> PaymentTransactionBuilder {
        Self::simple_payment()
            .header(
                TransactionHeaderMother::simple_testnet_payment()
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
}

// TODO: NC - Update to use the new test data file
// TODO: NC - Remove old generation process
// TODO: NC - Can we auto build the TS and Python types?
// TODO: NC - Do we need an Object mother for addresses?
