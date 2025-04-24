#!/usr/bin/env bun

import { execSync } from "child_process";
import { join } from "path";
import { mkdirSync, existsSync, copyFileSync } from "fs";

// TODO: use full spec, for now just txns for testing purposes
const SPEC_PATH = join(process.cwd(), "specs", "algod.oas3_txns_only.json");

// Create output directories if they don't exist
const OUTPUT_DIR = join(process.cwd(), "api_clients");
const TYPESCRIPT_OUTPUT = join(OUTPUT_DIR, "typescript");
const PYTHON_OUTPUT = join(OUTPUT_DIR, "python");

// Template directories
const TEMPLATES_DIR = join(process.cwd(), "oas_templates");
const TYPESCRIPT_TEMPLATE = join(TEMPLATES_DIR, "typescript");
const PYTHON_TEMPLATE = join(TEMPLATES_DIR, "python");

if (!existsSync(OUTPUT_DIR)) {
    mkdirSync(OUTPUT_DIR, { recursive: true });
}

if (!existsSync(TYPESCRIPT_OUTPUT)) {
    mkdirSync(TYPESCRIPT_OUTPUT, { recursive: true });
}

if (!existsSync(PYTHON_OUTPUT)) {
    mkdirSync(PYTHON_OUTPUT, { recursive: true });
}

function copyIgnoreFile(templateDir: string, outputDir: string) {
    const ignoreFilePath = join(templateDir, ".openapi-generator-ignore");
    if (existsSync(ignoreFilePath)) {
        console.log("ensuring ignore rules are propagated before client generation");
        const destPath = join(outputDir, ".openapi-generator-ignore");
        copyFileSync(ignoreFilePath, destPath);
    }
}

function generateTypescriptClient() {
    console.log("Generating TypeScript client...");

    copyIgnoreFile(TYPESCRIPT_TEMPLATE, TYPESCRIPT_OUTPUT);

    const cmd = [
        "bunx openapi-generator-cli generate",
        `-i ${SPEC_PATH}`,
        "-g typescript-fetch",
        `-o ${TYPESCRIPT_OUTPUT}`,
        `-t ${TYPESCRIPT_TEMPLATE}`,
        "--additional-properties=npmName=@algorand/algod-client,npmVersion=1.0.0,supportsES6=true,typescriptThreePlus=true",
    ].join(" ");

    console.log(`Executing: ${cmd}`);
    execSync(cmd, { stdio: 'inherit' });
    console.log("TypeScript client generated successfully!");
}

function generatePythonClient() {
    console.log("Generating Python client...");

    copyIgnoreFile(PYTHON_TEMPLATE, PYTHON_OUTPUT);

    const cmd = [
        "bunx openapi-generator-cli generate",
        `-i ${SPEC_PATH}`,
        "-g python",
        `-o ${PYTHON_OUTPUT}`,
        `-t ${PYTHON_TEMPLATE}`,
        `-c ${PYTHON_TEMPLATE}/openapi-config.yaml`,
        "--additional-properties=packageName=algorand_algod_client,packageVersion=1.0.0",
        "--global-property=apis,models,apiTests=true,modelTests=true,supportingFiles",
    ].join(" ");

    console.log(`Executing: ${cmd}`);
    execSync(cmd, { stdio: 'inherit' });
    console.log("Python client generated successfully!");
}

function main() {
    try {
        generateTypescriptClient();
        generatePythonClient();
        console.log("All clients generated successfully!");
    } catch (error) {
        console.error("Error generating clients:", error);
        process.exit(1);
    }
}

main();
