#!/usr/bin/env bun

import { exec } from "child_process";
import { join } from "path";
import { mkdirSync, existsSync } from "fs";

// TODO: use full spec, for now just txns for testing purposes
const SPEC_PATH = join(process.cwd(), "specs", "algod.oas3.json");

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

function generateTypescriptClient() {
    console.log("Generating TypeScript client...");
    const cmd = [
        "bunx openapi-generator-cli generate",
        `-i ${SPEC_PATH}`,
        "-g typescript-fetch",
        `-o ${TYPESCRIPT_OUTPUT}`,
        `-t ${TYPESCRIPT_TEMPLATE}`,
        "--additional-properties=npmName=@algorand/algod-client,npmVersion=1.0.0,supportsES6=true,typescriptThreePlus=true",
    ].join(" ");

    try {
        console.log(cmd);
        exec(cmd);
        console.log("TypeScript client generated successfully!");
    } catch (error) {
        console.error("Error generating TypeScript client:", error);
        process.exit(1);
    }
}

function generatePythonClient() {
    console.log("Generating Python client...");
    const cmd = [
        "bunx openapi-generator-cli generate",
        `-i ${SPEC_PATH}`,
        "-g python",
        "--library asyncio",
        `-o ${PYTHON_OUTPUT}`,
        `-t ${PYTHON_TEMPLATE}`,
        `-c ${PYTHON_TEMPLATE}/openapi-config.yaml`,
        "--additional-properties=packageName=algorand_algod_client,packageVersion=1.0.0",
        "--global-property=apis,models,apiTests=true,modelTests=true,supportingFiles",
    ].join(" ");

    try {
        console.log(cmd);
        exec(cmd);
        console.log("Python client generated successfully!");
    } catch (error) {
        console.error("Error generating Python client:", error);
        process.exit(1);
    }
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
