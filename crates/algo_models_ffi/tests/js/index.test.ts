import { expect, test, beforeAll, describe, beforeEach } from "bun:test";
import * as ed from "@noble/ed25519";
import init, {
  type PayTransactionFields,
  encodePayment,
  attachSignature,
  decodePayment,
  getEncodedTransactionType,
  TransactionType,
} from "./pkg/algo_models_ffi";

describe("algo_models WASM", async () => {
  const privKey: Uint8Array = new Uint8Array([
    228, 37, 78, 229, 20, 67, 165, 234, 193, 70, 211, 78, 7, 165, 168, 197, 144,
    243, 236, 114, 114, 147, 229, 16, 106, 15, 230, 26, 218, 8, 39, 3,
  ]);

  const getHeader = (transactionType: TransactionType) => {
    return {
      sender: new Uint8Array([
        154, 33, 203, 60, 169, 134, 36, 253, 73, 75, 144, 135, 105, 171, 72, 68,
        33, 226, 186, 94, 9, 240, 140, 12, 209, 166, 101, 40, 185, 34, 235, 246,
      ]),
      fee: 1000n,
      transactionType,
      firstValid: 1000n,
      lastValid: 1500n,
      genesisHash: new Uint8Array([
        72, 99, 181, 24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203,
        15, 113, 240, 89, 167, 172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34,
      ]),
      genesisId: "testnet-v1.0",
    };
  };

  beforeAll(async () => {
    await init();
  });

  describe("payment", () => {
    const fields: PayTransactionFields = {
      header: getHeader("Payment"),
      receiver: new Uint8Array([
        247, 150, 167, 185, 149, 130, 87, 172, 85, 147, 5, 104, 184, 87, 44,
        206, 68, 62, 179, 88, 173, 129, 245, 68, 228, 113, 240, 228, 211, 170,
        89, 244,
      ]),
      amount: 1000000n,
    };
    const expectedSignedTxn = new Uint8Array([
      130, 163, 115, 105, 103, 196, 64, 110, 97, 3, 133, 117, 193, 222, 221,
      115, 216, 59, 216, 123, 232, 226, 237, 220, 171, 1, 51, 24, 243, 49, 148,
      31, 181, 185, 71, 7, 169, 76, 183, 120, 29, 137, 177, 17, 145, 94, 113,
      167, 18, 0, 78, 232, 191, 71, 207, 8, 173, 81, 111, 210, 48, 201, 198,
      136, 107, 186, 3, 207, 102, 192, 3, 163, 116, 120, 110, 137, 163, 97, 109,
      116, 206, 0, 15, 66, 64, 163, 102, 101, 101, 205, 3, 232, 162, 102, 118,
      205, 3, 232, 163, 103, 101, 110, 172, 116, 101, 115, 116, 110, 101, 116,
      45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72, 99, 181, 24, 164, 179,
      200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240, 89, 167, 172,
      32, 222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 205, 5, 220,
      163, 114, 99, 118, 196, 32, 247, 150, 167, 185, 149, 130, 87, 172, 85,
      147, 5, 104, 184, 87, 44, 206, 68, 62, 179, 88, 173, 129, 245, 68, 228,
      113, 240, 228, 211, 170, 89, 244, 163, 115, 110, 100, 196, 32, 154, 33,
      203, 60, 169, 134, 36, 253, 73, 75, 144, 135, 105, 171, 72, 68, 33, 226,
      186, 94, 9, 240, 140, 12, 209, 166, 101, 40, 185, 34, 235, 246, 164, 116,
      121, 112, 101, 163, 112, 97, 121,
    ]);
    const expectedBytesForSigning = new Uint8Array([
      84, 88, 137, 163, 97, 109, 116, 206, 0, 15, 66, 64, 163, 102, 101, 101,
      205, 3, 232, 162, 102, 118, 205, 3, 232, 163, 103, 101, 110, 172, 116,
      101, 115, 116, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32,
      72, 99, 181, 24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203,
      15, 113, 240, 89, 167, 172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34,
      162, 108, 118, 205, 5, 220, 163, 114, 99, 118, 196, 32, 247, 150, 167,
      185, 149, 130, 87, 172, 85, 147, 5, 104, 184, 87, 44, 206, 68, 62, 179,
      88, 173, 129, 245, 68, 228, 113, 240, 228, 211, 170, 89, 244, 163, 115,
      110, 100, 196, 32, 154, 33, 203, 60, 169, 134, 36, 253, 73, 75, 144, 135,
      105, 171, 72, 68, 33, 226, 186, 94, 9, 240, 140, 12, 209, 166, 101, 40,
      185, 34, 235, 246, 164, 116, 121, 112, 101, 163, 112, 97, 121,
    ]);

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
