# \ExperimentalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_assets_information**](ExperimentalApi.md#account_assets_information) | **GET** /v2/accounts/{address}/assets | Get a list of assets held by an account, inclusive of asset params.
[**experimental_check**](ExperimentalApi.md#experimental_check) | **GET** /v2/experimental | Returns OK if experimental API is enabled.
[**raw_transaction_async**](ExperimentalApi.md#raw_transaction_async) | **POST** /v2/transactions/async | Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.



## account_assets_information

> models::AccountAssetsInformation200Response account_assets_information(address, limit, next)
Get a list of assets held by an account, inclusive of asset params.

Lookup an account's asset holdings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**limit** | Option<**i32**> | Maximum number of results to return. |  |
**next** | Option<**String**> | The next page of results. Use the next token provided by the previous results. |  |

### Return type

[**models::AccountAssetsInformation200Response**](AccountAssetsInformation_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_check

> experimental_check()
Returns OK if experimental API is enabled.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## raw_transaction_async

> raw_transaction_async(rawtxn)
Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rawtxn** | **std::path::PathBuf** | The byte encoded signed transaction to broadcast to network | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/x-binary
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

