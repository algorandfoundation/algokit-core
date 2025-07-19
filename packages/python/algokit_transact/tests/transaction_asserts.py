from . import TransactionTestData
from algokit_transact import (
    FeeParams,
    assign_fee,
    encode_transaction,
    encode_signed_transaction,
    decode_transaction,
    get_encoded_transaction_type,
    SignedTransaction,
    get_transaction_id,
    get_transaction_id_raw,
    estimate_transaction_size, MultisigSignature, MultisigSubsignature, new_multisig_signature, apply_multisig_subsignature,
    merge_multisignatures,
)


def assert_example(test_data: TransactionTestData):
    """A human-readable example of forming a transaction and signing it"""
    sig = test_data.signing_private_key.sign(
        encode_transaction(test_data.transaction)
    ).signature
    signed_txn = SignedTransaction(
        transaction=test_data.transaction,
        signature=sig,
    )
    encoded_signed_txn = encode_signed_transaction(signed_txn)
    assert encoded_signed_txn == test_data.signed_bytes


def assert_multisig_example(test_data: TransactionTestData):
    """A multisig example of forming a transaction and signing it"""
    single_sig = test_data.signing_private_key.sign(
        encode_transaction(test_data.transaction)
    ).signature

    unsigned_multisig_signature = new_multisig_signature(1, 2, list(test_data.multisig_addresses))
    multisig_signature_0 = apply_multisig_subsignature(unsigned_multisig_signature, test_data.multisig_addresses[0], single_sig)
    multisig_signature_1 = apply_multisig_subsignature(unsigned_multisig_signature, test_data.multisig_addresses[1], single_sig)
    multisig_signature = merge_multisignatures(multisig_signature_0, multisig_signature_1)

    signed_txn = SignedTransaction(
        transaction=test_data.transaction,
        multisignature=multisig_signature
    )
    encoded_signed_txn = encode_signed_transaction(signed_txn)
    assert encoded_signed_txn == test_data.multisig_signed_bytes


def assert_assign_fee(test_data: TransactionTestData):
    """A fee can be calculated and assigned to a transaction"""
    min_fee = 2000
    txn_with_fee1 = assign_fee(
        test_data.transaction, FeeParams(fee_per_byte=0, min_fee=min_fee)
    )
    assert txn_with_fee1.fee == min_fee

    extra_fee = 3000
    txn_with_fee2 = assign_fee(
        test_data.transaction,
        FeeParams(fee_per_byte=0, min_fee=min_fee, extra_fee=extra_fee),
    )
    assert txn_with_fee2.fee == min_fee + extra_fee

    fee_per_byte = 100
    txn_with_fee3 = assign_fee(
        test_data.transaction,
        FeeParams(fee_per_byte=fee_per_byte, min_fee=1000),
    )
    txn_size = estimate_transaction_size(test_data.transaction)
    assert txn_with_fee3.fee == txn_size * fee_per_byte


def assert_transaction_id(test_data: TransactionTestData):
    """A transaction id can be obtained from a transaction"""
    assert get_transaction_id_raw(test_data.transaction) == test_data.id_raw
    assert get_transaction_id(test_data.transaction) == test_data.id


def assert_encoded_transaction_type(test_data: TransactionTestData):
    """The transaction type of an encoded transaction can be retrieved"""
    assert (
        get_encoded_transaction_type(test_data.unsigned_bytes)
        == test_data.transaction.transaction_type
    )


def assert_decode_without_prefix(test_data: TransactionTestData):
    """A transaction without TX prefix and valid fields is decoded properly"""
    decoded = decode_transaction(test_data.unsigned_bytes[2:])
    assert decoded == test_data.transaction


def assert_decode_with_prefix(test_data: TransactionTestData):
    """A transaction with TX prefix and valid fields is decoded properly"""
    decoded = decode_transaction(test_data.unsigned_bytes)
    assert decoded == test_data.transaction


def assert_encode_with_auth_address(test_data: TransactionTestData):
    """An auth address can be attached to a encoded transaction with a signature"""
    sig = test_data.signing_private_key.sign(test_data.unsigned_bytes).signature
    signed_txn = SignedTransaction(
        transaction=test_data.transaction,
        signature=sig,
        auth_address=test_data.rekeyed_sender_auth_address,
    )
    encoded_signed_txn = encode_signed_transaction(signed_txn)
    assert encoded_signed_txn == test_data.rekeyed_sender_signed_bytes


def assert_encode_with_signature(test_data: TransactionTestData):
    """A signature can be attached to a encoded transaction"""
    sig = test_data.signing_private_key.sign(test_data.unsigned_bytes).signature
    signed_txn = SignedTransaction(
        transaction=test_data.transaction,
        signature=sig,
    )
    encoded_signed_txn = encode_signed_transaction(signed_txn)
    assert encoded_signed_txn == test_data.signed_bytes


def assert_encode(test_data: TransactionTestData):
    """A transaction with valid fields is encoded properly"""
    assert encode_transaction(test_data.transaction) == test_data.unsigned_bytes
