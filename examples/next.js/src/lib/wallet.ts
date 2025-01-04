import type { KeyPairRecord } from "./types.js";

import {createMethod} from "./sha512";
import {encode} from "./hi-base32";
import {saveKeyPair} from "@/lib/db";

const sha512_256 = createMethod(256);
const HASH_BYTES_LENGTH = 32;
const ALGORAND_CHECKSUM_BYTE_LENGTH = 4;
const ALGORAND_ADDRESS_LENGTH = 58;

export async function sign(txnBytes: BufferSource, kpr: KeyPairRecord){
    const sig = await crypto.subtle.sign(
        {name: 'Ed25519'},
        kpr.keyPair.privateKey,
        txnBytes,
    );
    return new Uint8Array(sig)
}
/**
 * Get the address of a key pair
 * @param keyPair
 */
export async function getAddress(keyPair: CryptoKeyPair){
    const key= new Uint8Array(await crypto.subtle.exportKey("raw", keyPair.publicKey))
    return encodeAddress(key)
}

export async function exportKey(kpr: KeyPairRecord){
    console.log(kpr)
    return await crypto.subtle.exportKey("pkcs8", kpr.keyPair.privateKey)
}
/**
 * Create a new key pair
 * @param extractable
 * @param keyUsages
 */
export async function generateKey(extractable: boolean = false, keyUsages: KeyUsage[] = ["sign"]){
    const keyPair = (await crypto.subtle.generateKey({name: 'Ed25519'}, extractable, keyUsages) as CryptoKeyPair)
    await saveKeyPair(keyPair)
    return {keyPair, id: await getAddress(keyPair)} as KeyPairRecord
}

/**
 * Encode a public key to an address
 * @param publicKey
 */
export function encodeAddress(publicKey: Uint8Array) {
    // Concatenate the publicKey and the checksum, remove the extra '====' and return the base32 encoding
    return `${encode([...publicKey, ...generateChecksum(publicKey)])}`.slice(0, ALGORAND_ADDRESS_LENGTH);
}

/**
 * Generate the checksum of a public key
 * @param publicKey
 */
export function generateChecksum(publicKey: Uint8Array){
    return sha512_256.array(publicKey)
        .slice(
            HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH,
            HASH_BYTES_LENGTH
        );
}