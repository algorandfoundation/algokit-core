import pytest

from . import TEST_DATA
from algokit_transact import (
    FeeParams,
    assign_fee,
    encode_transaction,
    encode_signed_transaction,
    PaymentTransactionFields,
    TransactionType,
    decode_transaction,
    get_encoded_transaction_type,
    Transaction,
    SignedTransaction,
    address_from_string,
    address_from_pub_key,
    get_transaction_id,
    get_transaction_id_raw,
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
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,  # pretend this is a valid hash
        genesis_id="localnet",
        payment=PaymentTransactionFields(amount=1337, receiver=bob),
    )

    txn_with_fee = assign_fee(txn, FeeParams(fee_per_byte=0, min_fee=1000))

    assert txn_with_fee.fee == 1000

    sig = alice_keypair.sign(encode_transaction(txn_with_fee)).signature
    signed_txn = SignedTransaction(
        transaction=txn_with_fee,
        signature=sig,
    )
    encoded_signed_txn = encode_signed_transaction(signed_txn)

    assert len(encoded_signed_txn) > 0


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
    signed_txn = SignedTransaction(
        transaction=simple_payment.transaction,
        signature=sig,
    )
    encoded_signed_transaction = encode_signed_transaction(signed_txn)

    assert encoded_signed_transaction == simple_payment.signed_bytes


@pytest.mark.group_transaction_tests
def test_encode_with_auth_address():
    """An auth address can be attached to a encoded transaction with a signature"""
    sig = simple_payment.signing_private_key.sign(
        simple_payment.unsigned_bytes
    ).signature
    signed_txn = SignedTransaction(
        transaction=simple_payment.transaction,
        signature=sig,
        auth_address=simple_payment.rekeyed_sender_auth_address,
    )
    encoded_signed_transaction = encode_signed_transaction(signed_txn)

    assert encoded_signed_transaction == simple_payment.rekeyed_sender_signed_bytes


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
    assert get_transaction_id_raw(simple_payment.transaction) == simple_payment.id_raw
