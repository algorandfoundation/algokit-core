# AccountStateDelta

Application state delta.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **str** |  | 
**delta** | [**List[EvalDeltaKeyValue]**](EvalDeltaKeyValue.md) | Application state delta. | 

## Example

```python
from algokit_algod_api.models.account_state_delta import AccountStateDelta

# TODO update the JSON string below
json = "{}"
# create an instance of AccountStateDelta from a JSON string
account_state_delta_instance = AccountStateDelta.from_json(json)
# print the JSON string representation of the object
print(AccountStateDelta.to_json())

# convert the object into a dict
account_state_delta_dict = account_state_delta_instance.to_dict()
# create an instance of AccountStateDelta from a dict
account_state_delta_from_dict = AccountStateDelta.from_dict(account_state_delta_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


