from dataclasses import dataclass
from pathlib import Path
import json
from pprint import pprint
from algokit_transact import (
    Address,
    TransactionHeader,
    PayTransactionFields,
    TransactionType,
    Transaction,
)
from nacl.signing import SigningKey


@dataclass
class TransactionTestData:
    transaction: Transaction
    id: str
    raw_id: bytes
    unsigned_bytes: bytes
    signed_bytes: bytes
    signing_private_key: SigningKey

@dataclass
class TestData:
    simple_payment: TransactionTestData

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

    data = convert_values(
        convert_case_recursive(data)
    )

    simple_payment = data["simple_payment"]


    simple_payment["transaction"]["header"]["transaction_type"] = TransactionType.PAYMENT
    transaction = Transaction(
        header=TransactionHeader(**simple_payment["transaction"]["header"]),
        pay_fields=PayTransactionFields(**simple_payment["transaction"]["pay_fields"]),
    )

    return TestData(
        simple_payment= TransactionTestData(
            transaction=transaction,
            id=simple_payment["id"],
            raw_id=simple_payment["raw_id"],
            unsigned_bytes=simple_payment["unsigned_bytes"],
            signed_bytes=simple_payment["signed_bytes"],
            signing_private_key=SigningKey(simple_payment["signing_private_key"])
        )
    )
    
TEST_DATA = load_test_data()
