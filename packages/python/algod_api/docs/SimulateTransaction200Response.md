# SimulateTransaction200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **int** | The version of this response object. | 
**last_round** | **int** | The round immediately preceding this simulation. State changes through this round were used to run this simulation. | 
**txn_groups** | [**List[SimulateTransactionGroupResult]**](SimulateTransactionGroupResult.md) | A result object for each transaction group that was simulated. | 
**eval_overrides** | [**SimulationEvalOverrides**](SimulationEvalOverrides.md) |  | [optional] 
**exec_trace_config** | [**SimulateTraceConfig**](SimulateTraceConfig.md) |  | [optional] 
**initial_states** | [**SimulateInitialStates**](SimulateInitialStates.md) |  | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_transaction200_response import SimulateTransaction200Response

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateTransaction200Response from a JSON string
simulate_transaction200_response_instance = SimulateTransaction200Response.from_json(json)
# print the JSON string representation of the object
print(SimulateTransaction200Response.to_json())

# convert the object into a dict
simulate_transaction200_response_dict = simulate_transaction200_response_instance.to_dict()
# create an instance of SimulateTransaction200Response from a dict
simulate_transaction200_response_from_dict = SimulateTransaction200Response.from_dict(simulate_transaction200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


