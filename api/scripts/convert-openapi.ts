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
  sourceProperty: string; // e.g., "x-algorand-format" or "format"
  sourceValue: string; // e.g., "uint64"
  targetProperty: string; // e.g., "x-algokit-bigint"
  targetValue: boolean; // value to set
  removeSource?: boolean; // whether to remove the source property (default false)
}

interface RequiredFieldTransform {
  schemaName: string; // e.g., "ApplicationParams" - The OpenAPI schema name
  fieldName: string; // e.g., "approval-program" - The field name to transform
  makeRequired: boolean; // true = add to required array, false = remove from required array
}

interface FieldTransform {
  fieldName: string; // e.g., "action"
  removeItems?: string[]; // properties to remove from the target property, e.g., ["format"]
  addItems?: Record<string, any>; // properties to add to the target property, e.g., {"x-custom": true}
}

interface ProcessingConfig {
  sourceUrl: string;
  outputPath: string;
  converterEndpoint?: string;
  indent?: number;
  vendorExtensionTransforms?: VendorExtensionTransform[];
  requiredFieldTransforms?: RequiredFieldTransform[];
  fieldTransforms?: FieldTransform[];
}

// ===== TRANSFORMATIONS =====

// Known missing descriptions to auto-fix
const MISSING_DESCRIPTIONS = new Map([
  // Component responses
  ["components.responses.NodeStatusResponse.description", "Returns the current status of the node"],
  ["components.responses.CatchpointStartResponse.description", "Catchpoint start operation response"],
  ["components.responses.CatchpointAbortResponse.description", "Catchpoint abort operation response"],

  // Path responses
  ["paths.'/v2/transactions/async'(post).responses.200.description", "Transaction successfully submitted for asynchronous processing"],
  ["paths.'/v2/status'(get).responses.200.description", "Returns the current node status including sync status, version, and latest round"],
  ["paths.'/v2/catchup/{catchpoint}'(post).responses.200.description", "Catchpoint operation started successfully"],
  ["paths.'/v2/catchup/{catchpoint}'(post).responses.201.description", "Catchpoint operation created and started successfully"],
  ["paths.'/v2/catchup/{catchpoint}'(delete).responses.200.description", "Catchpoint operation aborted successfully"],
  ["paths.'/v2/ledger/sync/{round}'(post).responses.200.description", "Ledger sync to specified round initiated successfully"],
  ["paths.'/v2/shutdown'(post).responses.200.description", "Node shutdown initiated successfully"],
  [
    "paths.'/v2/status/wait-for-block-after/{round}'(get).responses.200.description",
    "Returns node status after the specified round is reached",
  ],
  ["paths.'/v2/ledger/sync'(delete).responses.200.description", "Ledger sync operation stopped successfully"],
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
      if (response && typeof response === "object" && !response.description) {
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
      if (!pathObj || typeof pathObj !== "object") continue;

      const methods = ["get", "post", "put", "delete", "patch", "head", "options", "trace"];

      for (const method of methods) {
        const operation = pathObj[method];
        if (!operation?.responses) continue;

        for (const [statusCode, response] of Object.entries(operation.responses)) {
          if (response && typeof response === "object" && !(response as any).description) {
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
    missingPaths.forEach((path) => console.warn(`  - ${path}`));
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
  transforms.forEach((t) => (transformCounts[`${t.sourceProperty}:${t.sourceValue}`] = 0));

  const transform = (obj: any): void => {
    if (!obj || typeof obj !== "object") return;

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
      obj.forEach((item) => transform(item));
    } else {
      Object.keys(obj).forEach((key) => transform(obj[key]));
    }
  };

  transform(spec);
  return transformCounts;
}

/**
 * Fix field naming - Add field rename extensions for better Rust ergonomics
 */
function fixFieldNaming(spec: OpenAPISpec): number {
  let fixedCount = 0;

  // Properties that should be renamed for better developer experience
  const fieldRenames = [
    { from: "application-index", to: "app_id" },
    { from: "asset-index", to: "asset_id" },
  ];

  const processObject = (obj: any): void => {
    if (!obj || typeof obj !== "object") return;

    if (Array.isArray(obj)) {
      obj.forEach((o) => processObject(o));
      return;
    }

    // Look for properties object in schemas
    if (obj.properties && typeof obj.properties === "object") {
      for (const [propName, propDef] of Object.entries(obj.properties as Record<string, any>)) {
        if (propDef && typeof propDef === "object") {
          const rename = fieldRenames.find((r) => r.from === propName);
          if (rename) {
            propDef["x-algokit-field-rename"] = rename.to;
            fixedCount++;
          }
        }
      }
    }

    // Recursively process nested objects
    for (const value of Object.values(obj)) {
      if (value && typeof value === "object") {
        processObject(value);
      }
    }
  };

  processObject(spec);
  return fixedCount;
}

/**
 * Fix TealValue bytes - Add base64 extension for TealValue.bytes fields
 */
function fixTealValueBytes(spec: OpenAPISpec): number {
  let fixedCount = 0;

  const processObject = (obj: any, schemaName?: string): void => {
    if (!obj || typeof obj !== "object") return;

    if (Array.isArray(obj)) {
      obj.forEach((o) => processObject(o));
      return;
    }

    // Check if this is a TealValue schema with bytes property
    if (schemaName === "TealValue" && obj.properties && obj.properties.bytes) {
      obj.properties.bytes["x-algokit-bytes-base64"] = true;
      fixedCount++;
    }

    // Recursively process schemas
    if (obj.schemas && typeof obj.schemas === "object") {
      for (const [name, schemaDef] of Object.entries(obj.schemas)) {
        processObject(schemaDef, name);
      }
    } else {
      // Recursively process other nested objects
      for (const [key, value] of Object.entries(obj)) {
        if (value && typeof value === "object") {
          processObject(value, key);
        }
      }
    }
  };

  processObject(spec);
  return fixedCount;
}

/**
 * Fix bigint - Add x-algokit-bigint: true to properties that represent large integers
 */
function fixBigInt(spec: OpenAPISpec): number {
  let fixedCount = 0;

  // Properties that commonly represent large integers in Algorand/blockchain context
  const bigIntFields = [
    { fieldName: "fee" },
    { fieldName: "min-fee" },
    { fieldName: "round" },
    { fieldName: "last-round" },
    { fieldName: "confirmed-round" },
    { fieldName: "asset-id" },
    { fieldName: "application-index" },
    { fieldName: "asset-index" },
    { fieldName: "current_round" },
    { fieldName: "online-money" },
    { fieldName: "total-money" },
    { fieldName: "amount" },
    { fieldName: "asset-closing-amount" },
    { fieldName: "closing-amount" },
    { fieldName: "close_rewards" },
    { fieldName: "id" },
    { fieldName: "index", excludedModels: ["LightBlockHeaderProof"] },
    { fieldName: "last-proposed" },
    { fieldName: "last-heartbeat" },
    { fieldName: "application-index" },
    { fieldName: "min-balance" },
    { fieldName: "amount-without-pending-rewards" },
    { fieldName: "pending-rewards" },
    { fieldName: "rewards" },
    { fieldName: "reward-base" },
    { fieldName: "vote-first-valid" },
    { fieldName: "vote-key-dilution" },
    { fieldName: "vote-last-valid" },
    { fieldName: "catchup-time" },
    { fieldName: "time-since-last-round" },
  ];

  const processObject = (obj: any, objName?: string): void => {
    if (!obj || typeof obj !== "object") return;

    if (Array.isArray(obj)) {
      obj.forEach((o) => processObject(o));
      return;
    }

    // Iterate through all properties
    for (const [key, value] of Object.entries(obj)) {
      // Check if this is a properties object (schema properties)
      if (key === "properties" && value && typeof value === "object") {
        for (const [propName, propDef] of Object.entries(value as Record<string, any>)) {
          if (propDef && typeof propDef === "object" && propDef.type === "integer" && !propDef["x-algokit-bigint"]) {
            if (bigIntFields.findIndex((f) => f.fieldName === propName && (!objName || !f.excludedModels?.includes(objName))) > -1) {
              propDef["x-algokit-bigint"] = true;
              fixedCount++;
            }
          }
        }
      }

      // Recursively process nested objects
      if (value && typeof value === "object") {
        processObject(value, key);
      }
    }
  };

  processObject(spec);
  return fixedCount;
}

/**
 * Transform specific properties by removing configured items and/or adding new items
 */
function transformProperties(spec: OpenAPISpec, transforms: FieldTransform[]): number {
  let transformedCount = 0;

  if (!transforms?.length) {
    return transformedCount;
  }

  const processObject = (obj: any, currentPath: string[] = []): void => {
    if (!obj || typeof obj !== "object") return;

    if (Array.isArray(obj)) {
      obj.forEach((item, index) => processObject(item, [...currentPath, index.toString()]));
      return;
    }

    // Check each configured transformation
    for (const transform of transforms) {
      const targetPath = `properties.${transform.fieldName}`;
      const fullPath = currentPath.join(".");

      // Check if current path matches the target property path
      if (fullPath.endsWith(targetPath)) {
        // Remove specified items from this property
        if (transform.removeItems) {
          for (const itemToRemove of transform.removeItems) {
            if (obj.hasOwnProperty(itemToRemove)) {
              delete obj[itemToRemove];
              transformedCount++;
            }
          }
        }

        // Add specified items to this property
        if (transform.addItems) {
          for (const [key, value] of Object.entries(transform.addItems)) {
            obj[key] = value;
            transformedCount++;
          }
        }
      }
    }

    // Recursively process nested objects
    for (const [key, value] of Object.entries(obj)) {
      if (value && typeof value === "object") {
        processObject(value, [...currentPath, key]);
      }
    }
  };

  processObject(spec);
  return transformedCount;
}

/**
 * Transform required fields in schemas
 *
 * This function adds or removes specified fields from the 'required' array of OpenAPI schemas.
 * If the required array becomes empty after removals, it's removed entirely.
 */
function transformRequiredFields(spec: OpenAPISpec, requiredFieldTransforms: RequiredFieldTransform[]): number {
  let transformedCount = 0;

  if (!spec.components?.schemas || !requiredFieldTransforms?.length) {
    return transformedCount;
  }

  for (const config of requiredFieldTransforms) {
    const schema = spec.components.schemas[config.schemaName];

    if (!schema) {
      console.warn(`⚠️  Schema ${config.schemaName} not found, skipping field transform for ${config.fieldName}`);
      continue;
    }

    // Initialize required array if it doesn't exist and we're making a field required
    if (config.makeRequired && !schema.required) {
      schema.required = [];
    }

    if (config.makeRequired) {
      // Make field required: add to required array if not already present
      if (!schema.required.includes(config.fieldName)) {
        schema.required.push(config.fieldName);
        transformedCount++;
        console.log(`ℹ️  Made ${config.fieldName} required in ${config.schemaName}`);
      }
    } else {
      // Make field optional: remove from required array
      if (schema.required && Array.isArray(schema.required)) {
        const originalLength = schema.required.length;
        schema.required = schema.required.filter((field: string) => field !== config.fieldName);

        // If the required array is now empty, remove it entirely
        if (schema.required.length === 0) {
          delete schema.required;
        }

        const removedCount = originalLength - (schema.required?.length || 0);
        if (removedCount > 0) {
          transformedCount += removedCount;
          console.log(`ℹ️  Made ${config.fieldName} optional in ${config.schemaName}`);
        }
      }
    }
  }

  return transformedCount;
}

// ===== MAIN PROCESSOR =====

class OpenAPIProcessor {
  constructor(private config: ProcessingConfig) {}

  /**
   * Apply typo fixes to raw JSON content
   */
  private patchTypos(content: string): string {
    const patches = [
      ["ana ccount", "an account"],
      ["since eposh", "since epoch"],
      ["* update\\n* update\\n* delete", "* update\\n* delete"],
    ];

    return patches.reduce((text, [find, replace]) => text.replaceAll(find, replace), content);
  }

  /**
   * Fetch spec from URL or file
   */
  private async fetchSpec(): Promise<OpenAPISpec> {
    console.log(`ℹ️  Fetching OpenAPI spec from ${this.config.sourceUrl}...`);

    // Check if it's a file path or URL
    if (this.config.sourceUrl.startsWith("http://") || this.config.sourceUrl.startsWith("https://")) {
      const response = await fetch(this.config.sourceUrl);
      if (!response.ok) {
        throw new Error(`Failed to fetch spec: ${response.status} ${response.statusText}`);
      }
      const rawContent = await response.text();
      const patchedContent = this.patchTypos(rawContent);
      const spec = JSON.parse(patchedContent);
      console.log("✅ Successfully fetched OpenAPI specification");
      return spec;
    } else {
      // Local file
      const spec = await SwaggerParser.parse(this.config.sourceUrl);
      console.log("✅ Successfully loaded OpenAPI specification from file");
      return spec as OpenAPISpec;
    }
  }

  /**
   * Convert Swagger 2.0 to OpenAPI 3.0
   */
  private async convertToOpenAPI3(spec: OpenAPISpec): Promise<OpenAPISpec> {
    if (!spec.swagger || spec.openapi) {
      console.log("ℹ️  Specification is already OpenAPI 3.0");
      return spec;
    }

    const endpoint = this.config.converterEndpoint || "https://converter.swagger.io/api/convert";
    console.log("ℹ️  Converting Swagger 2.0 to OpenAPI 3.0...");

    const response = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify(spec),
    });

    if (!response.ok) {
      throw new Error(`Conversion failed: ${response.status} ${response.statusText}`);
    }

    const converted = await response.json();
    console.log("✅ Successfully converted to OpenAPI 3.0");
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

    writeFileSync(this.config.outputPath, content, "utf8");
    console.log(`✅ Specification saved to ${this.config.outputPath}`);
  }

  /**
   * Process the OpenAPI specification
   */
  async process(): Promise<void> {
    try {
      console.log("ℹ️  Starting OpenAPI processing...");

      // Fetch and parse the spec
      let spec = await this.fetchSpec();

      // Convert to OpenAPI 3.0 if needed
      spec = await this.convertToOpenAPI3(spec);

      // Validate the spec
      console.log("ℹ️  Validating OpenAPI specification...");

      // Apply transformations
      console.log("ℹ️  Applying transformations...");

      // 1. Fix missing descriptions
      const descriptionCount = fixMissingDescriptions(spec);
      console.log(`ℹ️  Fixed ${descriptionCount} missing descriptions`);

      // 2. Fix pydantic recursion error
      const pydanticCount = fixPydanticRecursionError(spec);
      console.log(`ℹ️  Fixed ${pydanticCount} pydantic recursion errors`);

      // 3. Fix field naming
      const fieldNamingCount = fixFieldNaming(spec);
      console.log(`ℹ️  Added field rename extensions to ${fieldNamingCount} properties`);

      // 4. Fix TealValue bytes fields
      const tealValueCount = fixTealValueBytes(spec);
      console.log(`ℹ️  Added bytes base64 extensions to ${tealValueCount} TealValue.bytes properties`);

      // 5. Fix bigint properties
      const bigIntCount = fixBigInt(spec);
      console.log(`ℹ️  Added x-algokit-bigint to ${bigIntCount} properties`);

      // 6. Transform required fields if configured
      let transformedFieldsCount = 0;
      if (this.config.requiredFieldTransforms && this.config.requiredFieldTransforms.length > 0) {
        transformedFieldsCount = transformRequiredFields(spec, this.config.requiredFieldTransforms);
        console.log(`ℹ️  Transformed ${transformedFieldsCount} required field states`);
      }

       // 7. Transform properties if configured
       let transformedPropertiesCount = 0;
       if (this.config.fieldTransforms && this.config.fieldTransforms.length > 0) {
         transformedPropertiesCount = transformProperties(spec, this.config.fieldTransforms);
         console.log(`ℹ️  Applied ${transformedPropertiesCount} property transformations (additions/removals)`);
       }

      // 8. Transform vendor extensions if configured
      if (this.config.vendorExtensionTransforms && this.config.vendorExtensionTransforms.length > 0) {
        const transformCounts = transformVendorExtensions(spec, this.config.vendorExtensionTransforms);

        for (const [countKey, count] of Object.entries(transformCounts)) {
          const [sourceProperty, sourceValue] = countKey.split(":");
          const transform = this.config.vendorExtensionTransforms.find(
            (t) => t.sourceProperty === sourceProperty && t.sourceValue === sourceValue,
          );
          if (transform) {
            console.log(`ℹ️  Transformed ${count} ${sourceProperty}: ${sourceValue} to ${transform.targetProperty}`);
          }
        }
      }

      // Save the processed spec
      await SwaggerParser.validate(JSON.parse(JSON.stringify(spec)));
      console.log("✅ Specification is valid");

      await this.saveSpec(spec);

      console.log("✅ OpenAPI processing completed successfully!");
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
 * Fetch the latest stable tag from GitHub API for go-algorand
 */
async function getLatestStableTag(): Promise<string> {
  console.log("ℹ️  Fetching latest stable tag from GitHub...");

  try {
    const response = await fetch("https://api.github.com/repos/algorand/go-algorand/tags");
    if (!response.ok) {
      throw new Error(`GitHub API request failed: ${response.status} ${response.statusText}`);
    }

    const tags = await response.json();

    // Find the latest tag that contains '-stable'
    const stableTag = tags.find((tag: any) => tag.name.includes("-stable"));

    if (!stableTag) {
      throw new Error("No stable tag found in the repository");
    }

    console.log(`✅ Found latest stable tag: ${stableTag.name}`);
    return stableTag.name;
  } catch (error) {
    console.error("❌ Failed to fetch stable tag, falling back to master branch");
    console.error(error instanceof Error ? error.message : error);
    return "master";
  }
}

/**
 * Fetch the latest release tag from GitHub API for indexer
 */
async function getLatestIndexerTag(): Promise<string> {
  console.log("ℹ️  Fetching latest indexer release tag from GitHub...");

  try {
    const response = await fetch("https://api.github.com/repos/algorand/indexer/releases/latest");
    if (!response.ok) {
      throw new Error(`GitHub API request failed: ${response.status} ${response.statusText}`);
    }

    const release = await response.json();

    console.log(`✅ Found latest indexer release tag: ${release.tag_name}`);
    return release.tag_name;
  } catch (error) {
    console.error("❌ Failed to fetch indexer release tag, falling back to master branch");
    console.error(error instanceof Error ? error.message : error);
    return "master";
  }
}

/**
 * Process specifications for both algod and indexer
 */
async function processAlgorandSpecs() {
  await Promise.all([processAlgodSpec(), processIndexerSpec()]);
}

async function processAlgodSpec() {
  console.log("\n🔄 Processing Algod specification...");

  const stableTag = await getLatestStableTag();

  const config: ProcessingConfig = {
    sourceUrl: `https://raw.githubusercontent.com/algorand/go-algorand/${stableTag}/daemon/algod/api/algod.oas2.json`,
    outputPath: join(process.cwd(), "specs", "algod.oas3.json"),
    fieldTransforms: [
      {
        fieldName: "action",
        removeItems: ["format"]
      },
      {
        fieldName: "num-uint",
        removeItems: ["format"],
        addItems: {
          "minimum": 0,
          "maximum": 64,
        }
      },
      {
        fieldName: "num-byte-slice",
        removeItems: ["format"],
        addItems: {
          "minimum": 0,
          "maximum": 64,
        }
      },
      {
        fieldName: "extra-program-pages",
        removeItems: ["format"],
        addItems: {
          "minimum": 0,
          "maximum": 3,
        }
      }
    ],
    vendorExtensionTransforms: [
      {
        sourceProperty: "x-algorand-format",
        sourceValue: "uint64",
        targetProperty: "x-algokit-bigint",
        targetValue: true,
        removeSource: true,
      },
      {
        sourceProperty: "format",
        sourceValue: "uint64",
        targetProperty: "x-algokit-bigint",
        targetValue: true,
        removeSource: false,
      },
      {
        sourceProperty: "x-go-type",
        sourceValue: "uint64",
        targetProperty: "x-algokit-bigint",
        targetValue: true,
        removeSource: true,
      },
      {
        sourceProperty: "x-algorand-format",
        sourceValue: "SignedTransaction",
        targetProperty: "x-algokit-signed-txn",
        targetValue: true,
        removeSource: true,
      },
    ],
  };

  await processAlgorandSpec(config);
}

async function processIndexerSpec() {
  console.log("\n🔄 Processing Indexer specification...");

  const indexerTag = await getLatestIndexerTag();

  const config: ProcessingConfig = {
    sourceUrl: `https://raw.githubusercontent.com/algorand/indexer/${indexerTag}/api/indexer.oas2.json`,
    outputPath: join(process.cwd(), "specs", "indexer.oas3.json"),
    requiredFieldTransforms: [
      { schemaName: "ApplicationParams", fieldName: "approval-program", makeRequired: false },
      { schemaName: "ApplicationParams", fieldName: "clear-state-program", makeRequired: false },
    ],
    fieldTransforms: [
      {
        fieldName: "num-uint",
        removeItems: ["x-algorand-format"],
        addItems: {
          "minimum": 0,
          "maximum": 64,
        }
      },
      {
        fieldName: "num-byte-slice",
        removeItems: ["x-algorand-format"],
        addItems: {
          "minimum": 0,
          "maximum": 64,
        }
      },
      {
        fieldName: "extra-program-pages",
        addItems: {
          "minimum": 0,
          "maximum": 3,
        }
      }
    ],
    vendorExtensionTransforms: [
      {
        sourceProperty: "x-algorand-format",
        sourceValue: "uint64",
        targetProperty: "x-algokit-bigint",
        targetValue: true,
        removeSource: true,
      },
      {
        sourceProperty: "format",
        sourceValue: "uint64",
        targetProperty: "x-algokit-bigint",
        targetValue: true,
        removeSource: false,
      },
      {
        sourceProperty: "x-go-type",
        sourceValue: "uint64",
        targetProperty: "x-algokit-bigint",
        targetValue: true,
        removeSource: true,
      },
      {
        sourceProperty: "x-algorand-format",
        sourceValue: "SignedTransaction",
        targetProperty: "x-algokit-signed-txn",
        targetValue: true,
        removeSource: true,
      },
    ],
  };

  await processAlgorandSpec(config);
}

async function processAlgorandSpec(config: ProcessingConfig) {
  const processor = new OpenAPIProcessor(config);
  await processor.process();
}

// Example usage
async function main() {
  try {
    const args = process.argv.slice(2);

    // Support for individual spec processing or both
    if (args.includes("--algod-only")) {
      await processAlgodSpec();
    } else if (args.includes("--indexer-only")) {
      await processIndexerSpec();
    } else {
      // Process both by default
      await processAlgorandSpecs();
    }
  } catch (error) {
    console.error("❌ Fatal error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

// Run if this is the main module
if (import.meta.main) {
  main();
}
