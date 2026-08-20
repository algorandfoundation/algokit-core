import msgpack
import pytest
from nacl.signing import SigningKey

from algokit_composer import (
    AssetOptInParams,
    AssetTransferParams,
    AlgoKitComposerError,
    OfflineKeyRegParams,
    OnlineKeyRegParams,
    PaymentParams,
    SimulateOptions,
    SimulateTraceConfig,
    TxnParams,
    TxnParamsKind,
    build_signed_simulate_request,
    build_unsigned_simulate_request,
    compose,
    map_simulate_response,
    sign_transactions,
)
from algokit_transact import (
    Transaction,
    TransactionType,
    decode_signed_transaction,
    decode_transaction,
)

from . import (
    RECEIVER_ADDRESS,
    SENDER_ADDRESS,
    common_params,
    composer_params,
)


# Polytest Suite: Composer

# Polytest Group: Simulate Tests


def _simulate_options(**overrides):
    """A SimulateOptions with everything unset, plus any overrides."""
    fields = {
        "skip_signatures": None,
        "round": None,
        "allow_empty_signatures": None,
        "allow_more_logging": None,
        "allow_unnamed_resources": None,
        "extra_opcode_budget": None,
        "exec_trace_config": None,
        "fix_signers": None,
    }
    fields.update(overrides)
    return SimulateOptions(**fields)


def _unsigned_payment():
    """One composed, unsigned payment transaction as encoded bytes."""
    return compose(
        [
            TxnParams(
                kind=TxnParamsKind.PAYMENT,
                payment=PaymentParams(
                    common=common_params(),
                    receiver=RECEIVER_ADDRESS,
                    amount=1000,
                    close_remainder_to=None,
                ),
                asset_transfer=None,
                asset_opt_in=None,
                online_key_reg=None,
                offline_key_reg=None,
            )
        ],
        composer_params(),
    )


def _signed_payment():
    """The same payment, signed, as encoded signed-transaction bytes."""
    signing_key = SigningKey.generate()
    return sign_transactions(_unsigned_payment(), [bytes(signing_key)])


def _simulate_response(groups):
    """MsgPack-encode a minimal algod SimulateResponse with the given groups."""
    return msgpack.packb(
        {"version": 2, "last-round": 0, "txn-groups": groups},
        use_bin_type=True,
    )


def _txn_map(encoded):
    """Unpack an encoded transaction, dropping the "TX" domain-separator prefix."""
    body = encoded[2:] if encoded[:2] == b"TX" else encoded
    return msgpack.unpackb(body, raw=False)


def _group(txn_bytes_list, **extra):
    """One txn-group entry whose results echo the supplied transactions."""
    group = {
        "txn-results": [
            {"txn-result": {"pool-error": "", "txn": {"txn": _txn_map(t)}}}
            for t in txn_bytes_list
        ]
    }
    group.update(extra)
    return group


@pytest.mark.group_simulate_tests
def test_map_simulate_response_rejects_group_count_mismatch():
    """map_simulate_response() errors when the response does not carry exactly one group"""
    txns = _unsigned_payment()
    response = _simulate_response([_group(txns), _group(txns)])

    with pytest.raises(AlgoKitComposerError):
        map_simulate_response(txns, response)

@pytest.mark.group_simulate_tests
def test_map_simulate_response_surfaces_group_failure():
    """map_simulate_response() reports failure_message and failed_at as data rather than raising"""
    txns = _unsigned_payment()
    response = _simulate_response(
        [_group(txns, **{"failure-message": "assert failed", "failed-at": [0]})]
    )

    result = map_simulate_response(txns, response)

    assert result.failure_message == "assert failed"
    assert result.failed_at == [0]
    # The per-transaction results survive a failing group.
    assert len(result.txn_results) == 1

@pytest.mark.group_simulate_tests
def test_map_simulate_response_returns_per_transaction_results():
    """map_simulate_response() projects tx ids and confirmations from the first group"""
    txns = _unsigned_payment()
    result = map_simulate_response(txns, _simulate_response([_group(txns)]))

    assert len(result.txn_results) == 1
    assert len(result.tx_ids) == 1
    assert result.tx_ids[0] == result.txn_results[0].tx_id
    assert result.txn_results[0].confirmation
    assert result.failure_message is None

@pytest.mark.group_simulate_tests
def test_build_simulate_request_rejects_empty_group():
    """Building a simulate request with no transactions returns an error"""
    with pytest.raises(AlgoKitComposerError):
        build_unsigned_simulate_request([], _simulate_options())

