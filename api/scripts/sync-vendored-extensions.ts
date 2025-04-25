#!/usr/bin/env bun

/**
 * OpenAPI Specification Extension Helper
 *
 * This script helps to add vendor extensions to the OpenAPI specification for
 * special case handling in the Mustache templates.
 *
 * Usage:
 *    bun run scripts/sync-vendored-extensions.ts <input_spec.json> <output_spec.json>
 */

import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';

interface OpenAPISpec {
  paths?: {
    [path: string]: {
      [method: string]: {
        [key: string]: any;
      };
    };
  };
  [key: string]: any;
}

/**
 * Add endpoint-specific extensions to the OpenAPI specification.
 * 
 * @param spec - The OpenAPI specification
 * @returns The modified OpenAPI specification
 */
function addEndpointExtensions(spec: OpenAPISpec): OpenAPISpec {
  // Add extensions for specific endpoints
  if (spec.paths) {
    // Handle /v2/transactions POST endpoint
    if (spec.paths['/v2/transactions'] && spec.paths['/v2/transactions']['post']) {
      const endpoint = spec.paths['/v2/transactions']['post'];
      // Use a single extension to identify the endpoint type
      endpoint['x-algorand-endpoint-type'] = 'raw-transaction';
    }

    // Additional endpoint special cases can be added here
    // Example: Handle /v2/accounts GET endpoint
    // if (spec.paths['/v2/accounts'] && spec.paths['/v2/accounts']['get']) {
    //   const endpoint = spec.paths['/v2/accounts']['get'];
    //   endpoint['x-algorand-endpoint-type'] = 'account';
    // }
  }

  return spec;
}

/**
 * Main function to process the OpenAPI specification.
 */
async function main() {
  const args = process.argv.slice(2);
  
  if (args.length !== 2) {
    console.error(`Usage: bun run ${process.argv[1]} <input_spec.json> <output_spec.json>`);
    process.exit(1);
  }

  const inputFile = resolve(args[0]);
  const outputFile = resolve(args[1]);

  try {
    // Read the input specification
    const spec = JSON.parse(readFileSync(inputFile, 'utf-8'));

    // Add extensions
    const modifiedSpec = addEndpointExtensions(spec);

    // Write the output specification
    writeFileSync(outputFile, JSON.stringify(modifiedSpec, null, 2));

    console.log(`Successfully added extensions to ${outputFile}`);
  } catch (error) {
    console.error(`Error processing specification: ${error}`);
    process.exit(1);
  }
}

// Run the main function
main().catch(console.error); 
