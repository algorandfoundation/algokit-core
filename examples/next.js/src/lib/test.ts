import init, {
    type PayTransactionFields,
    encodePayment,
    attachSignature,
} from "algo_models";

let { AlgorandTransactionCrafter } = require("@algorandfoundation/algo-models");
import * as ed from "@noble/ed25519";

import * as msgpack from "algo-msgpack-with-bigint";
import {encodeAddress, exportKey, generateKey, sign} from "@/lib/wallet";
import {KeyPairRecord} from "@/lib/types";
// Generate a ed25519 keypair
const privKey = ed.utils.randomPrivateKey();
console.log(privKey)
await init();

// console.log(encodeAddress())

// How many times it should run
const iterations = 2000;

// Defaults
const genId = "testnet-v1.0";
const genesisHash = "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=";
const amount = 1000000;
const from = "TIQ4WPFJQYSP2SKLSCDWTK2IIQQ6FOS6BHYIYDGRUZSSROJC5P3HBCZ67Y";
const to = "66LKPOMVQJL2YVMTAVULQVZMZZCD5M2YVWA7KRHEOHYOJU5KLH2PB7HRRY";
const algoCrafter = new AlgorandTransactionCrafter(genId, genesisHash);
const tx = algoCrafter
    .pay(amount, from, to)
    .addFirstValidRound(1000)
    .addLastValidRound(1500)
    // .addNote("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla hendrerit quam elit. Ut congue aliquet orci vitae laoreet. Nam pulvinar ex purus, et euismod odio ornare molestie. Nulla laoreet lectus diam, non efficitur arcu consequat id. Sed efficitur orci dui. Maecenas pulvinar massa sit amet auctor suscipit. Ut ipsum nibh, facilisis eget dolor a, egestas rutrum purus. Nulla molestie elit et turpis iaculis pellentesque. Nam augue risus, congue quis mauris ut, finibus auctor enim. Sed id sapien pretium lacus interdum finibus. Ut fermentum, ipsum vel sodales pellentesque, arcu ante tempor nisl, nec sollicitudin purus tellus ut mauris. Vivamus rhoncus, odio sed porta cursus, turpis arcu dictum purus, at hendrerit nisl dui in elit. Mauris gravida gravida lorem, ac aliquam mi pharetra in. Aenean sed malesuada libero. Curabitur sollicitudin nisi in sagittis posuere. Aliquam sit amet ante augue.Nullam vitae bibendum felis. Pellentesque interdum molestie neque eget pretium. Donec non bibendum nisi, sed faucibus nisi")
    .get();

let nativeKey: KeyPairRecord | undefined;

async function initNativeKey(){
    if(typeof nativeKey === "undefined"){
        nativeKey = await generateKey(true)
    }
}

export async function nativeTest(){
    await initNativeKey()
    if(typeof nativeKey === 'undefined') throw new Error('Native key is undefined. This should not happen.')

    // TODO: Export the key and use the same for everything
    //const exportedKey = await exportKey(nativeKey)

    const bytesForSigningNative: Uint8Array = tx.encode()

    console.log("Native", "start");
    const start = performance.now();
    const plist = []
    for (let i = 0; i < iterations; i++) {
        plist.push(sign(bytesForSigningNative, nativeKey).then((sigNative)=>algoCrafter.addSignature(bytesForSigningNative, sigNative)));
    }
    const result = await Promise.all(plist);
    const end = performance.now();
    console.log("Native", end - start);
    return result
}
export async function nativeWithRustTest(){
    await initNativeKey()
    if(typeof nativeKey === 'undefined') throw new Error('Native key is undefined. This should not happen.')



    const fields = {
        header: {
            // note: tx.note,
            sender: tx.snd,
            fee: tx.fee,
            transactionType: "Payment",
            firstValid: tx.fv,
            lastValid: tx.lv,
            genesisHash: tx.gh,
            genesisId: tx.gen,
        },
        receiver: tx.rcv,
        amount: tx.amt,
    } as PayTransactionFields;




    // TODO: Export the key and use the same for everything
    //const exportedKey = await exportKey(nativeKey)

    const btyesForSigning = encodePayment(fields);

    console.log("Native/Rust Start");
    const start = performance.now();
    const plist = []
    for (let i = 0; i < iterations; i++) {
        plist.push(sign(btyesForSigning, nativeKey).then((sigNative)=>attachSignature(btyesForSigning, sigNative)));
    }
    const result = await Promise.all(plist);
    const end = performance.now();
    console.log("Native/Rust Stop", end - start);
    return result
}
export async function jsTest(){
    console.log("JS Start");
    const bytesForSigningTs: Uint8Array = tx.encode(); // encoded msg ready - to be signed with EdDSA


    const start = performance.now();
    const plist = []
   for (let i = 0; i < iterations; i++) {
        // The encoding algorithm is a fork of the actual msgpack (https://github.com/EvanJRichard/msgpack-javascript)
        // After msgpack encoding a TX TAG is added as a prefix to the result.
        plist.push(ed.signAsync(bytesForSigningTs, privKey).then((sigTs)=>algoCrafter.addSignature(bytesForSigningTs, sigTs)));
    }
    const result = await Promise.all(plist);
    const end = performance.now();
    console.log("JS Stop", end - start);
    return result
}

export async function rustTest(){

    const fields = {
        header: {
            sender: tx.snd,
            fee: tx.fee,
            transactionType: "Payment",
            firstValid: tx.fv,
            lastValid: tx.lv,
            genesisHash: tx.gh,
            genesisId: tx.gen,
        },
        receiver: tx.rcv,
        amount: tx.amt,
    } as PayTransactionFields;
    const btyesForSigning = encodePayment(fields);

    console.log("Rust", "start");
    const start = performance.now();
    const plist = []

    for (let i = 0; i < iterations; i++) {
        // Signing with a ed25519 lib that has no idea about Algorand
        plist.push(ed.signAsync(btyesForSigning, privKey).then((sig)=>attachSignature(btyesForSigning, sig)));
    }
    const result = await Promise.all(plist);
    const end = performance.now();
    console.log("Rust", end - start);
    return result
}
export async function main() {
    const js = await jsTest()
    const rust = await rustTest()
    const native = await nativeTest()
    const nativeWithRust = await nativeWithRustTest()

    if(js.every((sig: Uint8Array, i) => {
        return sig.every((v, j) => v === rust[i][j])
    })){
        console.log("JS is equal to Rust")
    } else {
        console.log("JS is not equal to Rust")
        console.log(js)
        console.log(rust)
    }

    if(native.every((sig: Uint8Array, i) => {
        return sig.every((v, j) => v === nativeWithRust[i][j])
    })){
        console.log("Native is equal to NativeRust")
    } else {
        console.log("Native is not equal to NativeRust")
        console.log(native)
        console.log(nativeWithRust)
    }
}