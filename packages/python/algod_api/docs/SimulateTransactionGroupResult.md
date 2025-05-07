# SimulateTransactionGroupResult

Simulation result for an atomic transaction group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txn_results** | [**List[SimulateTransactionResult]**](SimulateTransactionResult.md) | Simulation result for individual transactions | 
**failure_message** | **str** | If present, indicates that the transaction group failed and specifies why that happened | [optional] 
**failed_at** | **List[int]** | If present, indicates which transaction in this group caused the failure. This array represents the path to the failing transaction. Indexes are zero based, the first element indicates the top-level transaction, and successive elements indicate deeper inner transactions. | [optional] 
**app_budget_added** | **int** | Total budget added during execution of app calls in the transaction group. | [optional] 
**app_budget_consumed** | **int** | Total budget consumed during execution of app calls in the transaction group. | [optional] 
**unnamed_resources_accessed** | [**SimulateUnnamedResourcesAccessed**](SimulateUnnamedResourcesAccessed.md) |  | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_transaction_group_result import SimulateTransactionGroupResult

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateTransactionGroupResult from a JSON string
simulate_transaction_group_result_instance = SimulateTransactionGroupResult.from_json(json)
# print the JSON string representation of the object
print(SimulateTransactionGroupResult.to_json())

# convert the object into a dict
simulate_transaction_group_result_dict = simulate_transaction_group_result_instance.to_dict()
# create an instance of SimulateTransactionGroupResult from a dict
simulate_transaction_group_result_from_dict = SimulateTransactionGroupResult.from_dict(simulate_transaction_group_result_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


