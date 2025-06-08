import * as path from "path";
import { Transaction } from "..";

const jsonString = await Bun.file(path.join(__dirname, "../../../../crates/algokit_transact_ffi/test_data.json")).text();

const defaultReviver = (key: string, value: unknown) => {
  if (Array.isArray(value) && value.every((n) => typeof n === "number")) {
    return new Uint8Array(value);
  }

  if (typeof value === "number" && ["fee", "amount", "firstValid", "lastValid"].includes(key)) {
    return BigInt(value);
  }

  return value;
};

export const parseJson = <T = any>(json: string, reviver: (_: string, value: unknown) => unknown = defaultReviver) => {
  return JSON.parse(json, reviver) as T;
};

type TransactionTestData = {
  transaction: Transaction;
  id: string;
  idRaw: Uint8Array;
  unsignedBytes: Uint8Array;
  signedBytes: Uint8Array;
  signingPrivateKey: Uint8Array;
};

export const testData = parseJson<Record<"simplePayment" | "optInAssetTransfer", TransactionTestData>>(jsonString);
