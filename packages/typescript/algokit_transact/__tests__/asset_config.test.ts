import { test, describe } from "bun:test";
import { testData } from "./common";
import {
  assertExample,
  assertTransactionId,
  assertAssignFee,
  assertEncodedTransactionType,
  assertDecodeWithoutPrefix,
  assertDecodeWithPrefix,
  assertEncodeWithAuthAddress,
  assertEncodeWithSignature,
  assertEncode,
} from "./transaction_asserts";

const txnTestData = Object.entries({
  ["asset create"]: testData.assetCreate,
  ["asset reconfigure"]: testData.assetReconfigure,
  ["asset destroy"]: testData.assetDestroy,
});

describe("AssetConfig", () => {
  // Polytest Suite: AssetConfig

  describe("Transaction Tests", () => {
    // Polytest Group: Transaction Tests

    for (const [label, testData] of txnTestData) {
      test("example", async () => {
        await assertExample(label, testData);
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

      test("encode with signature", () => {
        assertEncodeWithSignature(label, testData);
      });

      test("encode", () => {
        assertEncode(label, testData);
      });
    }
  });
});
