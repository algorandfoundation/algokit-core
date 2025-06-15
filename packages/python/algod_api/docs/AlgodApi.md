# algokit_algod_api.AlgodApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_catchup**](AlgodApi.md#abort_catchup) | **DELETE** /v2/catchup/{catchpoint} | Aborts a catchpoint catchup.
[**account_application_information**](AlgodApi.md#account_application_information) | **GET** /v2/accounts/{address}/applications/{application-id} | Get account information about a given app.
[**account_asset_information**](AlgodApi.md#account_asset_information) | **GET** /v2/accounts/{address}/assets/{asset-id} | Get account information about a given asset.
[**account_assets_information**](AlgodApi.md#account_assets_information) | **GET** /v2/accounts/{address}/assets | Get a list of assets held by an account, inclusive of asset params.
[**account_information**](AlgodApi.md#account_information) | **GET** /v2/accounts/{address} | Get account information.
[**add_participation_key**](AlgodApi.md#add_participation_key) | **POST** /v2/participation | Add a participation key to the node
[**append_keys**](AlgodApi.md#append_keys) | **POST** /v2/participation/{participation-id} | Append state proof keys to a participation key
[**delete_participation_key_by_id**](AlgodApi.md#delete_participation_key_by_id) | **DELETE** /v2/participation/{participation-id} | Delete a given participation key by ID
[**experimental_check**](AlgodApi.md#experimental_check) | **GET** /v2/experimental | Returns OK if experimental API is enabled.
[**generate_participation_keys**](AlgodApi.md#generate_participation_keys) | **POST** /v2/participation/generate/{address} | Generate and install participation keys to the node.
[**get_application_box_by_name**](AlgodApi.md#get_application_box_by_name) | **GET** /v2/applications/{application-id}/box | Get box information for a given application.
[**get_application_boxes**](AlgodApi.md#get_application_boxes) | **GET** /v2/applications/{application-id}/boxes | Get all box names for a given application.
[**get_application_by_id**](AlgodApi.md#get_application_by_id) | **GET** /v2/applications/{application-id} | Get application information.
[**get_asset_by_id**](AlgodApi.md#get_asset_by_id) | **GET** /v2/assets/{asset-id} | Get asset information.
[**get_block**](AlgodApi.md#get_block) | **GET** /v2/blocks/{round} | Get the block for the given round.
[**get_block_hash**](AlgodApi.md#get_block_hash) | **GET** /v2/blocks/{round}/hash | Get the block hash for the block on the given round.
[**get_block_logs**](AlgodApi.md#get_block_logs) | **GET** /v2/blocks/{round}/logs | Get all of the logs from outer and inner app calls in the given round
[**get_block_time_stamp_offset**](AlgodApi.md#get_block_time_stamp_offset) | **GET** /v2/devmode/blocks/offset | Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
[**get_block_txids**](AlgodApi.md#get_block_txids) | **GET** /v2/blocks/{round}/txids | Get the top level transaction IDs for the block on the given round.
[**get_config**](AlgodApi.md#get_config) | **GET** /debug/settings/config | Gets the merged config file.
[**get_debug_settings_prof**](AlgodApi.md#get_debug_settings_prof) | **GET** /debug/settings/pprof | 
[**get_genesis**](AlgodApi.md#get_genesis) | **GET** /genesis | Gets the genesis information.
[**get_ledger_state_delta**](AlgodApi.md#get_ledger_state_delta) | **GET** /v2/deltas/{round} | Get a LedgerStateDelta object for a given round
[**get_ledger_state_delta_for_transaction_group**](AlgodApi.md#get_ledger_state_delta_for_transaction_group) | **GET** /v2/deltas/txn/group/{id} | Get a LedgerStateDelta object for a given transaction group
[**get_light_block_header_proof**](AlgodApi.md#get_light_block_header_proof) | **GET** /v2/blocks/{round}/lightheader/proof | Gets a proof for a given light block header inside a state proof commitment
[**get_participation_key_by_id**](AlgodApi.md#get_participation_key_by_id) | **GET** /v2/participation/{participation-id} | Get participation key info given a participation ID
[**get_participation_keys**](AlgodApi.md#get_participation_keys) | **GET** /v2/participation | Return a list of participation keys
[**get_pending_transactions**](AlgodApi.md#get_pending_transactions) | **GET** /v2/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool.
[**get_pending_transactions_by_address**](AlgodApi.md#get_pending_transactions_by_address) | **GET** /v2/accounts/{address}/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool by address.
[**get_ready**](AlgodApi.md#get_ready) | **GET** /ready | Returns OK if healthy and fully caught up.
[**get_state_proof**](AlgodApi.md#get_state_proof) | **GET** /v2/stateproofs/{round} | Get a state proof that covers a given round
[**get_status**](AlgodApi.md#get_status) | **GET** /v2/status | Gets the current node status.
[**get_supply**](AlgodApi.md#get_supply) | **GET** /v2/ledger/supply | Get the current supply reported by the ledger.
[**get_sync_round**](AlgodApi.md#get_sync_round) | **GET** /v2/ledger/sync | Returns the minimum sync round the ledger is keeping in cache.
[**get_transaction_group_ledger_state_deltas_for_round**](AlgodApi.md#get_transaction_group_ledger_state_deltas_for_round) | **GET** /v2/deltas/{round}/txn/group | Get LedgerStateDelta objects for all transaction groups in a given round
[**get_transaction_proof**](AlgodApi.md#get_transaction_proof) | **GET** /v2/blocks/{round}/transactions/{txid}/proof | Get a proof for a transaction in a block.
[**get_version**](AlgodApi.md#get_version) | **GET** /versions | 
[**health_check**](AlgodApi.md#health_check) | **GET** /health | Returns OK if healthy.
[**metrics**](AlgodApi.md#metrics) | **GET** /metrics | Return metrics about algod functioning.
[**pending_transaction_information**](AlgodApi.md#pending_transaction_information) | **GET** /v2/transactions/pending/{txid} | Get a specific pending transaction.
[**put_debug_settings_prof**](AlgodApi.md#put_debug_settings_prof) | **PUT** /debug/settings/pprof | 
[**raw_transaction**](AlgodApi.md#raw_transaction) | **POST** /v2/transactions | Broadcasts a raw transaction or transaction group to the network.
[**raw_transaction_async**](AlgodApi.md#raw_transaction_async) | **POST** /v2/transactions/async | Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
[**set_block_time_stamp_offset**](AlgodApi.md#set_block_time_stamp_offset) | **POST** /v2/devmode/blocks/offset/{offset} | Given a timestamp offset in seconds, adds the offset to every subsequent block header&#39;s timestamp.
[**set_sync_round**](AlgodApi.md#set_sync_round) | **POST** /v2/ledger/sync/{round} | Given a round, tells the ledger to keep that round in its cache.
[**shutdown_node**](AlgodApi.md#shutdown_node) | **POST** /v2/shutdown | 
[**simulate_transaction**](AlgodApi.md#simulate_transaction) | **POST** /v2/transactions/simulate | Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
[**start_catchup**](AlgodApi.md#start_catchup) | **POST** /v2/catchup/{catchpoint} | Starts a catchpoint catchup.
[**swagger_json**](AlgodApi.md#swagger_json) | **GET** /swagger.json | Gets the current swagger spec.
[**teal_compile**](AlgodApi.md#teal_compile) | **POST** /v2/teal/compile | Compile TEAL source code to binary, produce its hash
[**teal_disassemble**](AlgodApi.md#teal_disassemble) | **POST** /v2/teal/disassemble | Disassemble program bytes into the TEAL source code.
[**teal_dryrun**](AlgodApi.md#teal_dryrun) | **POST** /v2/teal/dryrun | Provide debugging information for a transaction (or group).
[**transaction_params**](AlgodApi.md#transaction_params) | **GET** /v2/transactions/params | Get parameters for constructing a new transaction
[**unset_sync_round**](AlgodApi.md#unset_sync_round) | **DELETE** /v2/ledger/sync | Removes minimum sync round restriction from the ledger.
[**wait_for_block**](AlgodApi.md#wait_for_block) | **GET** /v2/status/wait-for-block-after/{round} | Gets the node status after waiting for a round after the given round.


# **abort_catchup**
> AbortCatchup200Response abort_catchup(catchpoint)

Aborts a catchpoint catchup.

Given a catchpoint, it aborts catching up to this catchpoint

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.abort_catchup200_response import AbortCatchup200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    catchpoint = 'catchpoint_example' # str | A catch point

    try:
        # Aborts a catchpoint catchup.
        api_response = api_instance.abort_catchup(catchpoint)
        print("The response of AlgodApi->abort_catchup:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->abort_catchup: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **catchpoint** | **str**| A catch point | 

### Return type

[**AbortCatchup200Response**](AbortCatchup200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Catchpoint operation aborted successfully |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_application_information**
> AccountApplicationInformation200Response account_application_information(address, application_id, format=format)

Get account information about a given app.

Given a specific account public key and application ID, this call returns the account's application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application's creator.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.account_application_information200_response import AccountApplicationInformation200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    address = 'address_example' # str | An account public key
    application_id = 56 # int | An application identifier
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get account information about a given app.
        api_response = api_instance.account_application_information(address, application_id, format=format)
        print("The response of AlgodApi->account_application_information:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->account_application_information: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **str**| An account public key | 
 **application_id** | **int**| An application identifier | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**AccountApplicationInformation200Response**](AccountApplicationInformation200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | AccountApplicationResponse describes the account&#39;s application local state and global state (AppLocalState and AppParams, if either exists) for a specific application ID. Global state will only be returned if the provided address is the application&#39;s creator. |  -  |
**400** | Malformed address or application ID |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_asset_information**
> AccountAssetInformation200Response account_asset_information(address, asset_id, format=format)

Get account information about a given asset.

Given a specific account public key and asset ID, this call returns the account's asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset's creator.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.account_asset_information200_response import AccountAssetInformation200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    address = 'address_example' # str | An account public key
    asset_id = 56 # int | An asset identifier
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get account information about a given asset.
        api_response = api_instance.account_asset_information(address, asset_id, format=format)
        print("The response of AlgodApi->account_asset_information:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->account_asset_information: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **str**| An account public key | 
 **asset_id** | **int**| An asset identifier | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**AccountAssetInformation200Response**](AccountAssetInformation200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | AccountAssetResponse describes the account&#39;s asset holding and asset parameters (if either exist) for a specific asset ID. Asset parameters will only be returned if the provided address is the asset&#39;s creator. |  -  |
**400** | Malformed address or asset ID |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_assets_information**
> AccountAssetsInformation200Response account_assets_information(address, limit=limit, next=next)

Get a list of assets held by an account, inclusive of asset params.

Lookup an account's asset holdings.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.account_assets_information200_response import AccountAssetsInformation200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    address = 'address_example' # str | An account public key
    limit = 56 # int | Maximum number of results to return. (optional)
    next = 'next_example' # str | The next page of results. Use the next token provided by the previous results. (optional)

    try:
        # Get a list of assets held by an account, inclusive of asset params.
        api_response = api_instance.account_assets_information(address, limit=limit, next=next)
        print("The response of AlgodApi->account_assets_information:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->account_assets_information: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **str**| An account public key | 
 **limit** | **int**| Maximum number of results to return. | [optional] 
 **next** | **str**| The next page of results. Use the next token provided by the previous results. | [optional] 

### Return type

[**AccountAssetsInformation200Response**](AccountAssetsInformation200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | AccountAssetsInformationResponse contains a list of assets held by an account. |  -  |
**400** | Malformed address |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_information**
> Account account_information(address, format=format, exclude=exclude)

Get account information.

Given a specific account public key, this call returns the account's status, balance and spendable amounts

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.account import Account
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    address = 'address_example' # str | An account public key
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
    exclude = 'exclude_example' # str | When set to `all` will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to `none`. (optional)

    try:
        # Get account information.
        api_response = api_instance.account_information(address, format=format, exclude=exclude)
        print("The response of AlgodApi->account_information:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->account_information: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **str**| An account public key | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 
 **exclude** | **str**| When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;. | [optional] 

### Return type

[**Account**](Account.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | AccountResponse wraps the Account type in a response. |  -  |
**400** | Bad request |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **add_participation_key**
> AddParticipationKey200Response add_participation_key(participationkey)

Add a participation key to the node

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.add_participation_key200_response import AddParticipationKey200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    participationkey = None # bytearray | The participation key to add to the node

    try:
        # Add a participation key to the node
        api_response = api_instance.add_participation_key(participationkey)
        print("The response of AlgodApi->add_participation_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->add_participation_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participationkey** | **bytearray**| The participation key to add to the node | 

### Return type

[**AddParticipationKey200Response**](AddParticipationKey200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/msgpack
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Participation ID of the submission |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Participation Key Not Found |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **append_keys**
> ParticipationKey append_keys(participation_id, keymap)

Append state proof keys to a participation key

Given a participation ID, append state proof keys to a particular set of participation keys

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.participation_key import ParticipationKey
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    participation_id = 'participation_id_example' # str | 
    keymap = None # bytearray | The state proof keys to add to an existing participation ID

    try:
        # Append state proof keys to a participation key
        api_response = api_instance.append_keys(participation_id, keymap)
        print("The response of AlgodApi->append_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->append_keys: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participation_id** | **str**|  | 
 **keymap** | **bytearray**| The state proof keys to add to an existing participation ID | 

### Return type

[**ParticipationKey**](ParticipationKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/msgpack
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A detailed description of a participation ID |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Participation Key Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_participation_key_by_id**
> delete_participation_key_by_id(participation_id)

Delete a given participation key by ID

Delete a given participation key by ID

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    participation_id = 'participation_id_example' # str | 

    try:
        # Delete a given participation key by ID
        api_instance.delete_participation_key_by_id(participation_id)
    except Exception as e:
        print("Exception when calling AlgodApi->delete_participation_key_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participation_id** | **str**|  | 

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Participation key got deleted by ID |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Participation Key Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **experimental_check**
> experimental_check()

Returns OK if experimental API is enabled.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Returns OK if experimental API is enabled.
        api_instance.experimental_check()
    except Exception as e:
        print("Exception when calling AlgodApi->experimental_check: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Experimental API enabled |  -  |
**404** | Experimental API not enabled |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **generate_participation_keys**
> str generate_participation_keys(address, first, last, dilution=dilution)

Generate and install participation keys to the node.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    address = 'address_example' # str | An account public key
    first = 56 # int | First round for participation key.
    last = 56 # int | Last round for participation key.
    dilution = 56 # int | Key dilution for two-level participation keys (defaults to sqrt of validity window). (optional)

    try:
        # Generate and install participation keys to the node.
        api_response = api_instance.generate_participation_keys(address, first, last, dilution=dilution)
        print("The response of AlgodApi->generate_participation_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->generate_participation_keys: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **str**| An account public key | 
 **first** | **int**| First round for participation key. | 
 **last** | **int**| Last round for participation key. | 
 **dilution** | **int**| Key dilution for two-level participation keys (defaults to sqrt of validity window). | [optional] 

### Return type

**str**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | An empty JSON object is returned if the generation process was started. Currently no status is available. |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_box_by_name**
> Box get_application_box_by_name(application_id, name)

Get box information for a given application.

Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form 'encoding:value'. For ints, use the form 'int:1234'. For raw bytes, use the form 'b64:A=='. For printable strings, use the form 'str:hello'. For addresses, use the form 'addr:XYZ...'.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.box import Box
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    application_id = 56 # int | An application identifier
    name = 'name_example' # str | A box name, in the goal app call arg form 'encoding:value'. For ints, use the form 'int:1234'. For raw bytes, use the form 'b64:A=='. For printable strings, use the form 'str:hello'. For addresses, use the form 'addr:XYZ...'.

    try:
        # Get box information for a given application.
        api_response = api_instance.get_application_box_by_name(application_id, name)
        print("The response of AlgodApi->get_application_box_by_name:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_application_box_by_name: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **application_id** | **int**| An application identifier | 
 **name** | **str**| A box name, in the goal app call arg form &#39;encoding:value&#39;. For ints, use the form &#39;int:1234&#39;. For raw bytes, use the form &#39;b64:A&#x3D;&#x3D;&#39;. For printable strings, use the form &#39;str:hello&#39;. For addresses, use the form &#39;addr:XYZ...&#39;. | 

### Return type

[**Box**](Box.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Box information |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Box Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_boxes**
> GetApplicationBoxes200Response get_application_boxes(application_id, max=max)

Get all box names for a given application.

Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_application_boxes200_response import GetApplicationBoxes200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    application_id = 56 # int | An application identifier
    max = 56 # int | Max number of box names to return. If max is not set, or max == 0, returns all box-names. (optional)

    try:
        # Get all box names for a given application.
        api_response = api_instance.get_application_boxes(application_id, max=max)
        print("The response of AlgodApi->get_application_boxes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_application_boxes: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **application_id** | **int**| An application identifier | 
 **max** | **int**| Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names. | [optional] 

### Return type

[**GetApplicationBoxes200Response**](GetApplicationBoxes200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Box names of an application |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_by_id**
> Application get_application_by_id(application_id)

Get application information.

Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.application import Application
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    application_id = 56 # int | An application identifier

    try:
        # Get application information.
        api_response = api_instance.get_application_by_id(application_id)
        print("The response of AlgodApi->get_application_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_application_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **application_id** | **int**| An application identifier | 

### Return type

[**Application**](Application.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Application information |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Application Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_asset_by_id**
> Asset get_asset_by_id(asset_id)

Get asset information.

Given a asset ID, it returns asset information including creator, name, total supply and special addresses.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.asset import Asset
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    asset_id = 56 # int | An asset identifier

    try:
        # Get asset information.
        api_response = api_instance.get_asset_by_id(asset_id)
        print("The response of AlgodApi->get_asset_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_asset_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **asset_id** | **int**| An asset identifier | 

### Return type

[**Asset**](Asset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Asset information |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Application Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block**
> GetBlock200Response get_block(round, format=format, header_only=header_only)

Get the block for the given round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_block200_response import GetBlock200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round from which to fetch block information.
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
    header_only = True # bool | If true, only the block header (exclusive of payset or certificate) may be included in response. (optional)

    try:
        # Get the block for the given round.
        api_response = api_instance.get_block(round, format=format, header_only=header_only)
        print("The response of AlgodApi->get_block:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_block: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round from which to fetch block information. | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 
 **header_only** | **bool**| If true, only the block header (exclusive of payset or certificate) may be included in response. | [optional] 

### Return type

[**GetBlock200Response**](GetBlock200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Encoded block object. |  -  |
**400** | Bad Request - Non integer number |  -  |
**401** | Invalid API Token |  -  |
**404** | None existing block  |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_hash**
> GetBlockHash200Response get_block_hash(round)

Get the block hash for the block on the given round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_block_hash200_response import GetBlockHash200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round from which to fetch block hash information.

    try:
        # Get the block hash for the block on the given round.
        api_response = api_instance.get_block_hash(round)
        print("The response of AlgodApi->get_block_hash:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_block_hash: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round from which to fetch block hash information. | 

### Return type

[**GetBlockHash200Response**](GetBlockHash200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Hash of a block header. |  -  |
**400** | Bad Request - Non integer number |  -  |
**401** | Invalid API Token |  -  |
**404** | None existing block  |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_logs**
> GetBlockLogs200Response get_block_logs(round)

Get all of the logs from outer and inner app calls in the given round

Get all of the logs from outer and inner app calls in the given round

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_block_logs200_response import GetBlockLogs200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round from which to fetch block log information.

    try:
        # Get all of the logs from outer and inner app calls in the given round
        api_response = api_instance.get_block_logs(round)
        print("The response of AlgodApi->get_block_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_block_logs: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round from which to fetch block log information. | 

### Return type

[**GetBlockLogs200Response**](GetBlockLogs200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | All logs emitted in the given round. Each app call, whether top-level or inner, that contains logs results in a separate AppCallLogs object. Therefore there may be multiple AppCallLogs with the same application ID and outer transaction ID in the event of multiple inner app calls to the same app. App calls with no logs are not included in the response. AppCallLogs are returned in the same order that their corresponding app call appeared in the block (pre-order traversal of inner app calls) |  -  |
**400** | Bad Request - Non integer number |  -  |
**401** | Invalid API Token |  -  |
**404** | Nonexistent block  |  -  |
**500** | Internal Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_time_stamp_offset**
> GetBlockTimeStampOffset200Response get_block_time_stamp_offset()

Returns the timestamp offset. Timestamp offsets can only be set in dev mode.

Gets the current timestamp offset.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_block_time_stamp_offset200_response import GetBlockTimeStampOffset200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
        api_response = api_instance.get_block_time_stamp_offset()
        print("The response of AlgodApi->get_block_time_stamp_offset:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_block_time_stamp_offset: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**GetBlockTimeStampOffset200Response**](GetBlockTimeStampOffset200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Response containing the timestamp offset in seconds |  -  |
**400** | TimeStamp offset not set. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_txids**
> GetBlockTxids200Response get_block_txids(round)

Get the top level transaction IDs for the block on the given round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_block_txids200_response import GetBlockTxids200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round from which to fetch block transaction IDs.

    try:
        # Get the top level transaction IDs for the block on the given round.
        api_response = api_instance.get_block_txids(round)
        print("The response of AlgodApi->get_block_txids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_block_txids: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round from which to fetch block transaction IDs. | 

### Return type

[**GetBlockTxids200Response**](GetBlockTxids200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Top level transaction IDs in a block. |  -  |
**400** | Bad Request - Non integer number |  -  |
**401** | Invalid API Token |  -  |
**404** | Non existing block |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_config**
> str get_config()

Gets the merged config file.

Returns the merged (defaults + overrides) config file in json.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Gets the merged config file.
        api_response = api_instance.get_config()
        print("The response of AlgodApi->get_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_config: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The merged config file in json. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_debug_settings_prof**
> DebugSettingsProf get_debug_settings_prof()

Retrieves the current settings for blocking and mutex profiles

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.debug_settings_prof import DebugSettingsProf
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        api_response = api_instance.get_debug_settings_prof()
        print("The response of AlgodApi->get_debug_settings_prof:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_debug_settings_prof: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**DebugSettingsProf**](DebugSettingsProf.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | DebugPprof is the response to the /debug/extra/pprof endpoint |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_genesis**
> Genesis get_genesis()

Gets the genesis information.

Returns the entire genesis file in json.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.genesis import Genesis
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Gets the genesis information.
        api_response = api_instance.get_genesis()
        print("The response of AlgodApi->get_genesis:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_genesis: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**Genesis**](Genesis.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The genesis file in json. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ledger_state_delta**
> object get_ledger_state_delta(round, format=format)

Get a LedgerStateDelta object for a given round

Get ledger deltas for a round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round for which the deltas are desired.
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get a LedgerStateDelta object for a given round
        api_response = api_instance.get_ledger_state_delta(round, format=format)
        print("The response of AlgodApi->get_ledger_state_delta:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_ledger_state_delta: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round for which the deltas are desired. | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

**object**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Contains ledger deltas |  -  |
**401** | Invalid API Token |  -  |
**404** | Could not find a delta for round |  -  |
**408** | timed out on request |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ledger_state_delta_for_transaction_group**
> object get_ledger_state_delta_for_transaction_group(id, format=format)

Get a LedgerStateDelta object for a given transaction group

Get a ledger delta for a given transaction group.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    id = 'id_example' # str | A transaction ID, or transaction group ID
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get a LedgerStateDelta object for a given transaction group
        api_response = api_instance.get_ledger_state_delta_for_transaction_group(id, format=format)
        print("The response of AlgodApi->get_ledger_state_delta_for_transaction_group:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_ledger_state_delta_for_transaction_group: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| A transaction ID, or transaction group ID | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

**object**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Response containing a ledger state delta for a single transaction group. |  -  |
**401** | Invalid API Token |  -  |
**404** | Could not find a delta for transaction ID or group ID |  -  |
**408** | timed out on request |  -  |
**500** | Internal Error |  -  |
**501** | Not Implemented |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_light_block_header_proof**
> LightBlockHeaderProof get_light_block_header_proof(round)

Gets a proof for a given light block header inside a state proof commitment

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.light_block_header_proof import LightBlockHeaderProof
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round to which the light block header belongs.

    try:
        # Gets a proof for a given light block header inside a state proof commitment
        api_response = api_instance.get_light_block_header_proof(round)
        print("The response of AlgodApi->get_light_block_header_proof:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_light_block_header_proof: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round to which the light block header belongs. | 

### Return type

[**LightBlockHeaderProof**](LightBlockHeaderProof.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Proof of a light block header. |  -  |
**401** | Invalid API Token |  -  |
**404** | Could not create proof since some data is missing |  -  |
**408** | timed out on request |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_participation_key_by_id**
> ParticipationKey get_participation_key_by_id(participation_id)

Get participation key info given a participation ID

Given a participation ID, return information about that participation key

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.participation_key import ParticipationKey
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    participation_id = 'participation_id_example' # str | 

    try:
        # Get participation key info given a participation ID
        api_response = api_instance.get_participation_key_by_id(participation_id)
        print("The response of AlgodApi->get_participation_key_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_participation_key_by_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participation_id** | **str**|  | 

### Return type

[**ParticipationKey**](ParticipationKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A detailed description of a participation ID |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Participation Key Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_participation_keys**
> List[ParticipationKey] get_participation_keys()

Return a list of participation keys

Return a list of participation keys

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.participation_key import ParticipationKey
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Return a list of participation keys
        api_response = api_instance.get_participation_keys()
        print("The response of AlgodApi->get_participation_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_participation_keys: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[ParticipationKey]**](ParticipationKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A list of participation keys |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Participation Key Not Found |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pending_transactions**
> GetPendingTransactionsByAddress200Response get_pending_transactions(max=max, format=format)

Get a list of unconfirmed transactions currently in the transaction pool.

Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions.


### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_pending_transactions_by_address200_response import GetPendingTransactionsByAddress200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    max = 56 # int | Truncated number of transactions to display. If max=0, returns all pending txns. (optional)
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get a list of unconfirmed transactions currently in the transaction pool.
        api_response = api_instance.get_pending_transactions(max=max, format=format)
        print("The response of AlgodApi->get_pending_transactions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_pending_transactions: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **max** | **int**| Truncated number of transactions to display. If max&#x3D;0, returns all pending txns. | [optional] 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**GetPendingTransactionsByAddress200Response**](GetPendingTransactionsByAddress200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A potentially truncated list of transactions currently in the node&#39;s transaction pool. You can compute whether or not the list is truncated if the number of elements in the **top-transactions** array is fewer than **total-transactions**. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pending_transactions_by_address**
> GetPendingTransactionsByAddress200Response get_pending_transactions_by_address(address, max=max, format=format)

Get a list of unconfirmed transactions currently in the transaction pool by address.

Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions.


### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_pending_transactions_by_address200_response import GetPendingTransactionsByAddress200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    address = 'address_example' # str | An account public key
    max = 56 # int | Truncated number of transactions to display. If max=0, returns all pending txns. (optional)
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get a list of unconfirmed transactions currently in the transaction pool by address.
        api_response = api_instance.get_pending_transactions_by_address(address, max=max, format=format)
        print("The response of AlgodApi->get_pending_transactions_by_address:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_pending_transactions_by_address: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **str**| An account public key | 
 **max** | **int**| Truncated number of transactions to display. If max&#x3D;0, returns all pending txns. | [optional] 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**GetPendingTransactionsByAddress200Response**](GetPendingTransactionsByAddress200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A potentially truncated list of transactions currently in the node&#39;s transaction pool. You can compute whether or not the list is truncated if the number of elements in the **top-transactions** array is fewer than **total-transactions**. |  -  |
**400** | Max must be a non-negative integer |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ready**
> get_ready()

Returns OK if healthy and fully caught up.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Returns OK if healthy and fully caught up.
        api_instance.get_ready()
    except Exception as e:
        print("Exception when calling AlgodApi->get_ready: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK. |  -  |
**500** | Internal Error |  -  |
**503** | Node not ready yet |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_state_proof**
> StateProof get_state_proof(round)

Get a state proof that covers a given round

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.state_proof import StateProof
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round for which a state proof is desired.

    try:
        # Get a state proof that covers a given round
        api_response = api_instance.get_state_proof(round)
        print("The response of AlgodApi->get_state_proof:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_state_proof: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round for which a state proof is desired. | 

### Return type

[**StateProof**](StateProof.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | StateProofResponse wraps the StateProof type in a response. |  -  |
**401** | Invalid API Token |  -  |
**404** | Could not find a state proof that covers a given round |  -  |
**408** | timed out on request |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_status**
> GetStatus200Response get_status()

Gets the current node status.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_status200_response import GetStatus200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Gets the current node status.
        api_response = api_instance.get_status()
        print("The response of AlgodApi->get_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_status: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**GetStatus200Response**](GetStatus200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Returns the current node status including sync status, version, and latest round |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_supply**
> GetSupply200Response get_supply()

Get the current supply reported by the ledger.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_supply200_response import GetSupply200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Get the current supply reported by the ledger.
        api_response = api_instance.get_supply()
        print("The response of AlgodApi->get_supply:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_supply: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**GetSupply200Response**](GetSupply200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Supply represents the current supply of MicroAlgos in the system. |  -  |
**401** | Invalid API Token |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sync_round**
> GetSyncRound200Response get_sync_round()

Returns the minimum sync round the ledger is keeping in cache.

Gets the minimum sync round for the ledger.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_sync_round200_response import GetSyncRound200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Returns the minimum sync round the ledger is keeping in cache.
        api_response = api_instance.get_sync_round()
        print("The response of AlgodApi->get_sync_round:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_sync_round: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**GetSyncRound200Response**](GetSyncRound200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Response containing the ledger&#39;s minimum sync round |  -  |
**400** | Sync round not set. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_group_ledger_state_deltas_for_round**
> GetTransactionGroupLedgerStateDeltasForRound200Response get_transaction_group_ledger_state_deltas_for_round(round, format=format)

Get LedgerStateDelta objects for all transaction groups in a given round

Get ledger deltas for transaction groups in a given round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_transaction_group_ledger_state_deltas_for_round200_response import GetTransactionGroupLedgerStateDeltasForRound200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round for which the deltas are desired.
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get LedgerStateDelta objects for all transaction groups in a given round
        api_response = api_instance.get_transaction_group_ledger_state_deltas_for_round(round, format=format)
        print("The response of AlgodApi->get_transaction_group_ledger_state_deltas_for_round:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_transaction_group_ledger_state_deltas_for_round: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round for which the deltas are desired. | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**GetTransactionGroupLedgerStateDeltasForRound200Response**](GetTransactionGroupLedgerStateDeltasForRound200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Response containing all ledger state deltas for transaction groups, with their associated Ids, in a single round. |  -  |
**401** | Invalid API Token |  -  |
**404** | Could not find deltas for round |  -  |
**408** | timed out on request |  -  |
**500** | Internal Error |  -  |
**501** | Not Implemented |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_proof**
> GetTransactionProof200Response get_transaction_proof(round, txid, hashtype=hashtype, format=format)

Get a proof for a transaction in a block.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_transaction_proof200_response import GetTransactionProof200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round in which the transaction appears.
    txid = 'txid_example' # str | The transaction ID for which to generate a proof.
    hashtype = 'hashtype_example' # str | The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256 (optional)
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get a proof for a transaction in a block.
        api_response = api_instance.get_transaction_proof(round, txid, hashtype=hashtype, format=format)
        print("The response of AlgodApi->get_transaction_proof:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_transaction_proof: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round in which the transaction appears. | 
 **txid** | **str**| The transaction ID for which to generate a proof. | 
 **hashtype** | **str**| The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256 | [optional] 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**GetTransactionProof200Response**](GetTransactionProof200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Proof of transaction in a block. |  -  |
**400** | Malformed round number or transaction ID |  -  |
**401** | Invalid API token |  -  |
**404** | Non-existent block or transaction |  -  |
**500** | Internal error, including protocol not supporting proofs. |  -  |
**0** | Unknown error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_version**
> Version get_version()

Retrieves the supported API versions, binary build versions, and genesis information.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.version import Version
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        api_response = api_instance.get_version()
        print("The response of AlgodApi->get_version:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->get_version: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**Version**](Version.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | VersionsResponse is the response to &#39;GET /versions&#39; |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **health_check**
> health_check()

Returns OK if healthy.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Returns OK if healthy.
        api_instance.health_check()
    except Exception as e:
        print("Exception when calling AlgodApi->health_check: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **metrics**
> metrics()

Return metrics about algod functioning.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Return metrics about algod functioning.
        api_instance.metrics()
    except Exception as e:
        print("Exception when calling AlgodApi->metrics: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | text with \\#-comments and key:value lines |  -  |
**404** | metrics were compiled out |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pending_transaction_information**
> PendingTransactionResponse pending_transaction_information(txid, format=format)

Get a specific pending transaction.

Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed:
- transaction committed (committed round > 0)
- transaction still in the pool (committed round = 0, pool error = "")
- transaction removed from pool due to error (committed round = 0, pool error != "")
Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error.


### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.pending_transaction_response import PendingTransactionResponse
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    txid = 'txid_example' # str | A transaction ID
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Get a specific pending transaction.
        api_response = api_instance.pending_transaction_information(txid, format=format)
        print("The response of AlgodApi->pending_transaction_information:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->pending_transaction_information: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **txid** | **str**| A transaction ID | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**PendingTransactionResponse**](PendingTransactionResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round &gt; 0) - transaction still in the pool (committed round &#x3D; 0, pool error &#x3D; \&quot;\&quot;) - transaction removed from pool due to error (committed round &#x3D; 0, pool error !&#x3D; \&quot;\&quot;)  Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Transaction Not Found |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_debug_settings_prof**
> DebugSettingsProf put_debug_settings_prof()

Enables blocking and mutex profiles, and returns the old settings

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.debug_settings_prof import DebugSettingsProf
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        api_response = api_instance.put_debug_settings_prof()
        print("The response of AlgodApi->put_debug_settings_prof:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->put_debug_settings_prof: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**DebugSettingsProf**](DebugSettingsProf.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | DebugPprof is the response to the /debug/extra/pprof endpoint |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **raw_transaction**
> RawTransaction200Response raw_transaction(rawtxn)

Broadcasts a raw transaction or transaction group to the network.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.raw_transaction200_response import RawTransaction200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    rawtxn = None # bytearray | The byte encoded signed transaction to broadcast to network

    try:
        # Broadcasts a raw transaction or transaction group to the network.
        api_response = api_instance.raw_transaction(rawtxn)
        print("The response of AlgodApi->raw_transaction:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->raw_transaction: %s\n" % e)
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

# **raw_transaction_async**
> raw_transaction_async(rawtxn)

Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    rawtxn = None # bytearray | The byte encoded signed transaction to broadcast to network

    try:
        # Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
        api_instance.raw_transaction_async(rawtxn)
    except Exception as e:
        print("Exception when calling AlgodApi->raw_transaction_async: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rawtxn** | **bytearray**| The byte encoded signed transaction to broadcast to network | 

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/x-binary
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Transaction successfully submitted for asynchronous processing |  -  |
**400** | Bad Request - Malformed Algorand transaction  |  -  |
**401** | Invalid API Token |  -  |
**404** | Developer or Experimental API not enabled |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_block_time_stamp_offset**
> set_block_time_stamp_offset(offset)

Given a timestamp offset in seconds, adds the offset to every subsequent block header's timestamp.

Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    offset = 56 # int | The timestamp offset for blocks in dev mode.

    try:
        # Given a timestamp offset in seconds, adds the offset to every subsequent block header's timestamp.
        api_instance.set_block_time_stamp_offset(offset)
    except Exception as e:
        print("Exception when calling AlgodApi->set_block_time_stamp_offset: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **offset** | **int**| The timestamp offset for blocks in dev mode. | 

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |
**400** | Cannot set timestamp offset to a negative integer. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_sync_round**
> set_sync_round(round)

Given a round, tells the ledger to keep that round in its cache.

Sets the minimum sync round on the ledger.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round for which the deltas are desired.

    try:
        # Given a round, tells the ledger to keep that round in its cache.
        api_instance.set_sync_round(round)
    except Exception as e:
        print("Exception when calling AlgodApi->set_sync_round: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round for which the deltas are desired. | 

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Ledger sync to specified round initiated successfully |  -  |
**400** | Can not set sync round to an earlier round than the current round. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **shutdown_node**
> object shutdown_node(timeout=timeout)

Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    timeout = 0 # int |  (optional) (default to 0)

    try:
        api_response = api_instance.shutdown_node(timeout=timeout)
        print("The response of AlgodApi->shutdown_node:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->shutdown_node: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timeout** | **int**|  | [optional] [default to 0]

### Return type

**object**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Node shutdown initiated successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **simulate_transaction**
> SimulateTransaction200Response simulate_transaction(request, format=format)

Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.simulate_request import SimulateRequest
from algokit_algod_api.models.simulate_transaction200_response import SimulateTransaction200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    request = algokit_algod_api.SimulateRequest() # SimulateRequest | The transactions to simulate, along with any other inputs.
    format = 'format_example' # str | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)

    try:
        # Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
        api_response = api_instance.simulate_transaction(request, format=format)
        print("The response of AlgodApi->simulate_transaction:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->simulate_transaction: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request** | [**SimulateRequest**](SimulateRequest.md)| The transactions to simulate, along with any other inputs. | 
 **format** | **str**| Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | [optional] 

### Return type

[**SimulateTransaction200Response**](SimulateTransaction200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json, application/msgpack
 - **Accept**: application/json, application/msgpack

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Result of a transaction group simulation. |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **start_catchup**
> StartCatchup200Response start_catchup(catchpoint, min=min)

Starts a catchpoint catchup.

Given a catchpoint, it starts catching up to this catchpoint

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.start_catchup200_response import StartCatchup200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    catchpoint = 'catchpoint_example' # str | A catch point
    min = 56 # int | Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed. (optional)

    try:
        # Starts a catchpoint catchup.
        api_response = api_instance.start_catchup(catchpoint, min=min)
        print("The response of AlgodApi->start_catchup:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->start_catchup: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **catchpoint** | **str**| A catch point | 
 **min** | **int**| Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed. | [optional] 

### Return type

[**StartCatchup200Response**](StartCatchup200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Catchpoint operation started successfully |  -  |
**201** | Catchpoint operation created and started successfully |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**408** | Request Timeout |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **swagger_json**
> str swagger_json()

Gets the current swagger spec.

Returns the entire swagger spec in json.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Gets the current swagger spec.
        api_response = api_instance.swagger_json()
        print("The response of AlgodApi->swagger_json:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->swagger_json: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The current swagger spec |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **teal_compile**
> TealCompile200Response teal_compile(source, sourcemap=sourcemap)

Compile TEAL source code to binary, produce its hash

Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.teal_compile200_response import TealCompile200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    source = None # bytearray | TEAL source code to be compiled
    sourcemap = True # bool | When set to `true`, returns the source map of the program as a JSON. Defaults to `false`. (optional)

    try:
        # Compile TEAL source code to binary, produce its hash
        api_response = api_instance.teal_compile(source, sourcemap=sourcemap)
        print("The response of AlgodApi->teal_compile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->teal_compile: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source** | **bytearray**| TEAL source code to be compiled | 
 **sourcemap** | **bool**| When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;. | [optional] 

### Return type

[**TealCompile200Response**](TealCompile200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: text/plain
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Teal compile Result |  -  |
**400** | Bad Request - Teal Compile Error |  -  |
**401** | Invalid API Token |  -  |
**404** | Developer API not enabled |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **teal_disassemble**
> TealDisassemble200Response teal_disassemble(source)

Disassemble program bytes into the TEAL source code.

Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.teal_disassemble200_response import TealDisassemble200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    source = None # bytearray | TEAL program binary to be disassembled

    try:
        # Disassemble program bytes into the TEAL source code.
        api_response = api_instance.teal_disassemble(source)
        print("The response of AlgodApi->teal_disassemble:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->teal_disassemble: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source** | **bytearray**| TEAL program binary to be disassembled | 

### Return type

[**TealDisassemble200Response**](TealDisassemble200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/x-binary
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Teal disassembly Result |  -  |
**400** | Bad Request - Teal Compile Error |  -  |
**401** | Invalid API Token |  -  |
**404** | Developer API not enabled |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **teal_dryrun**
> TealDryrun200Response teal_dryrun(request=request)

Provide debugging information for a transaction (or group).

Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.dryrun_request import DryrunRequest
from algokit_algod_api.models.teal_dryrun200_response import TealDryrun200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    request = algokit_algod_api.DryrunRequest() # DryrunRequest | Transaction (or group) and any accompanying state-simulation data. (optional)

    try:
        # Provide debugging information for a transaction (or group).
        api_response = api_instance.teal_dryrun(request=request)
        print("The response of AlgodApi->teal_dryrun:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->teal_dryrun: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request** | [**DryrunRequest**](DryrunRequest.md)| Transaction (or group) and any accompanying state-simulation data. | [optional] 

### Return type

[**TealDryrun200Response**](TealDryrun200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json, application/msgpack
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | DryrunResponse contains per-txn debug information from a dryrun. |  -  |
**400** | Bad Request |  -  |
**401** | Invalid API Token |  -  |
**404** | Developer API not enabled |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **transaction_params**
> TransactionParams200Response transaction_params()

Get parameters for constructing a new transaction

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.transaction_params200_response import TransactionParams200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Get parameters for constructing a new transaction
        api_response = api_instance.transaction_params()
        print("The response of AlgodApi->transaction_params:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->transaction_params: %s\n" % e)
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

# **unset_sync_round**
> unset_sync_round()

Removes minimum sync round restriction from the ledger.

Unset the ledger sync round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)

    try:
        # Removes minimum sync round restriction from the ledger.
        api_instance.unset_sync_round()
    except Exception as e:
        print("Exception when calling AlgodApi->unset_sync_round: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Ledger sync operation stopped successfully |  -  |
**400** | Sync round not set. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **wait_for_block**
> GetStatus200Response wait_for_block(round)

Gets the node status after waiting for a round after the given round.

Waits for a block to appear after round {round} and returns the node's status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.

### Example

* Api Key Authentication (api_key):

```python
import algokit_algod_api
from algokit_algod_api.models.get_status200_response import GetStatus200Response
from algokit_algod_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = algokit_algod_api.Configuration(
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
with algokit_algod_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = algokit_algod_api.AlgodApi(api_client)
    round = 56 # int | The round to wait until returning status

    try:
        # Gets the node status after waiting for a round after the given round.
        api_response = api_instance.wait_for_block(round)
        print("The response of AlgodApi->wait_for_block:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AlgodApi->wait_for_block: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | **int**| The round to wait until returning status | 

### Return type

[**GetStatus200Response**](GetStatus200Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Returns node status after the specified round is reached |  -  |
**400** | Bad Request -- number must be non-negative integer |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

