# algorand_algod_client.NonparticipatingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transaction_params**](NonparticipatingApi.md#transaction_params) | **GET** /v2/transactions/params | Get parameters for constructing a new transaction


# **transaction_params**
> TransactionParams200Response transaction_params()

Get parameters for constructing a new transaction

### Example

* Api Key Authentication (api_key):

```python
import algorand_algod_client
from algorand_algod_client.models.transaction_params200_response import TransactionParams200Response
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
    api_instance = algorand_algod_client.NonparticipatingApi(api_client)

    try:
        # Get parameters for constructing a new transaction
        api_response = api_instance.transaction_params()
        print("The response of NonparticipatingApi->transaction_params:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling NonparticipatingApi->transaction_params: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**TransactionParams200Response**](TransactionParams200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | TransactionParams contains the parameters that help a client construct a new transaction. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

