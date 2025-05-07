# LightBlockHeaderProof

Proof of membership and position of a light block header.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | **int** | The index of the light block header in the vector commitment tree | 
**treedepth** | **int** | Represents the depth of the tree that is being proven, i.e. the number of edges from a leaf to the root. | 
**proof** | **bytearray** | The encoded proof. | 

## Example

```python
from algokit_algod_api.models.light_block_header_proof import LightBlockHeaderProof

# TODO update the JSON string below
json = "{}"
# create an instance of LightBlockHeaderProof from a JSON string
light_block_header_proof_instance = LightBlockHeaderProof.from_json(json)
# print the JSON string representation of the object
print(LightBlockHeaderProof.to_json())

# convert the object into a dict
light_block_header_proof_dict = light_block_header_proof_instance.to_dict()
# create an instance of LightBlockHeaderProof from a dict
light_block_header_proof_from_dict = LightBlockHeaderProof.from_dict(light_block_header_proof_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


