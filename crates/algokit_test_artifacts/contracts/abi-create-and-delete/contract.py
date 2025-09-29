from algopy import ARC4Contract, arc4, gtxn, TemplateVar, UInt64, String


class application(ARC4Contract):
    @arc4.abimethod(create="require")
    def create(self, pay_txn: gtxn.PaymentTransaction) -> String:
        """Create the application"""
        return String("created")

    @arc4.baremethod(allow_actions=["UpdateApplication"])
    def update(self) -> None:
        assert TemplateVar[bool]("UPDATABLE")

    @arc4.abimethod(allow_actions=["DeleteApplication"])
    def delete(self, pay_txn: gtxn.PaymentTransaction) -> String:
        """Delete the application"""
        assert TemplateVar[UInt64]("DELETABLE")
        return String("deleted")
