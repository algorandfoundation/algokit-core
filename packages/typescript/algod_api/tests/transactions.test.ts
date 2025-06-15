import { describe, test, expect, beforeAll } from "vitest";
import { algorandFixture } from "@algorandfoundation/algokit-utils/testing";
import * as algosdk from "algosdk";
import * as algodPackage from "@algorandfoundation/algokit-algod-api";
import { encodeTransaction, Transaction, Address } from "@algorandfoundation/algokit-transact";
import { Blob } from "buffer";
import { HttpFile } from "../http/http";
import { TransactionSignerAccount } from "@algorandfoundation/algokit-utils/types/account";

// Helper function to create and sign a transaction, then return it as an HttpFile
const createSignedTxnHttpFile = (
  sender: algosdk.Address & TransactionSignerAccount & algosdk.Account,
  receiver: Address,
  amount: number | bigint,
  apiSuggestedParams: algodPackage.TransactionParams200Response,
  note?: Uint8Array,
): HttpFile => {
  const transactionHeaderParams = {
    fee: BigInt(apiSuggestedParams.minFee || apiSuggestedParams.fee),
    firstValid: BigInt(apiSuggestedParams.lastRound),
    lastValid: BigInt(apiSuggestedParams.lastRound) + BigInt(1000),
    genesisHash: new Uint8Array(Buffer.from(apiSuggestedParams.genesisHash, "base64")),
    genesisId: apiSuggestedParams.genesisId,
  };

  const txn: Transaction = {
    transactionType: "Payment",
    payment: {
      amount: BigInt(amount),
      receiver: receiver,
    },
    sender: { address: String(sender.addr), pubKey: sender.publicKey },
    note: note,
    ...transactionHeaderParams,
  };

  const encodedTxn = encodeTransaction(txn);
  const algosdkTxn = algosdk.decodeUnsignedTransaction(Buffer.from(encodedTxn.slice(2)));
  const signedTxnBytes = algosdkTxn.signTxn(sender.sk);

  const binaryData = new Blob([signedTxnBytes], { type: "application/x-binary" });
  // @ts-ignore - we know this matches the HttpFile structure
  // TODO: find more elegant way to do this
  const httpFile: HttpFile = Object.assign(binaryData, { name: "transaction.txn" });
  return httpFile;
};

describe("Transaction API Tests", () => {
  const fixture = algorandFixture();
  let algodApi: algodPackage.AlgodApi;

  beforeAll(async () => {
    await fixture.newScope();
    const authConfig: algodPackage.AuthMethodsConfiguration = {
      api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    };
    const serverConfig = new algodPackage.ServerConfiguration("http://localhost:4001", {});
    const configurationParameters = {
      httpApi: new algodPackage.IsomorphicFetchHttpLibrary(),
      baseServer: serverConfig,
      authMethods: authConfig,
      promiseMiddleware: [],
    };
    const config = algodPackage.createConfiguration(configurationParameters);
    algodApi = new algodPackage.AlgodApi(config);
  });

  test("should get transaction parameters", async () => {
    const params = await algodApi.transactionParams();
    expect(params).toBeDefined();
    expect(params.consensusVersion).toBeTypeOf("string");
    expect(params.fee).toBeTypeOf("number");
    expect(params.genesisHash).toBeTypeOf("string");
    expect(params.genesisId).toBeTypeOf("string");
    expect(params.lastRound).toBeTypeOf("number");
    expect(params.minFee).toBeTypeOf("number");
  });

  test("should submit a raw transaction", async () => {
    const { testAccount: sender } = fixture.context;
    const suggestedParams = await algodApi.transactionParams();

    const httpFile = createSignedTxnHttpFile(sender, { address: String(sender.addr), pubKey: sender.publicKey }, 100000, suggestedParams);

    const result = await algodApi.rawTransaction(httpFile);
    expect(result).not.toBeNull();
    expect(result?.txId).toBeDefined();
  });

  test("should get pending transaction info", async () => {
    const { testAccount: sender } = fixture.context;
    const suggestedParams = await algodApi.transactionParams();
    const note = new Uint8Array(Buffer.from("pending test"));

    const httpFile = createSignedTxnHttpFile(
      sender,
      { address: String(sender.addr), pubKey: sender.publicKey },
      100000,
      suggestedParams,
      note,
    );

    const submitResult = await algodApi.rawTransaction(httpFile);
    expect(submitResult.txId).toBeDefined();

    await new Promise((resolve) => setTimeout(resolve, 100));

    const pendingTxnsResult = await algodApi.getPendingTransactions(10);
    expect(pendingTxnsResult).toBeDefined();
  });

  test.each(["json", "msgpack"])("should simulate transaction with format %s", async (format) => {
    const { testAccount: sender } = fixture.context;

    const suggestedParams = await algodApi.transactionParams();
    const signedTxnFile = createSignedTxnHttpFile(
      sender,
      { address: String(sender.addr), pubKey: sender.publicKey },
      100000,
      suggestedParams,
    );

    // Note: This implementation differs from algosdk's simulateTransaction method.
    // Here we manually convert the signed transaction to base64-encoded string,
    // while algosdk takes an array of SignedTransaction objects and handles the encoding internally.
    const txnGroup: algodPackage.SimulateRequestTransactionGroup = {
      "txns": [Buffer.from(await signedTxnFile.arrayBuffer()).toString("base64")],
    }

    const traceConfig: algodPackage.SimulateTraceConfig = {
      "enable": true,
      "stackChange": true,
      "scratchChange": true,
      "stateChange": true,
    }

    const simulateRequest: algodPackage.SimulateRequest = {
      "allowEmptySignatures": true,
      "allowMoreLogging": true,
      "allowUnnamedResources": true,
      "txnGroups": [txnGroup],
      "execTraceConfig": traceConfig,
    }

    const result = await algodApi.simulateTransaction(simulateRequest, format as "msgpack" | "json", {
      headers: { "Content-Type": `application/${format}` },
    } as any);

    expect(result).toBeDefined();
  });

  // TODO: Add more tests based on other endpoints in AlgodApi related to transactions
});
