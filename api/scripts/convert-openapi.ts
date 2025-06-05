#!/usr/bin/env bun

import { writeFileSync, mkdirSync } from "fs";
import { join, dirname } from "path";
import SwaggerParser from "@apidevtools/swagger-parser";

// ===== TYPES =====

interface OpenAPISpec {
  openapi?: string;
  swagger?: string;
  info?: any;
  paths?: Record<string, any>;
  components?: {
    schemas?: Record<string, any>;
    responses?: Record<string, any>;
    parameters?: Record<string, any>;
    [key: string]: any;
  };
  [key: string]: any;
}

interface VendorExtensionTransform {
  sourceProperty: string;  // e.g., "x-algorand-format" or "format"
  sourceValue: string;     // e.g., "uint64"
  targetProperty: string;  // e.g., "x-algokit-bigint"
  targetValue: boolean;    // value to set
  removeSource?: boolean;  // whether to remove the source property (default false)
}

interface ProcessingConfig {
  sourceUrl: string;
  outputPath: string;
  converterEndpoint?: string;
  indent?: number;
  vendorExtensionTransforms?: VendorExtensionTransform[];
}

// ===== TRANSFORMATIONS =====

// Known missing descriptions to auto-fix
const MISSING_DESCRIPTIONS = new Map([
  // Component responses
  ['components.responses.NodeStatusResponse.description', 'Returns the current status of the node'],
  ['components.responses.CatchpointStartResponse.description', 'Catchpoint start operation response'],
  ['components.responses.CatchpointAbortResponse.description', 'Catchpoint abort operation response'],

  // Path responses
  ["paths.'/v2/transactions/async'(post).responses.200.description", 'Transaction successfully submitted for asynchronous processing'],
  ["paths.'/v2/status'(get).responses.200.description", 'Returns the current node status including sync status, version, and latest round'],
  ["paths.'/v2/catchup/{catchpoint}'(post).responses.200.description", 'Catchpoint operation started successfully'],
  ["paths.'/v2/catchup/{catchpoint}'(post).responses.201.description", 'Catchpoint operation created and started successfully'],
  ["paths.'/v2/catchup/{catchpoint}'(delete).responses.200.description", 'Catchpoint operation aborted successfully'],
  ["paths.'/v2/ledger/sync/{round}'(post).responses.200.description", 'Ledger sync to specified round initiated successfully'],
  ["paths.'/v2/shutdown'(post).responses.200.description", 'Node shutdown initiated successfully'],
  ["paths.'/v2/status/wait-for-block-after/{round}'(get).responses.200.description", 'Returns node status after the specified round is reached'],
  ["paths.'/v2/ledger/sync'(delete).responses.200.description", 'Ledger sync operation stopped successfully'],
]);

/**
 * Find and fix missing descriptions in the spec
 */
function fixMissingDescriptions(spec: OpenAPISpec): number {
  let fixedCount = 0;
  const missingPaths: string[] = [];

  // Check component responses
  if (spec.components?.responses) {
    for (const [name, response] of Object.entries(spec.components.responses)) {
      if (response && typeof response === 'object' && !response.description) {
        const path = `components.responses.${name}.description`;
        const description = MISSING_DESCRIPTIONS.get(path);

        if (description) {
          response.description = description;
          fixedCount++;
        } else {
          missingPaths.push(path);
        }
      }
    }
  }

  // Check path responses
  if (spec.paths) {
    for (const [pathName, pathObj] of Object.entries(spec.paths)) {
      if (!pathObj || typeof pathObj !== 'object') continue;

      const methods = ['get', 'post', 'put', 'delete', 'patch', 'head', 'options', 'trace'];

      for (const method of methods) {
        const operation = pathObj[method];
        if (!operation?.responses) continue;

        for (const [statusCode, response] of Object.entries(operation.responses)) {
          if (response && typeof response === 'object' && !(response as any).description) {
            const path = `paths.'${pathName}'(${method}).responses.${statusCode}.description`;
            const description = MISSING_DESCRIPTIONS.get(path);

            if (description) {
              (response as any).description = description;
              fixedCount++;
            } else {
              missingPaths.push(path);
            }
          }
        }
      }
    }
  }

  // Report new missing descriptions
  if (missingPaths.length > 0) {
    console.warn(`⚠️  Found ${missingPaths.length} new missing descriptions:`);
    missingPaths.forEach(path => console.warn(`  - ${path}`));
  }

  return fixedCount;
}

