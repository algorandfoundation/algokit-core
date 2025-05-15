# SimulationEvalOverrides

The set of parameters and limits override during simulation. If this set of parameters is present, then evaluation parameters may differ from standard evaluation in certain ways.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_empty_signatures** | **bool** | If true, transactions without signatures are allowed and simulated as if they were properly signed. | [optional] 
**allow_unnamed_resources** | **bool** | If true, allows access to unnamed resources during simulation. | [optional] 
**max_log_calls** | **int** | The maximum log calls one can make during simulation | [optional] 
**max_log_size** | **int** | The maximum byte number to log during simulation | [optional] 
**extra_opcode_budget** | **int** | The extra opcode budget added to each transaction group during simulation | [optional] 
**fix_signers** | **bool** | If true, signers for transactions that are missing signatures will be fixed during evaluation. | [optional] 

## Example

```python
from algokit_algod_api.models.simulation_eval_overrides import SimulationEvalOverrides

# TODO update the JSON string below
json = "{}"
# create an instance of SimulationEvalOverrides from a JSON string
simulation_eval_overrides_instance = SimulationEvalOverrides.from_json(json)
# print the JSON string representation of the object
print(SimulationEvalOverrides.to_json())

# convert the object into a dict
simulation_eval_overrides_dict = simulation_eval_overrides_instance.to_dict()
# create an instance of SimulationEvalOverrides from a dict
simulation_eval_overrides_from_dict = SimulationEvalOverrides.from_dict(simulation_eval_overrides_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


