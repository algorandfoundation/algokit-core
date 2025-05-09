# Version

algod version information.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build** | [**BuildVersion**](BuildVersion.md) |  | 
**genesis_hash_b64** | **bytearray** |  | 
**genesis_id** | **str** |  | 
**versions** | **List[str]** |  | 

## Example

```python
from algokit_algod_api.models.version import Version

# TODO update the JSON string below
json = "{}"
# create an instance of Version from a JSON string
version_instance = Version.from_json(json)
# print the JSON string representation of the object
print(Version.to_json())

# convert the object into a dict
version_dict = version_instance.to_dict()
# create an instance of Version from a dict
version_from_dict = Version.from_dict(version_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


