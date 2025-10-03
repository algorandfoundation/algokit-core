from typing import override
import typing
from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
from algokit_transact import (
    OnApplicationComplete,
    SignedTransaction,
    Transaction,
    encode_transaction,
)
from algokit_utils import AlgodClient, TransactionSigner
from algokit_utils.algokit_utils_ffi import (
    AbiMethod,
    AbiMethodArg,
    AbiMethodArgType,
    AppCallMethodCallParams,
    AppCallParams,
    AppCreateParams,
    AppMethodCallArg,
    Composer,
    PaymentParams,
    TransactionSignerGetter,
    AbiValue,
)
from algosdk.mnemonic import to_private_key
from nacl.signing import SigningKey
import base64
import pytest
import requests

MN = "gas net tragic valid celery want good neglect maid nuclear core false chunk place asthma three acoustic moon box million finish bargain onion ability shallow"
SEED_B64: str = to_private_key(MN)  # type: ignore
SEED_BYTES = base64.b64decode(SEED_B64)
KEY = SigningKey(SEED_BYTES[:32])
ADDR = "ON6AOPBATSSEL47ML7EPXATHGH7INOWONHWITMQEDRPXHTMDJYMPQXROMA"


class TestSigner(TransactionSigner):
    def __init__(self, private_key: bytes):
        self.signing_key = SigningKey(private_key)

    @override
    async def sign_transactions(  # type: ignore
        self, transactions: list[Transaction], indices: list[int]
    ) -> list[SignedTransaction]:
        stxns = []
        for transaction in transactions:
            tx_for_signing = encode_transaction(transaction)
            sig = self.signing_key.sign(tx_for_signing)
            stxns.append(
                SignedTransaction(transaction=transaction, signature=sig.signature)
            )

        return stxns

    @override
    async def sign_transaction(self, transaction: Transaction) -> SignedTransaction:  # type: ignore
        return (await self.sign_transactions([transaction], [0]))[0]


class SignerGetter(TransactionSignerGetter):
    @override
    def get_signer(self, address: str) -> TransactionSigner:  # type: ignore
        return TestSigner(SEED_BYTES[:32])

    @override
    def register_account(self, address: str, mnemonic: str) -> None:  # type: ignore
        # No-op: backwards compatibility
        pass


class MultiAccountSignerGetter(TransactionSignerGetter):
    """TransactionSignerGetter implementation that manages multiple test accounts"""

    def __init__(self):
        self.signers: dict[str, TestSigner] = {}
        # Register default test account
        self.register_account(ADDR, MN)

    @override
    def get_signer(self, address: str) -> TransactionSigner:  # type: ignore
        if address in self.signers:
            return self.signers[address]

        raise Exception(f"No signer registered for address: {address}")

    @override
    def register_account(self, address: str, mnemonic_phrase: str) -> None:  # type: ignore
        """Register an account with its mnemonic for signing"""
        # Convert mnemonic to private key
        private_key = to_private_key(mnemonic_phrase)  # type: ignore
        private_key_bytes = base64.b64decode(private_key)

        # Create and store the signer
        self.signers[address] = TestSigner(private_key_bytes[:32])


class HttpClientImpl(HttpClient):
    @override
    async def request(  # type: ignore
        self,
        method: HttpMethod,
        path: str,
        query: typing.Optional[dict[str, str]],
        body: typing.Optional[bytes],
        headers: typing.Optional[dict[str, str]],
    ) -> HttpResponse:
        headers = headers or {}
        headers["X-Algo-API-Token"] = "a" * 64

        if method == HttpMethod.GET:
            res = requests.get(
                f"http://localhost:4001/{path}", params=query, headers=headers
            )
        elif method == HttpMethod.POST:
            res = requests.post(
                f"http://localhost:4001/{path}",
                params=query,
                data=body,
                headers=headers,
            )
        else:
            raise NotImplementedError(
                f"HTTP method {method} not implemented in test client"
            )

        if res.status_code != 200:
            from algokit_utils.algokit_http_client import HttpError

            raise HttpError.RequestError(
                f"HTTP request failed: {res.status_code} {res.text}"
            )

        # NOTE: Headers needing to be lowercase was a bit surprising, so we need to make sure we document that
        headers = {k.lower(): v for k, v in res.headers.items()}

        return HttpResponse(body=res.content, headers=headers)


