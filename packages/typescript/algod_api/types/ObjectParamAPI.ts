import { ResponseContext, RequestContext, HttpFile } from '../http/http';
import { Configuration, ConfigurationOptions } from '../configuration'
import type { Middleware } from '../middleware';

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

import { ObservableAlgodApi } from "./ObservableAPI";
import { AlgodApiRequestFactory, AlgodApiResponseProcessor} from "../apis/AlgodApi";

export interface AlgodApiAbortCatchupRequest {
    /**
     * A catch point
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiabortCatchup
     */
    catchpoint: string
}

export interface AlgodApiAccountApplicationInformationRequest {
    /**
     * An account public key
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiaccountApplicationInformation
     */
    address: string
    /**
     * An application identifier
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApiaccountApplicationInformation
     */
    applicationId: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApiaccountApplicationInformation
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiAccountAssetInformationRequest {
    /**
     * An account public key
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiaccountAssetInformation
     */
    address: string
    /**
     * An asset identifier
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApiaccountAssetInformation
     */
    assetId: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApiaccountAssetInformation
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiAccountAssetsInformationRequest {
    /**
     * An account public key
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiaccountAssetsInformation
     */
    address: string
    /**
     * Maximum number of results to return.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApiaccountAssetsInformation
     */
    limit?: number
    /**
     * The next page of results. Use the next token provided by the previous results.
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiaccountAssetsInformation
     */
    next?: string
}

export interface AlgodApiAccountInformationRequest {
    /**
     * An account public key
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiaccountInformation
     */
    address: string
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApiaccountInformation
     */
    format?: 'json' | 'msgpack'
    /**
     * When set to &#x60;all&#x60; will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to &#x60;none&#x60;.
     * Defaults to: undefined
     * @type &#39;all&#39; | &#39;none&#39;
     * @memberof AlgodApiaccountInformation
     */
    exclude?: 'all' | 'none'
}

export interface AlgodApiAddParticipationKeyRequest {
    /**
     * The participation key to add to the node
     * @type HttpFile
     * @memberof AlgodApiaddParticipationKey
     */
    participationkey: HttpFile
}

export interface AlgodApiAppendKeysRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApiappendKeys
     */
    participationId: string
    /**
     * The state proof keys to add to an existing participation ID
     * @type HttpFile
     * @memberof AlgodApiappendKeys
     */
    keymap: HttpFile
}

export interface AlgodApiDeleteParticipationKeyByIDRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApideleteParticipationKeyByID
     */
    participationId: string
}

export interface AlgodApiExperimentalCheckRequest {
}

export interface AlgodApiGenerateParticipationKeysRequest {
    /**
     * An account public key
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApigenerateParticipationKeys
     */
    address: string
    /**
     * First round for participation key.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigenerateParticipationKeys
     */
    first: number
    /**
     * Last round for participation key.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigenerateParticipationKeys
     */
    last: number
    /**
     * Key dilution for two-level participation keys (defaults to sqrt of validity window).
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigenerateParticipationKeys
     */
    dilution?: number
}

export interface AlgodApiGetApplicationBoxByNameRequest {
    /**
     * An application identifier
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetApplicationBoxByName
     */
    applicationId: number
    /**
     * A box name, in the goal app call arg form \&#39;encoding:value\&#39;. For ints, use the form \&#39;int:1234\&#39;. For raw bytes, use the form \&#39;b64:A&#x3D;&#x3D;\&#39;. For printable strings, use the form \&#39;str:hello\&#39;. For addresses, use the form \&#39;addr:XYZ...\&#39;.
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApigetApplicationBoxByName
     */
    name: string
}

export interface AlgodApiGetApplicationBoxesRequest {
    /**
     * An application identifier
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetApplicationBoxes
     */
    applicationId: number
    /**
     * Max number of box names to return. If max is not set, or max &#x3D;&#x3D; 0, returns all box-names.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetApplicationBoxes
     */
    max?: number
}

export interface AlgodApiGetApplicationByIDRequest {
    /**
     * An application identifier
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetApplicationByID
     */
    applicationId: number
}

export interface AlgodApiGetAssetByIDRequest {
    /**
     * An asset identifier
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetAssetByID
     */
    assetId: number
}

export interface AlgodApiGetBlockRequest {
    /**
     * The round from which to fetch block information.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetBlock
     */
    round: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetBlock
     */
    format?: 'json' | 'msgpack'
    /**
     * If true, only the block header (exclusive of payset or certificate) may be included in response.
     * Defaults to: undefined
     * @type boolean
     * @memberof AlgodApigetBlock
     */
    headerOnly?: boolean
}

export interface AlgodApiGetBlockHashRequest {
    /**
     * The round from which to fetch block hash information.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetBlockHash
     */
    round: number
}

export interface AlgodApiGetBlockLogsRequest {
    /**
     * The round from which to fetch block log information.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetBlockLogs
     */
    round: number
}

export interface AlgodApiGetBlockTimeStampOffsetRequest {
}

export interface AlgodApiGetBlockTxidsRequest {
    /**
     * The round from which to fetch block transaction IDs.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetBlockTxids
     */
    round: number
}

