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

# Polytest Suite: AssetConfig


# Polytest Group: Transaction Tests
txn_test_data = {
    "asset create": TEST_DATA.asset_create,
    "asset reconfigure": TEST_DATA.asset_reconfigure,
    "asset destroy": TEST_DATA.asset_destroy,
}


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_example(test_data):
    """A human-readable example of forming a transaction and signing it"""
    assert_example(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_get_transaction_id(test_data):
    """A transaction id can be obtained from a transaction"""
    assert_transaction_id(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_assign_fee(test_data):
    """A fee can be calculated and assigned to a transaction"""
    assert_assign_fee(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_get_encoded_transaction_type(test_data):
    """The transaction type of an encoded transaction can be retrieved"""
    assert_encoded_transaction_type(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_decode_without_prefix(test_data):
    """A transaction without TX prefix and valid fields is decoded properly"""
    assert_decode_without_prefix(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_decode_with_prefix(test_data):
    """A transaction with TX prefix and valid fields is decoded properly"""
    assert_decode_with_prefix(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_encode_with_auth_address(test_data):
    """An auth address can be attached to a encoded transaction with a signature"""
    assert_encode_with_auth_address(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_encode_with_signature(test_data):
    """A signature can be attached to a encoded transaction"""
    assert_encode_with_signature(test_data)


@pytest.mark.group_transaction_tests
@pytest.mark.parametrize(
    "test_data",
    txn_test_data.values(),
    ids=txn_test_data.keys(),
)
def test_encode(test_data):
    """A transaction with valid fields is encoded properly"""
    assert_encode(test_data)
