import init, {
  type PayTransactionFields,
  encodePayment,
  attachSignature,
} from "./pkg/algo_models";

import {AlgodClient} from "@awesome-algorand/algod-fetch";

let { AlgorandTransactionCrafter } = require("@algorandfoundation/algo-models");
import * as ed from "@noble/ed25519";

import * as msgpack from "algo-msgpack-with-bigint";

const client = new AlgodClient({BASE: "https://testnet-api.algonode.cloud"})

const data = await client.public.getBlock({round: 400})
console.log(data)
async function main() {
  // Generate a ed25519 keypair
  const privKey = ed.utils.randomPrivateKey();

  /* ***************************************************************
   *
   *  TS version: https://github.com/algorandfoundation/algo-models
   *
   ****************************************************************** */

  // sample vars
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
    .get();

  // The encoding algorithm is a fork of the actual msgpack (https://github.com/EvanJRichard/msgpack-javascript)
  // After msgpack encoding a TX TAG is added as a prefix to the result.
  const bytesForSigningTs: Uint8Array = tx.encode(); // encoded msg ready - to be signed with EdDSA

  const sigTs = await ed.signAsync(bytesForSigningTs, privKey);
  const signedTxTs = algoCrafter.addSignature(bytesForSigningTs, sigTs);

  /* ***************************************************************
   *
   *  Rust (WASM) version
   *
   ****************************************************************** */

  // init the wasm module, this would likely be done at the top level of our module
  // so users don't have to do it manually
  await init();

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

  // Signing with a ed25519 lib that has no idea about Algorand
  const sig = await ed.signAsync(btyesForSigning, privKey);
  const signedTx = attachSignature(btyesForSigning, sig);

  if (btyesForSigning.toString() !== bytesForSigningTs.toString()) {
    console.error("TS:", msgpack.decode(signedTxTs.slice(2)));
    console.error("Rust:", msgpack.decode(signedTx.slice(2)));
    throw new Error("Encoded transactions are not equal");
  } else {
    console.log("Encoded transactions are equal");
  }

  if (signedTxTs.toString() !== signedTx.toString()) {
    console.error("TS:", msgpack.decode(signedTxTs));
    console.error("Rust:", msgpack.decode(signedTx));
    throw new Error("Signed transactions are not equal");
  } else {
    console.log("Signed transactions are equal");
  }
}

main();
