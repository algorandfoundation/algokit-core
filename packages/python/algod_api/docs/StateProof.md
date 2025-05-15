# StateProof

Represents a state proof and its corresponding message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | [**StateProofMessage**](StateProofMessage.md) |  | 
**state_proof** | **bytearray** | The encoded StateProof for the message. | 

## Example

```python
from algokit_algod_api.models.state_proof import StateProof

# TODO update the JSON string below
json = "{}"
# create an instance of StateProof from a JSON string
state_proof_instance = StateProof.from_json(json)
# print the JSON string representation of the object
print(StateProof.to_json())

# convert the object into a dict
state_proof_dict = state_proof_instance.to_dict()
# create an instance of StateProof from a dict
state_proof_from_dict = StateProof.from_dict(state_proof_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


