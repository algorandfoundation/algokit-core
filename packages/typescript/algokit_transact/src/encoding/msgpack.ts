import { decode as msgpackDecode, encode as msgpackEncode } from '@msgpack/msgpack'

export function encodeMsgpack<T>(data: T): Uint8Array {
  return new Uint8Array(msgpackEncode(data, { sortKeys: true, ignoreUndefined: true, useBigInt64: true }))
}

export function decodeMsgpack<T>(encoded: Uint8Array): T {
  return msgpackDecode(encoded) as T
}
