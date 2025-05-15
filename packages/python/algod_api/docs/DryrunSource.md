# DryrunSource

DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field_name** | **str** | FieldName is what kind of sources this is. If lsig then it goes into the transactions[this.TxnIndex].LogicSig. If approv or clearp it goes into the Approval Program or Clear State Program of application[this.AppIndex]. | 
**source** | **str** |  | 
**txn_index** | **int** |  | 
**app_index** | **int** |  | 

## Example

```python
from algokit_algod_api.models.dryrun_source import DryrunSource

# TODO update the JSON string below
json = "{}"
# create an instance of DryrunSource from a JSON string
dryrun_source_instance = DryrunSource.from_json(json)
# print the JSON string representation of the object
print(DryrunSource.to_json())

# convert the object into a dict
dryrun_source_dict = dryrun_source_instance.to_dict()
# create an instance of DryrunSource from a dict
dryrun_source_from_dict = DryrunSource.from_dict(dryrun_source_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


