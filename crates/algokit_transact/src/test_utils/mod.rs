use crate::{
    transactions::{payment::PaymentTransactionFieldsBuilder, TransactionBuilder},
    Address,
};
use base64::{prelude::BASE64_STANDARD, Engine};

pub struct TransactionMother {}

impl TransactionMother {
    pub fn simple_payment() -> TransactionBuilder {
        TransactionBuilder::new_testnet()
            .sender(
                Address::from_string("RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q")
                    .unwrap(),
            )
            .payment(
                PaymentTransactionFieldsBuilder::default()
                    .amount(101000)
                    .receiver(
                        Address::from_string(
                            "VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ",
                        )
                        .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .fee(1000)
            .first_valid(50659540)
            .last_valid(50660540)
            .to_owned()
    }

    pub fn payment_with_note() -> TransactionBuilder {
        Self::simple_payment()
            .note(
                BASE64_STANDARD
                    .decode("MGFhNTBkMjctYjhmNy00ZDc3LWExZmItNTUxZmQ1NWRmMmJj")
                    .unwrap(),
            )
            .to_owned()
    }
}
