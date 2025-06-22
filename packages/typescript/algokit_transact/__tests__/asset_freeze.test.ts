import { expect, test, describe } from "bun:test";
import * as ed from "@noble/ed25519";
import {
  encodeTransaction,
  decodeTransaction,
  getEncodedTransactionType,
  Transaction,
  addressFromPubKey,
  addressFromString,
  getTransactionIdRaw,
  getTransactionId,
  assignFee,
  SignedTransaction,
  encodeSignedTransaction,
} from "..";

// We'll need to add test data once implementation is complete
// const assetFreeze = testData.assetFreeze;
// const assetUnfreeze = testData.assetUnfreeze;

describe("Asset Freeze", () => {
  describe("Transaction Tests", () => {
    test("example", async () => {
      const aliceSk = ed.utils.randomPrivateKey();
      const alicePubKey = await ed.getPublicKeyAsync(aliceSk);
      const alice = addressFromPubKey(alicePubKey);
      const targetAccount = addressFromString("JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA");

      // Example 1: Freeze an asset
      const freezeTxn: Transaction = {
        transactionType: "AssetFreeze",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65), // pretend this is a valid hash
        genesisId: "localnet",
        assetFreeze: {
          assetId: 12345n,
          freezeTarget: targetAccount,
          frozen: true,
        },
      };

      const freezeTxnWithFee = assignFee(freezeTxn, { feePerByte: 0n, minFee: 1000n });
      expect(freezeTxnWithFee.fee).toBe(1000n);

      // Example 2: Unfreeze an asset
      const unfreezeTxn: Transaction = {
        transactionType: "AssetFreeze",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65),
        genesisId: "localnet",
        assetFreeze: {
          assetId: 12345n,
          freezeTarget: targetAccount,
          frozen: false,
        },
      };

      const unfreezeTxnWithFee = assignFee(unfreezeTxn, { feePerByte: 0n, minFee: 1000n });
      expect(unfreezeTxnWithFee.fee).toBe(1000n);
    });

    test("asset freeze transaction encoding", async () => {
      const aliceSk = ed.utils.randomPrivateKey();
      const alicePubKey = await ed.getPublicKeyAsync(aliceSk);
      const alice = addressFromPubKey(alicePubKey);
      const targetAccount = addressFromString("JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA");

      const freezeTxn: Transaction = {
        transactionType: "AssetFreeze",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        fee: 1000n,
        genesisHash: new Uint8Array(32).fill(65),
        genesisId: "localnet",
        assetFreeze: {
          assetId: 12345n,
          freezeTarget: targetAccount,
          frozen: true,
        },
      };

      // Test encoding and decoding
      const encoded = encodeTransaction(freezeTxn);
      const decoded = decodeTransaction(encoded);

      expect(decoded.transactionType).toBe("AssetFreeze");
      expect(decoded.assetFreeze?.assetId).toBe(12345n);
      expect(decoded.assetFreeze?.frozen).toBe(true);
      expect(decoded.assetFreeze?.freezeTarget.address).toBe(targetAccount.address);

      // Test transaction type detection
      expect(getEncodedTransactionType(encoded)).toBe("AssetFreeze");
    });

    test("get transaction id", async () => {
      const aliceSk = ed.utils.randomPrivateKey();
      const alicePubKey = await ed.getPublicKeyAsync(aliceSk);
      const alice = addressFromPubKey(alicePubKey);
      const targetAccount = addressFromString("JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA");

      const freezeTxn: Transaction = {
        transactionType: "AssetFreeze",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        fee: 1000n,
        genesisHash: new Uint8Array(32).fill(65),
        genesisId: "localnet",
        assetFreeze: {
          assetId: 12345n,
          freezeTarget: targetAccount,
          frozen: true,
        },
      };

      const txId = getTransactionId(freezeTxn);
      const txIdRaw = getTransactionIdRaw(freezeTxn);

      expect(txId.length).toBeGreaterThan(0);
      expect(txIdRaw.length).toBe(32);
    });
  });
});
