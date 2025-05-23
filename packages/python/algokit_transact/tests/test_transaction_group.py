import pytest

from . import TEST_DATA
from algokit_transact import group_transactions

simple_payment = TEST_DATA.simple_payment
opt_in_asset_transfer = TEST_DATA.opt_in_asset_transfer

# Polytest Suite: Transaction Group

# Polytest Group: Transaction Group Tests


@pytest.mark.group_transaction_group_tests
def test_group_transactions():
    """A collection of transactions can be grouped"""
    expected_group_id = bytes(
        [
            202,
            79,
            82,
            7,
            197,
            237,
            213,
            55,
            117,
            226,
            131,
            74,
            221,
            85,
            86,
            215,
            64,
            133,
            212,
            7,
            58,
            234,
            248,
            162,
            222,
            53,
            161,
            29,
            141,
            101,
            133,
            49,
        ]
    )

    txs = [simple_payment.transaction, opt_in_asset_transfer.transaction]
    grouped_txs = group_transactions(txs)

    assert len(grouped_txs) == len(txs)

    for i in range(len(txs)):
        assert txs[i].group is None
        assert grouped_txs[i].group == expected_group_id
