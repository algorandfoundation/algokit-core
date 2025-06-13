// TODO: better import syntax?
import {BaseAPIRequestFactory, RequiredError, COLLECTION_FORMATS} from './baseapi';
import {Configuration} from '../configuration';
import {RequestContext, HttpMethod, ResponseContext, HttpFile} from '../http/http';
import {ObjectSerializer} from '../models/ObjectSerializer';
import {ApiException} from './exception';
import {canConsumeForm, isCodeInRange} from '../util';
import {SecurityAuthentication} from '../auth/auth';


import { AbortCatchup200Response } from '../models/AbortCatchup200Response';
import { Account } from '../models/Account';
import { AccountApplicationInformation200Response } from '../models/AccountApplicationInformation200Response';
import { AccountAssetInformation200Response } from '../models/AccountAssetInformation200Response';
import { AccountAssetsInformation200Response } from '../models/AccountAssetsInformation200Response';
import { AddParticipationKey200Response } from '../models/AddParticipationKey200Response';
import { Application } from '../models/Application';
import { Asset } from '../models/Asset';
import { Box } from '../models/Box';
import { DebugSettingsProf } from '../models/DebugSettingsProf';
import { DryrunRequest } from '../models/DryrunRequest';
import { ErrorResponse } from '../models/ErrorResponse';
import { Genesis } from '../models/Genesis';
import { GetApplicationBoxes200Response } from '../models/GetApplicationBoxes200Response';
import { GetBlock200Response } from '../models/GetBlock200Response';
import { GetBlockHash200Response } from '../models/GetBlockHash200Response';
import { GetBlockLogs200Response } from '../models/GetBlockLogs200Response';
import { GetBlockTimeStampOffset200Response } from '../models/GetBlockTimeStampOffset200Response';
import { GetBlockTxids200Response } from '../models/GetBlockTxids200Response';
import { GetPendingTransactionsByAddress200Response } from '../models/GetPendingTransactionsByAddress200Response';
import { GetStatus200Response } from '../models/GetStatus200Response';
import { GetSupply200Response } from '../models/GetSupply200Response';
import { GetSyncRound200Response } from '../models/GetSyncRound200Response';
import { GetTransactionGroupLedgerStateDeltasForRound200Response } from '../models/GetTransactionGroupLedgerStateDeltasForRound200Response';
import { GetTransactionProof200Response } from '../models/GetTransactionProof200Response';
import { LightBlockHeaderProof } from '../models/LightBlockHeaderProof';
import { ParticipationKey } from '../models/ParticipationKey';
import { PendingTransactionResponse } from '../models/PendingTransactionResponse';
import { RawTransaction200Response } from '../models/RawTransaction200Response';
import { SimulateRequest } from '../models/SimulateRequest';
import { SimulateTransaction200Response } from '../models/SimulateTransaction200Response';
import { StartCatchup200Response } from '../models/StartCatchup200Response';
import { StateProof } from '../models/StateProof';
import { TealCompile200Response } from '../models/TealCompile200Response';
import { TealDisassemble200Response } from '../models/TealDisassemble200Response';
import { TealDryrun200Response } from '../models/TealDryrun200Response';
import { TransactionParams200Response } from '../models/TransactionParams200Response';
import { Version } from '../models/Version';

/**
 * no description
 */
