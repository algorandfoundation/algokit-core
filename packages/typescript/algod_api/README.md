## @algorandfoundation/algokit-algod-api@0.0.1

This generator creates TypeScript/JavaScript client that utilizes fetch-api.

### Building

To build and compile the typescript sources to javascript use:
```
npm install
npm run build
```

### Publishing

First build the package then run ```npm publish```

### Consuming

Navigate to the folder of your consuming project and run one of the following commands.

_published:_

```bash
npm install @algorandfoundation/algokit-algod-api@0.0.1 --save
```

_unPublished (not recommended):_

```bash
npm install PATH_TO_GENERATED_PACKAGE --save
```

### Usage

Below code snippet shows exemplary usage of the configuration and the API based 
on the typical `PetStore` example used for OpenAPI. 

```typescript
import * as algodApi from '@algorandfoundation/algokit-algod-api'

// Covers all auth methods included in your OpenAPI yaml definition
const authConfig: algodApi.AuthMethodsConfiguration = {
    "api_key": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
}

// Create configuration parameter object
const serverConfig = new algodApi.ServerConfiguration('http://localhost:4001', {})
const configurationParameters = {
    httpApi: new algodApi.IsomorphicFetchHttpLibrary(), // Can also be ignored - default is usually fine
    baseServer: serverConfig, // First server is default
    authMethods: authConfig, // No auth is default
    promiseMiddleware: [],
}

// Convert to actual configuration
const config = algodApi.createConfiguration(configurationParameters);

// Use configuration with your_api
const api = new algodApi.AlgodApi(config);
const response = await api.transactionParams()
console.log(response?.lastRound)
```

### Testing

To run the tests, use the following command:

```bash
npm run test
```
