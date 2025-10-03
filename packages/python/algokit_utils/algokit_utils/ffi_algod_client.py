"""
Python implementation of AlgodClientTrait foreign trait.

This enables Python to provide async algod operations to Rust test code,
with Python controlling the async context while Rust orchestrates test logic.
"""

import asyncio
import base64
import json
from typing import List
from algokit_utils.algokit_http_client import HttpClient, HttpMethod
from algokit_utils.algokit_utils_ffi import (
    AlgodClientTrait,
    AccountInfo,
    TransactionInfo,
    SuggestedParams,
    UtilsError,
)


class PythonAlgodClient(AlgodClientTrait):  # type: ignore
    """Python implementation of async AlgodClient trait using HttpClient"""

    def __init__(self, http_client: HttpClient):
        self.http_client = http_client

    async def send_transaction(self, txn: List[int]) -> str:  # type: ignore
        """Send transaction bytes and return transaction ID"""
        txn_bytes = bytes(txn)  # Convert from List[int] to bytes

        try:
            response = await self.http_client.request(
                method=HttpMethod.POST,
                path="v2/transactions",
                query=None,
                body=txn_bytes,
                headers={"Content-Type": "application/x-binary"},
            )

            result = json.loads(response.body.decode("utf-8"))
            return result["txId"]
        except Exception as e:
            raise UtilsError.UtilsError(f"Failed to send transaction: {e}")

    async def get_account_info(self, address: str) -> AccountInfo:  # type: ignore
        """Get account information from algod"""
        try:
            response = await self.http_client.request(
                method=HttpMethod.GET,
                path=f"v2/accounts/{address}",
                query=None,
                body=None,
                headers=None,
            )

            data = json.loads(response.body.decode("utf-8"))
            return AccountInfo(
                balance=data.get("amount", 0),
                min_balance=data.get("min-balance", 0),
                created_apps=[app["id"] for app in data.get("created-apps", [])],
                created_assets=[
                    asset["index"] for asset in data.get("created-assets", [])
                ],
            )
        except Exception as e:
            raise UtilsError.UtilsError(f"Failed to get account info: {e}")

    async def get_transaction_info(self, tx_id: str) -> TransactionInfo:  # type: ignore
        """Get transaction information by ID"""
        try:
            # Try pending transactions first
            response = await self.http_client.request(
                method=HttpMethod.GET,
                path=f"v2/transactions/pending/{tx_id}",
                query=None,
                body=None,
                headers=None,
            )

            data = json.loads(response.body.decode("utf-8"))
        except Exception:
            # If not found in pending, try confirmed transactions
            try:
                response = await self.http_client.request(
                    method=HttpMethod.GET,
                    path=f"v2/transactions/{tx_id}",
                    query=None,
                    body=None,
                    headers=None,
                )

                data = json.loads(response.body.decode("utf-8"))
            except Exception as e:
                raise UtilsError.UtilsError(f"Transaction not found: {e}")

        return TransactionInfo(
            tx_id=tx_id,
            confirmed_round=data.get("confirmed-round"),
            asset_id=data.get("asset-index"),
            app_id=data.get("application-index"),
        )

    async def wait_for_confirmation(self, tx_id: str) -> TransactionInfo:  # type: ignore
        """Wait for transaction confirmation"""
        for attempt in range(10):  # Wait up to 10 rounds
            try:
                info = await self.get_transaction_info(tx_id)
                if info.confirmed_round:
                    return info
            except:
                pass
            await asyncio.sleep(1)

        raise UtilsError.UtilsError(
            f"Transaction {tx_id} not confirmed after 10 rounds"
        )

    async def get_suggested_params(self) -> SuggestedParams:  # type: ignore
        """Get suggested transaction parameters"""
        try:
            response = await self.http_client.request(
                method=HttpMethod.GET,
                path="v2/transactions/params",
                query=None,
                body=None,
                headers=None,
            )

            data = json.loads(response.body.decode("utf-8"))
            return SuggestedParams(
                fee=data.get("fee", 1000),
                first_valid_round=data["last-round"],
                last_valid_round=data["last-round"] + 1000,
                genesis_hash=list(base64.b64decode(data["genesis-hash"])),
                genesis_id=data["genesis-id"],
            )
        except Exception as e:
            raise UtilsError.UtilsError(f"Failed to get suggested params: {e}")