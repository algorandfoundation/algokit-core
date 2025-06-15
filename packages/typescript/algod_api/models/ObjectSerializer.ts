export * from '../models/AbortCatchup200Response';
export * from '../models/Account';
export * from '../models/AccountApplicationInformation200Response';
export * from '../models/AccountAssetHolding';
export * from '../models/AccountAssetInformation200Response';
export * from '../models/AccountAssetsInformation200Response';
export * from '../models/AccountParticipation';
export * from '../models/AccountStateDelta';
export * from '../models/AddParticipationKey200Response';
export * from '../models/AppCallLogs';
export * from '../models/Application';
export * from '../models/ApplicationInitialStates';
export * from '../models/ApplicationKVStorage';
export * from '../models/ApplicationLocalReference';
export * from '../models/ApplicationLocalState';
export * from '../models/ApplicationParams';
export * from '../models/ApplicationStateOperation';
export * from '../models/ApplicationStateSchema';
export * from '../models/Asset';
export * from '../models/AssetHolding';
export * from '../models/AssetHoldingReference';
export * from '../models/AssetParams';
export * from '../models/AvmKeyValue';
export * from '../models/AvmValue';
export * from '../models/Box';
export * from '../models/BoxDescriptor';
export * from '../models/BoxReference';
export * from '../models/BuildVersion';
export * from '../models/DebugSettingsProf';
export * from '../models/DryrunRequest';
export * from '../models/DryrunSource';
export * from '../models/DryrunState';
export * from '../models/DryrunTxnResult';
export * from '../models/ErrorResponse';
export * from '../models/EvalDelta';
export * from '../models/EvalDeltaKeyValue';
export * from '../models/Genesis';
export * from '../models/GenesisAllocation';
export * from '../models/GenesisAllocationState';
export * from '../models/GetApplicationBoxes200Response';
export * from '../models/GetBlock200Response';
export * from '../models/GetBlockHash200Response';
export * from '../models/GetBlockLogs200Response';
export * from '../models/GetBlockTimeStampOffset200Response';
export * from '../models/GetBlockTxids200Response';
export * from '../models/GetPendingTransactionsByAddress200Response';
export * from '../models/GetStatus200Response';
export * from '../models/GetSupply200Response';
export * from '../models/GetSyncRound200Response';
export * from '../models/GetTransactionGroupLedgerStateDeltasForRound200Response';
export * from '../models/GetTransactionProof200Response';
export * from '../models/KvDelta';
export * from '../models/LedgerStateDeltaForTransactionGroup';
export * from '../models/LightBlockHeaderProof';
export * from '../models/ParticipationKey';
export * from '../models/PendingTransactionResponse';
export * from '../models/RawTransaction200Response';
export * from '../models/ScratchChange';
export * from '../models/SimulateInitialStates';
export * from '../models/SimulateRequest';
export * from '../models/SimulateRequestTransactionGroup';
export * from '../models/SimulateTraceConfig';
export * from '../models/SimulateTransaction200Response';
export * from '../models/SimulateTransactionGroupResult';
export * from '../models/SimulateTransactionResult';
export * from '../models/SimulateUnnamedResourcesAccessed';
export * from '../models/SimulationEvalOverrides';
export * from '../models/SimulationOpcodeTraceUnit';
export * from '../models/SimulationTransactionExecTrace';
export * from '../models/StartCatchup200Response';
export * from '../models/StateProof';
export * from '../models/StateProofMessage';
export * from '../models/TealCompile200Response';
export * from '../models/TealDisassemble200Response';
export * from '../models/TealDryrun200Response';
export * from '../models/TealKeyValue';
export * from '../models/TealValue';
export * from '../models/TransactionParams200Response';
export * from '../models/Version';

import { AbortCatchup200Response } from '../models/AbortCatchup200Response';
import { Account                       , AccountSigTypeEnum      } from '../models/Account';
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
import { GetTransactionProof200Response    , GetTransactionProof200ResponseHashtypeEnum   } from '../models/GetTransactionProof200Response';
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
import { decodeMsgpackToJson, encodeJsonToMsgpack, ModelType, supportedModels } from "@algorandfoundation/algokit-transact";
import { stringifyJSON, parseJSON, IntDecoding } from "../bigint-utils";

/* tslint:disable:no-unused-variable */
let primitives = [
                    "string",
                    "boolean",
                    "double",
                    "integer",
                    "long",
                    "float",
                    "number",
                    "bigint",
                    "any"
                 ];

let enumsMap: Set<string> = new Set<string>([
    "AccountSigTypeEnum",
    "GetTransactionProof200ResponseHashtypeEnum",
]);

