# ParticipationKey

Represents a participation key used by the node.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** | The key&#39;s ParticipationID. | 
**address** | **str** | Address the key was generated for. | 
**effective_first_valid** | **int** | When registered, this is the first round it may be used. | [optional] 
**effective_last_valid** | **int** | When registered, this is the last round it may be used. | [optional] 
**last_vote** | **int** | Round when this key was last used to vote. | [optional] 
**last_block_proposal** | **int** | Round when this key was last used to propose a block. | [optional] 
**last_state_proof** | **int** | Round when this key was last used to generate a state proof. | [optional] 
**key** | [**AccountParticipation**](AccountParticipation.md) |  | 

## Example

```python
from algokit_algod_api.models.participation_key import ParticipationKey

# TODO update the JSON string below
json = "{}"
# create an instance of ParticipationKey from a JSON string
participation_key_instance = ParticipationKey.from_json(json)
# print the JSON string representation of the object
print(ParticipationKey.to_json())

# convert the object into a dict
participation_key_dict = participation_key_instance.to_dict()
# create an instance of ParticipationKey from a dict
participation_key_from_dict = ParticipationKey.from_dict(participation_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


