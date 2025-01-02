import { expect, test, describe } from "bun:test";
import * as ed from "@noble/ed25519";
import init, {
  type PayTransactionFields,
  encodePayment,
  attachSignature,
  decodePayment,
  getEncodedTransactionType,
} from "./pkg/algo_models_ffi";
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
    const fields: PayTransactionFields = testData.fields;
    const expectedSignedTxn = testData.expectedSignedTxn;
    const expectedBytesForSigning = testData.expectedBytesForSigning;

    test("encode", () => {
      expect(encodePayment(fields)).toEqual(expectedBytesForSigning);
    });

    test("encode with signature", async () => {
      const sig = await ed.signAsync(expectedBytesForSigning, privKey);
      const signedTx = attachSignature(expectedBytesForSigning, sig);
      expect(signedTx).toEqual(expectedSignedTxn);
    });

    test("decode (with TX prefix)", () => {
      expect(decodePayment(expectedBytesForSigning)).toEqual(fields);
    });

    test("decode (without TX prefix)", () => {
      expect(decodePayment(expectedBytesForSigning.slice(2))).toEqual(fields);
    });

    test("getEncodedTransactionType", () => {
      expect(getEncodedTransactionType(expectedBytesForSigning)).toBe(
        "Payment"
      );
    });

    // TODO: Decide if this is the behavior we want or if there should be input validation on encode
    test("encode/decode with extra field", () => {
      const extraField = { ...fields, foo: "bar" };
      const encoded = encodePayment(extraField);
      expect(decodePayment(encoded)).not.toContainKey("foo");
    });

    test("DecodingError: 0 bytes", () => {
      expect(() => decodePayment(new Uint8Array(0))).toThrow(
        "DecodingError: attempted to decode 0 bytes"
      );
    });

    test("DecodingError: malformed bytes", () => {
      const badBytes = expectedBytesForSigning.slice();
      badBytes[13] = 37;
      expect(() => decodePayment(badBytes)).toThrow(
        "DecodingError: Error ocurred during decoding: missing field `fee`"
      );
    });

    test("Error: invalid type", () => {
      const badFields = { ...fields, header: { fee: "foo" } };
      // @ts-expect-error known bad type for testing purposes
      expect(() => encodePayment(badFields)).toThrow(
        'Error: invalid type: string "foo", expected u64'
      );
    });
  });
});
