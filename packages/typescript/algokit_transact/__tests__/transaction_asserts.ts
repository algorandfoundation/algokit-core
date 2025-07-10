import {
  assignFee,
  decodeTransaction,
  encodeSignedTransaction,
  encodeTransaction,
  estimateTransactionSize,
  getEncodedTransactionType,
  getTransactionId,
  getTransactionIdRaw,
  SignedTransaction,
  MultisigSignature,
  MultisigSubsignature,
} from "..";
import { expect } from "bun:test";
import * as ed from "@noble/ed25519";
import { TransactionTestData } from "./common";

export const assertExample = async (label: string, testData: TransactionTestData) => {
  const signedTxn: SignedTransaction = {
    transaction: testData.transaction,
    signature: await ed.signAsync(encodeTransaction(testData.transaction), testData.signingPrivateKey),
  };
  const encodedSignedTxn = encodeSignedTransaction(signedTxn);

  expect(encodedSignedTxn, label).toEqual(testData.signedBytes);
};

export const assertTransactionId = (label: string, testData: TransactionTestData) => {
  expect(getTransactionIdRaw(testData.transaction), label).toEqual(testData.idRaw);
  expect(getTransactionId(testData.transaction), label).toEqual(testData.id);
};

export const assertEncodedTransactionType = (label: string, testData: TransactionTestData) => {
  expect(getEncodedTransactionType(testData.unsignedBytes), label).toBe(testData.transaction.transactionType);
};

export const assertDecodeWithoutPrefix = (label: string, testData: TransactionTestData) => {
  const decoded = decodeTransaction(testData.unsignedBytes.slice(2));
  expect(decoded, label).toEqual(testData.transaction);
};

export const assertDecodeWithPrefix = (label: string, testData: TransactionTestData) => {
  const decoded = decodeTransaction(testData.unsignedBytes);
  expect(decoded, label).toEqual(testData.transaction);
};

export const assertEncodeWithAuthAddress = async (label: string, testData: TransactionTestData) => {
  const sig = await ed.signAsync(testData.unsignedBytes, testData.signingPrivateKey);
  const signedTxn: SignedTransaction = {
    transaction: testData.transaction,
    signature: sig,
    authAddress: testData.rekeyedSenderAuthAddress,
  };
  const encodedSignedTxn = encodeSignedTransaction(signedTxn);

  expect(encodedSignedTxn, label).toEqual(testData.rekeyedSenderSignedBytes);
};

export const assertEncodeWithSignature = async (label: string, testData: TransactionTestData) => {
  const sig = await ed.signAsync(testData.unsignedBytes, testData.signingPrivateKey);
  const signedTxn: SignedTransaction = {
    transaction: testData.transaction,
    signature: sig,
  };
  const encodedSignedTxn = encodeSignedTransaction(signedTxn);

  expect(encodedSignedTxn, label).toEqual(testData.signedBytes);
};

export const assertEncode = (label: string, testData: TransactionTestData) => {
  expect(encodeTransaction(testData.transaction), label).toEqual(testData.unsignedBytes);
};

export const assertAssignFee = (label: string, testData: TransactionTestData) => {
  const minFee = BigInt(2000);
  const txnWithFee1 = assignFee(testData.transaction, { feePerByte: 0n, minFee });
  expect(txnWithFee1.fee, label).toEqual(minFee);

  const extraFee = BigInt(3000);
  const txnWithFee2 = assignFee(testData.transaction, { feePerByte: 0n, minFee, extraFee });
  expect(txnWithFee2.fee, label).toEqual(minFee + extraFee);

  const feePerByte = BigInt(100);
  const txnWithFee3 = assignFee(testData.transaction, { feePerByte, minFee: 1000n });
  const txnSize = estimateTransactionSize(testData.transaction);
  expect(txnWithFee3.fee, label).toEqual(txnSize * feePerByte);
};

export const assertMultisigExample = async (label: string, testData: TransactionTestData) => {
  const singleSig = await ed.signAsync(encodeTransaction(testData.transaction), testData.signingPrivateKey);

  // FIXME: Just as in the Python test, when we get a function that creates multisigs, that's what we should use here.
  const multisigSignature: MultisigSignature = {
    address: "",
    version: 1,
    threshold: 2,
    subsignatures: [
      {
        address: testData.multisigAddresses[0],
        signature: singleSig,
      },
      {
        address: testData.multisigAddresses[1],
        signature: singleSig,
      },
    ] as MultisigSubsignature[],
  };

  const signedTxn: SignedTransaction = {
    transaction: testData.transaction,
    multisignature: multisigSignature,
  };
  const encodedSignedTxn = encodeSignedTransaction(signedTxn);

  expect(encodedSignedTxn, label).toEqual(testData.multisigSignedBytes);
};