@pytest.mark.group_simulate_tests
def test_simulate_options_merge_per_field():
    """Setting one SimulateOptions field leaves the remaining request fields unset"""
    encoded = build_signed_simulate_request(
        _signed_payment(), _simulate_options(extra_opcode_budget=1000)
    )
    request = msgpack.unpackb(encoded, raw=False)

    assert request["extra-opcode-budget"] == 1000
    for untouched in ("round", "allow-more-logging", "allow-unnamed-resources"):
        assert untouched not in request

@pytest.mark.group_simulate_tests
def test_skip_signatures_forces_allow_empty_signatures_and_fix_signers():
    """skip_signatures overrides caller-supplied allow_empty_signatures and fix_signers to true"""
    encoded = build_signed_simulate_request(
        _signed_payment(),
        _simulate_options(
            skip_signatures=True, allow_empty_signatures=False, fix_signers=False
        ),
    )
    request = msgpack.unpackb(encoded, raw=False)

    assert request["allow-empty-signatures"] is True
    assert request["fix-signers"] is True

@pytest.mark.group_simulate_tests
def test_build_signed_simulate_request_preserves_signatures():
    """build_signed_simulate_request() keeps the supplied signatures on each txn entry"""
    encoded = build_signed_simulate_request(_signed_payment(), _simulate_options())
    request = msgpack.unpackb(encoded, raw=False)

    assert all("sig" in entry for entry in request["txn-groups"][0]["txns"])

@pytest.mark.group_simulate_tests
def test_build_unsigned_simulate_request_omits_signatures():
    """build_unsigned_simulate_request() produces txn entries carrying only a txn key, with no sig"""
    encoded = build_unsigned_simulate_request(_unsigned_payment(), _simulate_options())
    request = msgpack.unpackb(encoded, raw=False)

    entries = request["txn-groups"][0]["txns"]
    assert entries, "expected at least one transaction entry"
    # No sig key at all, rather than a zero-filled one.
    assert all(list(entry.keys()) == ["txn"] for entry in entries)
    assert request["allow-empty-signatures"] is True
    assert request["fix-signers"] is True

# Polytest Group: Composer Tests


@pytest.mark.group_composer_tests
def test_compose_payment():
    """compose() with a single Payment TxnParams produces the canonical encoded transaction"""
    txn_params = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(),
            receiver=RECEIVER_ADDRESS,
            amount=101_000,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )

    encoded = compose([txn_params], composer_params())
    assert len(encoded) == 1

    decoded = decode_transaction(encoded[0])
    assert decoded.transaction_type == TransactionType.PAYMENT
    assert decoded.payment.amount == 101_000
    assert decoded.payment.receiver == RECEIVER_ADDRESS
    assert decoded.sender == SENDER_ADDRESS
    # Single-entry compose must not assign a group.
    assert decoded.group is None


@pytest.mark.group_composer_tests
def test_compose_asset_opt_in():
    """compose() with an AssetOptIn TxnParams produces a 0-amount asset transfer to self"""
    txn_params = TxnParams(
        kind=TxnParamsKind.ASSET_OPT_IN,
        payment=None,
        asset_transfer=None,
        asset_opt_in=AssetOptInParams(
            common=common_params(),
            asset_id=107_686_045,
        ),
        online_key_reg=None,
        offline_key_reg=None,
    )

    encoded = compose([txn_params], composer_params())
    decoded = decode_transaction(encoded[0])

    assert decoded.transaction_type == TransactionType.ASSET_TRANSFER
    assert decoded.asset_transfer.amount == 0
    assert decoded.asset_transfer.asset_id == 107_686_045
    # Opt-in is the canonical 0-amount self-transfer: sender == receiver.
    assert decoded.asset_transfer.receiver == SENDER_ADDRESS


@pytest.mark.group_composer_tests
def test_compose_asset_transfer():
    """compose() with an AssetTransfer TxnParams produces the canonical encoded transaction"""
    txn_params = TxnParams(
        kind=TxnParamsKind.ASSET_TRANSFER,
        payment=None,
        asset_transfer=AssetTransferParams(
            common=common_params(),
            asset_id=107_686_045,
            receiver=RECEIVER_ADDRESS,
            amount=42,
            clawback_target=None,
            close_asset_to=None,
        ),
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )

    encoded = compose([txn_params], composer_params())
    decoded = decode_transaction(encoded[0])

    assert decoded.transaction_type == TransactionType.ASSET_TRANSFER
    assert decoded.asset_transfer.asset_id == 107_686_045
    assert decoded.asset_transfer.amount == 42
    assert decoded.asset_transfer.receiver == RECEIVER_ADDRESS


