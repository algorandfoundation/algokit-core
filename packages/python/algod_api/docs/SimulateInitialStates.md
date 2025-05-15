# SimulateInitialStates

Initial states of resources that were accessed during simulation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_initial_states** | [**List[ApplicationInitialStates]**](ApplicationInitialStates.md) | The initial states of accessed application before simulation. The order of this array is arbitrary. | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_initial_states import SimulateInitialStates

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateInitialStates from a JSON string
simulate_initial_states_instance = SimulateInitialStates.from_json(json)
# print the JSON string representation of the object
print(SimulateInitialStates.to_json())

# convert the object into a dict
simulate_initial_states_dict = simulate_initial_states_instance.to_dict()
# create an instance of SimulateInitialStates from a dict
simulate_initial_states_from_dict = SimulateInitialStates.from_dict(simulate_initial_states_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


