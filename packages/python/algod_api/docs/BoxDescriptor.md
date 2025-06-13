# BoxDescriptor

Box descriptor describes a Box.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **bytearray** | Base64 encoded box name | 

## Example

```python
from algokit_algod_api.models.box_descriptor import BoxDescriptor

# TODO update the JSON string below
json = "{}"
# create an instance of BoxDescriptor from a JSON string
box_descriptor_instance = BoxDescriptor.from_json(json)
# print the JSON string representation of the object
print(BoxDescriptor.to_json())

# convert the object into a dict
box_descriptor_dict = box_descriptor_instance.to_dict()
# create an instance of BoxDescriptor from a dict
box_descriptor_from_dict = BoxDescriptor.from_dict(box_descriptor_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


