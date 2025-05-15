# Account

Account information at a given round.  Definition: data/basics/userBalance.go : AccountData 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **str** | the account public key | 
**amount** | **int** | \\[algo\\] total number of MicroAlgos in the account | 
**min_balance** | **int** | MicroAlgo balance required by the account.  The requirement grows based on asset and application usage. | 
**amount_without_pending_rewards** | **int** | specifies the amount of MicroAlgos in the account, without the pending rewards. | 
**apps_local_state** | [**List[ApplicationLocalState]**](ApplicationLocalState.md) | \\[appl\\] applications local data stored in this account.  Note the raw object uses &#x60;map[int] -&gt; AppLocalState&#x60; for this type. | [optional] 
**total_apps_opted_in** | **int** | The count of all applications that have been opted in, equivalent to the count of application local data (AppLocalState objects) stored in this account. | 
**apps_total_schema** | [**ApplicationStateSchema**](ApplicationStateSchema.md) |  | [optional] 
**apps_total_extra_pages** | **int** | \\[teap\\] the sum of all extra application program pages for this account. | [optional] 
**assets** | [**List[AssetHolding]**](AssetHolding.md) | \\[asset\\] assets held by this account.  Note the raw object uses &#x60;map[int] -&gt; AssetHolding&#x60; for this type. | [optional] 
**total_assets_opted_in** | **int** | The count of all assets that have been opted in, equivalent to the count of AssetHolding objects held by this account. | 
**created_apps** | [**List[Application]**](Application.md) | \\[appp\\] parameters of applications created by this account including app global data.  Note: the raw account uses &#x60;map[int] -&gt; AppParams&#x60; for this type. | [optional] 
**total_created_apps** | **int** | The count of all apps (AppParams objects) created by this account. | 
**created_assets** | [**List[Asset]**](Asset.md) | \\[apar\\] parameters of assets created by this account.  Note: the raw account uses &#x60;map[int] -&gt; Asset&#x60; for this type. | [optional] 
**total_created_assets** | **int** | The count of all assets (AssetParams objects) created by this account. | 
**total_boxes** | **int** | \\[tbx\\] The number of existing boxes created by this account&#39;s app. | [optional] 
**total_box_bytes** | **int** | \\[tbxb\\] The total number of bytes used by this account&#39;s app&#39;s box keys and values. | [optional] 
**participation** | [**AccountParticipation**](AccountParticipation.md) |  | [optional] 
**incentive_eligible** | **bool** | Whether or not the account can receive block incentives if its balance is in range at proposal time. | [optional] 
**pending_rewards** | **int** | amount of MicroAlgos of pending rewards in this account. | 
**reward_base** | **int** | \\[ebase\\] used as part of the rewards computation. Only applicable to accounts which are participating. | [optional] 
**rewards** | **int** | \\[ern\\] total rewards of MicroAlgos the account has received, including pending rewards. | 
**round** | **int** | The round for which this information is relevant. | 
**status** | **str** | \\[onl\\] delegation status of the account&#39;s MicroAlgos * Offline - indicates that the associated account is delegated. *  Online  - indicates that the associated account used as part of the delegation pool. *   NotParticipating - indicates that the associated account is neither a delegator nor a delegate. | 
**sig_type** | **str** | Indicates what type of signature is used by this account, must be one of: * sig * msig * lsig | [optional] 
**auth_addr** | **str** | \\[spend\\] the address against which signing should be checked. If empty, the address of the current account is used. This field can be updated in any transaction by setting the RekeyTo field. | [optional] 
**last_proposed** | **int** | The round in which this account last proposed the block. | [optional] 
**last_heartbeat** | **int** | The round in which this account last went online, or explicitly renewed their online status. | [optional] 

## Example

```python
from algokit_algod_api.models.account import Account

# TODO update the JSON string below
json = "{}"
# create an instance of Account from a JSON string
account_instance = Account.from_json(json)
# print the JSON string representation of the object
print(Account.to_json())

# convert the object into a dict
account_dict = account_instance.to_dict()
# create an instance of Account from a dict
account_from_dict = Account.from_dict(account_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


