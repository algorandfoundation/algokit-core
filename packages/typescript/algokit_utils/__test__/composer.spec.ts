import { addressFromString, Composer, Transaction } from "../";
import { expect, describe, test } from "bun:test";

describe("Composer", () => {
  test("create composer", () => {
    const composer = new Composer();
    expect(composer).toBeDefined();
  });
  //
  // tx = Transaction(
  //     transaction_type=TransactionType.PAYMENT,
  //     sender=address_from_string(
  //         "LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"
  //     ),
  //     first_valid=1,
  //     last_valid=10,
  //     genesis_hash=b"a" * 32,
  //     genesis_id="",
  //     payment=PaymentTransactionFields(
  //         receiver=address_from_string(
  //             "LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"
  //         ),
  //         amount=1000,  # microAlgos
  //     ),
  // )
  test("add transaction", () => {
    const composer = new Composer();
    const tx: Transaction = {
      transactionType: "Payment",
      sender: addressFromString("LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"),
      firstValid: 1n,
      lastValid: 10n,
      genesisHash: Buffer.from("a".repeat(64)),
      genesisId: "",
      payment: {
        receiver: addressFromString("LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"),
        amount: 1000n, // microAlgos
      },
    };

    composer.addTransaction(tx);
  });
});
