import { ResponseContext, RequestContext, HttpFile } from '../http/http';
import { Configuration, ConfigurationOptions, PromiseConfigurationOptions } from '../configuration'
import { PromiseMiddleware, Middleware, PromiseMiddlewareWrapper } from '../middleware';

import { AbortCatchup200Response } from '../models/AbortCatchup200Response';
import { Account } from '../models/Account';
import { AccountApplicationInformation200Response } from '../models/AccountApplicationInformation200Response';
import { AccountAssetHolding } from '../models/AccountAssetHolding';
import { AccountAssetInformation200Response } from '../models/AccountAssetInformation200Response';
import { AccountAssetsInformation200Response } from '../models/AccountAssetsInformation200Response';
import { AccountParticipation } from '../models/AccountParticipation';
import { AccountStateDelta } from '../models/AccountStateDelta';
import { AddParticipationKey200Response } from '../models/AddParticipationKey200Response';
import { AppCallLogs } from '../models/AppCallLogs';
import { Application } from '../models/Application';
import { ApplicationInitialStates } from '../models/ApplicationInitialStates';
import { ApplicationKVStorage } from '../models/ApplicationKVStorage';
import { ApplicationLocalReference } from '../models/ApplicationLocalReference';
import { ApplicationLocalState } from '../models/ApplicationLocalState';
import { ApplicationParams } from '../models/ApplicationParams';
import { ApplicationStateOperation } from '../models/ApplicationStateOperation';
import { ApplicationStateSchema } from '../models/ApplicationStateSchema';
import { Asset } from '../models/Asset';
import { AssetHolding } from '../models/AssetHolding';
import { AssetHoldingReference } from '../models/AssetHoldingReference';
import { AssetParams } from '../models/AssetParams';
import { AvmKeyValue } from '../models/AvmKeyValue';
import { AvmValue } from '../models/AvmValue';
import { Box } from '../models/Box';
import { BoxDescriptor } from '../models/BoxDescriptor';
import { BoxReference } from '../models/BoxReference';
import { BuildVersion } from '../models/BuildVersion';
import { DebugSettingsProf } from '../models/DebugSettingsProf';
import { DryrunRequest } from '../models/DryrunRequest';
import { DryrunSource } from '../models/DryrunSource';
import { DryrunState } from '../models/DryrunState';
import { DryrunTxnResult } from '../models/DryrunTxnResult';
import { ErrorResponse } from '../models/ErrorResponse';
import { EvalDelta } from '../models/EvalDelta';
import { EvalDeltaKeyValue } from '../models/EvalDeltaKeyValue';
import { Genesis } from '../models/Genesis';
import { GenesisAllocation } from '../models/GenesisAllocation';
import { GenesisAllocationState } from '../models/GenesisAllocationState';
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
import { KvDelta } from '../models/KvDelta';
import { LedgerStateDeltaForTransactionGroup } from '../models/LedgerStateDeltaForTransactionGroup';
import { LightBlockHeaderProof } from '../models/LightBlockHeaderProof';
import { ParticipationKey } from '../models/ParticipationKey';
import { PendingTransactionResponse } from '../models/PendingTransactionResponse';
import { RawTransaction200Response } from '../models/RawTransaction200Response';
import { ScratchChange } from '../models/ScratchChange';
import { SimulateInitialStates } from '../models/SimulateInitialStates';
import { SimulateRequest } from '../models/SimulateRequest';
import { SimulateRequestTransactionGroup } from '../models/SimulateRequestTransactionGroup';
import { SimulateTraceConfig } from '../models/SimulateTraceConfig';
import { SimulateTransaction200Response } from '../models/SimulateTransaction200Response';
import { SimulateTransactionGroupResult } from '../models/SimulateTransactionGroupResult';
import { SimulateTransactionResult } from '../models/SimulateTransactionResult';
import { SimulateUnnamedResourcesAccessed } from '../models/SimulateUnnamedResourcesAccessed';
import { SimulationEvalOverrides } from '../models/SimulationEvalOverrides';
import { SimulationOpcodeTraceUnit } from '../models/SimulationOpcodeTraceUnit';
import { SimulationTransactionExecTrace } from '../models/SimulationTransactionExecTrace';
import { StartCatchup200Response } from '../models/StartCatchup200Response';
import { StateProof } from '../models/StateProof';
import { StateProofMessage } from '../models/StateProofMessage';
import { TealCompile200Response } from '../models/TealCompile200Response';
import { TealDisassemble200Response } from '../models/TealDisassemble200Response';
import { TealDryrun200Response } from '../models/TealDryrun200Response';
import { TealKeyValue } from '../models/TealKeyValue';
import { TealValue } from '../models/TealValue';
import { TransactionParams200Response } from '../models/TransactionParams200Response';
import { Version } from '../models/Version';
import { ObservableAlgodApi } from './ObservableAPI';

