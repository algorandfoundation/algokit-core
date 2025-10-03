from dataclasses import dataclass
from pathlib import Path
import json
import re
from typing import Any
from algokit_transact import (
    PaymentTransactionFields,
    TransactionType,
    Transaction,
    AssetTransferTransactionFields,
    AssetConfigTransactionFields,
    AssetFreezeTransactionFields,
    AppCallTransactionFields,
    KeyRegistrationTransactionFields,
    OnApplicationComplete,
    HeartbeatTransactionFields,
    StateSchema,
    HeartbeatProof,
)
from algokit_transact.algokit_transact_ffi import (
    StateProofTransactionFields,
    StateProof,
    MerkleArrayProof,
    Reveal,
    StateProofMessage,
    HashFactory,
    SigslotCommit,
    Participant,
    FalconSignatureStruct,
    FalconVerifier,
    MerkleSignatureVerifier,
)
from nacl.signing import SigningKey


@dataclass
class TransactionTestData:
    transaction: Transaction
    id: str
    id_raw: bytes
    unsigned_bytes: bytes
    signed_bytes: bytes
    signing_private_key: SigningKey
    rekeyed_sender_auth_address: str
    rekeyed_sender_signed_bytes: bytes
    multisig_addresses: tuple[str, str]
    multisig_signed_bytes: bytes


@dataclass
class TestData:
    simple_payment: TransactionTestData
    opt_in_asset_transfer: TransactionTestData
    asset_create: TransactionTestData
    asset_destroy: TransactionTestData
    asset_config: TransactionTestData
    asset_freeze: TransactionTestData
    asset_unfreeze: TransactionTestData
    app_call: TransactionTestData
    app_create: TransactionTestData
    app_update: TransactionTestData
    app_delete: TransactionTestData
    online_key_registration: TransactionTestData
    offline_key_registration: TransactionTestData
    non_participation_key_registration: TransactionTestData
    heartbeat: TransactionTestData
    state_proof: TransactionTestData