let typeMap: {[index: string]: any} = {
    "AbortCatchup200Response": AbortCatchup200Response,
    "Account": Account,
    "AccountApplicationInformation200Response": AccountApplicationInformation200Response,
    "AccountAssetHolding": AccountAssetHolding,
    "AccountAssetInformation200Response": AccountAssetInformation200Response,
    "AccountAssetsInformation200Response": AccountAssetsInformation200Response,
    "AccountParticipation": AccountParticipation,
    "AccountStateDelta": AccountStateDelta,
    "AddParticipationKey200Response": AddParticipationKey200Response,
    "AppCallLogs": AppCallLogs,
    "Application": Application,
    "ApplicationInitialStates": ApplicationInitialStates,
    "ApplicationKVStorage": ApplicationKVStorage,
    "ApplicationLocalReference": ApplicationLocalReference,
    "ApplicationLocalState": ApplicationLocalState,
    "ApplicationParams": ApplicationParams,
    "ApplicationStateOperation": ApplicationStateOperation,
    "ApplicationStateSchema": ApplicationStateSchema,
    "Asset": Asset,
    "AssetHolding": AssetHolding,
    "AssetHoldingReference": AssetHoldingReference,
    "AssetParams": AssetParams,
    "AvmKeyValue": AvmKeyValue,
    "AvmValue": AvmValue,
    "Box": Box,
    "BoxDescriptor": BoxDescriptor,
    "BoxReference": BoxReference,
    "BuildVersion": BuildVersion,
    "DebugSettingsProf": DebugSettingsProf,
    "DryrunRequest": DryrunRequest,
    "DryrunSource": DryrunSource,
    "DryrunState": DryrunState,
    "DryrunTxnResult": DryrunTxnResult,
    "ErrorResponse": ErrorResponse,
    "EvalDelta": EvalDelta,
    "EvalDeltaKeyValue": EvalDeltaKeyValue,
    "Genesis": Genesis,
    "GenesisAllocation": GenesisAllocation,
    "GenesisAllocationState": GenesisAllocationState,
    "GetApplicationBoxes200Response": GetApplicationBoxes200Response,
    "GetBlock200Response": GetBlock200Response,
    "GetBlockHash200Response": GetBlockHash200Response,
    "GetBlockLogs200Response": GetBlockLogs200Response,
    "GetBlockTimeStampOffset200Response": GetBlockTimeStampOffset200Response,
    "GetBlockTxids200Response": GetBlockTxids200Response,
    "GetPendingTransactionsByAddress200Response": GetPendingTransactionsByAddress200Response,
    "GetStatus200Response": GetStatus200Response,
    "GetSupply200Response": GetSupply200Response,
    "GetSyncRound200Response": GetSyncRound200Response,
    "GetTransactionGroupLedgerStateDeltasForRound200Response": GetTransactionGroupLedgerStateDeltasForRound200Response,
    "GetTransactionProof200Response": GetTransactionProof200Response,
    "KvDelta": KvDelta,
    "LedgerStateDeltaForTransactionGroup": LedgerStateDeltaForTransactionGroup,
    "LightBlockHeaderProof": LightBlockHeaderProof,
    "ParticipationKey": ParticipationKey,
    "PendingTransactionResponse": PendingTransactionResponse,
    "RawTransaction200Response": RawTransaction200Response,
    "ScratchChange": ScratchChange,
    "SimulateInitialStates": SimulateInitialStates,
    "SimulateRequest": SimulateRequest,
    "SimulateRequestTransactionGroup": SimulateRequestTransactionGroup,
    "SimulateTraceConfig": SimulateTraceConfig,
    "SimulateTransaction200Response": SimulateTransaction200Response,
    "SimulateTransactionGroupResult": SimulateTransactionGroupResult,
    "SimulateTransactionResult": SimulateTransactionResult,
    "SimulateUnnamedResourcesAccessed": SimulateUnnamedResourcesAccessed,
    "SimulationEvalOverrides": SimulationEvalOverrides,
    "SimulationOpcodeTraceUnit": SimulationOpcodeTraceUnit,
    "SimulationTransactionExecTrace": SimulationTransactionExecTrace,
    "StartCatchup200Response": StartCatchup200Response,
    "StateProof": StateProof,
    "StateProofMessage": StateProofMessage,
    "TealCompile200Response": TealCompile200Response,
    "TealDisassemble200Response": TealDisassemble200Response,
    "TealDryrun200Response": TealDryrun200Response,
    "TealKeyValue": TealKeyValue,
    "TealValue": TealValue,
    "TransactionParams200Response": TransactionParams200Response,
    "Version": Version,
}

