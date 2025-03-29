import pytest
from . import TEST_DATA, PRIV_KEY
from algo_models import (
    TransactionHeader,
    encode_transaction,
    PayTransactionFields,
    TransactionType,
    attach_signature,
    decode_transaction,
    get_encoded_transaction_type,
    Transaction,
    address_from_string,
    address_from_pub_key,
)
from nacl.signing import SigningKey

# Polytest Suite: Payment

# Polytest Group: Transaction Tests


@pytest.mark.group_transaction_tests
def test_example():
    """A human-readable example of forming a transaction and signing it"""
    alice_keypair = SigningKey.generate()  # Keypair generated from PyNaCl
    alice = address_from_pub_key(alice_keypair.verify_key.__bytes__())
    bob = address_from_string(
        "B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y"
    )

    txn = Transaction(
        header=TransactionHeader(
            transaction_type=TransactionType.PAYMENT,
            fee=1000,
            first_valid=1337,
            last_valid=1347,
            sender=alice,
            genesis_hash=b"A" * 32,  # pretend this is a valid hash
            genesis_id="localnet",
        ),
        pay_fields=PayTransactionFields(amount=1337, receiver=bob),
    )

    sig = alice_keypair.sign(encode_transaction(txn)).signature
    signed_txn = attach_signature(encode_transaction(txn), sig)


@pytest.mark.group_transaction_tests
def test_get_encoded_transaction_type():
    """The transaction type of an encoded transaction can be retrieved"""
    assert (
        get_encoded_transaction_type(TEST_DATA["expected_bytes_for_signing"])
        == TransactionType.PAYMENT
    )


@pytest.mark.group_transaction_tests
def test_decode_without_prefix():
    """A transaction without TX prefix and valid fields is decoded properly"""
    assert (
        decode_transaction(TEST_DATA["expected_bytes_for_signing"][2:])
        == TEST_DATA["transaction"]
    )


@pytest.mark.group_transaction_tests
def test_decode_with_prefix():
    """A transaction with TX prefix and valid fields is decoded properly"""
    assert (
        decode_transaction(TEST_DATA["expected_bytes_for_signing"])
        == TEST_DATA["transaction"]
    )


@pytest.mark.group_transaction_tests
def test_encode_with_signature():
    """A signature can be attached to a encoded transaction"""
    sig = PRIV_KEY.sign(TEST_DATA["expected_bytes_for_signing"]).signature
    print(len(sig))
    signed_tx = attach_signature(TEST_DATA["expected_bytes_for_signing"], sig)
    assert signed_tx == TEST_DATA["expected_signed_txn"]


@pytest.mark.group_transaction_tests
def test_encode():
    """A transaction with valid fields is encoded properly"""
    assert (
        encode_transaction(TEST_DATA["transaction"])
        == TEST_DATA["expected_bytes_for_signing"]
    )