def convert_values(obj: Any) -> Any:
    """
    Recursively convert values in the data structure to appropriate types.

    This manually converts nested Record types that uniffi doesn't automatically
    handle when passed as dict parameters. When a dict is passed to a constructor
    expecting a Record type, uniffi stores it as-is, but later serialization
    expects actual objects with attributes.

    Types handled:
    - StateSchema: num_uints, num_byte_slices
    - HeartbeatProof: sig, pk, pk2, pk1_sig, pk2_sig
    - StateProof nested types:
      * HashFactory: hash_type
      * MerkleArrayProof: hash_factory, path, tree_depth
      * FalconVerifier: public_key
      * FalconSignatureStruct: signature, vector_commitment_index, proof, verifying_key
      * SigslotCommit: sig, lower_sig_weight (defaults to 0 if missing)
      * MerkleSignatureVerifier: commitment, key_lifetime
      * Participant: verifier, weight
      * Reveal: position (defaults to 0 if missing), sigslot, participant
      * StateProofMessage: block_headers_commitment, voters_commitment, etc.
      * StateProof: sig_commit, signed_weight, sig_proofs, part_proofs, etc.
    - OnApplicationComplete enum conversion
    - Byte array conversion (list of 0-255 ints -> bytes)
    """
    if isinstance(obj, dict):
        # Convert StateSchema objects
        if "num_uints" in obj and "num_byte_slices" in obj and len(obj) == 2:
            return StateSchema(
                num_uints=obj["num_uints"], num_byte_slices=obj["num_byte_slices"]
            )

        # Convert HeartbeatProof objects
        if set(obj.keys()) == {"sig", "pk", "pk2", "pk1_sig", "pk2_sig"}:
            return HeartbeatProof(
                sig=convert_values(obj["sig"]),
                pk=convert_values(obj["pk"]),
                pk2=convert_values(obj["pk2"]),
                pk1_sig=convert_values(obj["pk1_sig"]),
                pk2_sig=convert_values(obj["pk2_sig"]),
            )

        # Convert HashFactory objects
        if set(obj.keys()) == {"hash_type"}:
            return HashFactory(hash_type=obj["hash_type"])

        # Convert MerkleArrayProof objects
        if set(obj.keys()) == {"hash_factory", "path", "tree_depth"}:
            return MerkleArrayProof(
                hash_factory=convert_values(obj["hash_factory"]),
                path=convert_values(obj["path"]),
                tree_depth=obj["tree_depth"],
            )

        # Convert FalconVerifier objects
        if set(obj.keys()) == {"public_key"}:
            return FalconVerifier(public_key=convert_values(obj["public_key"]))

        # Convert FalconSignatureStruct objects
        if set(obj.keys()) == {
            "signature",
            "vector_commitment_index",
            "proof",
            "verifying_key",
        }:
            return FalconSignatureStruct(
                signature=convert_values(obj["signature"]),
                vector_commitment_index=obj["vector_commitment_index"],
                proof=convert_values(obj["proof"]),
                verifying_key=convert_values(obj["verifying_key"]),
            )

        # Convert SigslotCommit objects - handle missing lower_sig_weight
        if "sig" in obj and isinstance(obj["sig"], dict):
            return SigslotCommit(
                sig=convert_values(obj["sig"]),
                lower_sig_weight=obj.get("lower_sig_weight", 0),
            )

        # Convert MerkleSignatureVerifier objects
        if set(obj.keys()) == {"commitment", "key_lifetime"}:
            return MerkleSignatureVerifier(
                commitment=convert_values(obj["commitment"]),
                key_lifetime=obj["key_lifetime"],
            )

        # Convert Participant objects
        if set(obj.keys()) == {"verifier", "weight"}:
            return Participant(
                verifier=convert_values(obj["verifier"]),
                weight=obj["weight"],
            )

        # Convert Reveal objects - handle missing position field
        if {"sigslot", "participant"}.issubset(obj.keys()):
            return Reveal(
                position=obj.get("position", 0),  # Default to 0 if missing
                sigslot=convert_values(obj["sigslot"]),
                participant=convert_values(obj["participant"]),
            )

        # Convert StateProofMessage objects
        if set(obj.keys()) == {
            "block_headers_commitment",
            "first_attested_round",
            "last_attested_round",
            "ln_proven_weight",
            "voters_commitment",
        }:
            return StateProofMessage(
                block_headers_commitment=convert_values(
                    obj["block_headers_commitment"]
                ),
                first_attested_round=obj["first_attested_round"],
                last_attested_round=obj["last_attested_round"],
                ln_proven_weight=obj["ln_proven_weight"],
                voters_commitment=convert_values(obj["voters_commitment"]),
            )

        # Convert StateProof objects - handle missing merkle_signature_salt_version
        state_proof_keys = {
            "sig_commit",
            "signed_weight",
            "sig_proofs",
            "part_proofs",
            "reveals",
            "positions_to_reveal",
        }
        if state_proof_keys.issubset(obj.keys()):
            return StateProof(
                sig_commit=convert_values(obj["sig_commit"]),
                signed_weight=obj["signed_weight"],
                sig_proofs=convert_values(obj["sig_proofs"]),
                part_proofs=convert_values(obj["part_proofs"]),
                merkle_signature_salt_version=obj.get(
                    "merkle_signature_salt_version", 0
                ),
                reveals=convert_values(obj["reveals"]),
                positions_to_reveal=obj["positions_to_reveal"],
            )

        # Convert on_complete field if present
        if "on_complete" in obj:
            obj = obj.copy()
            obj["on_complete"] = convert_on_complete(obj["on_complete"])

        return {key: convert_values(value) for key, value in obj.items()}

    elif isinstance(obj, list) and all(
        isinstance(x, int) and 0 <= x <= 255 for x in obj
    ):
        # Convert list of integers (0-255) to bytes
        return bytes(obj)
    elif isinstance(obj, list):
        return [convert_values(x) for x in obj]
    return obj


def camel_to_snake(name: str) -> str:
    name = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", name).lower()


def convert_case_recursive(obj: Any) -> Any:
    if isinstance(obj, dict):
        return {
            camel_to_snake(key): convert_case_recursive(value)
            for key, value in obj.items()
        }
    elif isinstance(obj, list):
        return [convert_case_recursive(x) for x in obj]
    return obj