type MimeTypeDescriptor = {
    type: string;
    subtype: string;
    subtypeTokens: string[];
};

/**
 * Every mime-type consists of a type, subtype, and optional parameters.
 * The subtype can be composite, including information about the content format.
 * For example: `application/json-patch+json`, `application/merge-patch+json`.
 *
 * This helper transforms a string mime-type into an internal representation.
 * This simplifies the implementation of predicates that in turn define common rules for parsing or stringifying
 * the payload.
 */
const parseMimeType = (mimeType: string): MimeTypeDescriptor => {
    const [type = '', subtype = ''] = mimeType.split('/');
    return {
        type,
        subtype,
        subtypeTokens: subtype.split('+'),
    };
};

type MimeTypePredicate = (mimeType: string) => boolean;

// This factory creates a predicate function that checks a string mime-type against defined rules.
const mimeTypePredicateFactory = (predicate: (descriptor: MimeTypeDescriptor) => boolean): MimeTypePredicate => (mimeType) => predicate(parseMimeType(mimeType));

// Use this factory when you need to define a simple predicate based only on type and, if applicable, subtype.
const mimeTypeSimplePredicateFactory = (type: string, subtype?: string): MimeTypePredicate => mimeTypePredicateFactory((descriptor) => {
    if (descriptor.type !== type) return false;
    if (subtype != null && descriptor.subtype !== subtype) return false;
    return true;
});

// Creating a set of named predicates that will help us determine how to handle different mime-types
const isTextLikeMimeType = mimeTypeSimplePredicateFactory('text');
const isJsonMimeType = mimeTypeSimplePredicateFactory('application', 'json');
const isJsonLikeMimeType = mimeTypePredicateFactory((descriptor) => descriptor.type === 'application' && descriptor.subtypeTokens.some((item) => item === 'json'));
const isOctetStreamMimeType = mimeTypeSimplePredicateFactory('application', 'octet-stream');
const isFormUrlencodedMimeType = mimeTypeSimplePredicateFactory('application', 'x-www-form-urlencoded');
const isBinaryMimeType = mimeTypeSimplePredicateFactory('application', 'x-binary');
const isMsgpackMimeType = mimeTypePredicateFactory((descriptor) => descriptor.subtypeTokens.indexOf('msgpack') !== -1 || descriptor.subtype === 'msgpack');

// Defining a list of mime-types in the order of prioritization for handling.
const supportedMimeTypePredicatesWithPriority: MimeTypePredicate[] = [
    isMsgpackMimeType,
    isJsonMimeType,
    isJsonLikeMimeType,
    isTextLikeMimeType,
    isOctetStreamMimeType,
    isBinaryMimeType,
    isFormUrlencodedMimeType,
];

const nullableSuffix = " | null";
const optionalSuffix = " | undefined";
const arrayPrefix = "Array<";
const arraySuffix = ">";
const mapPrefix = "{ [key: string]: ";
const mapSuffix = "; }";

export class ObjectSerializer {
    public static findCorrectType(data: any, expectedType: string) {
        if (data == undefined) {
            return expectedType;
        } else if (primitives.indexOf(expectedType.toLowerCase()) !== -1) {
            return expectedType;
        } else if (expectedType === "Date") {
            return expectedType;
        } else {
            if (enumsMap.has(expectedType)) {
                return expectedType;
            }

            if (!typeMap[expectedType]) {
                return expectedType; // w/e we don't know the type
            }

            // Check the discriminator
            let discriminatorProperty = typeMap[expectedType].discriminator;
            if (discriminatorProperty == null) {
                return expectedType; // the type does not have a discriminator. use it.
            } else {
                if (data[discriminatorProperty]) {
                    var discriminatorType = data[discriminatorProperty];
                    let mapping = typeMap[expectedType].mapping;
                    if (mapping != undefined && mapping[discriminatorType]) {
                        return mapping[discriminatorType]; // use the type given in the discriminator
                    } else if(typeMap[discriminatorType]) {
                        return discriminatorType;
                    } else {
                        return expectedType; // discriminator did not map to a type
                    }
                } else {
                    return expectedType; // discriminator was not present (or an empty string)
                }
            }
        }
    }

