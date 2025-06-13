import { ResponseContext, RequestContext, HttpFile } from '../http/http';
import { Configuration, ConfigurationOptions } from '../configuration'
import type { Middleware } from '../middleware';
import { Observable, of, from } from '../rxjsStub';
import {mergeMap, map} from  '../rxjsStub';
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

import { AlgodApiRequestFactory, AlgodApiResponseProcessor} from "../apis/AlgodApi";
export class ObservableAlgodApi {
    private requestFactory: AlgodApiRequestFactory;
    private responseProcessor: AlgodApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: AlgodApiRequestFactory,
        responseProcessor?: AlgodApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new AlgodApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new AlgodApiResponseProcessor();
    }

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param catchpoint A catch point
     */
    public abortCatchupResponse(catchpoint: string, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.abortCatchup(catchpoint, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param catchpoint A catch point
     */
    public abortCatchup(catchpoint: string, _options?: ConfigurationOptions): Observable<AbortCatchup200Response> {
        return this.abortCatchupResponse(catchpoint, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.abortCatchupResponse(rsp)));
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param address An account public key
     * @param applicationId An application identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountApplicationInformationResponse(address: string, applicationId: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.accountApplicationInformation(address, applicationId, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param address An account public key
     * @param applicationId An application identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountApplicationInformation(address: string, applicationId: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<AccountApplicationInformation200Response> {
        return this.accountApplicationInformationResponse(address, applicationId, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.accountApplicationInformationResponse(rsp)));
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param address An account public key
     * @param assetId An asset identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountAssetInformationResponse(address: string, assetId: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.accountAssetInformation(address, assetId, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param address An account public key
     * @param assetId An asset identifier
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public accountAssetInformation(address: string, assetId: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<AccountAssetInformation200Response> {
        return this.accountAssetInformationResponse(address, assetId, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.accountAssetInformationResponse(rsp)));
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param address An account public key
     * @param [limit] Maximum number of results to return.
     * @param [next] The next page of results. Use the next token provided by the previous results.
     */
    public accountAssetsInformationResponse(address: string, limit?: number, next?: string, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.accountAssetsInformation(address, limit, next, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param address An account public key
     * @param [limit] Maximum number of results to return.
     * @param [next] The next page of results. Use the next token provided by the previous results.
     */
    public accountAssetsInformation(address: string, limit?: number, next?: string, _options?: ConfigurationOptions): Observable<AccountAssetsInformation200Response> {
        return this.accountAssetsInformationResponse(address, limit, next, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.accountAssetsInformationResponse(rsp)));
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param address An account public key
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [exclude] When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;.
     */
    public accountInformationResponse(address: string, format?: 'json' | 'msgpack', exclude?: 'all' | 'none', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.accountInformation(address, format, exclude, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param address An account public key
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [exclude] When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;.
     */
    public accountInformation(address: string, format?: 'json' | 'msgpack', exclude?: 'all' | 'none', _options?: ConfigurationOptions): Observable<Account> {
        return this.accountInformationResponse(address, format, exclude, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.accountInformationResponse(rsp)));
    }

    /**
     * Add a participation key to the node
     * @param participationkey The participation key to add to the node
     */
    public addParticipationKeyResponse(participationkey: HttpFile, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.addParticipationKey(participationkey, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Add a participation key to the node
     * @param participationkey The participation key to add to the node
     */
    public addParticipationKey(participationkey: HttpFile, _options?: ConfigurationOptions): Observable<AddParticipationKey200Response> {
        return this.addParticipationKeyResponse(participationkey, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.addParticipationKeyResponse(rsp)));
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param participationId
     * @param keymap The state proof keys to add to an existing participation ID
     */
    public appendKeysResponse(participationId: string, keymap: HttpFile, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.appendKeys(participationId, keymap, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param participationId
     * @param keymap The state proof keys to add to an existing participation ID
     */
    public appendKeys(participationId: string, keymap: HttpFile, _options?: ConfigurationOptions): Observable<ParticipationKey> {
        return this.appendKeysResponse(participationId, keymap, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.appendKeysResponse(rsp)));
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param participationId
     */
    public deleteParticipationKeyByIDResponse(participationId: string, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.deleteParticipationKeyByID(participationId, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param participationId
     */
    public deleteParticipationKeyByID(participationId: string, _options?: ConfigurationOptions): Observable<void> {
        return this.deleteParticipationKeyByIDResponse(participationId, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.deleteParticipationKeyByIDResponse(rsp)));
    }

    /**
     * Returns OK if experimental API is enabled.
     */
    public experimentalCheckResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.experimentalCheck(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Returns OK if experimental API is enabled.
     */
    public experimentalCheck(_options?: ConfigurationOptions): Observable<void> {
        return this.experimentalCheckResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.experimentalCheckResponse(rsp)));
    }

    /**
     * Generate and install participation keys to the node.
     * @param address An account public key
     * @param first First round for participation key.
     * @param last Last round for participation key.
     * @param [dilution] Key dilution for two-level participation keys (defaults to sqrt of validity window).
     */
    public generateParticipationKeysResponse(address: string, first: number, last: number, dilution?: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.generateParticipationKeys(address, first, last, dilution, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Generate and install participation keys to the node.
     * @param address An account public key
     * @param first First round for participation key.
     * @param last Last round for participation key.
     * @param [dilution] Key dilution for two-level participation keys (defaults to sqrt of validity window).
     */
    public generateParticipationKeys(address: string, first: number, last: number, dilution?: number, _options?: ConfigurationOptions): Observable<string> {
        return this.generateParticipationKeysResponse(address, first, last, dilution, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.generateParticipationKeysResponse(rsp)));
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param applicationId An application identifier
     * @param name A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;.
     */
    public getApplicationBoxByNameResponse(applicationId: number, name: string, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getApplicationBoxByName(applicationId, name, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param applicationId An application identifier
     * @param name A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;.
     */
    public getApplicationBoxByName(applicationId: number, name: string, _options?: ConfigurationOptions): Observable<Box> {
        return this.getApplicationBoxByNameResponse(applicationId, name, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getApplicationBoxByNameResponse(rsp)));
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param applicationId An application identifier
     * @param [max] Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names.
     */
    public getApplicationBoxesResponse(applicationId: number, max?: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getApplicationBoxes(applicationId, max, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param applicationId An application identifier
     * @param [max] Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names.
     */
    public getApplicationBoxes(applicationId: number, max?: number, _options?: ConfigurationOptions): Observable<GetApplicationBoxes200Response> {
        return this.getApplicationBoxesResponse(applicationId, max, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getApplicationBoxesResponse(rsp)));
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param applicationId An application identifier
     */
    public getApplicationByIDResponse(applicationId: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getApplicationByID(applicationId, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param applicationId An application identifier
     */
    public getApplicationByID(applicationId: number, _options?: ConfigurationOptions): Observable<Application> {
        return this.getApplicationByIDResponse(applicationId, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getApplicationByIDResponse(rsp)));
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param assetId An asset identifier
     */
    public getAssetByIDResponse(assetId: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getAssetByID(assetId, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param assetId An asset identifier
     */
    public getAssetByID(assetId: number, _options?: ConfigurationOptions): Observable<Asset> {
        return this.getAssetByIDResponse(assetId, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getAssetByIDResponse(rsp)));
    }

    /**
     * Get the block for the given round.
     * @param round The round from which to fetch block information.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [headerOnly] If true, only the block header (exclusive of payset or certificate) may be included in response.
     */
    public getBlockResponse(round: number, format?: 'json' | 'msgpack', headerOnly?: boolean, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getBlock(round, format, headerOnly, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get the block for the given round.
     * @param round The round from which to fetch block information.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * @param [headerOnly] If true, only the block header (exclusive of payset or certificate) may be included in response.
     */
    public getBlock(round: number, format?: 'json' | 'msgpack', headerOnly?: boolean, _options?: ConfigurationOptions): Observable<GetBlock200Response> {
        return this.getBlockResponse(round, format, headerOnly, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getBlockResponse(rsp)));
    }

    /**
     * Get the block hash for the block on the given round.
     * @param round The round from which to fetch block hash information.
     */
    public getBlockHashResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getBlockHash(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get the block hash for the block on the given round.
     * @param round The round from which to fetch block hash information.
     */
    public getBlockHash(round: number, _options?: ConfigurationOptions): Observable<GetBlockHash200Response> {
        return this.getBlockHashResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getBlockHashResponse(rsp)));
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param round The round from which to fetch block log information.
     */
    public getBlockLogsResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getBlockLogs(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param round The round from which to fetch block log information.
     */
    public getBlockLogs(round: number, _options?: ConfigurationOptions): Observable<GetBlockLogs200Response> {
        return this.getBlockLogsResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getBlockLogsResponse(rsp)));
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     */
    public getBlockTimeStampOffsetResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getBlockTimeStampOffset(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     */
    public getBlockTimeStampOffset(_options?: ConfigurationOptions): Observable<GetBlockTimeStampOffset200Response> {
        return this.getBlockTimeStampOffsetResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getBlockTimeStampOffsetResponse(rsp)));
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param round The round from which to fetch block transaction IDs.
     */
    public getBlockTxidsResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getBlockTxids(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param round The round from which to fetch block transaction IDs.
     */
    public getBlockTxids(round: number, _options?: ConfigurationOptions): Observable<GetBlockTxids200Response> {
        return this.getBlockTxidsResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getBlockTxidsResponse(rsp)));
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     */
    public getConfigResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getConfig(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     */
    public getConfig(_options?: ConfigurationOptions): Observable<string> {
        return this.getConfigResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getConfigResponse(rsp)));
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     */
    public getDebugSettingsProfResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getDebugSettingsProf(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     */
    public getDebugSettingsProf(_options?: ConfigurationOptions): Observable<DebugSettingsProf> {
        return this.getDebugSettingsProfResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getDebugSettingsProfResponse(rsp)));
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     */
    public getGenesisResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getGenesis(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     */
    public getGenesis(_options?: ConfigurationOptions): Observable<Genesis> {
        return this.getGenesisResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getGenesisResponse(rsp)));
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDeltaResponse(round: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getLedgerStateDelta(round, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDelta(round: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<any> {
        return this.getLedgerStateDeltaResponse(round, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getLedgerStateDeltaResponse(rsp)));
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param id A transaction ID, or transaction group ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDeltaForTransactionGroupResponse(id: string, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getLedgerStateDeltaForTransactionGroup(id, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param id A transaction ID, or transaction group ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getLedgerStateDeltaForTransactionGroup(id: string, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<any> {
        return this.getLedgerStateDeltaForTransactionGroupResponse(id, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getLedgerStateDeltaForTransactionGroupResponse(rsp)));
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param round The round to which the light block header belongs.
     */
    public getLightBlockHeaderProofResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getLightBlockHeaderProof(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param round The round to which the light block header belongs.
     */
    public getLightBlockHeaderProof(round: number, _options?: ConfigurationOptions): Observable<LightBlockHeaderProof> {
        return this.getLightBlockHeaderProofResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getLightBlockHeaderProofResponse(rsp)));
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param participationId
     */
    public getParticipationKeyByIDResponse(participationId: string, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getParticipationKeyByID(participationId, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param participationId
     */
    public getParticipationKeyByID(participationId: string, _options?: ConfigurationOptions): Observable<ParticipationKey> {
        return this.getParticipationKeyByIDResponse(participationId, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getParticipationKeyByIDResponse(rsp)));
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     */
    public getParticipationKeysResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getParticipationKeys(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     */
    public getParticipationKeys(_options?: ConfigurationOptions): Observable<Array<ParticipationKey>> {
        return this.getParticipationKeysResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getParticipationKeysResponse(rsp)));
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactionsResponse(max?: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getPendingTransactions(max, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactions(max?: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<GetPendingTransactionsByAddress200Response> {
        return this.getPendingTransactionsResponse(max, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getPendingTransactionsResponse(rsp)));
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param address An account public key
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactionsByAddressResponse(address: string, max?: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getPendingTransactionsByAddress(address, max, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param address An account public key
     * @param [max] Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getPendingTransactionsByAddress(address: string, max?: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<GetPendingTransactionsByAddress200Response> {
        return this.getPendingTransactionsByAddressResponse(address, max, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getPendingTransactionsByAddressResponse(rsp)));
    }

    /**
     * Returns OK if healthy and fully caught up.
     */
    public getReadyResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getReady(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Returns OK if healthy and fully caught up.
     */
    public getReady(_options?: ConfigurationOptions): Observable<void> {
        return this.getReadyResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getReadyResponse(rsp)));
    }

    /**
     * Get a state proof that covers a given round
     * @param round The round for which a state proof is desired.
     */
    public getStateProofResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getStateProof(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get a state proof that covers a given round
     * @param round The round for which a state proof is desired.
     */
    public getStateProof(round: number, _options?: ConfigurationOptions): Observable<StateProof> {
        return this.getStateProofResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getStateProofResponse(rsp)));
    }

    /**
     * Gets the current node status.
     */
    public getStatusResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getStatus(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Gets the current node status.
     */
    public getStatus(_options?: ConfigurationOptions): Observable<GetStatus200Response> {
        return this.getStatusResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getStatusResponse(rsp)));
    }

    /**
     * Get the current supply reported by the ledger.
     */
    public getSupplyResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getSupply(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get the current supply reported by the ledger.
     */
    public getSupply(_options?: ConfigurationOptions): Observable<GetSupply200Response> {
        return this.getSupplyResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getSupplyResponse(rsp)));
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     */
    public getSyncRoundResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getSyncRound(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     */
    public getSyncRound(_options?: ConfigurationOptions): Observable<GetSyncRound200Response> {
        return this.getSyncRoundResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getSyncRoundResponse(rsp)));
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionGroupLedgerStateDeltasForRoundResponse(round: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getTransactionGroupLedgerStateDeltasForRound(round, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param round The round for which the deltas are desired.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionGroupLedgerStateDeltasForRound(round: number, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<GetTransactionGroupLedgerStateDeltasForRound200Response> {
        return this.getTransactionGroupLedgerStateDeltasForRoundResponse(round, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getTransactionGroupLedgerStateDeltasForRoundResponse(rsp)));
    }

    /**
     * Get a proof for a transaction in a block.
     * @param round The round in which the transaction appears.
     * @param txid The transaction ID for which to generate a proof.
     * @param [hashtype] The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionProofResponse(round: number, txid: string, hashtype?: 'sha512_256' | 'sha256', format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getTransactionProof(round, txid, hashtype, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get a proof for a transaction in a block.
     * @param round The round in which the transaction appears.
     * @param txid The transaction ID for which to generate a proof.
     * @param [hashtype] The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public getTransactionProof(round: number, txid: string, hashtype?: 'sha512_256' | 'sha256', format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<GetTransactionProof200Response> {
        return this.getTransactionProofResponse(round, txid, hashtype, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getTransactionProofResponse(rsp)));
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     */
    public getVersionResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.getVersion(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     */
    public getVersion(_options?: ConfigurationOptions): Observable<Version> {
        return this.getVersionResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.getVersionResponse(rsp)));
    }

    /**
     * Returns OK if healthy.
     */
    public healthCheckResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.healthCheck(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Returns OK if healthy.
     */
    public healthCheck(_options?: ConfigurationOptions): Observable<void> {
        return this.healthCheckResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.healthCheckResponse(rsp)));
    }

    /**
     * Return metrics about algod functioning.
     */
    public metricsResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.metrics(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Return metrics about algod functioning.
     */
    public metrics(_options?: ConfigurationOptions): Observable<void> {
        return this.metricsResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.metricsResponse(rsp)));
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param txid A transaction ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public pendingTransactionInformationResponse(txid: string, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.pendingTransactionInformation(txid, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param txid A transaction ID
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public pendingTransactionInformation(txid: string, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<PendingTransactionResponse> {
        return this.pendingTransactionInformationResponse(txid, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.pendingTransactionInformationResponse(rsp)));
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     */
    public putDebugSettingsProfResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.putDebugSettingsProf(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     */
    public putDebugSettingsProf(_options?: ConfigurationOptions): Observable<DebugSettingsProf> {
        return this.putDebugSettingsProfResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.putDebugSettingsProfResponse(rsp)));
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransactionResponse(rawtxn: HttpFile, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.rawTransaction(rawtxn, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransaction(rawtxn: HttpFile, _options?: ConfigurationOptions): Observable<RawTransaction200Response> {
        return this.rawTransactionResponse(rawtxn, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.rawTransactionResponse(rsp)));
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransactionAsyncResponse(rawtxn: HttpFile, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.rawTransactionAsync(rawtxn, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param rawtxn The byte encoded signed transaction to broadcast to network
     */
    public rawTransactionAsync(rawtxn: HttpFile, _options?: ConfigurationOptions): Observable<void> {
        return this.rawTransactionAsyncResponse(rawtxn, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.rawTransactionAsyncResponse(rsp)));
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param offset The timestamp offset for blocks in dev mode.
     */
    public setBlockTimeStampOffsetResponse(offset: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.setBlockTimeStampOffset(offset, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param offset The timestamp offset for blocks in dev mode.
     */
    public setBlockTimeStampOffset(offset: number, _options?: ConfigurationOptions): Observable<void> {
        return this.setBlockTimeStampOffsetResponse(offset, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.setBlockTimeStampOffsetResponse(rsp)));
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param round The round for which the deltas are desired.
     */
    public setSyncRoundResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.setSyncRound(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param round The round for which the deltas are desired.
     */
    public setSyncRound(round: number, _options?: ConfigurationOptions): Observable<void> {
        return this.setSyncRoundResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.setSyncRoundResponse(rsp)));
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param [timeout]
     */
    public shutdownNodeResponse(timeout?: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.shutdownNode(timeout, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param [timeout]
     */
    public shutdownNode(timeout?: number, _options?: ConfigurationOptions): Observable<any> {
        return this.shutdownNodeResponse(timeout, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.shutdownNodeResponse(rsp)));
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param request The transactions to simulate, along with any other inputs.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public simulateTransactionResponse(request: SimulateRequest, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.simulateTransaction(request, format, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param request The transactions to simulate, along with any other inputs.
     * @param [format] Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     */
    public simulateTransaction(request: SimulateRequest, format?: 'json' | 'msgpack', _options?: ConfigurationOptions): Observable<SimulateTransaction200Response> {
        return this.simulateTransactionResponse(request, format, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.simulateTransactionResponse(rsp)));
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param catchpoint A catch point
     * @param [min] Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed.
     */
    public startCatchupResponse(catchpoint: string, min?: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.startCatchup(catchpoint, min, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param catchpoint A catch point
     * @param [min] Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed.
     */
    public startCatchup(catchpoint: string, min?: number, _options?: ConfigurationOptions): Observable<StartCatchup200Response> {
        return this.startCatchupResponse(catchpoint, min, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.startCatchupResponse(rsp)));
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     */
    public swaggerJSONResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.swaggerJSON(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     */
    public swaggerJSON(_options?: ConfigurationOptions): Observable<string> {
        return this.swaggerJSONResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.swaggerJSONResponse(rsp)));
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param source TEAL source code to be compiled
     * @param [sourcemap] When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;.
     */
    public tealCompileResponse(source: HttpFile, sourcemap?: boolean, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.tealCompile(source, sourcemap, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param source TEAL source code to be compiled
     * @param [sourcemap] When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;.
     */
    public tealCompile(source: HttpFile, sourcemap?: boolean, _options?: ConfigurationOptions): Observable<TealCompile200Response> {
        return this.tealCompileResponse(source, sourcemap, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.tealCompileResponse(rsp)));
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param source TEAL program binary to be disassembled
     */
    public tealDisassembleResponse(source: string, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.tealDisassemble(source, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param source TEAL program binary to be disassembled
     */
    public tealDisassemble(source: string, _options?: ConfigurationOptions): Observable<TealDisassemble200Response> {
        return this.tealDisassembleResponse(source, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.tealDisassembleResponse(rsp)));
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param [request] Transaction (or group) and any accompanying state-simulation data.
     */
    public tealDryrunResponse(request?: DryrunRequest, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.tealDryrun(request, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param [request] Transaction (or group) and any accompanying state-simulation data.
     */
    public tealDryrun(request?: DryrunRequest, _options?: ConfigurationOptions): Observable<TealDryrun200Response> {
        return this.tealDryrunResponse(request, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.tealDryrunResponse(rsp)));
    }

    /**
     * Get parameters for constructing a new transaction
     */
    public transactionParamsResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.transactionParams(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Get parameters for constructing a new transaction
     */
    public transactionParams(_options?: ConfigurationOptions): Observable<TransactionParams200Response> {
        return this.transactionParamsResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.transactionParamsResponse(rsp)));
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     */
    public unsetSyncRoundResponse(_options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.unsetSyncRound(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     */
    public unsetSyncRound(_options?: ConfigurationOptions): Observable<void> {
        return this.unsetSyncRoundResponse(_options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.unsetSyncRoundResponse(rsp)));
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param round The round to wait until returning status
     */
    public waitForBlockResponse(round: number, _options?: ConfigurationOptions): Observable<ResponseContext> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.waitForBlock(round, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable;
            }));
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param round The round to wait until returning status
     */
    public waitForBlock(round: number, _options?: ConfigurationOptions): Observable<GetStatus200Response> {
        return this.waitForBlockResponse(round, _options)
          .pipe(map((rsp: ResponseContext) => this.responseProcessor.waitForBlockResponse(rsp)));
    }

}
