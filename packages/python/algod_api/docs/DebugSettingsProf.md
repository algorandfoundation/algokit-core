# DebugSettingsProf

algod mutex and blocking profiling state.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_rate** | **int** | The rate of blocking events. The profiler aims to sample an average of one blocking event per rate nanoseconds spent blocked. To turn off profiling entirely, pass rate 0. | [optional] 
**mutex_rate** | **int** | The rate of mutex events. On average 1/rate events are reported. To turn off profiling entirely, pass rate 0 | [optional] 

## Example

```python
from algokit_algod_api.models.debug_settings_prof import DebugSettingsProf

# TODO update the JSON string below
json = "{}"
# create an instance of DebugSettingsProf from a JSON string
debug_settings_prof_instance = DebugSettingsProf.from_json(json)
# print the JSON string representation of the object
print(DebugSettingsProf.to_json())

# convert the object into a dict
debug_settings_prof_dict = debug_settings_prof_instance.to_dict()
# create an instance of DebugSettingsProf from a dict
debug_settings_prof_from_dict = DebugSettingsProf.from_dict(debug_settings_prof_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