    public static serialize(data: any, type: string, format: string): any {
        if (data == undefined) {
            return data;
        } else if (primitives.indexOf(type.toLowerCase()) !== -1) {
            return data;
        } else if (type.endsWith(nullableSuffix)) {
            let subType: string = type.slice(0, -nullableSuffix.length); // Type | null => Type
            return ObjectSerializer.serialize(data, subType, format);
        } else if (type.endsWith(optionalSuffix)) {
            let subType: string = type.slice(0, -optionalSuffix.length); // Type | undefined => Type
            return ObjectSerializer.serialize(data, subType, format);
        } else if (type.startsWith(arrayPrefix)) {
            let subType: string = type.slice(arrayPrefix.length, -arraySuffix.length); // Array<Type> => Type
            let transformedData: any[] = [];
            for (let date of data) {
                transformedData.push(ObjectSerializer.serialize(date, subType, format));
            }
            return transformedData;
        } else if (type.startsWith(mapPrefix)) {
            let subType: string = type.slice(mapPrefix.length, -mapSuffix.length); // { [key: string]: Type; } => Type
            let transformedData: { [key: string]: any } = {};
            for (let key in data) {
                transformedData[key] = ObjectSerializer.serialize(
                    data[key],
                    subType,
                    format,
                );
            }
            return transformedData;
        } else if (type === "Date") {
            if (format == "date") {
                let month = data.getMonth()+1
                month = month < 10 ? "0" + month.toString() : month.toString()
                let day = data.getDate();
                day = day < 10 ? "0" + day.toString() : day.toString();

                return data.getFullYear() + "-" + month + "-" + day;
            } else {
                return data.toISOString();
            }
        } else {
            if (enumsMap.has(type)) {
                return data;
            }
            if (!typeMap[type]) { // in case we dont know the type
                return data;
            }

            // Get the actual type of this object
            type = this.findCorrectType(data, type);

            // get the map for the correct type.
            let attributeTypes = typeMap[type].getAttributeTypeMap();
            let instance: {[index: string]: any} = {};
            for (let attributeType of attributeTypes) {
                instance[attributeType.baseName] = ObjectSerializer.serialize(data[attributeType.name], attributeType.type, attributeType.format);
            }
            return instance;
        }
    }

    public static deserialize(data: any, type: string, format: string): any {
        // polymorphism may change the actual type.
        type = ObjectSerializer.findCorrectType(data, type);
        if (data == undefined) {
            return data;
        } else if (primitives.indexOf(type.toLowerCase()) !== -1) {
            return data;
        } else if (type.endsWith(nullableSuffix)) {
            let subType: string = type.slice(0, -nullableSuffix.length); // Type | null => Type
            return ObjectSerializer.deserialize(data, subType, format);
        } else if (type.endsWith(optionalSuffix)) {
            let subType: string = type.slice(0, -optionalSuffix.length); // Type | undefined => Type
            return ObjectSerializer.deserialize(data, subType, format);
        } else if (type.startsWith(arrayPrefix)) {
            let subType: string = type.slice(arrayPrefix.length, -arraySuffix.length); // Array<Type> => Type
            let transformedData: any[] = [];
            for (let date of data) {
                transformedData.push(ObjectSerializer.deserialize(date, subType, format));
            }
            return transformedData;
        } else if (type.startsWith(mapPrefix)) {
            let subType: string = type.slice(mapPrefix.length, -mapSuffix.length); // { [key: string]: Type; } => Type
            let transformedData: { [key: string]: any } = {};
            for (let key in data) {
                transformedData[key] = ObjectSerializer.deserialize(
                    data[key],
                    subType,
                    format,
                );
            }
            return transformedData;
        } else if (type === "Date") {
            return new Date(data);
        } else {
            if (enumsMap.has(type)) {// is Enum
                return data;
            }

            if (!typeMap[type]) { // dont know the type
                return data;
            }
            let instance = new typeMap[type]();
            let attributeTypes = typeMap[type].getAttributeTypeMap();
            for (let attributeType of attributeTypes) {
                let value = ObjectSerializer.deserialize(data[attributeType.baseName], attributeType.type, attributeType.format);
                if (value !== undefined) {
                    instance[attributeType.name] = value;
                }
            }
            return instance;
        }
    }


    /**
     * Normalize media type
     *
     * We currently do not handle any media types attributes, i.e. anything
     * after a semicolon. All content is assumed to be UTF-8 compatible.
     */
    public static normalizeMediaType(mediaType: string | undefined): string | undefined {
        if (mediaType === undefined) {
            return undefined;
        }
        return (mediaType.split(";")[0] ?? '').trim().toLowerCase();
    }

    /**
     * From a list of possible media types, choose the one we can handle best.
     *
     * The order of the given media types does not have any impact on the choice
     * made.
     */
    public static getPreferredMediaType(mediaTypes: Array<string>): string {
        /** According to OAS 3 we should default to json */
        if (mediaTypes.length === 0) {
            return "application/json";
        }

        const normalMediaTypes = mediaTypes.map(ObjectSerializer.normalizeMediaType);

        for (const predicate of supportedMimeTypePredicatesWithPriority) {
            for (const mediaType of normalMediaTypes) {
                if (mediaType != null && predicate(mediaType)) {
                    return mediaType;
                }
            }
        }

        throw new Error("None of the given media types are supported: " + mediaTypes.join(", "));
    }

