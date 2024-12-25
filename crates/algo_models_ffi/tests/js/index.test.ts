import { expect, test, beforeAll, describe, beforeEach } from "bun:test";
import * as ed from "@noble/ed25519";
import init, {
  type PayTransactionFields,
  encodePayment,
  attachSignature,
  decodePayment,
  getEncodedTransactionType,
} from "./pkg/algo_models_ffi";
let { AlgorandTransactionCrafter } = require("@algorandfoundation/algo-models");

describe("algo_models WASM", async () => {
  let privKey: Uint8Array;

  beforeAll(async () => {
    privKey = ed.utils.randomPrivateKey();
    await init();
  });

  describe("payment", () => {
    let fields: PayTransactionFields;
    let validTxn: any;
    let expectedBytesForSigning: Uint8Array;
    let expectedSignedTxn: Uint8Array;
    let expectedSig: Uint8Array;

    beforeAll(async () => {
      const genId = "testnet-v1.0";
      const genesisHash = "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=";
      const amount = 1000000;
      const from = "TIQ4WPFJQYSP2SKLSCDWTK2IIQQ6FOS6BHYIYDGRUZSSROJC5P3HBCZ67Y";
      const to = "66LKPOMVQJL2YVMTAVULQVZMZZCD5M2YVWA7KRHEOHYOJU5KLH2PB7HRRY";

      const algoCrafter = new AlgorandTransactionCrafter(genId, genesisHash);

      validTxn = algoCrafter
        .pay(amount, from, to)
        .addFirstValidRound(1000)
        .addLastValidRound(1500)
        .get();

      fields = {
        header: {
          sender: validTxn.snd,
          fee: BigInt(validTxn.fee),
          transactionType: "Payment",
          firstValid: BigInt(validTxn.fv),
          lastValid: BigInt(validTxn.lv),
          genesisHash: validTxn.gh,
          genesisId: validTxn.gen,
        },
        receiver: validTxn.rcv,
        amount: BigInt(validTxn.amt),
      } as PayTransactionFields;

      expectedBytesForSigning = validTxn.encode();
      expectedSig = await ed.signAsync(expectedBytesForSigning, privKey);

      expectedSignedTxn = algoCrafter.addSignature(
        expectedBytesForSigning,
        expectedSig
      );
    });

    let bytesForSigning: Uint8Array;

    beforeEach(() => {
      bytesForSigning = encodePayment(fields);
    });

    test("encode", () => {
      expect(bytesForSigning).toEqual(expectedBytesForSigning);
    });

    test("encode with signature", async () => {
      const sig = await ed.signAsync(bytesForSigning, privKey);
      const signedTx = attachSignature(bytesForSigning, sig);
      expect(signedTx).toEqual(expectedSignedTxn);
    });

    test("decode (with TX prefix)", () => {
      expect(decodePayment(bytesForSigning)).toEqual(fields);
    });

    test("decode (without TX prefix)", () => {
      expect(decodePayment(bytesForSigning.slice(2))).toEqual(fields);
    });

    test("getEncodedTransactionType", () => {
      expect(getEncodedTransactionType(bytesForSigning)).toBe("Payment");
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
      const badBytes = bytesForSigning.slice();
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
