from algokit_utils import Composer
from algokit_transact import (
    Transaction,
    TransactionType,
    address_from_string,
    PaymentTransactionFields,
)
import requests


class AlgodClient:
    def json(self, path: str):
        return requests.get("https://testnet-api.4160.nodely.dev" + path)


def test_new_composer():
    _ = Composer(AlgodClient())


def test_composer_add_transaction():
    tx = Transaction(
        transaction_type=TransactionType.PAYMENT,
        sender=address_from_string(
            "LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"
        ),
        first_valid=1,
        last_valid=10,
        genesis_hash=b"a" * 32,
        genesis_id="",
        payment=PaymentTransactionFields(
            receiver=address_from_string(
                "LAIXFJCAPMTKK5ZYQVWJE7F5P73PJ24QMJE774DHTVGRVH4JAS4RHD6VGQ"
            ),
            amount=1000,  # microAlgos
        ),
    )
    composer = Composer(AlgodClient())
    composer.add_transaction(tx)
