#!/usr/bin/env bun

import { execSync } from "child_process";
import { join } from "path";
import { parseArgs } from "util";
import { mkdirSync, existsSync, copyFileSync, rmSync, readdirSync } from "fs";

const LANGUAGE_OPTIONS = ["typescript", "python"] as string[];
type Language = (typeof LANGUAGE_OPTIONS)[number];

const API_OPTIONS = ["algod" /*, "indexer", "kmd"*/] as string[];
type Api = (typeof API_OPTIONS)[number];

// Parse command line arguments
const { positionals } = parseArgs({
  allowPositionals: true,
});

if (positionals.length !== 2) {
  throw new Error("Usage: bun scripts/generate-clients.ts <api> <language>");
}

const [apiArg, languageArg] = positionals;
const apis = API_OPTIONS.includes(apiArg) ? [apiArg] : API_OPTIONS;
const languages = LANGUAGE_OPTIONS.includes(languageArg) ? [languageArg] : LANGUAGE_OPTIONS;

const SPEC_PATH = join(process.cwd(), "specs", "algod.oas3.json");
const OUTPUT_DIR = join(process.cwd(), "..", "packages");

// Template directories
const TEMPLATES_DIR = join(process.cwd(), "oas_templates");
const TYPESCRIPT_TEMPLATE = join(TEMPLATES_DIR, "typescript");
const PYTHON_TEMPLATE = join(TEMPLATES_DIR, "python");

if (!existsSync(OUTPUT_DIR)) {
  mkdirSync(OUTPUT_DIR, { recursive: true });
}

/**
 * Ensures a clean output directory exists with proper .gitignore
 */
function prepareOutputDirectory(directory: string) {
  // Remove the directory if it exists
  const fileIgnoreList = ["release.config.cjs", "bun.lock", "poetry.lock"];
  if (existsSync(directory)) {
    const files = readdirSync(directory);
    for (const file of files.filter((f) => !fileIgnoreList.includes(f))) {
      const filePath = join(directory, file);
      rmSync(filePath, { recursive: true, force: true });
    }
  } else {
    // Create a fresh directory
    mkdirSync(directory, { recursive: true });
  }

  console.log(`Prepared clean directory: ${directory}`);
}

function copyIgnoreFile(templateDir: string, outputDir: string) {
  const ignoreFilePath = join(templateDir, ".openapi-generator-ignore");
  if (existsSync(ignoreFilePath)) {
    console.log("ensuring ignore rules are propagated before client generation");
    const destPath = join(outputDir, ".openapi-generator-ignore");
    copyFileSync(ignoreFilePath, destPath);
  }
}

function generateTypescriptClient(outputDir: string) {
  copyIgnoreFile(TYPESCRIPT_TEMPLATE, outputDir);

  const cmd = [
    "bunx openapi-generator-cli generate",
    `-i ${SPEC_PATH}`,
    "-g typescript",
    `-o ${outputDir}`,
    `-t ${TYPESCRIPT_TEMPLATE}`,
    `-c ${TYPESCRIPT_TEMPLATE}/openapi-config.yaml`,
  ].join(" ");

  console.log(`Executing: ${cmd}`);
  execSync(cmd, { stdio: "inherit" });
}

function generatePythonClient(outputDir: string) {
  copyIgnoreFile(PYTHON_TEMPLATE, outputDir);

  const cmd = [
    "bunx openapi-generator-cli generate",
    `-i ${SPEC_PATH}`,
    "-g python",
    `-o ${outputDir}`,
    `-t ${PYTHON_TEMPLATE}`,
    `-c ${PYTHON_TEMPLATE}/openapi-config.yaml`,
    "--global-property=apis,models,apiTests=false,modelTests=false,supportingFiles",
  ].join(" ");

  console.log(`Executing: ${cmd}`);
  execSync(cmd, { stdio: "inherit" });
}
function main() {
  try {
    for (const api of apis) {
      const api_package = `${api}_api`;

      if (languages.includes("typescript")) {
        const outputDir = join(OUTPUT_DIR, "typescript", api_package);
        console.log(`Generating TypeScript ${apis} client...`);
        prepareOutputDirectory(outputDir);
        generateTypescriptClient(outputDir);
        console.log(`TypeScript ${apis} client generated successfully!`);
      }

      if (languages.includes("python")) {
        const outputDir = join(OUTPUT_DIR, "python", api_package);
        console.log(`Generating Python ${apis} client...`);
        prepareOutputDirectory(outputDir);
        generatePythonClient(outputDir);
        console.log(`Python ${apis} client generated successfully!`);
      }
    }

    console.log("Client generation completed!");
  } catch (error) {
    console.error("Error generating clients:", error);
    process.exit(1);
  }
}

main();
