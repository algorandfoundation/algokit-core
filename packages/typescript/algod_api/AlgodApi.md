# .AlgodApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abortCatchup**](AlgodApi.md#abortCatchup) | **DELETE** /v2/catchup/{catchpoint} | Aborts a catchpoint catchup.
[**accountApplicationInformation**](AlgodApi.md#accountApplicationInformation) | **GET** /v2/accounts/{address}/applications/{application-id} | Get account information about a given app.
[**accountAssetInformation**](AlgodApi.md#accountAssetInformation) | **GET** /v2/accounts/{address}/assets/{asset-id} | Get account information about a given asset.
[**accountAssetsInformation**](AlgodApi.md#accountAssetsInformation) | **GET** /v2/accounts/{address}/assets | Get a list of assets held by an account, inclusive of asset params.
[**accountInformation**](AlgodApi.md#accountInformation) | **GET** /v2/accounts/{address} | Get account information.
[**addParticipationKey**](AlgodApi.md#addParticipationKey) | **POST** /v2/participation | Add a participation key to the node
[**appendKeys**](AlgodApi.md#appendKeys) | **POST** /v2/participation/{participation-id} | Append state proof keys to a participation key
[**deleteParticipationKeyByID**](AlgodApi.md#deleteParticipationKeyByID) | **DELETE** /v2/participation/{participation-id} | Delete a given participation key by ID
[**experimentalCheck**](AlgodApi.md#experimentalCheck) | **GET** /v2/experimental | Returns OK if experimental API is enabled.
[**generateParticipationKeys**](AlgodApi.md#generateParticipationKeys) | **POST** /v2/participation/generate/{address} | Generate and install participation keys to the node.
[**getApplicationBoxByName**](AlgodApi.md#getApplicationBoxByName) | **GET** /v2/applications/{application-id}/box | Get box information for a given application.
[**getApplicationBoxes**](AlgodApi.md#getApplicationBoxes) | **GET** /v2/applications/{application-id}/boxes | Get all box names for a given application.
[**getApplicationByID**](AlgodApi.md#getApplicationByID) | **GET** /v2/applications/{application-id} | Get application information.
[**getAssetByID**](AlgodApi.md#getAssetByID) | **GET** /v2/assets/{asset-id} | Get asset information.
[**getBlock**](AlgodApi.md#getBlock) | **GET** /v2/blocks/{round} | Get the block for the given round.
[**getBlockHash**](AlgodApi.md#getBlockHash) | **GET** /v2/blocks/{round}/hash | Get the block hash for the block on the given round.
[**getBlockLogs**](AlgodApi.md#getBlockLogs) | **GET** /v2/blocks/{round}/logs | Get all of the logs from outer and inner app calls in the given round
[**getBlockTimeStampOffset**](AlgodApi.md#getBlockTimeStampOffset) | **GET** /v2/devmode/blocks/offset | Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
[**getBlockTxids**](AlgodApi.md#getBlockTxids) | **GET** /v2/blocks/{round}/txids | Get the top level transaction IDs for the block on the given round.
[**getConfig**](AlgodApi.md#getConfig) | **GET** /debug/settings/config | Gets the merged config file.
[**getDebugSettingsProf**](AlgodApi.md#getDebugSettingsProf) | **GET** /debug/settings/pprof | 
[**getGenesis**](AlgodApi.md#getGenesis) | **GET** /genesis | Gets the genesis information.
[**getLedgerStateDelta**](AlgodApi.md#getLedgerStateDelta) | **GET** /v2/deltas/{round} | Get a LedgerStateDelta object for a given round
[**getLedgerStateDeltaForTransactionGroup**](AlgodApi.md#getLedgerStateDeltaForTransactionGroup) | **GET** /v2/deltas/txn/group/{id} | Get a LedgerStateDelta object for a given transaction group
[**getLightBlockHeaderProof**](AlgodApi.md#getLightBlockHeaderProof) | **GET** /v2/blocks/{round}/lightheader/proof | Gets a proof for a given light block header inside a state proof commitment
[**getParticipationKeyByID**](AlgodApi.md#getParticipationKeyByID) | **GET** /v2/participation/{participation-id} | Get participation key info given a participation ID
[**getParticipationKeys**](AlgodApi.md#getParticipationKeys) | **GET** /v2/participation | Return a list of participation keys
[**getPendingTransactions**](AlgodApi.md#getPendingTransactions) | **GET** /v2/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool.
[**getPendingTransactionsByAddress**](AlgodApi.md#getPendingTransactionsByAddress) | **GET** /v2/accounts/{address}/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool by address.
[**getReady**](AlgodApi.md#getReady) | **GET** /ready | Returns OK if healthy and fully caught up.
[**getStateProof**](AlgodApi.md#getStateProof) | **GET** /v2/stateproofs/{round} | Get a state proof that covers a given round
[**getStatus**](AlgodApi.md#getStatus) | **GET** /v2/status | Gets the current node status.
[**getSupply**](AlgodApi.md#getSupply) | **GET** /v2/ledger/supply | Get the current supply reported by the ledger.
[**getSyncRound**](AlgodApi.md#getSyncRound) | **GET** /v2/ledger/sync | Returns the minimum sync round the ledger is keeping in cache.
[**getTransactionGroupLedgerStateDeltasForRound**](AlgodApi.md#getTransactionGroupLedgerStateDeltasForRound) | **GET** /v2/deltas/{round}/txn/group | Get LedgerStateDelta objects for all transaction groups in a given round
[**getTransactionProof**](AlgodApi.md#getTransactionProof) | **GET** /v2/blocks/{round}/transactions/{txid}/proof | Get a proof for a transaction in a block.
[**getVersion**](AlgodApi.md#getVersion) | **GET** /versions | 
[**healthCheck**](AlgodApi.md#healthCheck) | **GET** /health | Returns OK if healthy.
[**metrics**](AlgodApi.md#metrics) | **GET** /metrics | Return metrics about algod functioning.
[**pendingTransactionInformation**](AlgodApi.md#pendingTransactionInformation) | **GET** /v2/transactions/pending/{txid} | Get a specific pending transaction.
[**putDebugSettingsProf**](AlgodApi.md#putDebugSettingsProf) | **PUT** /debug/settings/pprof | 
[**rawTransaction**](AlgodApi.md#rawTransaction) | **POST** /v2/transactions | Broadcasts a raw transaction or transaction group to the network.
[**rawTransactionAsync**](AlgodApi.md#rawTransactionAsync) | **POST** /v2/transactions/async | Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
[**setBlockTimeStampOffset**](AlgodApi.md#setBlockTimeStampOffset) | **POST** /v2/devmode/blocks/offset/{offset} | Given a timestamp offset in seconds, adds the offset to every subsequent block header\&#39;s timestamp.
[**setSyncRound**](AlgodApi.md#setSyncRound) | **POST** /v2/ledger/sync/{round} | Given a round, tells the ledger to keep that round in its cache.
[**shutdownNode**](AlgodApi.md#shutdownNode) | **POST** /v2/shutdown | 
[**simulateTransaction**](AlgodApi.md#simulateTransaction) | **POST** /v2/transactions/simulate | Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
[**startCatchup**](AlgodApi.md#startCatchup) | **POST** /v2/catchup/{catchpoint} | Starts a catchpoint catchup.
[**swaggerJSON**](AlgodApi.md#swaggerJSON) | **GET** /swagger.json | Gets the current swagger spec.
[**tealCompile**](AlgodApi.md#tealCompile) | **POST** /v2/teal/compile | Compile TEAL source code to binary, produce its hash
[**tealDisassemble**](AlgodApi.md#tealDisassemble) | **POST** /v2/teal/disassemble | Disassemble program bytes into the TEAL source code.
[**tealDryrun**](AlgodApi.md#tealDryrun) | **POST** /v2/teal/dryrun | Provide debugging information for a transaction (or group).
[**transactionParams**](AlgodApi.md#transactionParams) | **GET** /v2/transactions/params | Get parameters for constructing a new transaction
[**unsetSyncRound**](AlgodApi.md#unsetSyncRound) | **DELETE** /v2/ledger/sync | Removes minimum sync round restriction from the ledger.
[**waitForBlock**](AlgodApi.md#waitForBlock) | **GET** /v2/status/wait-for-block-after/{round} | Gets the node status after waiting for a round after the given round.


# **abortCatchup**
> AbortCatchup200Response abortCatchup()

Given a catchpoint, it aborts catching up to this catchpoint

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAbortCatchupRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAbortCatchupRequest = {
    // A catch point
  catchpoint: "4#6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS",
};

const data = await apiInstance.abortCatchup(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **catchpoint** | [**string**] | A catch point | defaults to undefined


### Return type

**AbortCatchup200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **accountApplicationInformation**
> AccountApplicationInformation200Response accountApplicationInformation()

Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAccountApplicationInformationRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAccountApplicationInformationRequest = {
    // An account public key
  address: "W8Q6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS9H",
    // An application identifier
  applicationId: 1,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.accountApplicationInformation(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**string**] | An account public key | defaults to undefined
 **applicationId** | [**number**] | An application identifier | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**AccountApplicationInformation200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | AccountApplicationResponse describes the account\&#39;s application local state and global state (AppLocalState and AppParams, if either exists) for a specific application ID. Global state will only be returned if the provided address is the application\&#39;s creator. |  -  |
**400** | Malformed address or application ID |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **accountAssetInformation**
> AccountAssetInformation200Response accountAssetInformation()

Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAccountAssetInformationRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAccountAssetInformationRequest = {
    // An account public key
  address: "W8Q6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS9H",
    // An asset identifier
  assetId: 1,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.accountAssetInformation(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**string**] | An account public key | defaults to undefined
 **assetId** | [**number**] | An asset identifier | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**AccountAssetInformation200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | AccountAssetResponse describes the account\&#39;s asset holding and asset parameters (if either exist) for a specific asset ID. Asset parameters will only be returned if the provided address is the asset\&#39;s creator. |  -  |
**400** | Malformed address or asset ID |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **accountAssetsInformation**
> AccountAssetsInformation200Response accountAssetsInformation()

Lookup an account\'s asset holdings.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAccountAssetsInformationRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAccountAssetsInformationRequest = {
    // An account public key
  address: "W8Q6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS9H",
    // Maximum number of results to return. (optional)
  limit: 1,
    // The next page of results. Use the next token provided by the previous results. (optional)
  next: "next_example",
};

const data = await apiInstance.accountAssetsInformation(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**string**] | An account public key | defaults to undefined
 **limit** | [**number**] | Maximum number of results to return. | (optional) defaults to undefined
 **next** | [**string**] | The next page of results. Use the next token provided by the previous results. | (optional) defaults to undefined


### Return type

**AccountAssetsInformation200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **accountInformation**
> Account accountInformation()

Given a specific account public key, this call returns the account\'s status, balance and spendable amounts

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAccountInformationRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAccountInformationRequest = {
    // An account public key
  address: "W8Q6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS9H",
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
    // When set to `all` will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to `none`. (optional)
  exclude: "all",
};

const data = await apiInstance.accountInformation(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**string**] | An account public key | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined
 **exclude** | [**&#39;all&#39; | &#39;none&#39;**]**Array<&#39;all&#39; &#124; &#39;none&#39;>** | When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;. | (optional) defaults to undefined


### Return type

**Account**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **addParticipationKey**
> AddParticipationKey200Response addParticipationKey(participationkey)


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAddParticipationKeyRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAddParticipationKeyRequest = {
    // The participation key to add to the node
  participationkey: { data: Buffer.from(fs.readFileSync('/path/to/file', 'utf-8')), name: '/path/to/file' },
};

const data = await apiInstance.addParticipationKey(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participationkey** | **HttpFile**| The participation key to add to the node |


### Return type

**AddParticipationKey200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **appendKeys**
> ParticipationKey appendKeys(keymap)

Given a participation ID, append state proof keys to a particular set of participation keys

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiAppendKeysRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiAppendKeysRequest = {
  
  participationId: "participation-id_example",
    // The state proof keys to add to an existing participation ID
  keymap: { data: Buffer.from(fs.readFileSync('/path/to/file', 'utf-8')), name: '/path/to/file' },
};

const data = await apiInstance.appendKeys(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **keymap** | **HttpFile**| The state proof keys to add to an existing participation ID |
 **participationId** | [**string**] |  | defaults to undefined


### Return type

**ParticipationKey**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **deleteParticipationKeyByID**
> void deleteParticipationKeyByID()

Delete a given participation key by ID

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiDeleteParticipationKeyByIDRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiDeleteParticipationKeyByIDRequest = {
  
  participationId: "participation-id_example",
};

const data = await apiInstance.deleteParticipationKeyByID(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participationId** | [**string**] |  | defaults to undefined


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **experimentalCheck**
> void experimentalCheck()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.experimentalCheck(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Experimental API enabled |  -  |
**404** | Experimental API not enabled |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **generateParticipationKeys**
> string generateParticipationKeys()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGenerateParticipationKeysRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGenerateParticipationKeysRequest = {
    // An account public key
  address: "address_example",
    // First round for participation key.
  first: 1,
    // Last round for participation key.
  last: 1,
    // Key dilution for two-level participation keys (defaults to sqrt of validity window). (optional)
  dilution: 1,
};

const data = await apiInstance.generateParticipationKeys(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**string**] | An account public key | defaults to undefined
 **first** | [**number**] | First round for participation key. | defaults to undefined
 **last** | [**number**] | Last round for participation key. | defaults to undefined
 **dilution** | [**number**] | Key dilution for two-level participation keys (defaults to sqrt of validity window). | (optional) defaults to undefined


### Return type

**string**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getApplicationBoxByName**
> Box getApplicationBoxByName()

Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetApplicationBoxByNameRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetApplicationBoxByNameRequest = {
    // An application identifier
  applicationId: 1,
    // A box name, in the goal app call arg form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
  name: "name_example",
};

const data = await apiInstance.getApplicationBoxByName(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **applicationId** | [**number**] | An application identifier | defaults to undefined
 **name** | [**string**] | A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;. | defaults to undefined


### Return type

**Box**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getApplicationBoxes**
> GetApplicationBoxes200Response getApplicationBoxes()

Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetApplicationBoxesRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetApplicationBoxesRequest = {
    // An application identifier
  applicationId: 1,
    // Max number of box names to return. If max is not set, or max == 0, returns all box-names. (optional)
  max: 1,
};

const data = await apiInstance.getApplicationBoxes(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **applicationId** | [**number**] | An application identifier | defaults to undefined
 **max** | [**number**] | Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names. | (optional) defaults to undefined


### Return type

**GetApplicationBoxes200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getApplicationByID**
> Application getApplicationByID()

Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetApplicationByIDRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetApplicationByIDRequest = {
    // An application identifier
  applicationId: 1,
};

const data = await apiInstance.getApplicationByID(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **applicationId** | [**number**] | An application identifier | defaults to undefined


### Return type

**Application**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getAssetByID**
> Asset getAssetByID()

Given a asset ID, it returns asset information including creator, name, total supply and special addresses.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetAssetByIDRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetAssetByIDRequest = {
    // An asset identifier
  assetId: 1,
};

const data = await apiInstance.getAssetByID(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **assetId** | [**number**] | An asset identifier | defaults to undefined


### Return type

**Asset**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getBlock**
> GetBlock200Response getBlock()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetBlockRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetBlockRequest = {
    // The round from which to fetch block information.
  round: 0,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
    // If true, only the block header (exclusive of payset or certificate) may be included in response. (optional)
  headerOnly: true,
};

const data = await apiInstance.getBlock(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round from which to fetch block information. | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined
 **headerOnly** | [**boolean**] | If true, only the block header (exclusive of payset or certificate) may be included in response. | (optional) defaults to undefined


### Return type

**GetBlock200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getBlockHash**
> GetBlockHash200Response getBlockHash()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetBlockHashRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetBlockHashRequest = {
    // The round from which to fetch block hash information.
  round: 0,
};

const data = await apiInstance.getBlockHash(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round from which to fetch block hash information. | defaults to undefined


### Return type

**GetBlockHash200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getBlockLogs**
> GetBlockLogs200Response getBlockLogs()

Get all of the logs from outer and inner app calls in the given round

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetBlockLogsRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetBlockLogsRequest = {
    // The round from which to fetch block log information.
  round: 0,
};

const data = await apiInstance.getBlockLogs(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round from which to fetch block log information. | defaults to undefined


### Return type

**GetBlockLogs200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getBlockTimeStampOffset**
> GetBlockTimeStampOffset200Response getBlockTimeStampOffset()

Gets the current timestamp offset.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getBlockTimeStampOffset(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**GetBlockTimeStampOffset200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Response containing the timestamp offset in seconds |  -  |
**400** | TimeStamp offset not set. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getBlockTxids**
> GetBlockTxids200Response getBlockTxids()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetBlockTxidsRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetBlockTxidsRequest = {
    // The round from which to fetch block transaction IDs.
  round: 0,
};

const data = await apiInstance.getBlockTxids(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round from which to fetch block transaction IDs. | defaults to undefined


### Return type

**GetBlockTxids200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getConfig**
> string getConfig()

Returns the merged (defaults + overrides) config file in json.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getConfig(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**string**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The merged config file in json. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getDebugSettingsProf**
> DebugSettingsProf getDebugSettingsProf()

Retrieves the current settings for blocking and mutex profiles

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getDebugSettingsProf(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**DebugSettingsProf**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | DebugPprof is the response to the /debug/extra/pprof endpoint |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getGenesis**
> Genesis getGenesis()

Returns the entire genesis file in json.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getGenesis(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**Genesis**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The genesis file in json. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getLedgerStateDelta**
> any getLedgerStateDelta()

Get ledger deltas for a round.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetLedgerStateDeltaRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetLedgerStateDeltaRequest = {
    // The round for which the deltas are desired.
  round: 0,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.getLedgerStateDelta(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round for which the deltas are desired. | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**any**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getLedgerStateDeltaForTransactionGroup**
> any getLedgerStateDeltaForTransactionGroup()

Get a ledger delta for a given transaction group.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetLedgerStateDeltaForTransactionGroupRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetLedgerStateDeltaForTransactionGroupRequest = {
    // A transaction ID, or transaction group ID
  id: "8",
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.getLedgerStateDeltaForTransactionGroup(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] | A transaction ID, or transaction group ID | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**any**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getLightBlockHeaderProof**
> LightBlockHeaderProof getLightBlockHeaderProof()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetLightBlockHeaderProofRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetLightBlockHeaderProofRequest = {
    // The round to which the light block header belongs.
  round: 0,
};

const data = await apiInstance.getLightBlockHeaderProof(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round to which the light block header belongs. | defaults to undefined


### Return type

**LightBlockHeaderProof**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getParticipationKeyByID**
> ParticipationKey getParticipationKeyByID()

Given a participation ID, return information about that participation key

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetParticipationKeyByIDRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetParticipationKeyByIDRequest = {
  
  participationId: "participation-id_example",
};

const data = await apiInstance.getParticipationKeyByID(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **participationId** | [**string**] |  | defaults to undefined


### Return type

**ParticipationKey**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getParticipationKeys**
> Array<ParticipationKey> getParticipationKeys()

Return a list of participation keys

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getParticipationKeys(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**Array<ParticipationKey>**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getPendingTransactions**
> GetPendingTransactionsByAddress200Response getPendingTransactions()

Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetPendingTransactionsRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetPendingTransactionsRequest = {
    // Truncated number of transactions to display. If max=0, returns all pending txns. (optional)
  max: 1,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.getPendingTransactions(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **max** | [**number**] | Truncated number of transactions to display. If max&#x3D;0, returns all pending txns. | (optional) defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**GetPendingTransactionsByAddress200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A potentially truncated list of transactions currently in the node\&#39;s transaction pool. You can compute whether or not the list is truncated if the number of elements in the **top-transactions** array is fewer than **total-transactions**. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getPendingTransactionsByAddress**
> GetPendingTransactionsByAddress200Response getPendingTransactionsByAddress()

Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetPendingTransactionsByAddressRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetPendingTransactionsByAddressRequest = {
    // An account public key
  address: "W8Q6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS9H",
    // Truncated number of transactions to display. If max=0, returns all pending txns. (optional)
  max: 1,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.getPendingTransactionsByAddress(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**string**] | An account public key | defaults to undefined
 **max** | [**number**] | Truncated number of transactions to display. If max&#x3D;0, returns all pending txns. | (optional) defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**GetPendingTransactionsByAddress200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/msgpack


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A potentially truncated list of transactions currently in the node\&#39;s transaction pool. You can compute whether or not the list is truncated if the number of elements in the **top-transactions** array is fewer than **total-transactions**. |  -  |
**400** | Max must be a non-negative integer |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getReady**
> void getReady()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getReady(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getStateProof**
> StateProof getStateProof()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetStateProofRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetStateProofRequest = {
    // The round for which a state proof is desired.
  round: 0,
};

const data = await apiInstance.getStateProof(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round for which a state proof is desired. | defaults to undefined


### Return type

**StateProof**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getStatus**
> GetStatus200Response getStatus()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getStatus(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**GetStatus200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getSupply**
> GetSupply200Response getSupply()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getSupply(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**GetSupply200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Supply represents the current supply of MicroAlgos in the system. |  -  |
**401** | Invalid API Token |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getSyncRound**
> GetSyncRound200Response getSyncRound()

Gets the minimum sync round for the ledger.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getSyncRound(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**GetSyncRound200Response**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Response containing the ledger\&#39;s minimum sync round |  -  |
**400** | Sync round not set. |  -  |
**401** | Invalid API Token |  -  |
**500** | Internal Error |  -  |
**503** | Service Temporarily Unavailable |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getTransactionGroupLedgerStateDeltasForRound**
> GetTransactionGroupLedgerStateDeltasForRound200Response getTransactionGroupLedgerStateDeltasForRound()

Get ledger deltas for transaction groups in a given round.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetTransactionGroupLedgerStateDeltasForRoundRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetTransactionGroupLedgerStateDeltasForRoundRequest = {
    // The round for which the deltas are desired.
  round: 0,
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.getTransactionGroupLedgerStateDeltasForRound(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round for which the deltas are desired. | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**GetTransactionGroupLedgerStateDeltasForRound200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getTransactionProof**
> GetTransactionProof200Response getTransactionProof()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiGetTransactionProofRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiGetTransactionProofRequest = {
    // The round in which the transaction appears.
  round: 1,
    // The transaction ID for which to generate a proof.
  txid: "8",
    // The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256 (optional)
  hashtype: "sha512_256",
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.getTransactionProof(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round in which the transaction appears. | defaults to undefined
 **txid** | [**string**] | The transaction ID for which to generate a proof. | defaults to undefined
 **hashtype** | [**&#39;sha512_256&#39; | &#39;sha256&#39;**]**Array<&#39;sha512_256&#39; &#124; &#39;sha256&#39;>** | The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256 | (optional) defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**GetTransactionProof200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **getVersion**
> Version getVersion()

Retrieves the supported API versions, binary build versions, and genesis information.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.getVersion(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**Version**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | VersionsResponse is the response to \&#39;GET /versions\&#39; |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **healthCheck**
> void healthCheck()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.healthCheck(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK. |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **metrics**
> void metrics()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.metrics(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | text with \\#-comments and key:value lines |  -  |
**404** | metrics were compiled out |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **pendingTransactionInformation**
> PendingTransactionResponse pendingTransactionInformation()

Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiPendingTransactionInformationRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiPendingTransactionInformationRequest = {
    // A transaction ID
  txid: "8",
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.pendingTransactionInformation(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **txid** | [**string**] | A transaction ID | defaults to undefined
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**PendingTransactionResponse**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **putDebugSettingsProf**
> DebugSettingsProf putDebugSettingsProf()

Enables blocking and mutex profiles, and returns the old settings

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.putDebugSettingsProf(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**DebugSettingsProf**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | DebugPprof is the response to the /debug/extra/pprof endpoint |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **rawTransaction**
> RawTransaction200Response rawTransaction(rawtxn)


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiRawTransactionRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiRawTransactionRequest = {
    // The byte encoded signed transaction to broadcast to network
  rawtxn: { data: Buffer.from(fs.readFileSync('/path/to/file', 'utf-8')), name: '/path/to/file' },
};

const data = await apiInstance.rawTransaction(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rawtxn** | **HttpFile**| The byte encoded signed transaction to broadcast to network |


### Return type

**RawTransaction200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **rawTransactionAsync**
> void rawTransactionAsync(rawtxn)


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiRawTransactionAsyncRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiRawTransactionAsyncRequest = {
    // The byte encoded signed transaction to broadcast to network
  rawtxn: { data: Buffer.from(fs.readFileSync('/path/to/file', 'utf-8')), name: '/path/to/file' },
};

const data = await apiInstance.rawTransactionAsync(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rawtxn** | **HttpFile**| The byte encoded signed transaction to broadcast to network |


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **setBlockTimeStampOffset**
> void setBlockTimeStampOffset()

Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiSetBlockTimeStampOffsetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiSetBlockTimeStampOffsetRequest = {
    // The timestamp offset for blocks in dev mode.
  offset: 0,
};

const data = await apiInstance.setBlockTimeStampOffset(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **offset** | [**number**] | The timestamp offset for blocks in dev mode. | defaults to undefined


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **setSyncRound**
> void setSyncRound()

Sets the minimum sync round on the ledger.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiSetSyncRoundRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiSetSyncRoundRequest = {
    // The round for which the deltas are desired.
  round: 0,
};

const data = await apiInstance.setSyncRound(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round for which the deltas are desired. | defaults to undefined


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **shutdownNode**
> any shutdownNode()

Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiShutdownNodeRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiShutdownNodeRequest = {
  
  timeout: 0,
};

const data = await apiInstance.shutdownNode(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timeout** | [**number**] |  | (optional) defaults to 0


### Return type

**any**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Node shutdown initiated successfully |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **simulateTransaction**
> SimulateTransaction200Response simulateTransaction(request)


### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiSimulateTransactionRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiSimulateTransactionRequest = {
    // The transactions to simulate, along with any other inputs.
  request: {
    txnGroups: [
      {
        txns: [
          "txns_example",
        ],
      },
    ],
    round: 1,
    allowEmptySignatures: true,
    allowMoreLogging: true,
    allowUnnamedResources: true,
    extraOpcodeBudget: 1,
    execTraceConfig: {
      enable: true,
      stackChange: true,
      scratchChange: true,
      stateChange: true,
    },
    fixSigners: true,
  },
    // Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. (optional)
  format: "json",
};

const data = await apiInstance.simulateTransaction(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request** | **SimulateRequest**| The transactions to simulate, along with any other inputs. |
 **format** | [**&#39;json&#39; | &#39;msgpack&#39;**]**Array<&#39;json&#39; &#124; &#39;msgpack&#39;>** | Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON. | (optional) defaults to undefined


### Return type

**SimulateTransaction200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **startCatchup**
> StartCatchup200Response startCatchup()

Given a catchpoint, it starts catching up to this catchpoint

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiStartCatchupRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiStartCatchupRequest = {
    // A catch point
  catchpoint: "4#6ZGCKEC0L3O4GI7XHK0JCY075UA034PGMUG8IQ37LLR1PLNJKTGOS",
    // Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed. (optional)
  min: 1,
};

const data = await apiInstance.startCatchup(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **catchpoint** | [**string**] | A catch point | defaults to undefined
 **min** | [**number**] | Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed. | (optional) defaults to undefined


### Return type

**StartCatchup200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **swaggerJSON**
> string swaggerJSON()

Returns the entire swagger spec in json.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.swaggerJSON(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**string**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The current swagger spec |  -  |
**0** | Unknown Error |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **tealCompile**
> TealCompile200Response tealCompile(source)

Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiTealCompileRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiTealCompileRequest = {
    // TEAL source code to be compiled
  source: { data: Buffer.from(fs.readFileSync('/path/to/file', 'utf-8')), name: '/path/to/file' },
    // When set to `true`, returns the source map of the program as a JSON. Defaults to `false`. (optional)
  sourcemap: true,
};

const data = await apiInstance.tealCompile(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source** | **HttpFile**| TEAL source code to be compiled |
 **sourcemap** | [**boolean**] | When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;. | (optional) defaults to undefined


### Return type

**TealCompile200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **tealDisassemble**
> TealDisassemble200Response tealDisassemble(source)

Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiTealDisassembleRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiTealDisassembleRequest = {
    // TEAL program binary to be disassembled
  source: 'YQ==',
};

const data = await apiInstance.tealDisassemble(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source** | **string**| TEAL program binary to be disassembled |


### Return type

**TealDisassemble200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **tealDryrun**
> TealDryrun200Response tealDryrun()

Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiTealDryrunRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiTealDryrunRequest = {
    // Transaction (or group) and any accompanying state-simulation data. (optional)
  request: {
    txns: [
      "txns_example",
    ],
    accounts: [
      {
        address: "address_example",
        amount: 1,
        minBalance: 1,
        amountWithoutPendingRewards: 1,
        appsLocalState: [
          {
            id: 1,
            schema: {
              numUint: 1,
              numByteSlice: 1,
            },
            keyValue: [
              {
                key: "key_example",
                value: {
                  type: 1,
                  bytes: "bytes_example",
                  uint: 1,
                },
              },
            ],
          },
        ],
        totalAppsOptedIn: 1,
        appsTotalSchema: {
          numUint: 1,
          numByteSlice: 1,
        },
        appsTotalExtraPages: 1,
        assets: [
          {
            amount: 1,
            assetId: 1,
            isFrozen: true,
          },
        ],
        totalAssetsOptedIn: 1,
        createdApps: [
          {
            id: 1,
            params: {
              creator: "creator_example",
              approvalProgram: 'YQ==',
              clearStateProgram: 'YQ==',
              extraProgramPages: 1,
              localStateSchema: {
                numUint: 1,
                numByteSlice: 1,
              },
              globalStateSchema: {
                numUint: 1,
                numByteSlice: 1,
              },
              globalState: [
                {
                  key: "key_example",
                  value: {
                    type: 1,
                    bytes: "bytes_example",
                    uint: 1,
                  },
                },
              ],
              version: 1,
            },
          },
        ],
        totalCreatedApps: 1,
        createdAssets: [
          {
            index: 1,
            params: {
              clawback: "clawback_example",
              creator: "creator_example",
              decimals: 0,
              defaultFrozen: true,
              freeze: "freeze_example",
              manager: "manager_example",
              metadataHash: 'YQ==',
              name: "name_example",
              nameB64: 'YQ==',
              reserve: "reserve_example",
              total: 1,
              unitName: "unitName_example",
              unitNameB64: 'YQ==',
              url: "url_example",
              urlB64: 'YQ==',
            },
          },
        ],
        totalCreatedAssets: 1,
        totalBoxes: 1,
        totalBoxBytes: 1,
        participation: {
          selectionParticipationKey: 'YQ==',
          voteFirstValid: 1,
          voteKeyDilution: 1,
          voteLastValid: 1,
          voteParticipationKey: 'YQ==',
          stateProofKey: 'YQ==',
        },
        incentiveEligible: true,
        pendingRewards: 1,
        rewardBase: 1,
        rewards: 1,
        round: 1,
        status: "status_example",
        sigType: "sig",
        authAddr: "authAddr_example",
        lastProposed: 1,
        lastHeartbeat: 1,
      },
    ],
    apps: [
      {
        id: 1,
        params: {
          creator: "creator_example",
          approvalProgram: 'YQ==',
          clearStateProgram: 'YQ==',
          extraProgramPages: 1,
          localStateSchema: {
            numUint: 1,
            numByteSlice: 1,
          },
          globalStateSchema: {
            numUint: 1,
            numByteSlice: 1,
          },
          globalState: [
            {
              key: "key_example",
              value: {
                type: 1,
                bytes: "bytes_example",
                uint: 1,
              },
            },
          ],
          version: 1,
        },
      },
    ],
    protocolVersion: "protocolVersion_example",
    round: 1,
    latestTimestamp: 1,
    sources: [
      {
        fieldName: "fieldName_example",
        source: "source_example",
        txnIndex: 1,
        appIndex: 1,
      },
    ],
  },
};

const data = await apiInstance.tealDryrun(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request** | **DryrunRequest**| Transaction (or group) and any accompanying state-simulation data. |


### Return type

**TealDryrun200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **transactionParams**
> TransactionParams200Response transactionParams()


### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.transactionParams(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**TransactionParams200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **unsetSyncRound**
> void unsetSyncRound()

Unset the ledger sync round.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request = {};

const data = await apiInstance.unsetSyncRound(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**void**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **waitForBlock**
> GetStatus200Response waitForBlock()

Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.

### Example


```typescript
import { createConfiguration, AlgodApi } from '';
import type { AlgodApiWaitForBlockRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AlgodApi(configuration);

const request: AlgodApiWaitForBlockRequest = {
    // The round to wait until returning status
  round: 0,
};

const data = await apiInstance.waitForBlock(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **round** | [**number**] | The round to wait until returning status | defaults to undefined


### Return type

**GetStatus200Response**

### Authorization

[api_key](README.md#api_key)

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

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)


