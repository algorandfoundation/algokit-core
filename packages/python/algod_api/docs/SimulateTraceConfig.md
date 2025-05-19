# SimulateTraceConfig

An object that configures simulation execution trace.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable** | **bool** | A boolean option for opting in execution trace features simulation endpoint. | [optional] 
**stack_change** | **bool** | A boolean option enabling returning stack changes together with execution trace during simulation. | [optional] 
**scratch_change** | **bool** | A boolean option enabling returning scratch slot changes together with execution trace during simulation. | [optional] 
**state_change** | **bool** | A boolean option enabling returning application state changes (global, local, and box changes) with the execution trace during simulation. | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_trace_config import SimulateTraceConfig

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateTraceConfig from a JSON string
simulate_trace_config_instance = SimulateTraceConfig.from_json(json)
# print the JSON string representation of the object
print(SimulateTraceConfig.to_json())

# convert the object into a dict
simulate_trace_config_dict = simulate_trace_config_instance.to_dict()
# create an instance of SimulateTraceConfig from a dict
simulate_trace_config_from_dict = SimulateTraceConfig.from_dict(simulate_trace_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


