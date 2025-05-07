import pytest
from . import TEST_DATA
from algokit_transact import (
    encode_transaction,
    PaymentTransactionFields,
    TransactionType,
    attach_signature,
    decode_transaction,
    get_encoded_transaction_type,
    Transaction,
    address_from_string,
    address_from_pub_key,
    get_transaction_id,
    get_transaction_raw_id,
)
from nacl.signing import SigningKey

simple_payment = TEST_DATA.simple_payment

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
        transaction_type=TransactionType.PAYMENT,
        fee=1000,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,  # pretend this is a valid hash
        genesis_id="localnet",
        payment=PaymentTransactionFields(amount=1337, receiver=bob),
    )

    sig = alice_keypair.sign(encode_transaction(txn)).signature
    signed_txn = attach_signature(encode_transaction(txn), sig)
    assert len(signed_txn) > 0


@pytest.mark.group_transaction_tests
def test_get_encoded_transaction_type():
    """The transaction type of an encoded transaction can be retrieved"""
    assert (
        get_encoded_transaction_type(simple_payment.unsigned_bytes)
        == simple_payment.transaction.transaction_type
    )


@pytest.mark.group_transaction_tests
def test_decode_without_prefix():
    """A transaction without TX prefix and valid fields is decoded properly"""
    assert (
        decode_transaction(simple_payment.unsigned_bytes[2:])
        == simple_payment.transaction
    )


@pytest.mark.group_transaction_tests
def test_decode_with_prefix():
    """A transaction with TX prefix and valid fields is decoded properly"""
    assert (
        decode_transaction(simple_payment.unsigned_bytes) == simple_payment.transaction
    )


@pytest.mark.group_transaction_tests
def test_encode_with_signature():
    """A signature can be attached to a encoded transaction"""
    sig = simple_payment.signing_private_key.sign(
        simple_payment.unsigned_bytes
    ).signature
    print(len(sig))
    signed_tx = attach_signature(simple_payment.unsigned_bytes, sig)
    assert signed_tx == simple_payment.signed_bytes


@pytest.mark.group_transaction_tests
def test_encode():
    """A transaction with valid fields is encoded properly"""
    assert (
        encode_transaction(simple_payment.transaction) == simple_payment.unsigned_bytes
    )


@pytest.mark.group_transaction_tests
def test_get_transaction_id():
    """A transaction id can be obtained from a transaction"""

    assert get_transaction_id(simple_payment.transaction) == simple_payment.id
    assert get_transaction_raw_id(simple_payment.transaction) == simple_payment.raw_id
