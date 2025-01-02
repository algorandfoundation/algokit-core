# This script is used to generate test_data.json, which is then used in the language binding tests
# This script should only be ran when the data is updated (i.e. a new transaction type is added)

from algosdk import account, transaction, encoding, constants
from algosdk.v2client.algod import AlgodClient
import base64
import json

[sender_sk, sender_addr] = account.generate_account()
[_, receiver_addr] = account.generate_account()


algod = AlgodClient("", "https://testnet-api.4160.nodely.dev")
sp = algod.suggested_params()


def get_bytes_for_signing(tx: transaction.Transaction):
    return list(constants.txid_prefix + base64.b64decode(encoding.msgpack_encode(pay)))


def addr_bytes(addr):
    return list(encoding.decode_address(addr))


def b64_bytes(data):
    return list(base64.b64decode(data))


pay = transaction.PaymentTxn(
    sender=sender_addr,
    receiver=receiver_addr,
    amt=1000,
    close_remainder_to=None,
    note=None,
    lease=None,
    rekey_to=None,
    sp=sp,
)

stxn = pay.sign(sender_sk)

data = {
    "privKey": list(base64.b64decode(sender_sk))[0:32],
    "fields": {
        "header": {
            "sender": addr_bytes(pay.sender),
            "fee": pay.fee,
            "transactionType": "Payment",
            "firstValid": pay.first_valid_round,
            "lastValid": pay.last_valid_round,
            "genesisHash": b64_bytes(pay.genesis_hash),
            "genesisId": pay.genesis_id,
        },
        "receiver": addr_bytes(pay.receiver),
        "amount": pay.amt,
    },
    "expectedBytesForSigning": get_bytes_for_signing(pay),
    "expectedSignedTxn": b64_bytes(encoding.msgpack_encode(stxn)),
}


with open("./test_data.json", "w") as f:
    json.dump(data, f, indent=2)
