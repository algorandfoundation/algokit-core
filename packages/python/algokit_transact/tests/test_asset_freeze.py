import pytest

from . import TEST_DATA
from algokit_transact import (
    FeeParams,
    assign_fee,
    encode_transaction,
    encode_signed_transaction,
    AssetFreezeTransactionFields,
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

# We'll need to add test data once the implementation is complete
# asset_freeze = TEST_DATA.asset_freeze
# asset_unfreeze = TEST_DATA.asset_unfreeze


def test_example():
    """A human-readable example of forming an asset freeze transaction and signing it"""
    alice_keypair = SigningKey.generate()  # Keypair generated from PyNaCl
    alice = address_from_pub_key(alice_keypair.verify_key.__bytes__())

    target_account = address_from_string(
        "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
    )

    # Example 1: Freeze an asset
    freeze_txn = Transaction(
        transaction_type=TransactionType.ASSET_FREEZE,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,  # pretend this is a valid hash
        genesis_id="localnet",
        asset_freeze=AssetFreezeTransactionFields(
            asset_id=12345,
            freeze_target=target_account,
            frozen=True,
        ),
    )

    freeze_txn_with_fee = assign_fee(
        freeze_txn, FeeParams(fee_per_byte=0, min_fee=1000)
    )
    assert freeze_txn_with_fee.fee == 1000

    # Example 2: Unfreeze an asset
    unfreeze_txn = Transaction(
        transaction_type=TransactionType.ASSET_FREEZE,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,
        genesis_id="localnet",
        asset_freeze=AssetFreezeTransactionFields(
            asset_id=12345,
            freeze_target=target_account,
            frozen=False,
        ),
    )

    unfreeze_txn_with_fee = assign_fee(
        unfreeze_txn, FeeParams(fee_per_byte=0, min_fee=1000)
    )
    assert unfreeze_txn_with_fee.fee == 1000


def test_asset_freeze_transaction_encoding():
    """Test basic encoding/decoding of asset freeze transactions"""
    alice_keypair = SigningKey.generate()
    alice = address_from_pub_key(alice_keypair.verify_key.__bytes__())
    target_account = address_from_string(
        "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
    )

    freeze_txn = Transaction(
        transaction_type=TransactionType.ASSET_FREEZE,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        fee=1000,
        genesis_hash=b"A" * 32,
        genesis_id="localnet",
        asset_freeze=AssetFreezeTransactionFields(
            asset_id=12345,
            freeze_target=target_account,
            frozen=True,
        ),
    )

    # Test encoding and decoding
    encoded = encode_transaction(freeze_txn)
    decoded = decode_transaction(encoded)

    assert decoded.transaction_type == TransactionType.ASSET_FREEZE
    assert decoded.asset_freeze.asset_id == 12345
    assert decoded.asset_freeze.frozen == True
    assert decoded.asset_freeze.freeze_target.address == target_account.address

    # Test transaction type detection
    assert get_encoded_transaction_type(encoded) == TransactionType.ASSET_FREEZE


def test_asset_freeze_transaction_id():
    """Test transaction ID generation for asset freeze transactions"""
    alice_keypair = SigningKey.generate()
    alice = address_from_pub_key(alice_keypair.verify_key.__bytes__())
    target_account = address_from_string(
        "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"
    )

    freeze_txn = Transaction(
        transaction_type=TransactionType.ASSET_FREEZE,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        fee=1000,
        genesis_hash=b"A" * 32,
        genesis_id="localnet",
        asset_freeze=AssetFreezeTransactionFields(
            asset_id=12345,
            freeze_target=target_account,
            frozen=True,
        ),
    )

    tx_id = get_transaction_id(freeze_txn)
    tx_id_raw = get_transaction_id_raw(freeze_txn)

    assert len(tx_id) > 0
    assert len(tx_id_raw) == 32
