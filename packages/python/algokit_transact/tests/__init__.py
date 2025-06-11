from dataclasses import dataclass
from pathlib import Path
import json
from pprint import pprint
from algokit_transact import (
    Address,
    PaymentTransactionFields,
    TransactionType,
    Transaction,
    AssetTransferTransactionFields,
)
from nacl.signing import SigningKey


@dataclass
class TransactionTestData:
    transaction: Transaction
    id: str
    id_raw: bytes
    unsigned_bytes: bytes
    signed_bytes: bytes
    signing_private_key: SigningKey
    rekeyed_sender_auth_address: Address
    rekeyed_sender_signed_bytes: bytes


@dataclass
class TestData:
    simple_payment: TransactionTestData
    opt_in_asset_transfer: TransactionTestData


def convert_values(obj):
    if isinstance(obj, dict):
        if "address" in obj and "pub_key" in obj:
            pprint(Address(**obj))
            return Address(address=obj["address"], pub_key=bytes(obj["pub_key"]))
        return {key: convert_values(value) for key, value in obj.items()}
    elif isinstance(obj, list) and all(isinstance(x, int) for x in obj):
        return bytes(obj)
    elif isinstance(obj, list):
        return [convert_values(x) for x in obj]
    return obj


def camel_to_snake(name):
    import re

    name = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", name).lower()


def convert_case_recursive(obj):
    if isinstance(obj, dict):
        return {
            camel_to_snake(key): convert_case_recursive(value)
            for key, value in obj.items()
        }
    elif isinstance(obj, list):
        return [convert_case_recursive(x) for x in obj]
    return obj


def load_test_data():
    # Get the path to test_data.json relative to this test file
    test_data_path = (
        Path(__file__).parent.parent.parent.parent.parent
        / "crates"
        / "algokit_transact_ffi"
        / "test_data.json"
    )

    with open(test_data_path) as f:
        data = json.load(f)

    data = convert_values(convert_case_recursive(data))

    simple_payment_txn = data["simple_payment"].pop("transaction")
    _ = simple_payment_txn.pop("transaction_type")
    simple_payment_txn_data = simple_payment_txn.pop("payment")
    simple_payment_signing_private_key = data["simple_payment"].pop(
        "signing_private_key"
    )

    opt_in_asset_transfer_txn = data["opt_in_asset_transfer"].pop("transaction")
    _ = opt_in_asset_transfer_txn.pop("transaction_type")
    opt_in_asset_transfer_txn_data = opt_in_asset_transfer_txn.pop("asset_transfer")
    opt_in_asset_transfer_signing_private_key = data["opt_in_asset_transfer"].pop(
        "signing_private_key"
    )

    return TestData(
        simple_payment=TransactionTestData(
            **data["simple_payment"],
            transaction=Transaction(
                **simple_payment_txn,
                transaction_type=TransactionType.PAYMENT,
                payment=PaymentTransactionFields(**simple_payment_txn_data),
            ),
            signing_private_key=SigningKey(simple_payment_signing_private_key),
        ),
        opt_in_asset_transfer=TransactionTestData(
            **data["opt_in_asset_transfer"],
            transaction=Transaction(
                **opt_in_asset_transfer_txn,
                transaction_type=TransactionType.ASSET_TRANSFER,
                asset_transfer=AssetTransferTransactionFields(
                    **opt_in_asset_transfer_txn_data
                ),
            ),
            signing_private_key=SigningKey(opt_in_asset_transfer_signing_private_key),
        ),
    )


TEST_DATA = load_test_data()
