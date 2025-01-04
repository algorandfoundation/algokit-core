# \DataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sync_round**](DataApi.md#get_sync_round) | **GET** /v2/ledger/sync | Returns the minimum sync round the ledger is keeping in cache.
[**set_sync_round**](DataApi.md#set_sync_round) | **POST** /v2/ledger/sync/{round} | Given a round, tells the ledger to keep that round in its cache.
[**unset_sync_round**](DataApi.md#unset_sync_round) | **DELETE** /v2/ledger/sync | Removes minimum sync round restriction from the ledger.



## get_sync_round

> models::GetSyncRound200Response get_sync_round()
Returns the minimum sync round the ledger is keeping in cache.

Gets the minimum sync round for the ledger.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSyncRound200Response**](GetSyncRound_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_sync_round

> set_sync_round(round)
Given a round, tells the ledger to keep that round in its cache.

Sets the minimum sync round on the ledger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round for which the deltas are desired. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unset_sync_round

> unset_sync_round()
Removes minimum sync round restriction from the ledger.

Unset the ledger sync round.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

