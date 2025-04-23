import { expect, test, describe } from "bun:test";
import { testData } from "./common.ts";
import * as ed from "@noble/ed25519";
import {
  encodeTransaction,
  attachSignature,
  decodeTransaction,
  getEncodedTransactionType,
  Transaction,
  addressFromPubKey,
  addressFromString,
  getTransactionRawId,
  getTransactionId,
} from "../src/index";

const transaction: Transaction = testData.transaction;
const expectedSignedTxn = testData.expectedSignedTxn;
const expectedBytesForSigning = testData.expectedBytesForSigning;
const privKey = testData.privKey;

describe("Payment", () => {
  // Polytest Suite: Payment

  describe("Transaction Tests", () => {
    // Polytest Group: Transaction Tests

    test("decode without prefix", () => {
      expect(decodeTransaction(expectedBytesForSigning.slice(2))).toEqual(
        transaction
      );
    });

    test("decode with prefix", () => {
      expect(decodeTransaction(expectedBytesForSigning)).toEqual(transaction);
    });

    test("example", async () => {
      const aliceSk = ed.utils.randomPrivateKey();
      const alicePubKey = await ed.getPublicKeyAsync(aliceSk);
      const alice = addressFromPubKey(alicePubKey);

      const bob = addressFromString(
        "B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y"
      );

      const txn: Transaction = {
        transactionType: "Payment",
        sender: alice,
        fee: 1000n,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65), // pretend this is a valid hash
        genesisId: "localnet",
        payment: {
          amount: 1337n,
          receiver: bob,
        },
      };

      const sig = await ed.signAsync(encodeTransaction(txn), aliceSk);
      const signedTxn = attachSignature(encodeTransaction(txn), sig);
    });

    test("get encoded transaction type", () => {
      expect(getEncodedTransactionType(expectedBytesForSigning)).toBe(
        "Payment"
      );
    });

    test("encode with signature", async () => {
      const sig = await ed.signAsync(expectedBytesForSigning, privKey);
      const signedTx = attachSignature(expectedBytesForSigning, sig);
      expect(signedTx).toEqual(expectedSignedTxn);
    });

    test("encode", () => {
      expect(encodeTransaction(transaction)).toEqual(expectedBytesForSigning);
    });

    test("get transaction id", () => {
      expect(getTransactionRawId(transaction)).toEqual(
        Uint8Array.from([
          89, 237, 187, 95, 72, 48, 184, 21, 54, 185, 237, 245, 160, 212, 160,
          212, 214, 207, 239, 131, 123, 133, 183, 247, 179, 37, 169, 90, 79, 19,
          170, 171,
        ])
      );
      expect(getTransactionId(transaction)).toEqual(
        "LHW3WX2IGC4BKNVZ5X22BVFA2TLM734DPOC3P55TEWUVUTYTVKVQ"
      );
    });
  });
});
