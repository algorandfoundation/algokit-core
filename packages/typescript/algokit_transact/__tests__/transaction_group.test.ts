import { expect, test, describe } from "bun:test";
import { testData } from "./common.ts";
import { attachSignatures, encodeTransactions, groupTransactions } from "../src/index";
import * as ed from "@noble/ed25519";
import { writeFileSync } from "fs";

const simplePayment = testData.simplePayment;
const optInAssetTransfer = testData.optInAssetTransfer;

const getTestData = () => {
  const expectedGroupId = Uint8Array.from([
    202, 79, 82, 7, 197, 237, 213, 55, 117, 226, 131, 74, 221, 85, 86, 215, 64, 133, 212, 7, 58, 234, 248, 162, 222, 53, 161, 29, 141, 101,
    133, 49,
  ]);
  const txs = [simplePayment.transaction, optInAssetTransfer.transaction];

  return {
    txs,
    expectedGroupId,
  };
};

describe("Transaction Group", () => {
  // Polytest Suite: Transaction Group

  describe("Transaction Group Tests", () => {
    // Polytest Group: Transaction Group Tests

    test("group transactions", () => {
      const { txs, expectedGroupId } = getTestData();

      const groupedTxs = groupTransactions(txs);

      expect(groupedTxs.length).toBe(txs.length);
      for (let i = 0; i < txs.length; i++) {
        expect(txs[i].group).toBeUndefined();
        expect(groupedTxs[i].group).toEqual(expectedGroupId);
      }
    });

    test("encode transactions", () => {
      const { txs } = getTestData();
      const groupedTxs = groupTransactions(txs);

      const encodedGroupedTxs = encodeTransactions(groupedTxs);

      expect(encodedGroupedTxs.length).toBe(txs.length);
      for (let i = 0; i < encodedGroupedTxs.length; i++) {
        expect(encodedGroupedTxs[i].length).toBeGreaterThan(0);
      }
    });

    test("encode transactions with signature", async () => {
      const { txs } = getTestData();
      const groupedTxs = groupTransactions(txs);
      const encodedGroupedTxs = encodeTransactions(groupedTxs);
      const tx1Sig = await ed.signAsync(encodedGroupedTxs[0], simplePayment.signingPrivateKey);
      const tx2Sig = await ed.signAsync(encodedGroupedTxs[1], optInAssetTransfer.signingPrivateKey);

      // TODO: NC - The above setup is not ideal.
      // TODO: NC - Do we want a worked example like in the payment test?

      const encodedSignedGroupedTxs = attachSignatures(encodedGroupedTxs, [tx1Sig, tx2Sig]);

      expect(encodedSignedGroupedTxs.length).toBe(txs.length);
      // TODO: NC - Do we want snapshots?
      expect(encodedSignedGroupedTxs[0]).toMatchSnapshot();
      expect(encodedSignedGroupedTxs[1]).toMatchSnapshot();

      // TODO: NC - Remove this code when done
      const totalLength = encodedSignedGroupedTxs.reduce((sum, arr) => sum + arr.length, 0);
      const concatenated = new Uint8Array(totalLength);
      let offset = 0;
      for (const arr of encodedSignedGroupedTxs) {
        concatenated.set(arr, offset);
        offset += arr.length;
      }

      writeFileSync("txn3.txt", concatenated);
    });
  });
});
