# LedgerStateDeltaForTransactionGroup

Contains a ledger delta for a single transaction group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delta** | **object** | Ledger StateDelta object | 
**ids** | **List[str]** |  | 

## Example

```python
from algokit_algod_api.models.ledger_state_delta_for_transaction_group import LedgerStateDeltaForTransactionGroup

# TODO update the JSON string below
json = "{}"
# create an instance of LedgerStateDeltaForTransactionGroup from a JSON string
ledger_state_delta_for_transaction_group_instance = LedgerStateDeltaForTransactionGroup.from_json(json)
# print the JSON string representation of the object
print(LedgerStateDeltaForTransactionGroup.to_json())

# convert the object into a dict
ledger_state_delta_for_transaction_group_dict = ledger_state_delta_for_transaction_group_instance.to_dict()
# create an instance of LedgerStateDeltaForTransactionGroup from a dict
ledger_state_delta_for_transaction_group_from_dict = LedgerStateDeltaForTransactionGroup.from_dict(ledger_state_delta_for_transaction_group_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


