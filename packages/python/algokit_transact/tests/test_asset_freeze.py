import pytest

from tests.transaction_asserts import (
    assert_assign_fee,
    assert_decode_with_prefix,
    assert_decode_without_prefix,
    assert_encode,
    assert_encode_with_auth_address,
    assert_encode_with_signature,
    assert_encoded_transaction_type,
    assert_example,
    assert_transaction_id,
)
from . import TEST_DATA

txn_test_data = {
    "freeze": TEST_DATA.asset_freeze,
    "unfreeze": TEST_DATA.asset_unfreeze,
}

# Polytest Suite: Asset Freeze

# Polytest Group: Transaction Tests


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_example(test_data):
    assert_example(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_get_transaction_id(test_data):
    assert_transaction_id(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_assign_fee(test_data):
    assert_assign_fee(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_get_encoded_transaction_type(test_data):
    assert_encoded_transaction_type(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_decode_without_prefix(test_data):
    assert_decode_without_prefix(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_decode_with_prefix(test_data):
    assert_decode_with_prefix(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_encode_with_auth_address(test_data):
    assert_encode_with_auth_address(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_encode_with_signature(test_data):
    assert_encode_with_signature(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_encode(test_data):
    assert_encode(test_data)
