/**
 * Algod REST API.
 * API endpoint for algod operations.
 *
 * OpenAPI spec version: 0.0.1
 * Contact: contact@algorand.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { HttpFile } from '../http/http';

/**
* Describes an asset held by an account.  Definition: data/basics/userBalance.go : AssetHolding
*/
export class AssetHolding {
    /**
    * \\[a\\] number of units held.
    */
    'amount': number | bigint;
    /**
    * Asset ID of the holding.
    */
    'assetId': number;
    /**
    * \\[f\\] whether or not the holding is frozen.
    */
    'isFrozen': boolean;

    static readonly discriminator: string | undefined = undefined;

    static readonly mapping: {[index: string]: string} | undefined = undefined;

    static readonly attributeTypeMap: Array<{name: string, baseName: string, type: string, format: string}> = [
        {
            "name": "amount",
            "baseName": "amount",
            "type": "bigint",
            "format": ""
        },
        {
            "name": "assetId",
            "baseName": "asset-id",
            "type": "number",
            "format": ""
        },
        {
            "name": "isFrozen",
            "baseName": "is-frozen",
            "type": "boolean",
            "format": ""
        }    ];

    static getAttributeTypeMap() {
        return AssetHolding.attributeTypeMap;
    }

    public constructor() {
    }
}
