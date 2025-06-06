// Even with module and exports defined, bun still prefers main for some reason.
// See https://github.com/oven-sh/bun/issues/13430#issuecomment-2949903681
import { addressFromString, Composer, Transaction } from "../dist/algokit_utils.bundler.mjs";
import { expect, describe, test } from "bun:test";

class AlgodClient {
  async json(path: string) {
    const response = await (await fetch("https://testnet-api.4160.nodely.dev" + path)).text();

    console.debug("Response from algodClient:", response);
    return response;
  }
}

const algodClient = new AlgodClient();

describe("Composer", () => {
  test("create composer", () => {
    const composer = new Composer(algodClient);
    expect(composer).toBeDefined();
  });

  test("add transaction", () => {
    const composer = new Composer(algodClient);
    const tx: Transaction = {
      transactionType: "Payment",
      sender: addressFromString("LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"),
      firstValid: 1n,
      lastValid: 10n,
      genesisHash: Buffer.from("a".repeat(32)),
      genesisId: "",
      payment: {
        receiver: addressFromString("LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"),
        amount: 1000n, // microAlgos
      },
    };

    composer.addTransaction(tx);
    expect(composer.transactions.length).toBe(1);
  });

  test("toString", () => {
    const composer = new Composer(algodClient);
    const tx: Transaction = {
      transactionType: "Payment",
      sender: addressFromString("LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"),
      firstValid: 1n,
      lastValid: 10n,
      genesisHash: Buffer.from("a".repeat(32)),
      genesisId: "",
      payment: {
        receiver: addressFromString("LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"),
        amount: 1000n, // microAlgos
      },
    };

    composer.addTransaction(tx);
    expect(composer.toString()).toBeDefined();
    console.debug(composer.toString());

    console.debug(composer);
    console.debug(composer.valueOf());
    console.debug(JSON.stringify(composer, null, 2));
  });

  test("rustError", () => {
    const composer = new Composer(algodClient);
    expect(() => {
      composer.throwRustError();
    }).toThrow("TransactionsError: This is a Rust error thrown from the Composer");
  });

  test("params", async () => {
    const composer = new Composer(algodClient);
    const params = await composer.get_suggested_params();
    expect(params).toBeDefined();
    console.debug("Suggested Params:", params);
  });
});
