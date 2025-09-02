from dataclasses import dataclass
from pathlib import Path
import json
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
    StateSchema,
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


def convert_values(obj: Any) -> Any:
    """Recursively convert values in the data structure to appropriate types"""
    if isinstance(obj, dict):
        # Convert StateSchema objects
        if "num_uints" in obj and "num_byte_slices" in obj and len(obj) == 2:
            return StateSchema(
                num_uints=obj["num_uints"], num_byte_slices=obj["num_byte_slices"]
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
    import re

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