import { AlgodApiRequestFactory, AlgodApiResponseProcessor} from "../apis/AlgodApi";
export class PromiseAlgodApi {
    private api: ObservableAlgodApi

    public constructor(
        configuration: Configuration,
        requestFactory?: AlgodApiRequestFactory,
        responseProcessor?: AlgodApiResponseProcessor
    ) {
        this.api = new ObservableAlgodApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param catchpoint A catch point
     */
    public abortCatchupResponse(catchpoint: string, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.abortCatchupResponse(catchpoint, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param catchpoint A catch point
     */
    public abortCatchup(catchpoint: string, _options?: PromiseConfigurationOptions): Promise<AbortCatchup200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.abortCatchup(catchpoint, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param address An account public key
     * @param applicationId An application identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountApplicationInformationResponse(address: string, applicationId: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountApplicationInformationResponse(address, applicationId, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param address An account public key
     * @param applicationId An application identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountApplicationInformation(address: string, applicationId: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<AccountApplicationInformation200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountApplicationInformation(address, applicationId, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param address An account public key
     * @param assetId An asset identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountAssetInformationResponse(address: string, assetId: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountAssetInformationResponse(address, assetId, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param address An account public key
     * @param assetId An asset identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountAssetInformation(address: string, assetId: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<AccountAssetInformation200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountAssetInformation(address, assetId, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param address An account public key
     * @param [limit] Maximum number of results to return.
     * @param [next] The next page of results. Use the next token provided by the previous results.
     */
    public accountAssetsInformationResponse(address: string, limit?: number, next?: string, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountAssetsInformationResponse(address, limit, next, observableOptions);
        return result.toPromise();
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param address An account public key
     * @param [limit] Maximum number of results to return.
     * @param [next] The next page of results. Use the next token provided by the previous results.
     */
    public accountAssetsInformation(address: string, limit?: number, next?: string, _options?: PromiseConfigurationOptions): Promise<AccountAssetsInformation200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountAssetsInformation(address, limit, next, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param address An account public key
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [exclude] When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;.
     */
    public accountInformationResponse(address: string, format?: 'json' | 'msgpack', exclude?: 'all' | 'none', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountInformationResponse(address, format, exclude, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param address An account public key
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [exclude] When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;.
     */
    public accountInformation(address: string, format?: 'json' | 'msgpack', exclude?: 'all' | 'none', _options?: PromiseConfigurationOptions): Promise<Account> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.accountInformation(address, format, exclude, observableOptions);
        return result.toPromise();
    }

    /**
     * Add a participation key to the node
     * @param participationkey The participation key to add to the node
     */
    public addParticipationKeyResponse(participationkey: HttpFile, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.addParticipationKeyResponse(participationkey, observableOptions);
        return result.toPromise();
    }

    /**
     * Add a participation key to the node
     * @param participationkey The participation key to add to the node
     */
    public addParticipationKey(participationkey: HttpFile, _options?: PromiseConfigurationOptions): Promise<AddParticipationKey200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.addParticipationKey(participationkey, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param participationId
     * @param keymap The state proof keys to add to an existing participation ID
     */
    public appendKeysResponse(participationId: string, keymap: HttpFile, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.appendKeysResponse(participationId, keymap, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param participationId
     * @param keymap The state proof keys to add to an existing participation ID
     */
    public appendKeys(participationId: string, keymap: HttpFile, _options?: PromiseConfigurationOptions): Promise<ParticipationKey> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.appendKeys(participationId, keymap, observableOptions);
        return result.toPromise();
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param participationId
     */
    public deleteParticipationKeyByIDResponse(participationId: string, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.deleteParticipationKeyByIDResponse(participationId, observableOptions);
        return result.toPromise();
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param participationId
     */
    public deleteParticipationKeyByID(participationId: string, _options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.deleteParticipationKeyByID(participationId, observableOptions);
        return result.toPromise();
    }

    /**
     * Returns OK if experimental API is enabled.
     */
    public experimentalCheckResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.experimentalCheckResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns OK if experimental API is enabled.
     */
    public experimentalCheck(_options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.experimentalCheck(observableOptions);
        return result.toPromise();
    }

    /**
     * Generate and install participation keys to the node.
     * @param address An account public key
     * @param first First round for participation key.
     * @param last Last round for participation key.
     * @param [dilution] Key dilution for two-level participation keys (defaults to sqrt of validity window).
     */
    public generateParticipationKeysResponse(address: string, first: number, last: number, dilution?: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.generateParticipationKeysResponse(address, first, last, dilution, observableOptions);
        return result.toPromise();
    }

    /**
     * Generate and install participation keys to the node.
     * @param address An account public key
     * @param first First round for participation key.
     * @param last Last round for participation key.
     * @param [dilution] Key dilution for two-level participation keys (defaults to sqrt of validity window).
     */
    public generateParticipationKeys(address: string, first: number, last: number, dilution?: number, _options?: PromiseConfigurationOptions): Promise<string> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.generateParticipationKeys(address, first, last, dilution, observableOptions);
        return result.toPromise();
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param applicationId An application identifier
     * @param name A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;.
     */
    public getApplicationBoxByNameResponse(applicationId: number, name: string, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getApplicationBoxByNameResponse(applicationId, name, observableOptions);
        return result.toPromise();
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param applicationId An application identifier
     * @param name A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;.
     */
    public getApplicationBoxByName(applicationId: number, name: string, _options?: PromiseConfigurationOptions): Promise<Box> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getApplicationBoxByName(applicationId, name, observableOptions);
        return result.toPromise();
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param applicationId An application identifier
     * @param [max] Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names.
     */
    public getApplicationBoxesResponse(applicationId: number, max?: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getApplicationBoxesResponse(applicationId, max, observableOptions);
        return result.toPromise();
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param applicationId An application identifier
     * @param [max] Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names.
     */
    public getApplicationBoxes(applicationId: number, max?: number, _options?: PromiseConfigurationOptions): Promise<GetApplicationBoxes200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getApplicationBoxes(applicationId, max, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param applicationId An application identifier
     */
    public getApplicationByIDResponse(applicationId: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getApplicationByIDResponse(applicationId, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param applicationId An application identifier
     */
    public getApplicationByID(applicationId: number, _options?: PromiseConfigurationOptions): Promise<Application> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getApplicationByID(applicationId, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param assetId An asset identifier
     */
    public getAssetByIDResponse(assetId: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getAssetByIDResponse(assetId, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param assetId An asset identifier
     */
    public getAssetByID(assetId: number, _options?: PromiseConfigurationOptions): Promise<Asset> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getAssetByID(assetId, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the block for the given round.
     * @param round The round from which to fetch block information.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [headerOnly] If true, only the block header (exclusive of payset or certificate) may be included in response.
     */
    public getBlockResponse(round: number, format?: 'json' | 'msgpack', headerOnly?: boolean, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockResponse(round, format, headerOnly, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the block for the given round.
     * @param round The round from which to fetch block information.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [headerOnly] If true, only the block header (exclusive of payset or certificate) may be included in response.
     */
    public getBlock(round: number, format?: 'json' | 'msgpack', headerOnly?: boolean, _options?: PromiseConfigurationOptions): Promise<GetBlock200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlock(round, format, headerOnly, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the block hash for the block on the given round.
     * @param round The round from which to fetch block hash information.
     */
    public getBlockHashResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockHashResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the block hash for the block on the given round.
     * @param round The round from which to fetch block hash information.
     */
    public getBlockHash(round: number, _options?: PromiseConfigurationOptions): Promise<GetBlockHash200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockHash(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param round The round from which to fetch block log information.
     */
    public getBlockLogsResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockLogsResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param round The round from which to fetch block log information.
     */
    public getBlockLogs(round: number, _options?: PromiseConfigurationOptions): Promise<GetBlockLogs200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockLogs(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     */
    public getBlockTimeStampOffsetResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockTimeStampOffsetResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     */
    public getBlockTimeStampOffset(_options?: PromiseConfigurationOptions): Promise<GetBlockTimeStampOffset200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockTimeStampOffset(observableOptions);
        return result.toPromise();
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param round The round from which to fetch block transaction IDs.
     */
    public getBlockTxidsResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockTxidsResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param round The round from which to fetch block transaction IDs.
     */
    public getBlockTxids(round: number, _options?: PromiseConfigurationOptions): Promise<GetBlockTxids200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getBlockTxids(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     */
    public getConfigResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getConfigResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     */
    public getConfig(_options?: PromiseConfigurationOptions): Promise<string> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getConfig(observableOptions);
        return result.toPromise();
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     */
    public getDebugSettingsProfResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getDebugSettingsProfResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     */
    public getDebugSettingsProf(_options?: PromiseConfigurationOptions): Promise<DebugSettingsProf> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getDebugSettingsProf(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     */
    public getGenesisResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getGenesisResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     */
    public getGenesis(_options?: PromiseConfigurationOptions): Promise<Genesis> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getGenesis(observableOptions);
        return result.toPromise();
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDeltaResponse(round: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getLedgerStateDeltaResponse(round, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDelta(round: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<any> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getLedgerStateDelta(round, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param id A transaction ID, or transaction group ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDeltaForTransactionGroupResponse(id: string, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getLedgerStateDeltaForTransactionGroupResponse(id, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param id A transaction ID, or transaction group ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDeltaForTransactionGroup(id: string, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<any> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getLedgerStateDeltaForTransactionGroup(id, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param round The round to which the light block header belongs.
     */
    public getLightBlockHeaderProofResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getLightBlockHeaderProofResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param round The round to which the light block header belongs.
     */
    public getLightBlockHeaderProof(round: number, _options?: PromiseConfigurationOptions): Promise<LightBlockHeaderProof> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getLightBlockHeaderProof(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param participationId
     */
    public getParticipationKeyByIDResponse(participationId: string, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getParticipationKeyByIDResponse(participationId, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param participationId
     */
    public getParticipationKeyByID(participationId: string, _options?: PromiseConfigurationOptions): Promise<ParticipationKey> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getParticipationKeyByID(participationId, observableOptions);
        return result.toPromise();
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     */
    public getParticipationKeysResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getParticipationKeysResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     */
    public getParticipationKeys(_options?: PromiseConfigurationOptions): Promise<Array<ParticipationKey>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getParticipationKeys(observableOptions);
        return result.toPromise();
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactionsResponse(max?: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getPendingTransactionsResponse(max, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactions(max?: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<GetPendingTransactionsByAddress200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getPendingTransactions(max, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param address An account public key
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactionsByAddressResponse(address: string, max?: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getPendingTransactionsByAddressResponse(address, max, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param address An account public key
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactionsByAddress(address: string, max?: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<GetPendingTransactionsByAddress200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getPendingTransactionsByAddress(address, max, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Returns OK if healthy and fully caught up.
     */
    public getReadyResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getReadyResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns OK if healthy and fully caught up.
     */
    public getReady(_options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getReady(observableOptions);
        return result.toPromise();
    }

    /**
     * Get a state proof that covers a given round
     * @param round The round for which a state proof is desired.
     */
    public getStateProofResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getStateProofResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a state proof that covers a given round
     * @param round The round for which a state proof is desired.
     */
    public getStateProof(round: number, _options?: PromiseConfigurationOptions): Promise<StateProof> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getStateProof(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Gets the current node status.
     */
    public getStatusResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getStatusResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Gets the current node status.
     */
    public getStatus(_options?: PromiseConfigurationOptions): Promise<GetStatus200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getStatus(observableOptions);
        return result.toPromise();
    }

    /**
     * Get the current supply reported by the ledger.
     */
    public getSupplyResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getSupplyResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Get the current supply reported by the ledger.
     */
    public getSupply(_options?: PromiseConfigurationOptions): Promise<GetSupply200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getSupply(observableOptions);
        return result.toPromise();
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     */
    public getSyncRoundResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getSyncRoundResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     */
    public getSyncRound(_options?: PromiseConfigurationOptions): Promise<GetSyncRound200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getSyncRound(observableOptions);
        return result.toPromise();
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionGroupLedgerStateDeltasForRoundResponse(round: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getTransactionGroupLedgerStateDeltasForRoundResponse(round, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionGroupLedgerStateDeltasForRound(round: number, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<GetTransactionGroupLedgerStateDeltasForRound200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getTransactionGroupLedgerStateDeltasForRound(round, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a proof for a transaction in a block.
     * @param round The round in which the transaction appears.
     * @param txid The transaction ID for which to generate a proof.
     * @param [hashtype] The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionProofResponse(round: number, txid: string, hashtype?: 'sha512_256' | 'sha256', format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getTransactionProofResponse(round, txid, hashtype, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a proof for a transaction in a block.
     * @param round The round in which the transaction appears.
     * @param txid The transaction ID for which to generate a proof.
     * @param [hashtype] The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionProof(round: number, txid: string, hashtype?: 'sha512_256' | 'sha256', format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<GetTransactionProof200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getTransactionProof(round, txid, hashtype, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     */
    public getVersionResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getVersionResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     */
    public getVersion(_options?: PromiseConfigurationOptions): Promise<Version> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.getVersion(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns OK if healthy.
     */
    public healthCheckResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.healthCheckResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns OK if healthy.
     */
    public healthCheck(_options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.healthCheck(observableOptions);
        return result.toPromise();
    }

    /**
     * Return metrics about algod functioning.
     */
    public metricsResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.metricsResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Return metrics about algod functioning.
     */
    public metrics(_options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.metrics(observableOptions);
        return result.toPromise();
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param txid A transaction ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public pendingTransactionInformationResponse(txid: string, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.pendingTransactionInformationResponse(txid, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param txid A transaction ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public pendingTransactionInformation(txid: string, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<PendingTransactionResponse> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.pendingTransactionInformation(txid, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     */
    public putDebugSettingsProfResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.putDebugSettingsProfResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     */
    public putDebugSettingsProf(_options?: PromiseConfigurationOptions): Promise<DebugSettingsProf> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.putDebugSettingsProf(observableOptions);
        return result.toPromise();
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransactionResponse(rawtxn: HttpFile, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.rawTransactionResponse(rawtxn, observableOptions);
        return result.toPromise();
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransaction(rawtxn: HttpFile, _options?: PromiseConfigurationOptions): Promise<RawTransaction200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.rawTransaction(rawtxn, observableOptions);
        return result.toPromise();
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransactionAsyncResponse(rawtxn: HttpFile, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.rawTransactionAsyncResponse(rawtxn, observableOptions);
        return result.toPromise();
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransactionAsync(rawtxn: HttpFile, _options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.rawTransactionAsync(rawtxn, observableOptions);
        return result.toPromise();
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param offset The timestamp offset for blocks in dev mode.
     */
    public setBlockTimeStampOffsetResponse(offset: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.setBlockTimeStampOffsetResponse(offset, observableOptions);
        return result.toPromise();
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param offset The timestamp offset for blocks in dev mode.
     */
    public setBlockTimeStampOffset(offset: number, _options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.setBlockTimeStampOffset(offset, observableOptions);
        return result.toPromise();
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param round The round for which the deltas are desired.
     */
    public setSyncRoundResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.setSyncRoundResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param round The round for which the deltas are desired.
     */
    public setSyncRound(round: number, _options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.setSyncRound(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param [timeout]
     */
    public shutdownNodeResponse(timeout?: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.shutdownNodeResponse(timeout, observableOptions);
        return result.toPromise();
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param [timeout]
     */
    public shutdownNode(timeout?: number, _options?: PromiseConfigurationOptions): Promise<any> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.shutdownNode(timeout, observableOptions);
        return result.toPromise();
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param request The transactions to simulate, along with any other inputs.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public simulateTransactionResponse(request: SimulateRequest, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.simulateTransactionResponse(request, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param request The transactions to simulate, along with any other inputs.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public simulateTransaction(request: SimulateRequest, format?: 'json' | 'msgpack', _options?: PromiseConfigurationOptions): Promise<SimulateTransaction200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.simulateTransaction(request, format, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param catchpoint A catch point
     * @param [min] Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed.
     */
    public startCatchupResponse(catchpoint: string, min?: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.startCatchupResponse(catchpoint, min, observableOptions);
        return result.toPromise();
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param catchpoint A catch point
     * @param [min] Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed.
     */
    public startCatchup(catchpoint: string, min?: number, _options?: PromiseConfigurationOptions): Promise<StartCatchup200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.startCatchup(catchpoint, min, observableOptions);
        return result.toPromise();
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     */
    public swaggerJSONResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.swaggerJSONResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     */
    public swaggerJSON(_options?: PromiseConfigurationOptions): Promise<string> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.swaggerJSON(observableOptions);
        return result.toPromise();
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param source TEAL source code to be compiled
     * @param [sourcemap] When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;.
     */
    public tealCompileResponse(source: HttpFile, sourcemap?: boolean, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.tealCompileResponse(source, sourcemap, observableOptions);
        return result.toPromise();
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param source TEAL source code to be compiled
     * @param [sourcemap] When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;.
     */
    public tealCompile(source: HttpFile, sourcemap?: boolean, _options?: PromiseConfigurationOptions): Promise<TealCompile200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.tealCompile(source, sourcemap, observableOptions);
        return result.toPromise();
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param source TEAL program binary to be disassembled
     */
    public tealDisassembleResponse(source: string, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.tealDisassembleResponse(source, observableOptions);
        return result.toPromise();
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param source TEAL program binary to be disassembled
     */
    public tealDisassemble(source: string, _options?: PromiseConfigurationOptions): Promise<TealDisassemble200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.tealDisassemble(source, observableOptions);
        return result.toPromise();
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param [request] Transaction (or group) and any accompanying state-simulation data.
     */
    public tealDryrunResponse(request?: DryrunRequest, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.tealDryrunResponse(request, observableOptions);
        return result.toPromise();
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param [request] Transaction (or group) and any accompanying state-simulation data.
     */
    public tealDryrun(request?: DryrunRequest, _options?: PromiseConfigurationOptions): Promise<TealDryrun200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.tealDryrun(request, observableOptions);
        return result.toPromise();
    }

    /**
     * Get parameters for constructing a new transaction
     */
    public transactionParamsResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.transactionParamsResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Get parameters for constructing a new transaction
     */
    public transactionParams(_options?: PromiseConfigurationOptions): Promise<TransactionParams200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.transactionParams(observableOptions);
        return result.toPromise();
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     */
    public unsetSyncRoundResponse(_options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.unsetSyncRoundResponse(observableOptions);
        return result.toPromise();
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     */
    public unsetSyncRound(_options?: PromiseConfigurationOptions): Promise<void> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.unsetSyncRound(observableOptions);
        return result.toPromise();
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param round The round to wait until returning status
     */
    public waitForBlockResponse(round: number, _options?: PromiseConfigurationOptions): Promise<ResponseContext> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.waitForBlockResponse(round, observableOptions);
        return result.toPromise();
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param round The round to wait until returning status
     */
    public waitForBlock(round: number, _options?: PromiseConfigurationOptions): Promise<GetStatus200Response> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.waitForBlock(round, observableOptions);
        return result.toPromise();
    }


}



