# AppCallLogs

The logged messages from an app call along with the app ID and outer transaction ID. Logs appear in the same order that they were emitted.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**logs** | **List[bytearray]** | An array of logs | 
**application_index** | **int** | The application from which the logs were generated | 
**tx_id** | **str** | The transaction ID of the outer app call that lead to these logs | 

## Example

```python
from algokit_algod_api.models.app_call_logs import AppCallLogs

# TODO update the JSON string below
json = "{}"
# create an instance of AppCallLogs from a JSON string
app_call_logs_instance = AppCallLogs.from_json(json)
# print the JSON string representation of the object
print(AppCallLogs.to_json())

# convert the object into a dict
app_call_logs_dict = app_call_logs_instance.to_dict()
# create an instance of AppCallLogs from a dict
app_call_logs_from_dict = AppCallLogs.from_dict(app_call_logs_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


