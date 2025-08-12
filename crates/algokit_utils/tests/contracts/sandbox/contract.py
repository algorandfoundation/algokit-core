import typing as t
from algopy import (
    ARC4Contract,
    Txn,
    arc4,
    gtxn,
    Bytes,
    itxn,
    Asset,
    Application,
    Account,
)


StaticInts: t.TypeAlias = arc4.StaticArray[arc4.UInt64, t.Literal[4]]
DynamicInts: t.TypeAlias = arc4.DynamicArray[arc4.UInt64]
DynamicNestedInts: t.TypeAlias = arc4.DynamicArray[DynamicInts]
ReturnType: t.TypeAlias = arc4.Tuple[
    DynamicNestedInts, arc4.Tuple[DynamicInts, arc4.String]
]


class Sandbox(ARC4Contract):
    @arc4.abimethod
    def hello_world(self, name: arc4.String) -> arc4.String:
        return arc4.String("Hello, ") + name

    @arc4.abimethod
    def add(self, a: arc4.UInt64, b: arc4.UInt64) -> arc4.UInt64:
        return arc4.UInt64(a.native + b.native)

    @arc4.abimethod
    def get_pay_txn_amount(self, pay_txn: gtxn.PaymentTransaction) -> arc4.UInt64:
        return arc4.UInt64(pay_txn.amount)

    @arc4.abimethod
    def get_pay_txns_amount_sum(
        self,
        pay_txn_1: gtxn.PaymentTransaction,
        pay_txn_2: gtxn.PaymentTransaction,
        app_call_txn: gtxn.ApplicationCallTransaction,
    ) -> arc4.UInt64:
        return arc4.UInt64(
            pay_txn_1.amount
            + pay_txn_2.amount
            + arc4.UInt64.from_log(app_call_txn.last_log).native
        )

    @arc4.abimethod
    def echo_bytes(self, a: Bytes) -> Bytes:
        return a

    @arc4.abimethod
    def echo_static_array(self, arr: StaticInts) -> StaticInts:
        return arr

    @arc4.abimethod
    def echo_dynamic_array(self, arr: DynamicInts) -> DynamicInts:
        return arr

    @arc4.abimethod
    def nest_array_and_tuple(
        self, arr: DynamicNestedInts, tuple: arc4.Tuple[DynamicInts, arc4.String]
    ) -> ReturnType:
        (child_array, str) = tuple.native

        return ReturnType(
            (
                arr.copy(),
                arc4.Tuple[DynamicInts, arc4.String]((child_array.copy(), str)),
            )
        )

    @arc4.abimethod
    def echo_boolean(self, bool: arc4.Bool) -> arc4.Bool:
        return bool

    @arc4.abimethod
    def inner_pay_appl(self, appId: arc4.UInt64) -> arc4.UInt64:
        payTxn = itxn.Payment(
            receiver=Txn.sender,
            amount=100000,
        )

        result, _txn = arc4.abi_call[arc4.UInt64](
            "get_pay_txn_amount(pay)uint64", payTxn, app_id=appId.native
        )

        return result

    @arc4.abimethod
    def get_returned_value_of_app_call_txn(
        self, app_call_txn: gtxn.ApplicationCallTransaction
    ) -> arc4.UInt64:
        return arc4.UInt64.from_log(app_call_txn.last_log)

    @arc4.abimethod
    def more_than_15_args_with_ref_types(
        self,
        a1: arc4.UInt64,
        a2: arc4.UInt64,
        a3: arc4.UInt64,
        a4: arc4.UInt64,
        a5: arc4.UInt64,
        a6: arc4.UInt64,
        a7: arc4.UInt64,
        a8: arc4.UInt64,
        a9: arc4.UInt64,
        a10: arc4.UInt64,
        a11: arc4.UInt64,
        a12: arc4.UInt64,
        a13: arc4.UInt64,
        a14: arc4.UInt64,
        a15: arc4.UInt64,
        a16: arc4.UInt64,
        a17: arc4.UInt64,
        asset: Asset,
        a18: arc4.UInt64,
        application: Application,
        pay: gtxn.PaymentTransaction,
        account: Account,
    ) -> arc4.Tuple[arc4.UInt64, arc4.UInt64, arc4.UInt64, arc4.DynamicBytes]:
        result = arc4.Tuple[arc4.UInt64, arc4.UInt64, arc4.UInt64, arc4.DynamicBytes](
            (
                arc4.UInt64(asset.id),
                arc4.UInt64(application.id),
                arc4.UInt64(account.balance),
                arc4.DynamicBytes(pay.txn_id),
            )
        )
        return result

    @arc4.abimethod
    def more_than_15_args(
        self,
        a1: arc4.UInt64,
        a2: arc4.UInt64,
        a3: arc4.UInt64,
        a4: arc4.UInt64,
        a5: arc4.UInt64,
        a6: arc4.UInt64,
        a7: arc4.UInt64,
        a8: arc4.UInt64,
        a9: arc4.UInt64,
        a10: arc4.UInt64,
        a11: arc4.UInt64,
        a12: arc4.UInt64,
        a13: arc4.UInt64,
        a14: arc4.UInt64,
        a15: arc4.UInt64,
        a16: arc4.UInt64,
        a17: arc4.UInt64,
        a18: arc4.UInt64,
    ) -> arc4.DynamicArray[arc4.UInt64]:
        result = arc4.DynamicArray[arc4.UInt64](
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
            a10,
            a11,
            a12,
            a13,
            a14,
            a15,
            a16,
            a17,
            a18,
        )
        return result
