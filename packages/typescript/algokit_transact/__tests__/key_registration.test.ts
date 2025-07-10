import { expect, test, describe } from "bun:test";
import { testData } from "./common";
import * as ed from "@noble/ed25519";
import {
  encodeTransaction,
  decodeTransaction,
  getEncodedTransactionType,
  Transaction,
  addressFromPubKey,
  assignFee,
  SignedTransaction,
  encodeSignedTransaction,
} from "..";
import {
  assertAssignFee,
  assertDecodeWithoutPrefix,
  assertDecodeWithPrefix,
  assertEncode,
  assertEncodedTransactionType,
  assertEncodeWithAuthAddress,
  assertEncodeWithSignature,
  assertExample,
  assertTransactionId,
  assertMultisigExample,
} from "./transaction_asserts";

const txnTestData = Object.entries({
  ["online key registration"]: testData.onlineKeyRegistration,
  ["offline key registration"]: testData.offlineKeyRegistration,
  ["non-participation key registration"]: testData.nonParticipationKeyRegistration,
});

describe("Key Registration", () => {
  // Polytest Suite: Key Registration

  describe("Transaction Tests", () => {
    // Polytest Group: Transaction Tests

    for (const [label, testData] of txnTestData) {
      test("example", async () => {
        await assertExample(label, testData);
      });

      test("multisig example", async () => {
        await assertMultisigExample(label, testData);
      });

      test("get transaction id", () => {
        assertTransactionId(label, testData);
      });

      test("assign fee", () => {
        assertAssignFee(label, testData);
      });

      test("get encoded transaction type", () => {
        assertEncodedTransactionType(label, testData);
      });

      test("decode without prefix", () => {
        assertDecodeWithoutPrefix(label, testData);
      });

      test("decode with prefix", () => {
        assertDecodeWithPrefix(label, testData);
      });

      test("encode with auth address", async () => {
        await assertEncodeWithAuthAddress(label, testData);
      });

      test("encode with signature", async () => {
        await assertEncodeWithSignature(label, testData);
      });

      test("encode", () => {
        assertEncode(label, testData);
      });
    }
  });
});
