/**
 * Returns the given array split into chunks of `batchSize` batches.
 * @param array The array to chunk
 * @param batchSize The size of batches to split the array into
 * @returns A generator that yields the array split into chunks of `batchSize` batches
 */
import { Buffer } from 'buffer'

export function* chunkArray<T>(array: T[], batchSize: number): Generator<T[], void> {
  for (let i = 0; i < array.length; i += batchSize) yield array.slice(i, i + batchSize)
}

/**
 * Creates a standard `Error` instance with an optional `cause` for broader runtime support.
 * @param message The error message
 * @param cause Optional underlying cause to attach to the error
 * @returns An Error instance with the supplied message and optional cause
 */
export function createError(message: string, cause?: unknown): Error {
  const error = new Error(message)
  if (cause !== undefined) {
    ;(error as { cause?: unknown }).cause = cause
  }
  return error
}

export function toBytes(value: Uint8Array | string): Uint8Array {
  if (typeof value === 'string') {
    return Uint8Array.from(Buffer.from(value, 'base64'))
  }

  return new Uint8Array(value)
}

export function bytesToBase64(value: Uint8Array): string {
  return Buffer.from(value.buffer, value.byteOffset, value.byteLength).toString('base64')
}

export function bytesToUtf8(value: Uint8Array): string {
  return Buffer.from(value.buffer, value.byteOffset, value.byteLength).toString('utf-8')
}

export function ensureDecodedBytes(bytes: Uint8Array): Uint8Array {
  try {
    const buffer = Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength)
    const str = buffer.toString('utf8')
    if (
      str.length > 0 &&
      /^[A-Za-z0-9+/]*={0,2}$/.test(str) &&
      (str.includes('=') || str.includes('+') || str.includes('/') || (str.length % 4 === 0 && str.length >= 8))
    ) {
      const decoded = Buffer.from(str, 'base64')
      if (!decoded.equals(buffer)) {
        return new Uint8Array(decoded)
      }
    }
  } catch {
    // Not valid UTF-8 or base64, return as-is
  }
  return bytes
}
