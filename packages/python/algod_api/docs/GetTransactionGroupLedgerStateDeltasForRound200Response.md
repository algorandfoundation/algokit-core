# GetTransactionGroupLedgerStateDeltasForRound200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deltas** | [**List[LedgerStateDeltaForTransactionGroup]**](LedgerStateDeltaForTransactionGroup.md) |  | 

## Example

```python
from algokit_algod_api.models.get_transaction_group_ledger_state_deltas_for_round200_response import GetTransactionGroupLedgerStateDeltasForRound200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetTransactionGroupLedgerStateDeltasForRound200Response from a JSON string
get_transaction_group_ledger_state_deltas_for_round200_response_instance = GetTransactionGroupLedgerStateDeltasForRound200Response.from_json(json)
# print the JSON string representation of the object
print(GetTransactionGroupLedgerStateDeltasForRound200Response.to_json())

# convert the object into a dict
get_transaction_group_ledger_state_deltas_for_round200_response_dict = get_transaction_group_ledger_state_deltas_for_round200_response_instance.to_dict()
# create an instance of GetTransactionGroupLedgerStateDeltasForRound200Response from a dict
get_transaction_group_ledger_state_deltas_for_round200_response_from_dict = GetTransactionGroupLedgerStateDeltasForRound200Response.from_dict(get_transaction_group_ledger_state_deltas_for_round200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


