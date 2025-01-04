# SimulateUnnamedResourcesAccessed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | Option<**Vec<String>**> | The unnamed accounts that were referenced. The order of this array is arbitrary. | [optional]
**app_locals** | Option<[**Vec<models::ApplicationLocalReference>**](ApplicationLocalReference.md)> | The unnamed application local states that were referenced. The order of this array is arbitrary. | [optional]
**apps** | Option<**Vec<i32>**> | The unnamed applications that were referenced. The order of this array is arbitrary. | [optional]
**asset_holdings** | Option<[**Vec<models::AssetHoldingReference>**](AssetHoldingReference.md)> | The unnamed asset holdings that were referenced. The order of this array is arbitrary. | [optional]
**assets** | Option<**Vec<i32>**> | The unnamed assets that were referenced. The order of this array is arbitrary. | [optional]
**boxes** | Option<[**Vec<models::BoxReference>**](BoxReference.md)> | The unnamed boxes that were referenced. The order of this array is arbitrary. | [optional]
**extra_box_refs** | Option<**i32**> | The number of extra box references used to increase the IO budget. This is in addition to the references defined in the input transaction group and any referenced to unnamed boxes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


