import pytest
from . import TEST_DATA
from algokit_transact import (
    decode_transaction,
    AlgoKitTransactError,
)

# Polytest Suite: Generic Transaction

# Polytest Group: Generic Transaction Tests


@pytest.mark.group_generic_transaction_tests
def test_malformed_bytes():
    """Ensure a helpful error message is thrown when attempting to decode malformed bytes"""
    bad_bytes = bytearray(TEST_DATA.simple_payment.unsigned_bytes)[13:37]
    with pytest.raises(
        AlgoKitTransactError.DecodingError,
    ):
        decode_transaction(bad_bytes)


@pytest.mark.group_generic_transaction_tests
def test_encode_0_bytes():
    """Ensure a helpful error message is thrown when attempting to encode 0 bytes"""
    with pytest.raises(
        AlgoKitTransactError.InputError, match="attempted to decode 0 bytes"
    ):
        decode_transaction(bytes())
