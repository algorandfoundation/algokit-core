# AccountParticipation

AccountParticipation describes the parameters used by this account in consensus protocol.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**selection_participation_key** | **bytearray** | \\[sel\\] Selection public key (if any) currently registered for this round. | 
**vote_first_valid** | **int** | \\[voteFst\\] First round for which this participation is valid. | 
**vote_key_dilution** | **int** | \\[voteKD\\] Number of subkeys in each batch of participation keys. | 
**vote_last_valid** | **int** | \\[voteLst\\] Last round for which this participation is valid. | 
**vote_participation_key** | **bytearray** | \\[vote\\] root participation public key (if any) currently registered for this round. | 
**state_proof_key** | **bytearray** | \\[stprf\\] Root of the state proof key (if any) | [optional] 

## Example

```python
from algokit_algod_api.models.account_participation import AccountParticipation

# TODO update the JSON string below
json = "{}"
# create an instance of AccountParticipation from a JSON string
account_participation_instance = AccountParticipation.from_json(json)
# print the JSON string representation of the object
print(AccountParticipation.to_json())

# convert the object into a dict
account_participation_dict = account_participation_instance.to_dict()
# create an instance of AccountParticipation from a dict
account_participation_from_dict = AccountParticipation.from_dict(account_participation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


