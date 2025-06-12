import pytest

from . import TEST_DATA
from algokit_transact import (
    group_transactions,
    encode_transaction,
    encode_transactions,
    encode_signed_transaction,
    encode_signed_transactions,
    decode_transactions,
    decode_signed_transactions,
    SignedTransaction,
)

simple_payment = TEST_DATA.simple_payment
opt_in_asset_transfer = TEST_DATA.opt_in_asset_transfer


def simple_group():
    """Helper function to create a simple transaction group"""
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

    return {
        "txs": txs,
        "expected_group_id": expected_group_id,
    }


# Polytest Suite: Transaction Group

# Polytest Group: Transaction Group Tests


@pytest.mark.group_transaction_group_tests
def test_group_transactions():
    """A collection of transactions can be grouped"""
    data = simple_group()
    txs = data["txs"]
    expected_group_id = data["expected_group_id"]

    grouped_txs = group_transactions(txs)

    assert len(grouped_txs) == len(txs)
    for i in range(len(txs)):
        assert txs[i].group is None
        assert grouped_txs[i].group == expected_group_id


@pytest.mark.group_transaction_group_tests
def test_encode_transactions():
    """A collection of transactions can be encoded"""
    data = simple_group()
    txs = data["txs"]
    grouped_txs = group_transactions(txs)

    encoded_grouped_txs = encode_transactions(grouped_txs)

    assert len(encoded_grouped_txs) == len(txs)
    for i in range(len(encoded_grouped_txs)):
        assert encoded_grouped_txs[i] == encode_transaction(grouped_txs[i])

    decoded_grouped_txs = decode_transactions(encoded_grouped_txs)
    assert decoded_grouped_txs == grouped_txs


@pytest.mark.group_transaction_group_tests
def test_encode_signed_transactions():
    """A collection of signed transactions can be encoded"""
    data = simple_group()
    txs = data["txs"]
    grouped_txs = group_transactions(txs)
    encoded_grouped_txs = encode_transactions(grouped_txs)

    # Create signatures for each transaction
    tx_signatures = [
        simple_payment.signing_private_key.sign(encoded_grouped_txs[0]).signature,
        opt_in_asset_transfer.signing_private_key.sign(
            encoded_grouped_txs[1]
        ).signature,
    ]

    # Create SignedTransaction objects from grouped transactions and signatures
    signed_grouped_txs = [
        SignedTransaction(
            transaction=grouped_txs[i],
            signature=tx_signatures[i],
        )
        for i in range(len(grouped_txs))
    ]

    encoded_signed_grouped_txs = encode_signed_transactions(signed_grouped_txs)

    assert len(encoded_signed_grouped_txs) == len(txs)
    for i in range(len(encoded_signed_grouped_txs)):
        assert encoded_signed_grouped_txs[i] == encode_signed_transaction(
            SignedTransaction(
                transaction=grouped_txs[i],
                signature=tx_signatures[i],
            )
        )

    decoded_signed_grouped_txs = decode_signed_transactions(encoded_signed_grouped_txs)
    assert decoded_signed_grouped_txs == signed_grouped_txs