export interface AlgodApiGetConfigRequest {
}

export interface AlgodApiGetDebugSettingsProfRequest {
}

export interface AlgodApiGetGenesisRequest {
}

export interface AlgodApiGetLedgerStateDeltaRequest {
    /**
     * The round for which the deltas are desired.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetLedgerStateDelta
     */
    round: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetLedgerStateDelta
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiGetLedgerStateDeltaForTransactionGroupRequest {
    /**
     * A transaction ID, or transaction group ID
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApigetLedgerStateDeltaForTransactionGroup
     */
    id: string
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetLedgerStateDeltaForTransactionGroup
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiGetLightBlockHeaderProofRequest {
    /**
     * The round to which the light block header belongs.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetLightBlockHeaderProof
     */
    round: number
}

export interface AlgodApiGetParticipationKeyByIDRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApigetParticipationKeyByID
     */
    participationId: string
}

export interface AlgodApiGetParticipationKeysRequest {
}

export interface AlgodApiGetPendingTransactionsRequest {
    /**
     * Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetPendingTransactions
     */
    max?: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetPendingTransactions
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiGetPendingTransactionsByAddressRequest {
    /**
     * An account public key
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApigetPendingTransactionsByAddress
     */
    address: string
    /**
     * Truncated number of transactions to display. If max&#x3D;0, returns all pending txns.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetPendingTransactionsByAddress
     */
    max?: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetPendingTransactionsByAddress
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiGetReadyRequest {
}

export interface AlgodApiGetStateProofRequest {
    /**
     * The round for which a state proof is desired.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetStateProof
     */
    round: number
}

export interface AlgodApiGetStatusRequest {
}

export interface AlgodApiGetSupplyRequest {
}

export interface AlgodApiGetSyncRoundRequest {
}

export interface AlgodApiGetTransactionGroupLedgerStateDeltasForRoundRequest {
    /**
     * The round for which the deltas are desired.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetTransactionGroupLedgerStateDeltasForRound
     */
    round: number
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetTransactionGroupLedgerStateDeltasForRound
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiGetTransactionProofRequest {
    /**
     * The round in which the transaction appears.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApigetTransactionProof
     */
    round: number
    /**
     * The transaction ID for which to generate a proof.
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApigetTransactionProof
     */
    txid: string
    /**
     * The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256
     * Defaults to: undefined
     * @type &#39;sha512_256&#39; | &#39;sha256&#39;
     * @memberof AlgodApigetTransactionProof
     */
    hashtype?: 'sha512_256' | 'sha256'
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApigetTransactionProof
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiGetVersionRequest {
}

export interface AlgodApiHealthCheckRequest {
}

export interface AlgodApiMetricsRequest {
}

export interface AlgodApiPendingTransactionInformationRequest {
    /**
     * A transaction ID
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApipendingTransactionInformation
     */
    txid: string
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApipendingTransactionInformation
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiPutDebugSettingsProfRequest {
}

export interface AlgodApiRawTransactionRequest {
    /**
     * The byte encoded signed transaction to broadcast to network
     * @type HttpFile
     * @memberof AlgodApirawTransaction
     */
    rawtxn: HttpFile
}

export interface AlgodApiRawTransactionAsyncRequest {
    /**
     * The byte encoded signed transaction to broadcast to network
     * @type HttpFile
     * @memberof AlgodApirawTransactionAsync
     */
    rawtxn: HttpFile
}

export interface AlgodApiSetBlockTimeStampOffsetRequest {
    /**
     * The timestamp offset for blocks in dev mode.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApisetBlockTimeStampOffset
     */
    offset: number
}

export interface AlgodApiSetSyncRoundRequest {
    /**
     * The round for which the deltas are desired.
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApisetSyncRound
     */
    round: number
}

export interface AlgodApiShutdownNodeRequest {
    /**
     * 
     * Defaults to: 0
     * @type number
     * @memberof AlgodApishutdownNode
     */
    timeout?: number
}

export interface AlgodApiSimulateTransactionRequest {
    /**
     * The transactions to simulate, along with any other inputs.
     * @type SimulateRequest
     * @memberof AlgodApisimulateTransaction
     */
    request: SimulateRequest
    /**
     * Configures whether the response object is JSON or MessagePack encoded. If not provided, defaults to JSON.
     * Defaults to: undefined
     * @type &#39;json&#39; | &#39;msgpack&#39;
     * @memberof AlgodApisimulateTransaction
     */
    format?: 'json' | 'msgpack'
}

export interface AlgodApiStartCatchupRequest {
    /**
     * A catch point
     * Defaults to: undefined
     * @type string
     * @memberof AlgodApistartCatchup
     */
    catchpoint: string
    /**
     * Specify the minimum number of blocks which the ledger must be advanced by in order to start the catchup. This is useful for simplifying tools which support fast catchup, they can run the catchup unconditionally and the node will skip the catchup if it is not needed.
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApistartCatchup
     */
    min?: number
}

export interface AlgodApiSwaggerJSONRequest {
}

export interface AlgodApiTealCompileRequest {
    /**
     * TEAL source code to be compiled
     * @type HttpFile
     * @memberof AlgodApitealCompile
     */
    source: HttpFile
    /**
     * When set to &#x60;true&#x60;, returns the source map of the program as a JSON. Defaults to &#x60;false&#x60;.
     * Defaults to: undefined
     * @type boolean
     * @memberof AlgodApitealCompile
     */
    sourcemap?: boolean
}

export interface AlgodApiTealDisassembleRequest {
    /**
     * TEAL program binary to be disassembled
     * @type string
     * @memberof AlgodApitealDisassemble
     */
    source: string
}

export interface AlgodApiTealDryrunRequest {
    /**
     * Transaction (or group) and any accompanying state-simulation data.
     * @type DryrunRequest
     * @memberof AlgodApitealDryrun
     */
    request?: DryrunRequest
}

export interface AlgodApiTransactionParamsRequest {
}

export interface AlgodApiUnsetSyncRoundRequest {
}

export interface AlgodApiWaitForBlockRequest {
    /**
     * The round to wait until returning status
     * Minimum: 0
     * Defaults to: undefined
     * @type number
     * @memberof AlgodApiwaitForBlock
     */
    round: number
}

export class ObjectAlgodApi {
    private api: ObservableAlgodApi

    public constructor(configuration: Configuration, requestFactory?: AlgodApiRequestFactory, responseProcessor?: AlgodApiResponseProcessor) {
        this.api = new ObservableAlgodApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param param the request object
     */
    public abortCatchupResponse(param: AlgodApiAbortCatchupRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.abortCatchupResponse(param.catchpoint,  options).toPromise();
    }

    /**
     * Given a catchpoint, it aborts catching up to this catchpoint
     * Aborts a catchpoint catchup.
     * @param param the request object
     */
    public abortCatchup(param: AlgodApiAbortCatchupRequest, options?: ConfigurationOptions): Promise<AbortCatchup200Response> {
        return this.api.abortCatchup(param.catchpoint,  options).toPromise();
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param param the request object
     */
    public accountApplicationInformationResponse(param: AlgodApiAccountApplicationInformationRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.accountApplicationInformationResponse(param.address, param.applicationId, param.format,  options).toPromise();
    }

    /**
     * Given a specific account public key and application ID, this call returns the account\'s application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application\'s creator.
     * Get account information about a given app.
     * @param param the request object
     */
    public accountApplicationInformation(param: AlgodApiAccountApplicationInformationRequest, options?: ConfigurationOptions): Promise<AccountApplicationInformation200Response> {
        return this.api.accountApplicationInformation(param.address, param.applicationId, param.format,  options).toPromise();
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param param the request object
     */
    public accountAssetInformationResponse(param: AlgodApiAccountAssetInformationRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.accountAssetInformationResponse(param.address, param.assetId, param.format,  options).toPromise();
    }

    /**
     * Given a specific account public key and asset ID, this call returns the account\'s asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset\'s creator.
     * Get account information about a given asset.
     * @param param the request object
     */
    public accountAssetInformation(param: AlgodApiAccountAssetInformationRequest, options?: ConfigurationOptions): Promise<AccountAssetInformation200Response> {
        return this.api.accountAssetInformation(param.address, param.assetId, param.format,  options).toPromise();
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param param the request object
     */
    public accountAssetsInformationResponse(param: AlgodApiAccountAssetsInformationRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.accountAssetsInformationResponse(param.address, param.limit, param.next,  options).toPromise();
    }

    /**
     * Lookup an account\'s asset holdings.
     * Get a list of assets held by an account, inclusive of asset params.
     * @param param the request object
     */
    public accountAssetsInformation(param: AlgodApiAccountAssetsInformationRequest, options?: ConfigurationOptions): Promise<AccountAssetsInformation200Response> {
        return this.api.accountAssetsInformation(param.address, param.limit, param.next,  options).toPromise();
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param param the request object
     */
    public accountInformationResponse(param: AlgodApiAccountInformationRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.accountInformationResponse(param.address, param.format, param.exclude,  options).toPromise();
    }

    /**
     * Given a specific account public key, this call returns the account\'s status, balance and spendable amounts
     * Get account information.
     * @param param the request object
     */
    public accountInformation(param: AlgodApiAccountInformationRequest, options?: ConfigurationOptions): Promise<Account> {
        return this.api.accountInformation(param.address, param.format, param.exclude,  options).toPromise();
    }

    /**
     * Add a participation key to the node
     * @param param the request object
     */
    public addParticipationKeyResponse(param: AlgodApiAddParticipationKeyRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.addParticipationKeyResponse(param.participationkey,  options).toPromise();
    }

    /**
     * Add a participation key to the node
     * @param param the request object
     */
    public addParticipationKey(param: AlgodApiAddParticipationKeyRequest, options?: ConfigurationOptions): Promise<AddParticipationKey200Response> {
        return this.api.addParticipationKey(param.participationkey,  options).toPromise();
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param param the request object
     */
    public appendKeysResponse(param: AlgodApiAppendKeysRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.appendKeysResponse(param.participationId, param.keymap,  options).toPromise();
    }

    /**
     * Given a participation ID, append state proof keys to a particular set of participation keys
     * Append state proof keys to a participation key
     * @param param the request object
     */
    public appendKeys(param: AlgodApiAppendKeysRequest, options?: ConfigurationOptions): Promise<ParticipationKey> {
        return this.api.appendKeys(param.participationId, param.keymap,  options).toPromise();
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param param the request object
     */
    public deleteParticipationKeyByIDResponse(param: AlgodApiDeleteParticipationKeyByIDRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.deleteParticipationKeyByIDResponse(param.participationId,  options).toPromise();
    }

    /**
     * Delete a given participation key by ID
     * Delete a given participation key by ID
     * @param param the request object
     */
    public deleteParticipationKeyByID(param: AlgodApiDeleteParticipationKeyByIDRequest, options?: ConfigurationOptions): Promise<void> {
        return this.api.deleteParticipationKeyByID(param.participationId,  options).toPromise();
    }

    /**
     * Returns OK if experimental API is enabled.
     * @param param the request object
     */
    public experimentalCheckResponse(param: AlgodApiExperimentalCheckRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.experimentalCheckResponse( options).toPromise();
    }

    /**
     * Returns OK if experimental API is enabled.
     * @param param the request object
     */
    public experimentalCheck(param: AlgodApiExperimentalCheckRequest = {}, options?: ConfigurationOptions): Promise<void> {
        return this.api.experimentalCheck( options).toPromise();
    }

    /**
     * Generate and install participation keys to the node.
     * @param param the request object
     */
    public generateParticipationKeysResponse(param: AlgodApiGenerateParticipationKeysRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.generateParticipationKeysResponse(param.address, param.first, param.last, param.dilution,  options).toPromise();
    }

    /**
     * Generate and install participation keys to the node.
     * @param param the request object
     */
    public generateParticipationKeys(param: AlgodApiGenerateParticipationKeysRequest, options?: ConfigurationOptions): Promise<string> {
        return this.api.generateParticipationKeys(param.address, param.first, param.last, param.dilution,  options).toPromise();
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param param the request object
     */
    public getApplicationBoxByNameResponse(param: AlgodApiGetApplicationBoxByNameRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getApplicationBoxByNameResponse(param.applicationId, param.name,  options).toPromise();
    }

    /**
     * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form \'encoding:value\'. For ints, use the form \'int:1234\'. For raw bytes, use the form \'b64:A==\'. For printable strings, use the form \'str:hello\'. For addresses, use the form \'addr:XYZ...\'.
     * Get box information for a given application.
     * @param param the request object
     */
    public getApplicationBoxByName(param: AlgodApiGetApplicationBoxByNameRequest, options?: ConfigurationOptions): Promise<Box> {
        return this.api.getApplicationBoxByName(param.applicationId, param.name,  options).toPromise();
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param param the request object
     */
    public getApplicationBoxesResponse(param: AlgodApiGetApplicationBoxesRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getApplicationBoxesResponse(param.applicationId, param.max,  options).toPromise();
    }

    /**
     * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
     * Get all box names for a given application.
     * @param param the request object
     */
    public getApplicationBoxes(param: AlgodApiGetApplicationBoxesRequest, options?: ConfigurationOptions): Promise<GetApplicationBoxes200Response> {
        return this.api.getApplicationBoxes(param.applicationId, param.max,  options).toPromise();
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param param the request object
     */
    public getApplicationByIDResponse(param: AlgodApiGetApplicationByIDRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getApplicationByIDResponse(param.applicationId,  options).toPromise();
    }

    /**
     * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
     * Get application information.
     * @param param the request object
     */
    public getApplicationByID(param: AlgodApiGetApplicationByIDRequest, options?: ConfigurationOptions): Promise<Application> {
        return this.api.getApplicationByID(param.applicationId,  options).toPromise();
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param param the request object
     */
    public getAssetByIDResponse(param: AlgodApiGetAssetByIDRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getAssetByIDResponse(param.assetId,  options).toPromise();
    }

    /**
     * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
     * Get asset information.
     * @param param the request object
     */
    public getAssetByID(param: AlgodApiGetAssetByIDRequest, options?: ConfigurationOptions): Promise<Asset> {
        return this.api.getAssetByID(param.assetId,  options).toPromise();
    }

    /**
     * Get the block for the given round.
     * @param param the request object
     */
    public getBlockResponse(param: AlgodApiGetBlockRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getBlockResponse(param.round, param.format, param.headerOnly,  options).toPromise();
    }

    /**
     * Get the block for the given round.
     * @param param the request object
     */
    public getBlock(param: AlgodApiGetBlockRequest, options?: ConfigurationOptions): Promise<GetBlock200Response> {
        return this.api.getBlock(param.round, param.format, param.headerOnly,  options).toPromise();
    }

    /**
     * Get the block hash for the block on the given round.
     * @param param the request object
     */
    public getBlockHashResponse(param: AlgodApiGetBlockHashRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getBlockHashResponse(param.round,  options).toPromise();
    }

    /**
     * Get the block hash for the block on the given round.
     * @param param the request object
     */
    public getBlockHash(param: AlgodApiGetBlockHashRequest, options?: ConfigurationOptions): Promise<GetBlockHash200Response> {
        return this.api.getBlockHash(param.round,  options).toPromise();
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param param the request object
     */
    public getBlockLogsResponse(param: AlgodApiGetBlockLogsRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getBlockLogsResponse(param.round,  options).toPromise();
    }

    /**
     * Get all of the logs from outer and inner app calls in the given round
     * Get all of the logs from outer and inner app calls in the given round
     * @param param the request object
     */
    public getBlockLogs(param: AlgodApiGetBlockLogsRequest, options?: ConfigurationOptions): Promise<GetBlockLogs200Response> {
        return this.api.getBlockLogs(param.round,  options).toPromise();
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     * @param param the request object
     */
    public getBlockTimeStampOffsetResponse(param: AlgodApiGetBlockTimeStampOffsetRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getBlockTimeStampOffsetResponse( options).toPromise();
    }

    /**
     * Gets the current timestamp offset.
     * Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
     * @param param the request object
     */
    public getBlockTimeStampOffset(param: AlgodApiGetBlockTimeStampOffsetRequest = {}, options?: ConfigurationOptions): Promise<GetBlockTimeStampOffset200Response> {
        return this.api.getBlockTimeStampOffset( options).toPromise();
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param param the request object
     */
    public getBlockTxidsResponse(param: AlgodApiGetBlockTxidsRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getBlockTxidsResponse(param.round,  options).toPromise();
    }

    /**
     * Get the top level transaction IDs for the block on the given round.
     * @param param the request object
     */
    public getBlockTxids(param: AlgodApiGetBlockTxidsRequest, options?: ConfigurationOptions): Promise<GetBlockTxids200Response> {
        return this.api.getBlockTxids(param.round,  options).toPromise();
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     * @param param the request object
     */
    public getConfigResponse(param: AlgodApiGetConfigRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getConfigResponse( options).toPromise();
    }

    /**
     * Returns the merged (defaults + overrides) config file in json.
     * Gets the merged config file.
     * @param param the request object
     */
    public getConfig(param: AlgodApiGetConfigRequest = {}, options?: ConfigurationOptions): Promise<string> {
        return this.api.getConfig( options).toPromise();
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     * @param param the request object
     */
    public getDebugSettingsProfResponse(param: AlgodApiGetDebugSettingsProfRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getDebugSettingsProfResponse( options).toPromise();
    }

    /**
     * Retrieves the current settings for blocking and mutex profiles
     * @param param the request object
     */
    public getDebugSettingsProf(param: AlgodApiGetDebugSettingsProfRequest = {}, options?: ConfigurationOptions): Promise<DebugSettingsProf> {
        return this.api.getDebugSettingsProf( options).toPromise();
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     * @param param the request object
     */
    public getGenesisResponse(param: AlgodApiGetGenesisRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getGenesisResponse( options).toPromise();
    }

    /**
     * Returns the entire genesis file in json.
     * Gets the genesis information.
     * @param param the request object
     */
    public getGenesis(param: AlgodApiGetGenesisRequest = {}, options?: ConfigurationOptions): Promise<Genesis> {
        return this.api.getGenesis( options).toPromise();
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param param the request object
     */
    public getLedgerStateDeltaResponse(param: AlgodApiGetLedgerStateDeltaRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getLedgerStateDeltaResponse(param.round, param.format,  options).toPromise();
    }

    /**
     * Get ledger deltas for a round.
     * Get a LedgerStateDelta object for a given round
     * @param param the request object
     */
    public getLedgerStateDelta(param: AlgodApiGetLedgerStateDeltaRequest, options?: ConfigurationOptions): Promise<any> {
        return this.api.getLedgerStateDelta(param.round, param.format,  options).toPromise();
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param param the request object
     */
    public getLedgerStateDeltaForTransactionGroupResponse(param: AlgodApiGetLedgerStateDeltaForTransactionGroupRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getLedgerStateDeltaForTransactionGroupResponse(param.id, param.format,  options).toPromise();
    }

    /**
     * Get a ledger delta for a given transaction group.
     * Get a LedgerStateDelta object for a given transaction group
     * @param param the request object
     */
    public getLedgerStateDeltaForTransactionGroup(param: AlgodApiGetLedgerStateDeltaForTransactionGroupRequest, options?: ConfigurationOptions): Promise<any> {
        return this.api.getLedgerStateDeltaForTransactionGroup(param.id, param.format,  options).toPromise();
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param param the request object
     */
    public getLightBlockHeaderProofResponse(param: AlgodApiGetLightBlockHeaderProofRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getLightBlockHeaderProofResponse(param.round,  options).toPromise();
    }

    /**
     * Gets a proof for a given light block header inside a state proof commitment
     * @param param the request object
     */
    public getLightBlockHeaderProof(param: AlgodApiGetLightBlockHeaderProofRequest, options?: ConfigurationOptions): Promise<LightBlockHeaderProof> {
        return this.api.getLightBlockHeaderProof(param.round,  options).toPromise();
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param param the request object
     */
    public getParticipationKeyByIDResponse(param: AlgodApiGetParticipationKeyByIDRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getParticipationKeyByIDResponse(param.participationId,  options).toPromise();
    }

    /**
     * Given a participation ID, return information about that participation key
     * Get participation key info given a participation ID
     * @param param the request object
     */
    public getParticipationKeyByID(param: AlgodApiGetParticipationKeyByIDRequest, options?: ConfigurationOptions): Promise<ParticipationKey> {
        return this.api.getParticipationKeyByID(param.participationId,  options).toPromise();
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     * @param param the request object
     */
    public getParticipationKeysResponse(param: AlgodApiGetParticipationKeysRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getParticipationKeysResponse( options).toPromise();
    }

    /**
     * Return a list of participation keys
     * Return a list of participation keys
     * @param param the request object
     */
    public getParticipationKeys(param: AlgodApiGetParticipationKeysRequest = {}, options?: ConfigurationOptions): Promise<Array<ParticipationKey>> {
        return this.api.getParticipationKeys( options).toPromise();
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param param the request object
     */
    public getPendingTransactionsResponse(param: AlgodApiGetPendingTransactionsRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getPendingTransactionsResponse(param.max, param.format,  options).toPromise();
    }

    /**
     * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool.
     * @param param the request object
     */
    public getPendingTransactions(param: AlgodApiGetPendingTransactionsRequest = {}, options?: ConfigurationOptions): Promise<GetPendingTransactionsByAddress200Response> {
        return this.api.getPendingTransactions(param.max, param.format,  options).toPromise();
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param param the request object
     */
    public getPendingTransactionsByAddressResponse(param: AlgodApiGetPendingTransactionsByAddressRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getPendingTransactionsByAddressResponse(param.address, param.max, param.format,  options).toPromise();
    }

    /**
     * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 
     * Get a list of unconfirmed transactions currently in the transaction pool by address.
     * @param param the request object
     */
    public getPendingTransactionsByAddress(param: AlgodApiGetPendingTransactionsByAddressRequest, options?: ConfigurationOptions): Promise<GetPendingTransactionsByAddress200Response> {
        return this.api.getPendingTransactionsByAddress(param.address, param.max, param.format,  options).toPromise();
    }

    /**
     * Returns OK if healthy and fully caught up.
     * @param param the request object
     */
    public getReadyResponse(param: AlgodApiGetReadyRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getReadyResponse( options).toPromise();
    }

    /**
     * Returns OK if healthy and fully caught up.
     * @param param the request object
     */
    public getReady(param: AlgodApiGetReadyRequest = {}, options?: ConfigurationOptions): Promise<void> {
        return this.api.getReady( options).toPromise();
    }

    /**
     * Get a state proof that covers a given round
     * @param param the request object
     */
    public getStateProofResponse(param: AlgodApiGetStateProofRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getStateProofResponse(param.round,  options).toPromise();
    }

    /**
     * Get a state proof that covers a given round
     * @param param the request object
     */
    public getStateProof(param: AlgodApiGetStateProofRequest, options?: ConfigurationOptions): Promise<StateProof> {
        return this.api.getStateProof(param.round,  options).toPromise();
    }

    /**
     * Gets the current node status.
     * @param param the request object
     */
    public getStatusResponse(param: AlgodApiGetStatusRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getStatusResponse( options).toPromise();
    }

    /**
     * Gets the current node status.
     * @param param the request object
     */
    public getStatus(param: AlgodApiGetStatusRequest = {}, options?: ConfigurationOptions): Promise<GetStatus200Response> {
        return this.api.getStatus( options).toPromise();
    }

    /**
     * Get the current supply reported by the ledger.
     * @param param the request object
     */
    public getSupplyResponse(param: AlgodApiGetSupplyRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getSupplyResponse( options).toPromise();
    }

    /**
     * Get the current supply reported by the ledger.
     * @param param the request object
     */
    public getSupply(param: AlgodApiGetSupplyRequest = {}, options?: ConfigurationOptions): Promise<GetSupply200Response> {
        return this.api.getSupply( options).toPromise();
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     * @param param the request object
     */
    public getSyncRoundResponse(param: AlgodApiGetSyncRoundRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getSyncRoundResponse( options).toPromise();
    }

    /**
     * Gets the minimum sync round for the ledger.
     * Returns the minimum sync round the ledger is keeping in cache.
     * @param param the request object
     */
    public getSyncRound(param: AlgodApiGetSyncRoundRequest = {}, options?: ConfigurationOptions): Promise<GetSyncRound200Response> {
        return this.api.getSyncRound( options).toPromise();
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param param the request object
     */
    public getTransactionGroupLedgerStateDeltasForRoundResponse(param: AlgodApiGetTransactionGroupLedgerStateDeltasForRoundRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getTransactionGroupLedgerStateDeltasForRoundResponse(param.round, param.format,  options).toPromise();
    }

    /**
     * Get ledger deltas for transaction groups in a given round.
     * Get LedgerStateDelta objects for all transaction groups in a given round
     * @param param the request object
     */
    public getTransactionGroupLedgerStateDeltasForRound(param: AlgodApiGetTransactionGroupLedgerStateDeltasForRoundRequest, options?: ConfigurationOptions): Promise<GetTransactionGroupLedgerStateDeltasForRound200Response> {
        return this.api.getTransactionGroupLedgerStateDeltasForRound(param.round, param.format,  options).toPromise();
    }

    /**
     * Get a proof for a transaction in a block.
     * @param param the request object
     */
    public getTransactionProofResponse(param: AlgodApiGetTransactionProofRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getTransactionProofResponse(param.round, param.txid, param.hashtype, param.format,  options).toPromise();
    }

    /**
     * Get a proof for a transaction in a block.
     * @param param the request object
     */
    public getTransactionProof(param: AlgodApiGetTransactionProofRequest, options?: ConfigurationOptions): Promise<GetTransactionProof200Response> {
        return this.api.getTransactionProof(param.round, param.txid, param.hashtype, param.format,  options).toPromise();
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     * @param param the request object
     */
    public getVersionResponse(param: AlgodApiGetVersionRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.getVersionResponse( options).toPromise();
    }

    /**
     * Retrieves the supported API versions, binary build versions, and genesis information.
     * @param param the request object
     */
    public getVersion(param: AlgodApiGetVersionRequest = {}, options?: ConfigurationOptions): Promise<Version> {
        return this.api.getVersion( options).toPromise();
    }

    /**
     * Returns OK if healthy.
     * @param param the request object
     */
    public healthCheckResponse(param: AlgodApiHealthCheckRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.healthCheckResponse( options).toPromise();
    }

    /**
     * Returns OK if healthy.
     * @param param the request object
     */
    public healthCheck(param: AlgodApiHealthCheckRequest = {}, options?: ConfigurationOptions): Promise<void> {
        return this.api.healthCheck( options).toPromise();
    }

    /**
     * Return metrics about algod functioning.
     * @param param the request object
     */
    public metricsResponse(param: AlgodApiMetricsRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.metricsResponse( options).toPromise();
    }

    /**
     * Return metrics about algod functioning.
     * @param param the request object
     */
    public metrics(param: AlgodApiMetricsRequest = {}, options?: ConfigurationOptions): Promise<void> {
        return this.api.metrics( options).toPromise();
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param param the request object
     */
    public pendingTransactionInformationResponse(param: AlgodApiPendingTransactionInformationRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.pendingTransactionInformationResponse(param.txid, param.format,  options).toPromise();
    }

    /**
     * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 
     * Get a specific pending transaction.
     * @param param the request object
     */
    public pendingTransactionInformation(param: AlgodApiPendingTransactionInformationRequest, options?: ConfigurationOptions): Promise<PendingTransactionResponse> {
        return this.api.pendingTransactionInformation(param.txid, param.format,  options).toPromise();
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     * @param param the request object
     */
    public putDebugSettingsProfResponse(param: AlgodApiPutDebugSettingsProfRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.putDebugSettingsProfResponse( options).toPromise();
    }

    /**
     * Enables blocking and mutex profiles, and returns the old settings
     * @param param the request object
     */
    public putDebugSettingsProf(param: AlgodApiPutDebugSettingsProfRequest = {}, options?: ConfigurationOptions): Promise<DebugSettingsProf> {
        return this.api.putDebugSettingsProf( options).toPromise();
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param param the request object
     */
    public rawTransactionResponse(param: AlgodApiRawTransactionRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.rawTransactionResponse(param.rawtxn,  options).toPromise();
    }

    /**
     * Broadcasts a raw transaction or transaction group to the network.
     * @param param the request object
     */
    public rawTransaction(param: AlgodApiRawTransactionRequest, options?: ConfigurationOptions): Promise<RawTransaction200Response> {
        return this.api.rawTransaction(param.rawtxn,  options).toPromise();
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param param the request object
     */
    public rawTransactionAsyncResponse(param: AlgodApiRawTransactionAsyncRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.rawTransactionAsyncResponse(param.rawtxn,  options).toPromise();
    }

    /**
     * Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
     * @param param the request object
     */
    public rawTransactionAsync(param: AlgodApiRawTransactionAsyncRequest, options?: ConfigurationOptions): Promise<void> {
        return this.api.rawTransactionAsync(param.rawtxn,  options).toPromise();
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param param the request object
     */
    public setBlockTimeStampOffsetResponse(param: AlgodApiSetBlockTimeStampOffsetRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.setBlockTimeStampOffsetResponse(param.offset,  options).toPromise();
    }

    /**
     * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
     * Given a timestamp offset in seconds, adds the offset to every subsequent block header\'s timestamp.
     * @param param the request object
     */
    public setBlockTimeStampOffset(param: AlgodApiSetBlockTimeStampOffsetRequest, options?: ConfigurationOptions): Promise<void> {
        return this.api.setBlockTimeStampOffset(param.offset,  options).toPromise();
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param param the request object
     */
    public setSyncRoundResponse(param: AlgodApiSetSyncRoundRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.setSyncRoundResponse(param.round,  options).toPromise();
    }

    /**
     * Sets the minimum sync round on the ledger.
     * Given a round, tells the ledger to keep that round in its cache.
     * @param param the request object
     */
    public setSyncRound(param: AlgodApiSetSyncRoundRequest, options?: ConfigurationOptions): Promise<void> {
        return this.api.setSyncRound(param.round,  options).toPromise();
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param param the request object
     */
    public shutdownNodeResponse(param: AlgodApiShutdownNodeRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.shutdownNodeResponse(param.timeout,  options).toPromise();
    }

    /**
     * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
     * @param param the request object
     */
    public shutdownNode(param: AlgodApiShutdownNodeRequest = {}, options?: ConfigurationOptions): Promise<any> {
        return this.api.shutdownNode(param.timeout,  options).toPromise();
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param param the request object
     */
    public simulateTransactionResponse(param: AlgodApiSimulateTransactionRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.simulateTransactionResponse(param.request, param.format,  options).toPromise();
    }

    /**
     * Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
     * @param param the request object
     */
    public simulateTransaction(param: AlgodApiSimulateTransactionRequest, options?: ConfigurationOptions): Promise<SimulateTransaction200Response> {
        return this.api.simulateTransaction(param.request, param.format,  options).toPromise();
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param param the request object
     */
    public startCatchupResponse(param: AlgodApiStartCatchupRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.startCatchupResponse(param.catchpoint, param.min,  options).toPromise();
    }

    /**
     * Given a catchpoint, it starts catching up to this catchpoint
     * Starts a catchpoint catchup.
     * @param param the request object
     */
    public startCatchup(param: AlgodApiStartCatchupRequest, options?: ConfigurationOptions): Promise<StartCatchup200Response> {
        return this.api.startCatchup(param.catchpoint, param.min,  options).toPromise();
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     * @param param the request object
     */
    public swaggerJSONResponse(param: AlgodApiSwaggerJSONRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.swaggerJSONResponse( options).toPromise();
    }

    /**
     * Returns the entire swagger spec in json.
     * Gets the current swagger spec.
     * @param param the request object
     */
    public swaggerJSON(param: AlgodApiSwaggerJSONRequest = {}, options?: ConfigurationOptions): Promise<string> {
        return this.api.swaggerJSON( options).toPromise();
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param param the request object
     */
    public tealCompileResponse(param: AlgodApiTealCompileRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.tealCompileResponse(param.source, param.sourcemap,  options).toPromise();
    }

    /**
     * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Compile TEAL source code to binary, produce its hash
     * @param param the request object
     */
    public tealCompile(param: AlgodApiTealCompileRequest, options?: ConfigurationOptions): Promise<TealCompile200Response> {
        return this.api.tealCompile(param.source, param.sourcemap,  options).toPromise();
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param param the request object
     */
    public tealDisassembleResponse(param: AlgodApiTealDisassembleRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.tealDisassembleResponse(param.source,  options).toPromise();
    }

    /**
     * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Disassemble program bytes into the TEAL source code.
     * @param param the request object
     */
    public tealDisassemble(param: AlgodApiTealDisassembleRequest, options?: ConfigurationOptions): Promise<TealDisassemble200Response> {
        return this.api.tealDisassemble(param.source,  options).toPromise();
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param param the request object
     */
    public tealDryrunResponse(param: AlgodApiTealDryrunRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.tealDryrunResponse(param.request,  options).toPromise();
    }

    /**
     * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node\'s configuration file sets EnableDeveloperAPI to true.
     * Provide debugging information for a transaction (or group).
     * @param param the request object
     */
    public tealDryrun(param: AlgodApiTealDryrunRequest = {}, options?: ConfigurationOptions): Promise<TealDryrun200Response> {
        return this.api.tealDryrun(param.request,  options).toPromise();
    }

    /**
     * Get parameters for constructing a new transaction
     * @param param the request object
     */
    public transactionParamsResponse(param: AlgodApiTransactionParamsRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.transactionParamsResponse( options).toPromise();
    }

    /**
     * Get parameters for constructing a new transaction
     * @param param the request object
     */
    public transactionParams(param: AlgodApiTransactionParamsRequest = {}, options?: ConfigurationOptions): Promise<TransactionParams200Response> {
        return this.api.transactionParams( options).toPromise();
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     * @param param the request object
     */
    public unsetSyncRoundResponse(param: AlgodApiUnsetSyncRoundRequest = {}, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.unsetSyncRoundResponse( options).toPromise();
    }

    /**
     * Unset the ledger sync round.
     * Removes minimum sync round restriction from the ledger.
     * @param param the request object
     */
    public unsetSyncRound(param: AlgodApiUnsetSyncRoundRequest = {}, options?: ConfigurationOptions): Promise<void> {
        return this.api.unsetSyncRound( options).toPromise();
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param param the request object
     */
    public waitForBlockResponse(param: AlgodApiWaitForBlockRequest, options?: ConfigurationOptions): Promise<ResponseContext> {
        return this.api.waitForBlockResponse(param.round,  options).toPromise();
    }

    /**
     * Waits for a block to appear after round {round} and returns the node\'s status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
     * Gets the node status after waiting for a round after the given round.
     * @param param the request object
     */
    public waitForBlock(param: AlgodApiWaitForBlockRequest, options?: ConfigurationOptions): Promise<GetStatus200Response> {
        return this.api.waitForBlock(param.round,  options).toPromise();
    }

}
