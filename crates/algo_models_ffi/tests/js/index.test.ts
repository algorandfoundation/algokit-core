import { expect, test, describe } from "bun:test";
import * as ed from "@noble/ed25519";
import init, {
  type PayTransactionFields,
  encodeTransaction,
  attachSignature,
  decodeTransaction,
  getEncodedTransactionType,
  Transaction,
} from "../../../../dist/typescript/algo_models";
import path from "path";

describe("algo_models WASM", async () => {
  await init();

  const jsonString = await Bun.file(
    path.join(__dirname, "../test_data.json")
  ).text();

  const testData = JSON.parse(jsonString, (_, value) => {
    if (Array.isArray(value) && value.every((n) => typeof n === "number")) {
      return new Uint8Array(value);
    }

    if (
      typeof value === "number" &&
      ["fee", "amount", "firstValid", "lastValid"].includes(_)
    ) {
      return BigInt(value);
    }

    return value;
  });

  const privKey = testData.privKey;

  describe("payment", () => {
    const transaction: Transaction = testData.transaction;
    const expectedSignedTxn = testData.expectedSignedTxn;
    const expectedBytesForSigning = testData.expectedBytesForSigning;

    test("encode", () => {
      expect(encodeTransaction(transaction)).toEqual(expectedBytesForSigning);
    });

    test("encode with signature", async () => {
      const sig = await ed.signAsync(expectedBytesForSigning, privKey);
      const signedTx = attachSignature(expectedBytesForSigning, sig);
      expect(signedTx).toEqual(expectedSignedTxn);
    });

    test("decode (with TX prefix)", () => {
      expect(decodeTransaction(expectedBytesForSigning)).toEqual(transaction);
    });

    test("decode (without TX prefix)", () => {
      expect(decodeTransaction(expectedBytesForSigning.slice(2))).toEqual(transaction);
    });

    test("getEncodedTransactionType", () => {
      expect(getEncodedTransactionType(expectedBytesForSigning)).toBe(
        "Payment"
      );
    });

    // TODO: Decide if this is the behavior we want or if there should be input validation on encode
    test("encode/decode with extra field", () => {
      const extraField = { ...transaction, foo: "bar" };
      const encoded = encodeTransaction(extraField);
      expect(decodeTransaction(encoded)).not.toContainKey("foo");
    });

    test("DecodingError: 0 bytes", () => {
      expect(() => decodeTransaction(new Uint8Array(0))).toThrow(
        "DecodingError: attempted to decode 0 bytes"
      );
    });

    test("DecodingError: malformed bytes", () => {
      const badBytes = expectedBytesForSigning.slice();
      badBytes[13] = 37;
      expect(() => decodeTransaction(badBytes)).toThrow(
        "DecodingError: Error ocurred during decoding: missing field `fee`"
      );
    });

    test("Error: invalid type", () => {
      const badFields = { ...transaction, header: { fee: "foo" } };
      // @ts-expect-error known bad type for testing purposes
      expect(() => encodeTransaction(badFields)).toThrow(
        'Error: invalid type: string "foo", expected u64'
      );
    });
  });
});
