from pathlib import Path
import json
from pprint import pprint
from algo_models import (
    Address,
    TransactionHeader,
    PayTransactionFields,
    TransactionType,
    Transaction,
)
from nacl.signing import SigningKey


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
        / "algo_models_ffi"
        / "test_data.json"
    )

    with open(test_data_path) as f:
        data = json.load(f)

    data = convert_case_recursive(data)
    data = convert_values(data)

    data["transaction"]["header"]["transaction_type"] = TransactionType.PAYMENT

    data["transaction"]["header"] = TransactionHeader(**data["transaction"]["header"])

    data["transaction"] = Transaction(
        header=data["transaction"]["header"],
        pay_fields=PayTransactionFields(**data["transaction"]["pay_fields"]),
    )

    return data


TEST_DATA = load_test_data()
PRIV_KEY = SigningKey(TEST_DATA["priv_key"])
