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
* Box name and its content.
*/
export class Box {
    /**
    * The round for which this information is relevant
    */
    'round': number;
    /**
    * \\[name\\] box name, base64 encoded
    */
    'name': string;
    /**
    * \\[value\\] box value, base64 encoded.
    */
    'value': string;

    static readonly discriminator: string | undefined = undefined;

    static readonly mapping: {[index: string]: string} | undefined = undefined;

    static readonly attributeTypeMap: Array<{name: string, baseName: string, type: string, format: string}> = [
        {
            "name": "round",
            "baseName": "round",
            "type": "number",
            "format": ""
        },
        {
            "name": "name",
            "baseName": "name",
            "type": "string",
            "format": "byte"
        },
        {
            "name": "value",
            "baseName": "value",
            "type": "string",
            "format": "byte"
        }    ];

    static getAttributeTypeMap() {
        return Box.attributeTypeMap;
    }

    public constructor() {
    }
}
