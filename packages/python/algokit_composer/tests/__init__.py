"""Shared fixtures for algokit_composer Python tests.

The Python tests verify the FFI plumbing — that the bindings expose the right
shapes, that compose() and sign_transactions() produce decodable output, and
that errors propagate. Canonical byte-for-byte fixture matches are covered by
the Rust unit tests in algokit_composer / algokit_composer_ffi.
"""

# A well-formed testnet address — matches `AccountMother::account()` from the
# Rust test_utils, so Python and Rust test fixtures stay aligned.
SENDER_ADDRESS = "RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q"

# Second testnet address — matches `AccountMother::neil()`.
RECEIVER_ADDRESS = "JB3K6HTAXODO4THESLNYTSG6GQUFNEVIQG7A6ZYVDACR6WA3ZF52TKU5NA"

# The 32-byte testnet genesis hash (base64 "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=").
TESTNET_GENESIS_HASH = bytes([
    72, 99, 181, 24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203,
    15, 113, 240, 89, 167, 172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34,
])

TESTNET_GENESIS_ID = "testnet-v1.0"

# These match `TransactionHeaderMother::simple_testnet()` in the Rust test_utils.
FIRST_VALID = 50_659_540
LAST_VALID = 50_660_540


def common_params(sender: str = SENDER_ADDRESS):
    """Build a CommonTxnParams with no overrides — relies entirely on ComposerParams."""
    from algokit_composer import CommonTxnParams
    return CommonTxnParams(
        sender=sender,
        note=None,
        lease=None,
        rekey_to=None,
        static_fee=1000,
        extra_fee=None,
        max_fee=None,
        validity_window=None,
        first_valid_round=None,
        last_valid_round=None,
    )


def composer_params():
    """Build a ComposerParams pinned to testnet."""
    from algokit_composer import ComposerParams, SuggestedParams
    return ComposerParams(
        suggested_params=SuggestedParams(
            fee=1000,
            flat_fee=True,
            first_round_valid=FIRST_VALID,
            last_round_valid=LAST_VALID,
            genesis_hash=TESTNET_GENESIS_HASH,
            genesis_id=TESTNET_GENESIS_ID,
        ),
        default_validity_window=None,
    )
