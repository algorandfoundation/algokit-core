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

    // TODO: Test exceptions
  });
});
