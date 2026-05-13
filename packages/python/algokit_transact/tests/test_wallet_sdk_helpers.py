import pytest
from . import TEST_DATA
from algokit_transact import sign_algo25_transaction

# Polytest Suite: Wallet SDK Helpers

# Polytest Group: Wallet SDK Helper Tests

@pytest.mark.group_wallet_sdk_helper_tests
def test_sign_algo_25_transaction():
    """An encoded transaction signed via sign_algo25_transaction matches the canonical signed-transaction encoding"""
    data = TEST_DATA.simple_payment
    signed = sign_algo25_transaction(
        bytes(data.signing_private_key), data.unsigned_bytes
    )
    assert signed == data.signed_bytes