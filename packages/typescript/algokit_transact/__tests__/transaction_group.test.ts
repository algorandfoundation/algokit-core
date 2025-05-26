import { expect, test, describe } from "bun:test";
import { testData } from "./common.ts";
import { groupTransactions } from "../src/index";

const simplePayment = testData.simplePayment;
const optInAssetTransfer = testData.optInAssetTransfer;

describe("Transaction Group", () => {
  // Polytest Suite: Transaction Group

  describe("Transaction Group Tests", () => {
    // Polytest Group: Transaction Group Tests

  });

  describe("Transaction Group Tests", () => {
    // Polytest Group: Transaction Tests

    test("group transactions", () => {
      const expectedGroupId = Uint8Array.from([
        202, 79, 82, 7, 197, 237, 213, 55, 117, 226, 131, 74, 221, 85, 86, 215, 64, 133, 212, 7, 58, 234, 248, 162, 222, 53, 161, 29, 141,
        101, 133, 49,
      ]);
      const txs = [simplePayment.transaction, optInAssetTransfer.transaction];
      const grouped_txs = groupTransactions(txs);

      expect(grouped_txs.length).toBe(txs.length);
      for (let i = 0; i < txs.length; i++) {
        expect(txs[i].group).toBeUndefined();
        expect(grouped_txs[i].group).toEqual(expectedGroupId);
      }
    });
  });
});
