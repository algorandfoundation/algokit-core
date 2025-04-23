import algosdk from "algosdk";
import * as fs from "fs";
import * as path from "path";

const REPO_ROOT = path.resolve(__dirname, "../../");
process.chdir(REPO_ROOT);

const sender = algosdk.generateAccount();
const receiver = algosdk.generateAccount();

const sp: algosdk.SuggestedParams = {
  flatFee: true,
  fee: 1000n,
  firstValid: 49265002,
  lastValid: 49266002,
  genesisID: "testnet-v1.0",
  genesisHash: new Uint8Array([
    72, 99, 181, 24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15,
    113, 240, 89, 167, 172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34,
  ]),
  minFee: 1000n,
};

const pay = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
  sender: sender.addr,
  receiver: receiver.addr,
  amount: 1000,
  suggestedParams: sp,
});

const stxn = pay.signTxn(sender.sk);

const data = {
  privKey: Array.from(sender.sk.slice(0, 32)),
  transaction: {
    sender: {
      address: sender.addr.toString(),
      pubKey: Array.from(sender.addr.publicKey),
    },
    fee: 1000,
    transactionType: "Payment",
    firstValid: sp.firstValid,
    lastValid: sp.lastValid,
    genesisHash: Array.from(sp.genesisHash!),
    genesisId: sp.genesisID,
    payment: {
      receiver: {
        address: receiver.addr.toString(),
        pubKey: Array.from(receiver.addr.publicKey),
      },
      amount: pay.payment?.amount,
    },
  },
  expectedBytesForSigning: Array.from([...Buffer.from("TX"), ...pay.toByte()]),
  expectedSignedTxn: Array.from(stxn),
};

const replacer = (_: any, v: any) => {
  if (typeof v === "bigint") {
    return Number(v);
  }
  return v;
};

const content = JSON.stringify(data, replacer, 2);

fs.writeFileSync("crates/algokit_transact_ffi/test_data.json", content);
