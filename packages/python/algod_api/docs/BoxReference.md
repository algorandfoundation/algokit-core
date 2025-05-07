# BoxReference

References a box of an application.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app** | **int** | Application ID which this box belongs to | 
**name** | **bytearray** | Base64 encoded box name | 

## Example

```python
from algokit_algod_api.models.box_reference import BoxReference

# TODO update the JSON string below
json = "{}"
# create an instance of BoxReference from a JSON string
box_reference_instance = BoxReference.from_json(json)
# print the JSON string representation of the object
print(BoxReference.to_json())

# convert the object into a dict
box_reference_dict = box_reference_instance.to_dict()
# create an instance of BoxReference from a dict
box_reference_from_dict = BoxReference.from_dict(box_reference_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


