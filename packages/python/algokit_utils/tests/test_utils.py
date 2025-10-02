from typing import override
import typing
from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
from algokit_transact import OnApplicationComplete, SignedTransaction, Transaction, encode_transaction
from algokit_utils import AlgodClient, TransactionSigner
from algokit_utils.algokit_utils_ffi import (
    AbiMethod,
    AbiMethodArg,
    AbiMethodArgType,
    AbiType,
    AppCallMethodCallParams,
    AppCallParams,
    AppCreateParams,
    AppMethodCallArg,
    Composer,
    PaymentParams,
    StructField,
    StructFieldType,
    TransactionSignerGetter,
    AbiValue
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
    @override
    async def sign_transactions(  # type: ignore
        self, transactions: list[Transaction], indices: list[int]
    ) -> list[SignedTransaction]:
        stxns = []
        for transaction in transactions:
            tx_for_signing = encode_transaction(transaction)
            sig = KEY.sign(tx_for_signing)
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
        return TestSigner()


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
            raise Exception(f"HTTP request failed: {res.status_code} {res.text}")

        # NOTE: Headers needing to be lowercase was a bit surprising, so we need to make sure we document that
        headers = {k.lower(): v for k, v in res.headers.items()}

        return HttpResponse(body=res.content, headers=headers)


@pytest.mark.asyncio
async def test_composer():
    algod = AlgodClient(HttpClientImpl())

    composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

    composer.add_payment(
        params=PaymentParams(
            amount=1,
            receiver=ADDR,
            sender=ADDR,
        )
    )

    await composer.build()
    response = await composer.send()
    assert len(response.transaction_ids) == 1
    assert len(response.transaction_ids[0]) == 52
    print(response.transaction_ids)

bool_type = AbiType.bool()
uint_tuple_type = AbiType.tuple([AbiType.uint(64)])
uint_dynamic_array_type = AbiType.dynamic_array(AbiType.uint(64))
uint_in_struct_type = AbiType.struct_fields(name="uint_struct", fields=[StructField(name="uint_field", field_type=StructFieldType.TYPE(AbiType.uint(64)))])

def test_abi_uint_in_struct():
    expected_encoding = b'\x00\x00\x00\x00\x00\x00\x00\x07'
    uint_struct_val: AbiValue = AbiValue.struct_fields(fields={"uint_field": AbiValue.uint(7)})
    assert uint_in_struct_type.encode(uint_struct_val) == expected_encoding
    assert uint_in_struct_type.decode(expected_encoding).get_struct_fields() == {"uint_field": AbiValue.uint(7)}

def test_abi_uint_tuple():
    expected_encoding = b'\x00\x00\x00\x00\x00\x00\x00\x07'
    uint_tuple_val: AbiValue = AbiValue.array([AbiValue.uint(7)])
    assert uint_tuple_type.encode(uint_tuple_val) == expected_encoding
    assert uint_tuple_type.decode(expected_encoding).get_array() == [AbiValue.uint(7)]


def test_abi_uint_dynamic_array():
    expected_encoding = b'\x00\x01\x00\x00\x00\x00\x00\x00\x00\x07'
    uint_dynamic_array_val: AbiValue = AbiValue.array([AbiValue.uint(7)])

    assert uint_dynamic_array_type.encode(uint_dynamic_array_val) == expected_encoding
    assert uint_dynamic_array_type.decode(expected_encoding).get_array() == [AbiValue.uint(7)]

def test_abi_bool():
    bool_val: AbiValue = AbiValue.bool(True)
    assert bool_type.encode(bool_val) == b'\x80'
    assert bool_type.decode(b'\x80').get_bool() == True

bool_array_type = AbiType.from_string("bool[]")
def test_abi_bool_array():

    bool_array_val: AbiValue = AbiValue.array([AbiValue.bool(True)])
    assert bool_array_type.encode(bool_array_val) == b'\x00\x01\x80'
    assert bool_array_type.decode(b'\x00\x01\x80').get_array() == [AbiValue.bool(True)]

INT_1_PROG = bytes.fromhex('0b810143')

@pytest.mark.asyncio
async def test_app_create_and_call():
    algod = AlgodClient(HttpClientImpl())


    create_composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

    create_composer.add_app_create(
        params=AppCreateParams(
            sender=ADDR,
            on_complete=OnApplicationComplete.NO_OP,
            approval_program=INT_1_PROG,
            clear_state_program=INT_1_PROG,
        )
    )

    await create_composer.build()
    response = await create_composer.send()
    assert(len(response.transaction_ids) == 1)
    assert(len(response.transaction_ids[0]) == 52)

    app_id = response.app_ids[0]
    assert app_id

    call_composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

    call_composer.add_app_call(
        params=AppCallParams(
            sender=ADDR,
            app_id=app_id,
            on_complete=OnApplicationComplete.NO_OP,
        )
    )

    await call_composer.build()
    response = await call_composer.send()
    assert(len(response.transaction_ids) == 1)
    assert(len(response.transaction_ids[0]) == 52)

    method_composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

    method_composer.add_app_call_method_call(
        params=AppCallMethodCallParams(
                sender=ADDR,
                app_id=app_id,
                args=[AppMethodCallArg.ABI_VALUE(AbiValue.bool(True))],
                on_complete=OnApplicationComplete.NO_OP,
                method=AbiMethod(name="myMethod", args=[AbiMethodArg(arg_type=AbiMethodArgType.VALUE(bool_type), name="a", description="")], returns=None, description="")
        )
    )

    method_response = await method_composer.send()
    assert(len(method_response.transaction_ids) == 1)
    assert(len(method_response.transaction_ids[0]) == 52)
