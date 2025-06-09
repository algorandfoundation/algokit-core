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
  getTransactionIdRaw,
  getTransactionId,
  assignFee,
} from "..";

const simplePayment = testData.simplePayment;

describe("Payment", () => {
  // Polytest Suite: Payment

  describe("Transaction Tests", () => {
    // Polytest Group: Transaction Tests

    test("decode without prefix", () => {
      expect(decodeTransaction(simplePayment.unsignedBytes.slice(2))).toEqual(simplePayment.transaction);
    });

    test("decode with prefix", () => {
      expect(decodeTransaction(simplePayment.unsignedBytes)).toEqual(simplePayment.transaction);
    });

    test("example", async () => {
      const aliceSk = ed.utils.randomPrivateKey();
      const alicePubKey = await ed.getPublicKeyAsync(aliceSk);
      const alice = addressFromPubKey(alicePubKey);

      const bob = addressFromString("B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y");

      const txn: Transaction = {
        transactionType: "Payment",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65), // pretend this is a valid hash
        genesisId: "localnet",
        payment: {
          amount: 1337n,
          receiver: bob,
        },
      };

      const txnWithFee = assignFee(txn, {
        feePerByte: 0n,
        minFee: 1000n,
      });

      expect(txnWithFee.fee).toBe(1000n);

      const sig = await ed.signAsync(encodeTransaction(txnWithFee), aliceSk);
      const signedTxn = attachSignature(encodeTransaction(txnWithFee), sig);
      expect(signedTxn.length).toBeGreaterThan(0);
    });

    test("get encoded transaction type", () => {
      expect(getEncodedTransactionType(simplePayment.unsignedBytes)).toBe(simplePayment.transaction.transactionType);
    });

    test("encode with signature", async () => {
      const sig = await ed.signAsync(simplePayment.unsignedBytes, simplePayment.signingPrivateKey);
      const signedTx = attachSignature(simplePayment.unsignedBytes, sig);
      expect(signedTx).toEqual(simplePayment.signedBytes);
    });

    test("encode", () => {
      expect(encodeTransaction(simplePayment.transaction)).toEqual(simplePayment.unsignedBytes);
    });

    test("get transaction id", () => {
      expect(getTransactionIdRaw(simplePayment.transaction)).toEqual(simplePayment.idRaw);
      expect(getTransactionId(simplePayment.transaction)).toEqual(simplePayment.id);
    });
  });
});
