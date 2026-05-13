import pytest
from algokit_transact import (
    address_from_public_key,
    get_receiver_min_balance_fee,
    is_valid_algorand_address,
)

# Polytest Suite: Wallet SDK Helpers

# Polytest Group: Wallet SDK Helper Tests

@pytest.mark.group_wallet_sdk_helper_tests
def test_valid_algorand_address():
    """A well-formed Algorand address is reported as valid"""
    valid = address_from_public_key(bytes(32))
    assert is_valid_algorand_address(valid)


@pytest.mark.group_wallet_sdk_helper_tests
def test_invalid_algorand_address():
    """Malformed or wrong-length strings are reported as invalid addresses"""
    assert not is_valid_algorand_address("")
    assert not is_valid_algorand_address("not-an-address")
    # Right length, but invalid base32 / checksum
    assert not is_valid_algorand_address("A" * 58)


@pytest.mark.group_wallet_sdk_helper_tests
def test_receiver_min_balance_fee_shortfall():
    """The top-up equals the shortfall when the receiver is below its minimum balance"""
    assert get_receiver_min_balance_fee(0, 100_000) == 100_000
    assert get_receiver_min_balance_fee(40_000, 100_000) == 60_000


@pytest.mark.group_wallet_sdk_helper_tests
def test_receiver_min_balance_fee_covered():
    """No top-up is required when the receiver already meets or exceeds its minimum balance"""
    assert get_receiver_min_balance_fee(100_000, 100_000) == 0
    assert get_receiver_min_balance_fee(250_000, 100_000) == 0