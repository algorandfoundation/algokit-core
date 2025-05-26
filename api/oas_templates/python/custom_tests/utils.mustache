from random import randbytes, randint

import pytest
from algokit_utils import AlgorandClient, AssetCreateParams, SigningAccount

from algokit_algod_api.exceptions import ApiException


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
