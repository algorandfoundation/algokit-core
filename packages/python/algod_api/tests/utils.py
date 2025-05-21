from random import randbytes, randint

import pytest
from algokit_utils import AlgorandClient, AssetCreateParams, SigningAccount

from algokit_algod_api.exceptions import ApiException


def handle_api_exception(e: ApiException) -> None:
    """
    Handle API exceptions consistently.

    Args:
        e: API exception to handle
    """
    if e.status == 401:
        pytest.skip(f"Authentication required or failed: {e}")
    elif e.status == 404:
        pytest.skip(f"Endpoint not available or resource not found: {e}")
    elif e.status == 501:
        pytest.skip(f"API not implemented: {e}")
    else:
        pytest.fail(f"API Exception: {e}")

def create_random_asset(algorand: AlgorandClient, creator: SigningAccount) -> int:
    """
    Create a random asset for testing purposes.

    Returns:
        A dictionary representing the asset.
    """
    expected_total = randint(1, 10000)
    response = algorand.send.asset_create(
        AssetCreateParams(
            sender=creator.address,
            total=expected_total,
            decimals=0,
            default_frozen=False,
            unit_name="TEST",
            asset_name=f"Test {randint(1, 1000)}",
            url="https://example.com",
            note=randbytes(10),
        )
    )
    return response.asset_id