export class AlgodApiRequestFactory extends BaseAPIRequestFactory {

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param catchpoint A catch point
     */
    public async abortCatchup(catchpoint: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'catchpoint' is not null or undefined
        if (catchpoint === null || catchpoint === undefined) {
            throw new RequiredError("AlgodApi", "abortCatchup", "catchpoint");
        }


        // Path Params
        const localVarPath = '/v2/catchup/{catchpoint}'
            .replace('{' + 'catchpoint' + '}', encodeURIComponent(String(catchpoint)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param address An account public key
     * @param applicationId An application identifier
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async accountApplicationInformation(address: string, applicationId: number, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'address' is not null or undefined
        if (address === null || address === undefined) {
            throw new RequiredError("AlgodApi", "accountApplicationInformation", "address");
        }


        // verify required parameter 'applicationId' is not null or undefined
        if (applicationId === null || applicationId === undefined) {
            throw new RequiredError("AlgodApi", "accountApplicationInformation", "applicationId");
        }



        // Path Params
        const localVarPath = '/v2/accounts/{address}/applications/{application-id}'
            .replace('{' + 'address' + '}', encodeURIComponent(String(address)))
            .replace('{' + 'application-id' + '}', encodeURIComponent(String(applicationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param address An account public key
     * @param assetId An asset identifier
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async accountAssetInformation(address: string, assetId: number, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'address' is not null or undefined
        if (address === null || address === undefined) {
            throw new RequiredError("AlgodApi", "accountAssetInformation", "address");
        }


        // verify required parameter 'assetId' is not null or undefined
        if (assetId === null || assetId === undefined) {
            throw new RequiredError("AlgodApi", "accountAssetInformation", "assetId");
        }



        // Path Params
        const localVarPath = '/v2/accounts/{address}/assets/{asset-id}'
            .replace('{' + 'address' + '}', encodeURIComponent(String(address)))
            .replace('{' + 'asset-id' + '}', encodeURIComponent(String(assetId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param address An account public key
     * @param limit Maximum number of results to return.
     * @param next The next page of results. Use the next token provided by the previous results.
     */
    public async accountAssetsInformation(address: string, limit?: number, next?: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'address' is not null or undefined
        if (address === null || address === undefined) {
            throw new RequiredError("AlgodApi", "accountAssetsInformation", "address");
        }




        // Path Params
        const localVarPath = '/v2/accounts/{address}/assets'
            .replace('{' + 'address' + '}', encodeURIComponent(String(address)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (limit !== undefined) {
            requestContext.setQueryParam("limit", ObjectSerializer.serialize(limit, "number", ""));
        }

        // Query Params
        if (next !== undefined) {
            requestContext.setQueryParam("next", ObjectSerializer.serialize(next, "string", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param address An account public key
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param exclude When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;.
     */
    public async accountInformation(address: string, format?: 'json' | 'msgpack', exclude?: 'all' | 'none', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'address' is not null or undefined
        if (address === null || address === undefined) {
            throw new RequiredError("AlgodApi", "accountInformation", "address");
        }




        // Path Params
        const localVarPath = '/v2/accounts/{address}'
            .replace('{' + 'address' + '}', encodeURIComponent(String(address)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }

        // Query Params
        if (exclude !== undefined) {
            requestContext.setQueryParam("exclude", ObjectSerializer.serialize(exclude, "'all' | 'none'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Add a participation key to the node
     * @param participationkey The participation key to add to the node
     */
    public async addParticipationKey(participationkey: HttpFile, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'participationkey' is not null or undefined
        if (participationkey === null || participationkey === undefined) {
            throw new RequiredError("AlgodApi", "addParticipationKey", "participationkey");
        }


        // Path Params
        const localVarPath = '/v2/participation';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/msgpack"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(participationkey, "HttpFile", ""),
            contentType,
            "HttpFile"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param participationId 
     * @param keymap The state proof keys to add to an existing participation ID
     */
    public async appendKeys(participationId: string, keymap: HttpFile, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'participationId' is not null or undefined
        if (participationId === null || participationId === undefined) {
            throw new RequiredError("AlgodApi", "appendKeys", "participationId");
        }


        // verify required parameter 'keymap' is not null or undefined
        if (keymap === null || keymap === undefined) {
            throw new RequiredError("AlgodApi", "appendKeys", "keymap");
        }


        // Path Params
        const localVarPath = '/v2/participation/{participation-id}'
            .replace('{' + 'participation-id' + '}', encodeURIComponent(String(participationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/msgpack"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(keymap, "HttpFile", ""),
            contentType,
            "HttpFile"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param participationId 
     */
    public async deleteParticipationKeyByID(participationId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'participationId' is not null or undefined
        if (participationId === null || participationId === undefined) {
            throw new RequiredError("AlgodApi", "deleteParticipationKeyByID", "participationId");
        }


        // Path Params
        const localVarPath = '/v2/participation/{participation-id}'
            .replace('{' + 'participation-id' + '}', encodeURIComponent(String(participationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Returns OK if experimental API is enabled.
     */
    public async experimentalCheck(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/experimental';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Generate and install participation keys to the node.
     * @param address An account public key
     * @param first First round for participation key.
     * @param last Last round for participation key.
     * @param dilution Key dilution for two-level participation keys (defaults to sqrt of validity window).
     */
    public async generateParticipationKeys(address: string, first: number, last: number, dilution?: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'address' is not null or undefined
        if (address === null || address === undefined) {
            throw new RequiredError("AlgodApi", "generateParticipationKeys", "address");
        }


        // verify required parameter 'first' is not null or undefined
        if (first === null || first === undefined) {
            throw new RequiredError("AlgodApi", "generateParticipationKeys", "first");
        }


        // verify required parameter 'last' is not null or undefined
        if (last === null || last === undefined) {
            throw new RequiredError("AlgodApi", "generateParticipationKeys", "last");
        }



        // Path Params
        const localVarPath = '/v2/participation/generate/{address}'
            .replace('{' + 'address' + '}', encodeURIComponent(String(address)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (dilution !== undefined) {
            requestContext.setQueryParam("dilution", ObjectSerializer.serialize(dilution, "number", ""));
        }

        // Query Params
        if (first !== undefined) {
            requestContext.setQueryParam("first", ObjectSerializer.serialize(first, "number", ""));
        }

        // Query Params
        if (last !== undefined) {
            requestContext.setQueryParam("last", ObjectSerializer.serialize(last, "number", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param applicationId An application identifier
     * @param name A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;.
     */
    public async getApplicationBoxByName(applicationId: number, name: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'applicationId' is not null or undefined
        if (applicationId === null || applicationId === undefined) {
            throw new RequiredError("AlgodApi", "getApplicationBoxByName", "applicationId");
        }


        // verify required parameter 'name' is not null or undefined
        if (name === null || name === undefined) {
            throw new RequiredError("AlgodApi", "getApplicationBoxByName", "name");
        }


        // Path Params
        const localVarPath = '/v2/applications/{application-id}/box'
            .replace('{' + 'application-id' + '}', encodeURIComponent(String(applicationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (name !== undefined) {
            requestContext.setQueryParam("name", ObjectSerializer.serialize(name, "string", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param applicationId An application identifier
     * @param max Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names.
     */
    public async getApplicationBoxes(applicationId: number, max?: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'applicationId' is not null or undefined
        if (applicationId === null || applicationId === undefined) {
            throw new RequiredError("AlgodApi", "getApplicationBoxes", "applicationId");
        }



        // Path Params
        const localVarPath = '/v2/applications/{application-id}/boxes'
            .replace('{' + 'application-id' + '}', encodeURIComponent(String(applicationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (max !== undefined) {
            requestContext.setQueryParam("max", ObjectSerializer.serialize(max, "number", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param applicationId An application identifier
     */
    public async getApplicationByID(applicationId: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'applicationId' is not null or undefined
        if (applicationId === null || applicationId === undefined) {
            throw new RequiredError("AlgodApi", "getApplicationByID", "applicationId");
        }


        // Path Params
        const localVarPath = '/v2/applications/{application-id}'
            .replace('{' + 'application-id' + '}', encodeURIComponent(String(applicationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param assetId An asset identifier
     */
    public async getAssetByID(assetId: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'assetId' is not null or undefined
        if (assetId === null || assetId === undefined) {
            throw new RequiredError("AlgodApi", "getAssetByID", "assetId");
        }


        // Path Params
        const localVarPath = '/v2/assets/{asset-id}'
            .replace('{' + 'asset-id' + '}', encodeURIComponent(String(assetId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the block for the given round.
     * @param round The round from which to fetch block information.
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param headerOnly If true, only the block header (exclusive of payset or certificate) may be included in response.
     */
    public async getBlock(round: number, format?: 'json' | 'msgpack', headerOnly?: boolean, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getBlock", "round");
        }




        // Path Params
        const localVarPath = '/v2/blocks/{round}'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }

        // Query Params
        if (headerOnly !== undefined) {
            requestContext.setQueryParam("header-only", ObjectSerializer.serialize(headerOnly, "boolean", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the block hash for the block on the given round.
     * @param round The round from which to fetch block hash information.
     */
    public async getBlockHash(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getBlockHash", "round");
        }


        // Path Params
        const localVarPath = '/v2/blocks/{round}/hash'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param round The round from which to fetch block log information.
     */
    public async getBlockLogs(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getBlockLogs", "round");
        }


        // Path Params
        const localVarPath = '/v2/blocks/{round}/logs'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     */
    public async getBlockTimeStampOffset(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/devmode/blocks/offset';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param round The round from which to fetch block transaction IDs.
     */
    public async getBlockTxids(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getBlockTxids", "round");
        }


        // Path Params
        const localVarPath = '/v2/blocks/{round}/txids'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     */
    public async getConfig(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/debug/settings/config';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     */
    public async getDebugSettingsProf(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/debug/settings/pprof';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     */
    public async getGenesis(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/genesis';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param round The round for which the deltas are desired.
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async getLedgerStateDelta(round: number, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getLedgerStateDelta", "round");
        }



        // Path Params
        const localVarPath = '/v2/deltas/{round}'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param id A transaction ID, or transaction group ID
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async getLedgerStateDeltaForTransactionGroup(id: string, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("AlgodApi", "getLedgerStateDeltaForTransactionGroup", "id");
        }



        // Path Params
        const localVarPath = '/v2/deltas/txn/group/{id}'
            .replace('{' + 'id' + '}', encodeURIComponent(String(id)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param round The round to which the light block header belongs.
     */
    public async getLightBlockHeaderProof(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getLightBlockHeaderProof", "round");
        }


        // Path Params
        const localVarPath = '/v2/blocks/{round}/lightheader/proof'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param participationId 
     */
    public async getParticipationKeyByID(participationId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'participationId' is not null or undefined
        if (participationId === null || participationId === undefined) {
            throw new RequiredError("AlgodApi", "getParticipationKeyByID", "participationId");
        }


        // Path Params
        const localVarPath = '/v2/participation/{participation-id}'
            .replace('{' + 'participation-id' + '}', encodeURIComponent(String(participationId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     */
    public async getParticipationKeys(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/participation';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param max Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async getPendingTransactions(max?: number, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;



        // Path Params
        const localVarPath = '/v2/transactions/pending';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (max !== undefined) {
            requestContext.setQueryParam("max", ObjectSerializer.serialize(max, "number", ""));
        }

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param address An account public key
     * @param max Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async getPendingTransactionsByAddress(address: string, max?: number, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'address' is not null or undefined
        if (address === null || address === undefined) {
            throw new RequiredError("AlgodApi", "getPendingTransactionsByAddress", "address");
        }




        // Path Params
        const localVarPath = '/v2/accounts/{address}/transactions/pending'
            .replace('{' + 'address' + '}', encodeURIComponent(String(address)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (max !== undefined) {
            requestContext.setQueryParam("max", ObjectSerializer.serialize(max, "number", ""));
        }

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Returns OK if healthy and fully caught up.
     */
    public async getReady(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/ready';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get a state proof that covers a given round
     * @param round The round for which a state proof is desired.
     */
    public async getStateProof(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getStateProof", "round");
        }


        // Path Params
        const localVarPath = '/v2/stateproofs/{round}'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Gets the current node status.
     */
    public async getStatus(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/status';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the current supply reported by the ledger.
     */
    public async getSupply(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/ledger/supply';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     */
    public async getSyncRound(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/ledger/sync';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param round The round for which the deltas are desired.
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async getTransactionGroupLedgerStateDeltasForRound(round: number, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getTransactionGroupLedgerStateDeltasForRound", "round");
        }



        // Path Params
        const localVarPath = '/v2/deltas/{round}/txn/group'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get a proof for a transaction in a block.
     * @param round The round in which the transaction appears.
     * @param txid The transaction ID for which to generate a proof.
     * @param hashtype The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async getTransactionProof(round: number, txid: string, hashtype?: 'sha512_256' | 'sha256', format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "getTransactionProof", "round");
        }


        // verify required parameter 'txid' is not null or undefined
        if (txid === null || txid === undefined) {
            throw new RequiredError("AlgodApi", "getTransactionProof", "txid");
        }




        // Path Params
        const localVarPath = '/v2/blocks/{round}/transactions/{txid}/proof'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)))
            .replace('{' + 'txid' + '}', encodeURIComponent(String(txid)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (hashtype !== undefined) {
            requestContext.setQueryParam("hashtype", ObjectSerializer.serialize(hashtype, "'sha512_256' | 'sha256'", ""));
        }

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     */
    public async getVersion(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/versions';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Returns OK if healthy.
     */
    public async healthCheck(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/health';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Return metrics about algod functioning.
     */
    public async metrics(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/metrics';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param txid A transaction ID
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async pendingTransactionInformation(txid: string, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'txid' is not null or undefined
        if (txid === null || txid === undefined) {
            throw new RequiredError("AlgodApi", "pendingTransactionInformation", "txid");
        }



        // Path Params
        const localVarPath = '/v2/transactions/pending/{txid}'
            .replace('{' + 'txid' + '}', encodeURIComponent(String(txid)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     */
    public async putDebugSettingsProf(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/debug/settings/pprof';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.PUT);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public async rawTransaction(rawtxn: HttpFile, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'rawtxn' is not null or undefined
        if (rawtxn === null || rawtxn === undefined) {
            throw new RequiredError("AlgodApi", "rawTransaction", "rawtxn");
        }


        // Path Params
        const localVarPath = '/v2/transactions';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/x-binary"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(rawtxn, "HttpFile", ""),
            contentType,
            "HttpFile"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public async rawTransactionAsync(rawtxn: HttpFile, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'rawtxn' is not null or undefined
        if (rawtxn === null || rawtxn === undefined) {
            throw new RequiredError("AlgodApi", "rawTransactionAsync", "rawtxn");
        }


        // Path Params
        const localVarPath = '/v2/transactions/async';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/x-binary"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(rawtxn, "HttpFile", ""),
            contentType,
            "HttpFile"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param offset The timestamp offset for blocks in dev mode.
     */
    public async setBlockTimeStampOffset(offset: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'offset' is not null or undefined
        if (offset === null || offset === undefined) {
            throw new RequiredError("AlgodApi", "setBlockTimeStampOffset", "offset");
        }


        // Path Params
        const localVarPath = '/v2/devmode/blocks/offset/{offset}'
            .replace('{' + 'offset' + '}', encodeURIComponent(String(offset)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param round The round for which the deltas are desired.
     */
    public async setSyncRound(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "setSyncRound", "round");
        }


        // Path Params
        const localVarPath = '/v2/ledger/sync/{round}'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param timeout 
     */
    public async shutdownNode(timeout?: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;


        // Path Params
        const localVarPath = '/v2/shutdown';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (timeout !== undefined) {
            requestContext.setQueryParam("timeout", ObjectSerializer.serialize(timeout, "number", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param request The transactions to simulate, along with any other inputs.
     * @param format Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public async simulateTransaction(request: SimulateRequest, format?: 'json' | 'msgpack', _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'request' is not null or undefined
        if (request === null || request === undefined) {
            throw new RequiredError("AlgodApi", "simulateTransaction", "request");
        }



        // Path Params
        const localVarPath = '/v2/transactions/simulate';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (format !== undefined) {
            requestContext.setQueryParam("format", ObjectSerializer.serialize(format, "'json' | 'msgpack'", ""));
        }


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json",
        
            "application/msgpack"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(request, "SimulateRequest", ""),
            contentType,
            "SimulateRequest"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param catchpoint A catch point
     * @param min Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed.
     */
    public async startCatchup(catchpoint: string, min?: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'catchpoint' is not null or undefined
        if (catchpoint === null || catchpoint === undefined) {
            throw new RequiredError("AlgodApi", "startCatchup", "catchpoint");
        }



        // Path Params
        const localVarPath = '/v2/catchup/{catchpoint}'
            .replace('{' + 'catchpoint' + '}', encodeURIComponent(String(catchpoint)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (min !== undefined) {
            requestContext.setQueryParam("min", ObjectSerializer.serialize(min, "number", ""));
        }


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     */
    public async swaggerJSON(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/swagger.json';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param source TEAL source code to be compiled
     * @param sourcemap When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;.
     */
    public async tealCompile(source: HttpFile, sourcemap?: boolean, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'source' is not null or undefined
        if (source === null || source === undefined) {
            throw new RequiredError("AlgodApi", "tealCompile", "source");
        }



        // Path Params
        const localVarPath = '/v2/teal/compile';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (sourcemap !== undefined) {
            requestContext.setQueryParam("sourcemap", ObjectSerializer.serialize(sourcemap, "boolean", ""));
        }


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "text/plain"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(source, "HttpFile", ""),
            contentType,
            "HttpFile"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param source TEAL program binary to be disassembled
     */
    public async tealDisassemble(source: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'source' is not null or undefined
        if (source === null || source === undefined) {
            throw new RequiredError("AlgodApi", "tealDisassemble", "source");
        }


        // Path Params
        const localVarPath = '/v2/teal/disassemble';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/x-binary"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(source, "string", ""),
            contentType,
            "string"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param request Transaction (or group) and any accompanying state-simulation data.
     */
    public async tealDryrun(request?: DryrunRequest, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;


        // Path Params
        const localVarPath = '/v2/teal/dryrun';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json",
        
            "application/msgpack"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(request, "DryrunRequest", ""),
            contentType,
            "DryrunRequest"
        );
        requestContext.setBody(serializedBody);

        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get parameters for constructing a new transaction
     */
    public async transactionParams(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/transactions/params';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     */
    public async unsetSyncRound(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/v2/ledger/sync';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param round The round to wait until returning status
     */
    public async waitForBlock(round: number, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'round' is not null or undefined
        if (round === null || round === undefined) {
            throw new RequiredError("AlgodApi", "waitForBlock", "round");
        }


        // Path Params
        const localVarPath = '/v2/status/wait-for-block-after/{round}'
            .replace('{' + 'round' + '}', encodeURIComponent(String(round)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        let authMethod: SecurityAuthentication | undefined;
        // Apply auth methods
        authMethod = _config.authMethods["api_key"]
        if (authMethod?.applySecurityAuthentication) {
            await authMethod?.applySecurityAuthentication(requestContext);
        }
        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

}

export class AlgodApiResponseProcessor {

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to abortCatchup
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async abortCatchupResponse(response: ResponseContext): Promise<AbortCatchup200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: AbortCatchup200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AbortCatchup200Response", contentType),
                "AbortCatchup200Response", ""
            ) as AbortCatchup200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: AbortCatchup200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AbortCatchup200Response", contentType),
                "AbortCatchup200Response", ""
            ) as AbortCatchup200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to accountApplicationInformation
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async accountApplicationInformationResponse(response: ResponseContext): Promise<AccountApplicationInformation200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: AccountApplicationInformation200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AccountApplicationInformation200Response", contentType),
                "AccountApplicationInformation200Response", ""
            ) as AccountApplicationInformation200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Malformed address or application ID", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: AccountApplicationInformation200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AccountApplicationInformation200Response", contentType),
                "AccountApplicationInformation200Response", ""
            ) as AccountApplicationInformation200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to accountAssetInformation
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async accountAssetInformationResponse(response: ResponseContext): Promise<AccountAssetInformation200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: AccountAssetInformation200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AccountAssetInformation200Response", contentType),
                "AccountAssetInformation200Response", ""
            ) as AccountAssetInformation200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Malformed address or asset ID", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: AccountAssetInformation200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AccountAssetInformation200Response", contentType),
                "AccountAssetInformation200Response", ""
            ) as AccountAssetInformation200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to accountAssetsInformation
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async accountAssetsInformationResponse(response: ResponseContext): Promise<AccountAssetsInformation200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: AccountAssetsInformation200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AccountAssetsInformation200Response", contentType),
                "AccountAssetsInformation200Response", ""
            ) as AccountAssetsInformation200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Malformed address", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: AccountAssetsInformation200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AccountAssetsInformation200Response", contentType),
                "AccountAssetsInformation200Response", ""
            ) as AccountAssetsInformation200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to accountInformation
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async accountInformationResponse(response: ResponseContext): Promise<Account > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Account = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Account", contentType),
                "Account", ""
            ) as Account;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Account = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Account", contentType),
                "Account", ""
            ) as Account;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to addParticipationKey
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async addParticipationKeyResponse(response: ResponseContext): Promise<AddParticipationKey200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: AddParticipationKey200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AddParticipationKey200Response", contentType),
                "AddParticipationKey200Response", ""
            ) as AddParticipationKey200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Participation Key Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: AddParticipationKey200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "AddParticipationKey200Response", contentType),
                "AddParticipationKey200Response", ""
            ) as AddParticipationKey200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to appendKeys
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async appendKeysResponse(response: ResponseContext): Promise<ParticipationKey > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: ParticipationKey = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ParticipationKey", contentType),
                "ParticipationKey", ""
            ) as ParticipationKey;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Participation Key Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: ParticipationKey = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ParticipationKey", contentType),
                "ParticipationKey", ""
            ) as ParticipationKey;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to deleteParticipationKeyByID
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async deleteParticipationKeyByIDResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Participation Key Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to experimentalCheck
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async experimentalCheckResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Experimental API not enabled", undefined, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to generateParticipationKeys
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async generateParticipationKeysResponse(response: ResponseContext): Promise<string > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getApplicationBoxByName
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getApplicationBoxByNameResponse(response: ResponseContext): Promise<Box > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Box = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Box", contentType),
                "Box", ""
            ) as Box;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Box Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Box = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Box", contentType),
                "Box", ""
            ) as Box;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getApplicationBoxes
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getApplicationBoxesResponse(response: ResponseContext): Promise<GetApplicationBoxes200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetApplicationBoxes200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetApplicationBoxes200Response", contentType),
                "GetApplicationBoxes200Response", ""
            ) as GetApplicationBoxes200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetApplicationBoxes200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetApplicationBoxes200Response", contentType),
                "GetApplicationBoxes200Response", ""
            ) as GetApplicationBoxes200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getApplicationByID
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getApplicationByIDResponse(response: ResponseContext): Promise<Application > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Application = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Application", contentType),
                "Application", ""
            ) as Application;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Application Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Application = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Application", contentType),
                "Application", ""
            ) as Application;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getAssetByID
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getAssetByIDResponse(response: ResponseContext): Promise<Asset > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Asset = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Asset", contentType),
                "Asset", ""
            ) as Asset;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Application Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Asset = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Asset", contentType),
                "Asset", ""
            ) as Asset;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getBlock
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getBlockResponse(response: ResponseContext): Promise<GetBlock200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetBlock200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlock200Response", contentType),
                "GetBlock200Response", ""
            ) as GetBlock200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Non integer number", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "None existing block ", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetBlock200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlock200Response", contentType),
                "GetBlock200Response", ""
            ) as GetBlock200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getBlockHash
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getBlockHashResponse(response: ResponseContext): Promise<GetBlockHash200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetBlockHash200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockHash200Response", contentType),
                "GetBlockHash200Response", ""
            ) as GetBlockHash200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Non integer number", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "None existing block ", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetBlockHash200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockHash200Response", contentType),
                "GetBlockHash200Response", ""
            ) as GetBlockHash200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getBlockLogs
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getBlockLogsResponse(response: ResponseContext): Promise<GetBlockLogs200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetBlockLogs200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockLogs200Response", contentType),
                "GetBlockLogs200Response", ""
            ) as GetBlockLogs200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Non integer number", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Nonexistent block ", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetBlockLogs200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockLogs200Response", contentType),
                "GetBlockLogs200Response", ""
            ) as GetBlockLogs200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getBlockTimeStampOffset
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getBlockTimeStampOffsetResponse(response: ResponseContext): Promise<GetBlockTimeStampOffset200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetBlockTimeStampOffset200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockTimeStampOffset200Response", contentType),
                "GetBlockTimeStampOffset200Response", ""
            ) as GetBlockTimeStampOffset200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "TimeStamp offset not set.", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetBlockTimeStampOffset200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockTimeStampOffset200Response", contentType),
                "GetBlockTimeStampOffset200Response", ""
            ) as GetBlockTimeStampOffset200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getBlockTxids
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getBlockTxidsResponse(response: ResponseContext): Promise<GetBlockTxids200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetBlockTxids200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockTxids200Response", contentType),
                "GetBlockTxids200Response", ""
            ) as GetBlockTxids200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Non integer number", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Non existing block", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetBlockTxids200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetBlockTxids200Response", contentType),
                "GetBlockTxids200Response", ""
            ) as GetBlockTxids200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getConfig
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getConfigResponse(response: ResponseContext): Promise<string > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            return body;
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getDebugSettingsProf
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getDebugSettingsProfResponse(response: ResponseContext): Promise<DebugSettingsProf > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: DebugSettingsProf = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "DebugSettingsProf", contentType),
                "DebugSettingsProf", ""
            ) as DebugSettingsProf;
            return body;
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: DebugSettingsProf = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "DebugSettingsProf", contentType),
                "DebugSettingsProf", ""
            ) as DebugSettingsProf;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getGenesis
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getGenesisResponse(response: ResponseContext): Promise<Genesis > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Genesis = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Genesis", contentType),
                "Genesis", ""
            ) as Genesis;
            return body;
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Genesis = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Genesis", contentType),
                "Genesis", ""
            ) as Genesis;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getLedgerStateDelta
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getLedgerStateDeltaResponse(response: ResponseContext): Promise<any > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: any = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "any", contentType),
                "any", ""
            ) as any;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Could not find a delta for round", body, response.headers);
        }
        if (isCodeInRange("408", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "timed out on request", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: any = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "any", contentType),
                "any", ""
            ) as any;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getLedgerStateDeltaForTransactionGroup
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getLedgerStateDeltaForTransactionGroupResponse(response: ResponseContext): Promise<any > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: any = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "any", contentType),
                "any", ""
            ) as any;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Could not find a delta for transaction ID or group ID", body, response.headers);
        }
        if (isCodeInRange("408", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "timed out on request", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("501", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Not Implemented", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: any = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "any", contentType),
                "any", ""
            ) as any;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getLightBlockHeaderProof
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getLightBlockHeaderProofResponse(response: ResponseContext): Promise<LightBlockHeaderProof > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: LightBlockHeaderProof = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "LightBlockHeaderProof", contentType),
                "LightBlockHeaderProof", ""
            ) as LightBlockHeaderProof;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Could not create proof since some data is missing", body, response.headers);
        }
        if (isCodeInRange("408", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "timed out on request", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: LightBlockHeaderProof = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "LightBlockHeaderProof", contentType),
                "LightBlockHeaderProof", ""
            ) as LightBlockHeaderProof;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getParticipationKeyByID
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getParticipationKeyByIDResponse(response: ResponseContext): Promise<ParticipationKey > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: ParticipationKey = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ParticipationKey", contentType),
                "ParticipationKey", ""
            ) as ParticipationKey;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Participation Key Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: ParticipationKey = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ParticipationKey", contentType),
                "ParticipationKey", ""
            ) as ParticipationKey;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getParticipationKeys
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getParticipationKeysResponse(response: ResponseContext): Promise<Array<ParticipationKey> > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<ParticipationKey> = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Array<ParticipationKey>", contentType),
                "Array<ParticipationKey>", ""
            ) as Array<ParticipationKey>;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Participation Key Not Found", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Array<ParticipationKey> = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Array<ParticipationKey>", contentType),
                "Array<ParticipationKey>", ""
            ) as Array<ParticipationKey>;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getPendingTransactions
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getPendingTransactionsResponse(response: ResponseContext): Promise<GetPendingTransactionsByAddress200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetPendingTransactionsByAddress200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetPendingTransactionsByAddress200Response", contentType),
                "GetPendingTransactionsByAddress200Response", ""
            ) as GetPendingTransactionsByAddress200Response;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetPendingTransactionsByAddress200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetPendingTransactionsByAddress200Response", contentType),
                "GetPendingTransactionsByAddress200Response", ""
            ) as GetPendingTransactionsByAddress200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getPendingTransactionsByAddress
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getPendingTransactionsByAddressResponse(response: ResponseContext): Promise<GetPendingTransactionsByAddress200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetPendingTransactionsByAddress200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetPendingTransactionsByAddress200Response", contentType),
                "GetPendingTransactionsByAddress200Response", ""
            ) as GetPendingTransactionsByAddress200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Max must be a non-negative integer", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetPendingTransactionsByAddress200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetPendingTransactionsByAddress200Response", contentType),
                "GetPendingTransactionsByAddress200Response", ""
            ) as GetPendingTransactionsByAddress200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getReady
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getReadyResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Internal Error", undefined, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Node not ready yet", undefined, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getStateProof
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getStateProofResponse(response: ResponseContext): Promise<StateProof > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: StateProof = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "StateProof", contentType),
                "StateProof", ""
            ) as StateProof;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Could not find a state proof that covers a given round", body, response.headers);
        }
        if (isCodeInRange("408", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "timed out on request", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: StateProof = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "StateProof", contentType),
                "StateProof", ""
            ) as StateProof;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getStatus
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getStatusResponse(response: ResponseContext): Promise<GetStatus200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetStatus200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetStatus200Response", contentType),
                "GetStatus200Response", ""
            ) as GetStatus200Response;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetStatus200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetStatus200Response", contentType),
                "GetStatus200Response", ""
            ) as GetStatus200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getSupply
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getSupplyResponse(response: ResponseContext): Promise<GetSupply200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetSupply200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetSupply200Response", contentType),
                "GetSupply200Response", ""
            ) as GetSupply200Response;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetSupply200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetSupply200Response", contentType),
                "GetSupply200Response", ""
            ) as GetSupply200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getSyncRound
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getSyncRoundResponse(response: ResponseContext): Promise<GetSyncRound200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetSyncRound200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetSyncRound200Response", contentType),
                "GetSyncRound200Response", ""
            ) as GetSyncRound200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Sync round not set.", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetSyncRound200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetSyncRound200Response", contentType),
                "GetSyncRound200Response", ""
            ) as GetSyncRound200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getTransactionGroupLedgerStateDeltasForRound
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getTransactionGroupLedgerStateDeltasForRoundResponse(response: ResponseContext): Promise<GetTransactionGroupLedgerStateDeltasForRound200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetTransactionGroupLedgerStateDeltasForRound200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetTransactionGroupLedgerStateDeltasForRound200Response", contentType),
                "GetTransactionGroupLedgerStateDeltasForRound200Response", ""
            ) as GetTransactionGroupLedgerStateDeltasForRound200Response;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Could not find deltas for round", body, response.headers);
        }
        if (isCodeInRange("408", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "timed out on request", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("501", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Not Implemented", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetTransactionGroupLedgerStateDeltasForRound200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetTransactionGroupLedgerStateDeltasForRound200Response", contentType),
                "GetTransactionGroupLedgerStateDeltasForRound200Response", ""
            ) as GetTransactionGroupLedgerStateDeltasForRound200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getTransactionProof
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getTransactionProofResponse(response: ResponseContext): Promise<GetTransactionProof200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetTransactionProof200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetTransactionProof200Response", contentType),
                "GetTransactionProof200Response", ""
            ) as GetTransactionProof200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Malformed round number or transaction ID", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Non-existent block or transaction", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal error, including protocol not supporting proofs.", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetTransactionProof200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetTransactionProof200Response", contentType),
                "GetTransactionProof200Response", ""
            ) as GetTransactionProof200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to getVersion
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async getVersionResponse(response: ResponseContext): Promise<Version > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Version = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Version", contentType),
                "Version", ""
            ) as Version;
            return body;
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Version = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "Version", contentType),
                "Version", ""
            ) as Version;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to healthCheck
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async healthCheckResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to metrics
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async metricsResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "metrics were compiled out", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to pendingTransactionInformation
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async pendingTransactionInformationResponse(response: ResponseContext): Promise<PendingTransactionResponse > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: PendingTransactionResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "PendingTransactionResponse", contentType),
                "PendingTransactionResponse", ""
            ) as PendingTransactionResponse;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Transaction Not Found", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: PendingTransactionResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "PendingTransactionResponse", contentType),
                "PendingTransactionResponse", ""
            ) as PendingTransactionResponse;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to putDebugSettingsProf
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async putDebugSettingsProfResponse(response: ResponseContext): Promise<DebugSettingsProf > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: DebugSettingsProf = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "DebugSettingsProf", contentType),
                "DebugSettingsProf", ""
            ) as DebugSettingsProf;
            return body;
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: DebugSettingsProf = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "DebugSettingsProf", contentType),
                "DebugSettingsProf", ""
            ) as DebugSettingsProf;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to rawTransaction
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async rawTransactionResponse(response: ResponseContext): Promise<RawTransaction200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: RawTransaction200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "RawTransaction200Response", contentType),
                "RawTransaction200Response", ""
            ) as RawTransaction200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Malformed Algorand transaction ", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: RawTransaction200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "RawTransaction200Response", contentType),
                "RawTransaction200Response", ""
            ) as RawTransaction200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to rawTransactionAsync
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async rawTransactionAsyncResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Malformed Algorand transaction ", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Developer or Experimental API not enabled", undefined, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to setBlockTimeStampOffset
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async setBlockTimeStampOffsetResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Cannot set timestamp offset to a negative integer.", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to setSyncRound
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async setSyncRoundResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Can not set sync round to an earlier round than the current round.", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to shutdownNode
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async shutdownNodeResponse(response: ResponseContext): Promise<any > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: any = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "any", contentType),
                "any", ""
            ) as any;
            return body;
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: any = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "any", contentType),
                "any", ""
            ) as any;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to simulateTransaction
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async simulateTransactionResponse(response: ResponseContext): Promise<SimulateTransaction200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: SimulateTransaction200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "SimulateTransaction200Response", contentType),
                "SimulateTransaction200Response", ""
            ) as SimulateTransaction200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: SimulateTransaction200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "SimulateTransaction200Response", contentType),
                "SimulateTransaction200Response", ""
            ) as SimulateTransaction200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to startCatchup
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async startCatchupResponse(response: ResponseContext): Promise<StartCatchup200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: StartCatchup200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "StartCatchup200Response", contentType),
                "StartCatchup200Response", ""
            ) as StartCatchup200Response;
            return body;
        }
        if (isCodeInRange("201", response.httpStatusCode)) {
            const body: StartCatchup200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "StartCatchup200Response", contentType),
                "StartCatchup200Response", ""
            ) as StartCatchup200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("408", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Request Timeout", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: StartCatchup200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "StartCatchup200Response", contentType),
                "StartCatchup200Response", ""
            ) as StartCatchup200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to swaggerJSON
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async swaggerJSONResponse(response: ResponseContext): Promise<string > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            return body;
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "string", contentType),
                "string", ""
            ) as string;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to tealCompile
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async tealCompileResponse(response: ResponseContext): Promise<TealCompile200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: TealCompile200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TealCompile200Response", contentType),
                "TealCompile200Response", ""
            ) as TealCompile200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Teal Compile Error", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Developer API not enabled", undefined, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: TealCompile200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TealCompile200Response", contentType),
                "TealCompile200Response", ""
            ) as TealCompile200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to tealDisassemble
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async tealDisassembleResponse(response: ResponseContext): Promise<TealDisassemble200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: TealDisassemble200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TealDisassemble200Response", contentType),
                "TealDisassemble200Response", ""
            ) as TealDisassemble200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request - Teal Compile Error", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Developer API not enabled", undefined, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: TealDisassemble200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TealDisassemble200Response", contentType),
                "TealDisassemble200Response", ""
            ) as TealDisassemble200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to tealDryrun
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async tealDryrunResponse(response: ResponseContext): Promise<TealDryrun200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: TealDryrun200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TealDryrun200Response", contentType),
                "TealDryrun200Response", ""
            ) as TealDryrun200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Developer API not enabled", undefined, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: TealDryrun200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TealDryrun200Response", contentType),
                "TealDryrun200Response", ""
            ) as TealDryrun200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to transactionParams
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async transactionParamsResponse(response: ResponseContext): Promise<TransactionParams200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: TransactionParams200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TransactionParams200Response", contentType),
                "TransactionParams200Response", ""
            ) as TransactionParams200Response;
            return body;
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: TransactionParams200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "TransactionParams200Response", contentType),
                "TransactionParams200Response", ""
            ) as TransactionParams200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to unsetSyncRound
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async unsetSyncRoundResponse(response: ResponseContext): Promise<void > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return undefined;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Sync round not set.", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "void", contentType),
                "void", ""
            ) as void;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to waitForBlock
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async waitForBlockResponse(response: ResponseContext): Promise<GetStatus200Response > {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: GetStatus200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetStatus200Response", contentType),
                "GetStatus200Response", ""
            ) as GetStatus200Response;
            return body;
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Bad Request -- number must be non-negative integer", body, response.headers);
        }
        if (isCodeInRange("401", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Invalid API Token", body, response.headers);
        }
        if (isCodeInRange("500", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Internal Error", body, response.headers);
        }
        if (isCodeInRange("503", response.httpStatusCode)) {
            const body: ErrorResponse = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "ErrorResponse", contentType),
                "ErrorResponse", ""
            ) as ErrorResponse;
            throw new ApiException<ErrorResponse>(response.httpStatusCode, "Service Temporarily Unavailable", body, response.headers);
        }
        if (isCodeInRange("0", response.httpStatusCode)) {
            throw new ApiException<undefined>(response.httpStatusCode, "Unknown Error", undefined, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: GetStatus200Response = ObjectSerializer.deserialize(
                await decodeResponseBody(response, "GetStatus200Response", contentType),
                "GetStatus200Response", ""
            ) as GetStatus200Response;
            return body;
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

}

const decodeResponseBody = async (response: ResponseContext, expectedType: string, contentType: string | undefined): Promise<any> => {
    if (contentType && contentType.includes('msgpack')) {
        // Retrieve the response body as binary and let ObjectSerializer handle the decoding.
        const binaryData = await response.body.binary();
        let bytes: Uint8Array;

        if (typeof Blob !== 'undefined' && binaryData instanceof Blob) {
            bytes = new Uint8Array(await binaryData.arrayBuffer());
        } else if (binaryData instanceof Uint8Array) {
            bytes = binaryData;
        } else if (binaryData instanceof ArrayBuffer) {
            bytes = new Uint8Array(binaryData);
        } else {
            // Fallback: attempt to construct from any array-like object (including Buffer)
            bytes = Uint8Array.from(binaryData as any);
        }

        return ObjectSerializer.parse(bytes, contentType, expectedType);
    }

    // For non-binary payloads we can rely on the text representation.
    return ObjectSerializer.parse(await response.body.text(), contentType);
};
