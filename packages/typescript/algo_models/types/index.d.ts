/* tslint:disable */
/* eslint-disable */
/**
 * Get the transaction type from the encoded transaction.
 * This is particularly useful when decoding a transaction that has a unknow type
 */
export function getEncodedTransactionType(bytes: Uint8Array): TransactionType;
export function encodeTransaction(tx: Transaction): Uint8Array;
export function decodeTransaction(bytes: Uint8Array): Transaction;
export function attachSignature(encoded_tx: Uint8Array, signature: Uint8Array): Uint8Array;
export type TransactionType = "Payment" | "AssetTransfer" | "AssetFreeze" | "AssetConfig" | "KeyRegistration" | "ApplicationCall";

/**
 * The transaction header contains the fields that can be present in any transaction.
 * \"Header\" only indicates that these are common fields, NOT that they are the first fields in the transaction.
 */
export interface TransactionHeader {
    /**
     * The type of transaction
     */
    transactionType: TransactionType;
    /**
     * The sender of the transaction
     */
    sender: Uint8Array;
    fee: bigint;
    firstValid: bigint;
    lastValid: bigint;
    genesisHash?: Uint8Array;
    genesisId?: string;
    note?: Uint8Array;
    rekeyTo?: Uint8Array;
    lease?: Uint8Array;
    group?: Uint8Array;
}

export interface PayTransactionFields {
    receiver: Uint8Array;
    amount: bigint;
    closeRemainderTo?: Uint8Array;
}

export interface AssetTransferTransactionFields {
    assetId: bigint;
    amount: bigint;
    receiver: Uint8Array;
    assetSender?: Uint8Array;
    closeRemainderTo?: Uint8Array;
}

export interface Transaction {
    header: TransactionHeader;
    payFields?: PayTransactionFields;
    assetTransferFields?: AssetTransferTransactionFields;
}

