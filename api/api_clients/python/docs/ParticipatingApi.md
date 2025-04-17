# algorand_algod_client.ParticipatingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**raw_transaction**](ParticipatingApi.md#raw_transaction) | **POST** /v2/transactions | Broadcasts a raw transaction or transaction group to the network.


# **raw_transaction**
> RawTransaction200Response raw_transaction(rawtxn)

Broadcasts a raw transaction or transaction group to the network.

### Example

* Api Key Authentication (api_key):

```python
import algorand_algod_client
from algorand_algod_client.models.raw_transaction200_response import RawTransaction200Response
from algorand_algod_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algorand_algod_client.Configuration(
    host = "http://localhost"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with algorand_algod_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algorand_algod_client.ParticipatingApi(api_client)
    rawtxn = None # bytearray | The byte encoded signed transaction to broadcast to network

    try:
        # Broadcasts a raw transaction or transaction group to the network.
        api_response = api_instance.raw_transaction(rawtxn)
        print("The response of ParticipatingApi->raw_transaction:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ParticipatingApi->raw_transaction: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rawtxn** | **bytearray**| The byte encoded signed transaction to broadcast to network | 

### Return type

[**RawTransaction200Response**](RawTransaction200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/x-binary
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Transaction ID of the submission. |  -  |
**400** | Bad Request - Malformed Algorand transaction  |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