@pytest.mark.skip(reason="Will be refactored later. Keeping test for reference")
@pytest.mark.asyncio
async def test_composer():
    algod = AlgodClient(HttpClientImpl())

    composer = Composer(
        algod_client=algod,
        signer_getter=MultiAccountSignerGetter(),
    )

    composer.add_payment(
        params=PaymentParams(
            amount=1,
            receiver=ADDR,
            sender=ADDR,
            signer=None,
            rekey_to=None,
            note=None,
            lease=None,
            static_fee=None,
            extra_fee=None,
            max_fee=None,
            validity_window=None,
            first_valid_round=None,
            last_valid_round=None,
        )
    )

    await composer.build()
    response = await composer.send()
    assert len(response.transaction_ids) == 1
    assert len(response.transaction_ids[0]) == 52
    print(response.transaction_ids)


INT_1_PROG = bytes.fromhex("0b810143")


@pytest.mark.skip(reason="Will be refactored later. Keeping test for reference")
@pytest.mark.asyncio
async def test_app_create_and_call():
    algod = AlgodClient(HttpClientImpl())

    create_composer = Composer(
        algod_client=algod,
        signer_getter=MultiAccountSignerGetter(),
    )

    create_composer.add_app_create(
        params=AppCreateParams(
            sender=ADDR,
            on_complete=OnApplicationComplete.NO_OP,
            approval_program=INT_1_PROG,
            clear_state_program=INT_1_PROG,
            signer=None,
            rekey_to=None,
            note=None,
            lease=None,
            static_fee=None,
            extra_fee=None,
            max_fee=None,
            validity_window=None,
            first_valid_round=None,
            last_valid_round=None,
        )
    )

    await create_composer.build()
    response = await create_composer.send()
    assert len(response.transaction_ids) == 1
    assert len(response.transaction_ids[0]) == 52

    app_id = response.app_ids[0]
    assert app_id

    call_composer = Composer(
        algod_client=algod,
        signer_getter=MultiAccountSignerGetter(),
    )

    call_composer.add_app_call(
        params=AppCallParams(
            sender=ADDR,
            app_id=app_id,
            on_complete=OnApplicationComplete.NO_OP,
            signer=None,
            rekey_to=None,
            note=None,
            lease=None,
            static_fee=None,
            extra_fee=None,
            max_fee=None,
            validity_window=None,
            first_valid_round=None,
            last_valid_round=None,
        )
    )

    await call_composer.build()
    response = await call_composer.send()
    assert len(response.transaction_ids) == 1
    assert len(response.transaction_ids[0]) == 52

    method_composer = Composer(
        algod_client=algod,
        signer_getter=MultiAccountSignerGetter(),
    )

    method_composer.add_app_call_method_call(
        params=AppCallMethodCallParams(
            sender=ADDR,
            app_id=app_id,
            args=[AppMethodCallArg.ABI_VALUE(AbiValue.bool(True))],
            on_complete=OnApplicationComplete.NO_OP,
            method=AbiMethod(
                name="myMethod",
                args=[
                    AbiMethodArg(
                        arg_type=AbiMethodArgType.VALUE(bool_type),
                        name="a",
                        description="",
                    )
                ],
                returns=None,
                description="",
            ),
            signer=None,
            rekey_to=None,
            note=None,
            lease=None,
            static_fee=None,
            extra_fee=None,
            max_fee=None,
            validity_window=None,
            first_valid_round=None,
            last_valid_round=None,
        )
    )

    method_response = await method_composer.send()
    assert len(method_response.transaction_ids) == 1
    assert len(method_response.transaction_ids[0]) == 52
