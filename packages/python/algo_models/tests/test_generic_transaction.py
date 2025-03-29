import pytest
from . import TEST_DATA, PRIV_KEY
from algo_models import (
    decode_transaction,
    AlgoModelsError,
)


# Polytest Suite: Generic Transaction

# Polytest Group: Generic Transaction Tests


@pytest.mark.group_generic_transaction_tests
def test_malformed_bytes():
    """Ensure a helpful error message is thrown when attempting to decode malformed bytes"""
    bad_bytes = bytearray(TEST_DATA["expected_bytes_for_signing"])[13:37]
    with pytest.raises(
        AlgoModelsError.DecodingError,
    ):
        decode_transaction(bad_bytes)


@pytest.mark.group_generic_transaction_tests
def test_encode_0_bytes():
    """Ensure a helpful error message is thrown when attempting to encode 0 bytes"""
    with pytest.raises(
        AlgoModelsError.DecodingError, match="attempted to decode 0 bytes"
    ):
        decode_transaction(bytes())