/**
 * Fix pydantic recursion error by removing format: byte from AvmValue schema
 */
function fixPydanticRecursionError(spec: OpenAPISpec): number {
  let fixedCount = 0;

  // Check if AvmValue schema exists
  if (spec.components?.schemas?.AvmValue) {
    const avmValue = spec.components.schemas.AvmValue;

    // Check if it has properties.bytes with format: "byte"
    if (avmValue.properties?.bytes?.format === "byte") {
      delete avmValue.properties.bytes.format;
      fixedCount++;
      console.log('ℹ️  Removed format: "byte" from AvmValue.properties.bytes to fix pydantic recursion error');
    }
  }

  return fixedCount;
}

/**
 * Transform vendor extensions throughout the spec
 */
function transformVendorExtensions(spec: OpenAPISpec, transforms: VendorExtensionTransform[]): Record<string, number> {
  const transformCounts: Record<string, number> = {};

  // Initialize counts
  transforms.forEach(t => transformCounts[`${t.sourceProperty}:${t.sourceValue}`] = 0);

  const transform = (obj: any): void => {
    if (!obj || typeof obj !== 'object') return;

    // Check each configured transformation
    for (const transform of transforms) {
      if (obj[transform.sourceProperty] === transform.sourceValue) {
        // Add/set the target property
        obj[transform.targetProperty] = transform.targetValue;

        // Remove source property if configured to do so
        if (transform.removeSource) {
          delete obj[transform.sourceProperty];
        }

        // Increment count
        const countKey = `${transform.sourceProperty}:${transform.sourceValue}`;
        transformCounts[countKey]++;
      }
    }

    // Recursively process all properties
    if (Array.isArray(obj)) {
      obj.forEach(item => transform(item));
    } else {
      Object.keys(obj).forEach(key => transform(obj[key]));
    }
  };

  transform(spec);
  return transformCounts;
}

// ===== MAIN PROCESSOR =====

class OpenAPIProcessor {
  constructor(private config: ProcessingConfig) {}

  /**
   * Fetch spec from URL or file
   */
  private async fetchSpec(): Promise<OpenAPISpec> {
    console.log(`ℹ️  Fetching OpenAPI spec from ${this.config.sourceUrl}...`);

    // Check if it's a file path or URL
    if (this.config.sourceUrl.startsWith('http://') || this.config.sourceUrl.startsWith('https://')) {
      const response = await fetch(this.config.sourceUrl);
      if (!response.ok) {
        throw new Error(`Failed to fetch spec: ${response.status} ${response.statusText}`);
      }
      const spec = await response.json();
      console.log('✅ Successfully fetched OpenAPI specification');
      return spec;
    } else {
      // Local file
      const spec = await SwaggerParser.parse(this.config.sourceUrl);
      console.log('✅ Successfully loaded OpenAPI specification from file');
      return spec as OpenAPISpec;
    }
  }