    /**
     * Convert data to a string according the given media type
     */
    public static stringify(data: any, mediaType: string, typeHint?: string): any {
        if (isTextLikeMimeType(mediaType)) {
            return String(data);
        }

        if (isJsonLikeMimeType(mediaType)) {
            // Use BigInt-aware JSON.stringify for JSON-like content
            return stringifyJSON(data);
        }

        if (isOctetStreamMimeType(mediaType) || isBinaryMimeType(mediaType)) {
            // For binary types, return the data as is (assuming it's already in the correct format, e.g., BufferSource)
            return data;
        }

        if (isMsgpackMimeType(mediaType)) {
            try {
                // Use BigInt-aware JSON.stringify for msgpack
                const jsonStr = stringifyJSON(data);
                const modelEnum = typeHint && (supportedModels() as unknown as string[]).indexOf(typeHint) !== -1
                    ? typeHint as ModelType
                    : undefined;

                return modelEnum ? encodeJsonToMsgpack(modelEnum, jsonStr) : jsonStr;
            } catch (err) {
                console.error(err);
                // Fall through to default handling below
            }
        }

        throw new Error("The mediaType " + mediaType + " is not supported by ObjectSerializer.stringify.");
    }

    /**
     * Parse data from a payload according to the given media type.
     *
     * `rawData` can be either a string (for text-like payloads) **or** a binary container
     * (e.g. `Uint8Array`, `ArrayBuffer`, `Buffer`) when dealing with binary formats such as msgpack.
     *
     * `typeHint` is an optional fully-qualified model name used to help the msgpack
     * codec choose the correct Algokit schema when decoding.
     */
    public static parse(rawData: any, mediaType: string | undefined, typeHint?: string) {
        if (mediaType === undefined) {
            throw new Error("Cannot parse content. No Content-Type defined.");
        }

        // MsgPack handling — this comes first so that we do not attempt to treat
        // the binary payload as text.
        if (isMsgpackMimeType(mediaType)) {
            try {
                let bytes: Uint8Array;

                if (rawData instanceof Uint8Array) {
                    bytes = rawData;
                } else if (rawData instanceof ArrayBuffer) {
                    bytes = new Uint8Array(rawData);
                } else if (typeof Blob !== "undefined" && rawData instanceof Blob) {
                    // Synchronous parsing of a Blob is impossible; callers should convert
                    // the Blob to an ArrayBuffer first.
                    throw new Error("Blob instances must be converted to ArrayBuffer or Uint8Array before calling ObjectSerializer.parse.");
                } else {
                    // Attempt best-effort conversion for other array-like objects.
                    bytes = Uint8Array.from(rawData as any);
                }

                const modelEnum = typeHint && (supportedModels() as unknown as string[]).indexOf(typeHint) !== -1
                    ? typeHint as ModelType
                    : undefined;

                if (!modelEnum) {
                    throw new Error("No model enum found for type hint: " + typeHint);
                }

                const jsonStr = decodeMsgpackToJson(modelEnum, bytes);
                // Use BigInt-aware JSON.parse for msgpack decoded JSON
                return parseJSON(jsonStr, { intDecoding: IntDecoding.MIXED });
            } catch (err) {
                // Fall through to the default handling below so that we can surface
                // a consistent error message should msgpack decoding fail.
                console.error(err);
            }
        }

        if (isTextLikeMimeType(mediaType)) {
            return String(rawData);
        }

        if (isJsonLikeMimeType(mediaType)) {
            // Use BigInt-aware JSON.parse for JSON-like content
            const jsonString = typeof rawData === "string" ? rawData : String(rawData);
            return parseJSON(jsonString, { intDecoding: IntDecoding.MIXED });
        }

        if (isOctetStreamMimeType(mediaType) || isBinaryMimeType(mediaType)) {
            // Parsing binary data might depend on the context,
            // often it's handled directly by the HTTP library (e.g., as Blob or ArrayBuffer)
            // Returning rawData might be appropriate if downstream code expects a string representation,
            // but often binary data shouldn't be parsed back into a string here.
            // For now, we'll just return it, but this might need adjustment based on usage.
            return rawData;
        }

        throw new Error("The mediaType " + mediaType + " is not supported by ObjectSerializer.parse.");
    }
}