def convert_on_complete(value: str) -> OnApplicationComplete:
    """Convert string on_complete values to enum values"""
    on_complete_mapping = {
        "NoOp": OnApplicationComplete.NO_OP,
        "OptIn": OnApplicationComplete.OPT_IN,
        "CloseOut": OnApplicationComplete.CLOSE_OUT,
        "ClearState": OnApplicationComplete.CLEAR_STATE,
        "UpdateApplication": OnApplicationComplete.UPDATE_APPLICATION,
        "DeleteApplication": OnApplicationComplete.DELETE_APPLICATION,
    }
    return on_complete_mapping.get(value, value)


def create_transaction_test_data(test_data: dict[str, Any]) -> TransactionTestData:
    """Generic function to create TransactionTestData from test data"""
    # Extract transaction data and signing key
    transaction_data = test_data.pop("transaction")
    signing_private_key = test_data.pop("signing_private_key")

    # Extract transaction type and remove it from transaction data
    transaction_type_str = transaction_data.pop("transaction_type")

    # Map transaction types to their corresponding classes and field names
    transaction_type_mapping = {
        "Payment": {
            "type": TransactionType.PAYMENT,
            "field_name": "payment",
            "field_class": PaymentTransactionFields,
        },
        "AssetTransfer": {
            "type": TransactionType.ASSET_TRANSFER,
            "field_name": "asset_transfer",
            "field_class": AssetTransferTransactionFields,
        },
        "AssetConfig": {
            "type": TransactionType.ASSET_CONFIG,
            "field_name": "asset_config",
            "field_class": AssetConfigTransactionFields,
        },
        "AssetFreeze": {
            "type": TransactionType.ASSET_FREEZE,
            "field_name": "asset_freeze",
            "field_class": AssetFreezeTransactionFields,
        },
        "AppCall": {
            "type": TransactionType.APP_CALL,
            "field_name": "app_call",
            "field_class": AppCallTransactionFields,
        },
        "KeyRegistration": {
            "type": TransactionType.KEY_REGISTRATION,
            "field_name": "key_registration",
            "field_class": KeyRegistrationTransactionFields,
        },
        "Heartbeat": {
            "type": TransactionType.HEARTBEAT,
            "field_name": "heartbeat",
            "field_class": HeartbeatTransactionFields,
        },
        "StateProof": {
            "type": TransactionType.STATE_PROOF,
            "field_name": "state_proof",
            "field_class": StateProofTransactionFields,
        },
    }

    # Get the transaction type configuration
    transaction_config = transaction_type_mapping.get(transaction_type_str)
    if not transaction_config:
        raise ValueError(f"Unknown transaction type: {transaction_type_str}")

    # Extract the specific transaction field data
    transaction_field_data = transaction_data.pop(transaction_config["field_name"])

    # Handle assetFreeze objects - ensure frozen field defaults to false if missing
    if transaction_type_str == "AssetFreeze" and "frozen" not in transaction_field_data:
        transaction_field_data["frozen"] = False

    # Build the transaction kwargs
    transaction_kwargs = {
        **transaction_data,
        "transaction_type": transaction_config["type"],
        transaction_config["field_name"]: transaction_config["field_class"](
            **transaction_field_data
        ),
    }

    # default genesis_id to None
    if "genesis_id" not in transaction_kwargs:
        transaction_kwargs["genesis_id"] = None

    return TransactionTestData(
        **test_data,
        transaction=Transaction(**transaction_kwargs),
        signing_private_key=SigningKey(signing_private_key),
    )


def load_test_data() -> TestData:
    """Load and process test data from JSON file"""
    # Get the path to test_data.json relative to this test file
    test_data_path = (
        Path(__file__).parent.parent.parent.parent.parent
        / "crates"
        / "algokit_transact_ffi"
        / "test_data.json"
    )

    with open(test_data_path) as f:
        data = json.load(f)

    # Convert values and case
    data = convert_values(convert_case_recursive(data))

    # Create test data objects generically
    test_data_objects = {
        key: create_transaction_test_data(value.copy()) for key, value in data.items()
    }

    return TestData(**test_data_objects)


TEST_DATA = load_test_data()
