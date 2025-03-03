from pathlib import Path
import json
from algo_models import (
    TransactionHeader,
    encode_transaction,
    PayTransactionFields,
    TransactionType,
    attach_signature,
    decode_transaction,
    get_encoded_transaction_type,
    MsgPackError,
    Transaction
)
from nacl.signing import SigningKey
import pytest
from copy import deepcopy


def convert_lists_to_bytes(obj):
    if isinstance(obj, dict):
        return {key: convert_lists_to_bytes(value) for key, value in obj.items()}
    elif isinstance(obj, list) and all(isinstance(x, int) for x in obj):
        return bytes(obj)
    elif isinstance(obj, list):
        return [convert_lists_to_bytes(x) for x in obj]
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
    current_dir = Path(__file__).parent.parent
    test_data_path = current_dir / "test_data.json"

    with open(test_data_path) as f:
        data = json.load(f)

    data = convert_case_recursive(data)
    data = convert_lists_to_bytes(data)

    data["transaction"]["header"]["transaction_type"] = TransactionType.PAYMENT

    data["transaction"]["header"] = TransactionHeader(
        **data["transaction"]["header"]
    )

    data["transaction"] = Transaction(header=data["transaction"]["header"], pay_fields=PayTransactionFields(**data["transaction"]["pay_fields"]))

    return data


TEST_DATA = load_test_data()
PRIV_KEY = SigningKey(TEST_DATA["priv_key"])


def test_encode():
    assert (
        encode_transaction(TEST_DATA["transaction"]) == TEST_DATA["expected_bytes_for_signing"]
    )


def test_encode_with_signature():
    sig = PRIV_KEY.sign(TEST_DATA["expected_bytes_for_signing"]).signature
    print(len(sig))
    signed_tx = attach_signature(TEST_DATA["expected_bytes_for_signing"], sig)
    assert signed_tx == TEST_DATA["expected_signed_txn"]


def test_decode_with_tx_prefix():
    assert (
        decode_transaction(TEST_DATA["expected_bytes_for_signing"]) == TEST_DATA["transaction"]
    )


def test_decode_without_tx_prefix():
    assert (
        decode_transaction(TEST_DATA["expected_bytes_for_signing"][2:])
        == TEST_DATA["transaction"]
    )


def test_get_encoded_transaction_type():
    assert (
        get_encoded_transaction_type(TEST_DATA["expected_bytes_for_signing"])
        == TransactionType.PAYMENT
    )


def test_decoding_error_0_bytes():
    with pytest.raises(MsgPackError.DecodingError, match="attempted to decode 0 bytes"):
        decode_transaction(bytes())


def test_decoding_error_malformed_bytes():
    bad_bytes = bytearray(TEST_DATA["expected_bytes_for_signing"])
    bad_bytes[13] = 37
    with pytest.raises(
        MsgPackError.DecodingError,
        match="Error ocurred during decoding: missing field `fee`",
    ):
        decode_transaction(bad_bytes)


def test_error_invalid_type():
    bad_fields: PayTransactionFields = deepcopy(TEST_DATA["transaction"])
    bad_fields.header.fee = "foo"
    with pytest.raises(
        TypeError,
        match="'str' object cannot be interpreted as an integer",
    ):
        encode_transaction(bad_fields)
