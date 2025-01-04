# \PrivateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_catchup**](PrivateApi.md#abort_catchup) | **DELETE** /v2/catchup/{catchpoint} | Aborts a catchpoint catchup.
[**add_participation_key**](PrivateApi.md#add_participation_key) | **POST** /v2/participation | Add a participation key to the node
[**append_keys**](PrivateApi.md#append_keys) | **POST** /v2/participation/{participation-id} | Append state proof keys to a participation key
[**delete_participation_key_by_id**](PrivateApi.md#delete_participation_key_by_id) | **DELETE** /v2/participation/{participation-id} | Delete a given participation key by ID
[**generate_participation_keys**](PrivateApi.md#generate_participation_keys) | **POST** /v2/participation/generate/{address} | Generate and install participation keys to the node.
[**get_config**](PrivateApi.md#get_config) | **GET** /debug/settings/config | Gets the merged config file.
[**get_debug_settings_prof**](PrivateApi.md#get_debug_settings_prof) | **GET** /debug/settings/pprof | 
[**get_participation_key_by_id**](PrivateApi.md#get_participation_key_by_id) | **GET** /v2/participation/{participation-id} | Get participation key info given a participation ID
[**get_participation_keys**](PrivateApi.md#get_participation_keys) | **GET** /v2/participation | Return a list of participation keys
[**put_debug_settings_prof**](PrivateApi.md#put_debug_settings_prof) | **PUT** /debug/settings/pprof | 
[**shutdown_node**](PrivateApi.md#shutdown_node) | **POST** /v2/shutdown | 
[**start_catchup**](PrivateApi.md#start_catchup) | **POST** /v2/catchup/{catchpoint} | Starts a catchpoint catchup.



## abort_catchup

> models::AbortCatchup200Response abort_catchup(catchpoint)
Aborts a catchpoint catchup.

Given a catchpoint, it aborts catching up to this catchpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catchpoint** | **String** | A catch point | [required] |

### Return type

[**models::AbortCatchup200Response**](AbortCatchup_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_participation_key

> models::AddParticipationKey200Response add_participation_key(participationkey)
Add a participation key to the node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participationkey** | **std::path::PathBuf** | The participation key to add to the node | [required] |

### Return type

[**models::AddParticipationKey200Response**](AddParticipationKey_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/msgpack
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## append_keys

> models::ParticipationKey append_keys(participation_id, keymap)
Append state proof keys to a participation key

Given a participation ID, append state proof keys to a particular set of participation keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participation_id** | **String** |  | [required] |
**keymap** | **std::path::PathBuf** | The state proof keys to add to an existing participation ID | [required] |

### Return type

[**models::ParticipationKey**](ParticipationKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/msgpack
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_participation_key_by_id

> delete_participation_key_by_id(participation_id)
Delete a given participation key by ID

Delete a given participation key by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participation_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_participation_keys

> String generate_participation_keys(address, first, last, dilution)
Generate and install participation keys to the node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**first** | **i32** | First round for participation key. | [required] |
**last** | **i32** | Last round for participation key. | [required] |
**dilution** | Option<**i32**> | Key dilution for two-level participation keys (defaults to sqrt of validity window). |  |

### Return type

**String**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config

> String get_config()
Gets the merged config file.

Returns the merged (defaults + overrides) config file in json.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_debug_settings_prof

> models::DebugSettingsProf get_debug_settings_prof()


Retrieves the current settings for blocking and mutex profiles

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DebugSettingsProf**](DebugSettingsProf.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_participation_key_by_id

> models::ParticipationKey get_participation_key_by_id(participation_id)
Get participation key info given a participation ID

Given a participation ID, return information about that participation key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participation_id** | **String** |  | [required] |

### Return type

[**models::ParticipationKey**](ParticipationKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_participation_keys

> Vec<models::ParticipationKey> get_participation_keys()
Return a list of participation keys

Return a list of participation keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ParticipationKey>**](ParticipationKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_debug_settings_prof

> models::DebugSettingsProf put_debug_settings_prof()


Enables blocking and mutex profiles, and returns the old settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DebugSettingsProf**](DebugSettingsProf.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shutdown_node

> serde_json::Value shutdown_node(timeout)


Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timeout** | Option<**i32**> |  |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_catchup

> models::StartCatchup200Response start_catchup(catchpoint, min)
Starts a catchpoint catchup.

Given a catchpoint, it starts catching up to this catchpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catchpoint** | **String** | A catch point | [required] |
**min** | Option<**i32**> | Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed. |  |

### Return type

[**models::StartCatchup200Response**](StartCatchup_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

