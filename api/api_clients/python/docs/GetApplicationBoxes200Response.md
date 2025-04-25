# GetApplicationBoxes200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**round** | **int** | The round for which this information is relevant. | 
**next_token** | **str** | Used for pagination, when making another request provide this token with the next parameter. | [optional] 
**boxes** | [**List[Box]**](Box.md) |  | 

## Example

```python
from algorand_algod_client.models.get_application_boxes200_response import GetApplicationBoxes200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetApplicationBoxes200Response from a JSON string
get_application_boxes200_response_instance = GetApplicationBoxes200Response.from_json(json)
# print the JSON string representation of the object
print(GetApplicationBoxes200Response.to_json())

# convert the object into a dict
get_application_boxes200_response_dict = get_application_boxes200_response_instance.to_dict()
# create an instance of GetApplicationBoxes200Response from a dict
get_application_boxes200_response_from_dict = GetApplicationBoxes200Response.from_dict(get_application_boxes200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