@pytest.mark.group_composer_tests
def test_compose_online_keyreg():
    """compose() with an OnlineKeyReg TxnParams produces the canonical encoded transaction"""
    txn_params = TxnParams(
        kind=TxnParamsKind.ONLINE_KEY_REG,
        payment=None,
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=OnlineKeyRegParams(
            common=common_params(),
            vote_key=bytes(32),
            selection_key=bytes(32),
            state_proof_key=bytes(64),
            vote_first=50_000_000,
            vote_last=50_001_000,
            vote_key_dilution=1000,
        ),
        offline_key_reg=None,
    )

    encoded = compose([txn_params], composer_params())
    decoded = decode_transaction(encoded[0])

    assert decoded.transaction_type == TransactionType.KEY_REGISTRATION
    assert decoded.key_registration.vote_key == bytes(32)
    assert decoded.key_registration.selection_key == bytes(32)
    assert decoded.key_registration.state_proof_key == bytes(64)
    assert decoded.key_registration.vote_first == 50_000_000
    assert decoded.key_registration.vote_last == 50_001_000
    assert decoded.key_registration.vote_key_dilution == 1000


@pytest.mark.group_composer_tests
def test_compose_offline_keyreg():
    """compose() with an OfflineKeyReg TxnParams produces the canonical encoded transaction"""
    txn_params = TxnParams(
        kind=TxnParamsKind.OFFLINE_KEY_REG,
        payment=None,
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=OfflineKeyRegParams(common=common_params()),
    )

    encoded = compose([txn_params], composer_params())
    decoded = decode_transaction(encoded[0])

    assert decoded.transaction_type == TransactionType.KEY_REGISTRATION
    assert decoded.key_registration.vote_key is None
    assert decoded.key_registration.selection_key is None
    assert decoded.key_registration.state_proof_key is None
    assert decoded.key_registration.vote_first is None
    assert decoded.key_registration.vote_last is None
    assert decoded.key_registration.vote_key_dilution is None


@pytest.mark.group_composer_tests
def test_compose_multi_entry_assigns_shared_group():
    """compose() with more than one TxnParams returns transactions that share an atomic group ID"""
    p1 = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(),
            receiver=RECEIVER_ADDRESS,
            amount=1_000_000,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )
    p2 = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(),
            receiver=RECEIVER_ADDRESS,
            amount=200_000,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )

    encoded = compose([p1, p2], composer_params())
    assert len(encoded) == 2

    d1 = decode_transaction(encoded[0])
    d2 = decode_transaction(encoded[1])

    assert d1.group is not None
    assert d2.group is not None
    assert d1.group == d2.group


@pytest.mark.group_composer_tests
def test_sign_single_transaction():
    """sign_transactions() with one encoded transaction and one secret key produces the canonical signed bytes"""
    txn_params = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(),
            receiver=RECEIVER_ADDRESS,
            amount=1_000,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )

    encoded = compose([txn_params], composer_params())

    # Any deterministic 32-byte secret key — content doesn't matter for plumbing verification.
    secret_key = bytes(32)
    signing_key = SigningKey(secret_key)
    expected_signature = signing_key.sign(encoded[0]).signature

    signed = sign_transactions([encoded[0]], [secret_key])
    assert len(signed) == 1

    decoded = decode_signed_transaction(signed[0])
    assert decoded.signature == expected_signature


@pytest.mark.group_composer_tests
def test_sign_paired_transactions():
    """sign_transactions() pairs each transaction with the secret key at the same index"""
    p1 = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(),
            receiver=RECEIVER_ADDRESS,
            amount=1,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )
    p2 = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(sender=RECEIVER_ADDRESS),
            receiver=SENDER_ADDRESS,
            amount=2,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )

    encoded = compose([p1, p2], composer_params())

    key_a = bytes([1] * 32)
    key_b = bytes([2] * 32)
    signed = sign_transactions(encoded, [key_a, key_b])

    expected_a = SigningKey(key_a).sign(encoded[0]).signature
    expected_b = SigningKey(key_b).sign(encoded[1]).signature

    assert decode_signed_transaction(signed[0]).signature == expected_a
    assert decode_signed_transaction(signed[1]).signature == expected_b
    # Confirm the pairing is index-based, not crossed.
    assert decode_signed_transaction(signed[0]).signature != expected_b


@pytest.mark.group_composer_tests
def test_sign_rejects_length_mismatch():
    """sign_transactions() returns an error when txn count and key count differ"""
    txn_params = TxnParams(
        kind=TxnParamsKind.PAYMENT,
        payment=PaymentParams(
            common=common_params(),
            receiver=RECEIVER_ADDRESS,
            amount=1,
            close_remainder_to=None,
        ),
        asset_transfer=None,
        asset_opt_in=None,
        online_key_reg=None,
        offline_key_reg=None,
    )
    encoded = compose([txn_params], composer_params())

    with pytest.raises(AlgoKitComposerError.SignerCountMismatch):
        sign_transactions(encoded, [])