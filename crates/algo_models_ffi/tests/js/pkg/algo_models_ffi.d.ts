/* tslint:disable */
/* eslint-disable */
/**
 * Get the transaction type from the encoded transaction.
 * This is particularly useful when decoding a transaction that has a unknow type
 */
export function getEncodedTransactionType(bytes: Uint8Array): TransactionType;
export function encodePayment(tx: PayTransactionFields): Uint8Array;
export function decodePayment(bytes: Uint8Array): PayTransactionFields;
export function encodeAssetTransfer(tx: AssetTransferTransactionFields): Uint8Array;
export function decodeAssetTransfer(bytes: Uint8Array): AssetTransferTransactionFields;
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
    header: TransactionHeader;
    receiver: Uint8Array;
    amount: bigint;
    closeRemainderTo?: Uint8Array;
}

export interface AssetTransferTransactionFields {
    header: TransactionHeader;
    assetId: bigint;
    amount: bigint;
    receiver: Uint8Array;
    assetSender?: Uint8Array;
    closeRemainderTo?: Uint8Array;
}


export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly getEncodedTransactionType: (a: number, b: number) => [number, number, number];
  readonly encodePayment: (a: any) => [number, number, number, number];
  readonly decodePayment: (a: number, b: number) => [number, number, number];
  readonly encodeAssetTransfer: (a: any) => [number, number, number, number];
  readonly decodeAssetTransfer: (a: number, b: number) => [number, number, number];
  readonly attachSignature: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
