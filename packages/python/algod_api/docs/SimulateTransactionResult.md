# SimulateTransactionResult

Simulation result for an individual transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txn_result** | [**PendingTransactionResponse**](PendingTransactionResponse.md) |  | 
**app_budget_consumed** | **int** | Budget used during execution of an app call transaction. This value includes budged used by inner app calls spawned by this transaction. | [optional] 
**logic_sig_budget_consumed** | **int** | Budget used during execution of a logic sig transaction. | [optional] 
**exec_trace** | [**SimulationTransactionExecTrace**](SimulationTransactionExecTrace.md) |  | [optional] 
**unnamed_resources_accessed** | [**SimulateUnnamedResourcesAccessed**](SimulateUnnamedResourcesAccessed.md) |  | [optional] 
**fixed_signer** | **str** | The account that needed to sign this transaction when no signature was provided and the provided signer was incorrect. | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_transaction_result import SimulateTransactionResult

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateTransactionResult from a JSON string
simulate_transaction_result_instance = SimulateTransactionResult.from_json(json)
# print the JSON string representation of the object
print(SimulateTransactionResult.to_json())

# convert the object into a dict
simulate_transaction_result_dict = simulate_transaction_result_instance.to_dict()
# create an instance of SimulateTransactionResult from a dict
simulate_transaction_result_from_dict = SimulateTransactionResult.from_dict(simulate_transaction_result_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


