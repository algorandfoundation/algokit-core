# AssetParams

AssetParams specifies the parameters for an asset.  \\[apar\\] when part of an AssetConfig transaction.  Definition: data/transactions/asset.go : AssetParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clawback** | **str** | \\[c\\] Address of account used to clawback holdings of this asset.  If empty, clawback is not permitted. | [optional] 
**creator** | **str** | The address that created this asset. This is the address where the parameters for this asset can be found, and also the address where unwanted asset units can be sent in the worst case. | 
**decimals** | **int** | \\[dc\\] The number of digits to use after the decimal point when displaying this asset. If 0, the asset is not divisible. If 1, the base unit of the asset is in tenths. If 2, the base unit of the asset is in hundredths, and so on. This value must be between 0 and 19 (inclusive). | 
**default_frozen** | **bool** | \\[df\\] Whether holdings of this asset are frozen by default. | [optional] 
**freeze** | **str** | \\[f\\] Address of account used to freeze holdings of this asset.  If empty, freezing is not permitted. | [optional] 
**manager** | **str** | \\[m\\] Address of account used to manage the keys of this asset and to destroy it. | [optional] 
**metadata_hash** | **bytearray** | \\[am\\] A commitment to some unspecified asset metadata. The format of this metadata is up to the application. | [optional] 
**name** | **str** | \\[an\\] Name of this asset, as supplied by the creator. Included only when the asset name is composed of printable utf-8 characters. | [optional] 
**name_b64** | **bytearray** | Base64 encoded name of this asset, as supplied by the creator. | [optional] 
**reserve** | **str** | \\[r\\] Address of account holding reserve (non-minted) units of this asset. | [optional] 
**total** | **int** | \\[t\\] The total number of units of this asset. | 
**unit_name** | **str** | \\[un\\] Name of a unit of this asset, as supplied by the creator. Included only when the name of a unit of this asset is composed of printable utf-8 characters. | [optional] 
**unit_name_b64** | **bytearray** | Base64 encoded name of a unit of this asset, as supplied by the creator. | [optional] 
**url** | **str** | \\[au\\] URL where more information about the asset can be retrieved. Included only when the URL is composed of printable utf-8 characters. | [optional] 
**url_b64** | **bytearray** | Base64 encoded URL where more information about the asset can be retrieved. | [optional] 

## Example

```python
from algokit_algod_api.models.asset_params import AssetParams

# TODO update the JSON string below
json = "{}"
# create an instance of AssetParams from a JSON string
asset_params_instance = AssetParams.from_json(json)
# print the JSON string representation of the object
print(AssetParams.to_json())

# convert the object into a dict
asset_params_dict = asset_params_instance.to_dict()
# create an instance of AssetParams from a dict
asset_params_from_dict = AssetParams.from_dict(asset_params_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