  /**
   * Convert Swagger 2.0 to OpenAPI 3.0
   */
  private async convertToOpenAPI3(spec: OpenAPISpec): Promise<OpenAPISpec> {
    if (!spec.swagger || spec.openapi) {
      console.log('ℹ️  Specification is already OpenAPI 3.0');
      return spec;
    }

    const endpoint = this.config.converterEndpoint || "https://converter.swagger.io/api/convert";
    console.log('ℹ️  Converting Swagger 2.0 to OpenAPI 3.0...');

    const response = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify(spec),
    });

    if (!response.ok) {
      throw new Error(`Conversion failed: ${response.status} ${response.statusText}`);
    }

    const converted = await response.json();
    console.log('✅ Successfully converted to OpenAPI 3.0');
    return converted;
  }

  /**
   * Save spec to file
   */
  private async saveSpec(spec: OpenAPISpec): Promise<void> {
    const outputDir = dirname(this.config.outputPath);
    mkdirSync(outputDir, { recursive: true });

    const indent = this.config.indent || 2;
    const content = JSON.stringify(spec, null, indent);

    writeFileSync(this.config.outputPath, content, 'utf8');
    console.log(`✅ Specification saved to ${this.config.outputPath}`);
  }

  /**
   * Process the OpenAPI specification
   */
  async process(): Promise<void> {
    try {
      console.log('ℹ️  Starting OpenAPI processing...');

      // Fetch and parse the spec
      let spec = await this.fetchSpec();

      // Convert to OpenAPI 3.0 if needed
      spec = await this.convertToOpenAPI3(spec);

      // Validate the spec
      console.log('ℹ️  Validating OpenAPI specification...');

      // Apply transformations
      console.log('ℹ️  Applying transformations...');

      // 1. Fix missing descriptions
      const descriptionCount = fixMissingDescriptions(spec);
      console.log(`ℹ️  Fixed ${descriptionCount} missing descriptions`);

      // 2. Fix pydantic recursion error
      const pydanticCount = fixPydanticRecursionError(spec);
      console.log(`ℹ️  Fixed ${pydanticCount} pydantic recursion errors`);

      // 3. Transform vendor extensions if configured
      if (this.config.vendorExtensionTransforms && this.config.vendorExtensionTransforms.length > 0) {
        const transformCounts = transformVendorExtensions(spec, this.config.vendorExtensionTransforms);

        for (const [countKey, count] of Object.entries(transformCounts)) {
          const [sourceProperty, sourceValue] = countKey.split(':');
          const transform = this.config.vendorExtensionTransforms.find(t =>
            t.sourceProperty === sourceProperty && t.sourceValue === sourceValue
          );
          if (transform) {
            console.log(`ℹ️  Transformed ${count} ${sourceProperty}: ${sourceValue} to ${transform.targetProperty}`);
          }
        }
      }

      // Save the processed spec
      await SwaggerParser.validate(JSON.parse(JSON.stringify(spec)));
      console.log('✅ Specification is valid');

      await this.saveSpec(spec);

      console.log('✅ OpenAPI processing completed successfully!');
      console.log(`📄 Source: ${this.config.sourceUrl}`);
      console.log(`📄 Output: ${this.config.outputPath}`);

    } catch (error) {
      console.error(`❌ Processing failed: ${error instanceof Error ? error.message : error}`);
      throw error;
    }
  }
}

// ===== MAIN EXECUTION =====

/**
 * Fetch the latest stable tag from GitHub API
 */
async function getLatestStableTag(): Promise<string> {
  console.log('ℹ️  Fetching latest stable tag from GitHub...');

  try {
    const response = await fetch('https://api.github.com/repos/algorand/go-algorand/tags');
    if (!response.ok) {
      throw new Error(`GitHub API request failed: ${response.status} ${response.statusText}`);
    }

    const tags = await response.json();

    // Find the latest tag that contains '-stable'
    const stableTag = tags.find((tag: any) => tag.name.includes('-stable'));

    if (!stableTag) {
      throw new Error('No stable tag found in the repository');
    }

    console.log(`✅ Found latest stable tag: ${stableTag.name}`);
    return stableTag.name;
  } catch (error) {
    console.error('❌ Failed to fetch stable tag, falling back to master branch');
    console.error(error instanceof Error ? error.message : error);
    return 'master';
  }
}

async function processAlgorandSpec(config: ProcessingConfig) {
  const processor = new OpenAPIProcessor(config);
  await processor.process();
}

// Example usage
async function main() {
  try {
    // Get the latest stable tag
    const stableTag = await getLatestStableTag();

    // Default configuration with standard Algorand transformations
    const config: ProcessingConfig = {
      sourceUrl: `https://raw.githubusercontent.com/algorand/go-algorand/${stableTag}/daemon/algod/api/algod.oas2.json`,
      outputPath: join(process.cwd(), "specs", "algod.oas3.json"),
      vendorExtensionTransforms: [
        {
          sourceProperty: "x-algorand-format",
          sourceValue: "uint64",
          targetProperty: "x-algokit-bigint",
          targetValue: true,
          removeSource: true
        },
        {
          sourceProperty: "format",
          sourceValue: "uint64",
          targetProperty: "x-algokit-bigint",
          targetValue: true,
          removeSource: false
        },
        {
          sourceProperty: "x-go-type",
          sourceValue: "uint64",
          targetProperty: "x-algokit-bigint",
          targetValue: true,
          removeSource: true
        },
        {
          sourceProperty: "x-algorand-format",
          sourceValue: "SignedTransaction",
          targetProperty: "x-algokit-signed-txn",
          targetValue: true,
          removeSource: true
        }
      ]
    };

    await processAlgorandSpec(config);

  } catch (error) {
    console.error("❌ Fatal error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

// Run if this is the main module
if (import.meta.main) {
  main();
}
