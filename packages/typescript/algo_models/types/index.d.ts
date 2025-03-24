/* tslint:disable */
/* eslint-disable */
/**
 * Get the transaction type from the encoded transaction.
 * This is particularly useful when decoding a transaction that has an unknown type
 */
export function getEncodedTransactionType(bytes: Uint8Array): TransactionType;
export function encodeTransaction(tx: Transaction): Uint8Array;
export function decodeTransaction(bytes: Uint8Array): Transaction;
export function attachSignature(encoded_tx: Uint8Array, signature: Uint8Array): Uint8Array;
export function addressFromPubKey(pub_key: Uint8Array): Address;
export function addressFromString(address: string): Address;
export type TransactionType = "Payment" | "AssetTransfer" | "AssetFreeze" | "AssetConfig" | "KeyRegistration" | "ApplicationCall";

export interface Address {
    address: string;
    pubKey: Uint8Array;
}

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
    sender: Address;
    fee: bigint;
    firstValid: bigint;
    lastValid: bigint;
    genesisHash?: Uint8Array;
    genesisId?: string;
    note?: Uint8Array;
    rekeyTo?: Address;
    lease?: Uint8Array;
    group?: Uint8Array;
}

export interface PayTransactionFields {
    receiver: Address;
    amount: bigint;
    closeRemainderTo?: Address;
}

export interface AssetTransferTransactionFields {
    assetId: bigint;
    amount: bigint;
    receiver: Address;
    assetSender?: Address;
    closeRemainderTo?: Address;
}

export interface Transaction {
    header: TransactionHeader;
    payFields?: PayTransactionFields;
    assetTransferFields?: AssetTransferTransactionFields;
}

