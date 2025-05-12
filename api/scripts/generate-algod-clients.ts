#!/usr/bin/env bun

import { execSync } from "child_process";
import { join } from "path";
import { mkdirSync, existsSync, copyFileSync, rmSync, writeFileSync } from "fs";

// Parse command line arguments
const args = process.argv.slice(2);
const targetIndex = args.indexOf('--target');
const target = targetIndex !== -1 ? args[targetIndex + 1]?.toLowerCase() : 'all';

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

/**
 * Ensures a clean output directory exists with proper .gitignore
 */
function prepareOutputDirectory(directory: string) {
    // Remove the directory if it exists
    if (existsSync(directory)) {
        rmSync(directory, { recursive: true, force: true });
    }

    // Create a fresh directory
    mkdirSync(directory, { recursive: true });

    const gitkeepPath = join(directory, ".gitkeep");
    writeFileSync(gitkeepPath, "");

    console.log(`Prepared clean directory: ${directory}`);
}

function copyIgnoreFile(templateDir: string, outputDir: string) {
    const ignoreFilePath = join(templateDir, ".openapi-generator-ignore");
    if (existsSync(ignoreFilePath)) {
        console.log(
            "ensuring ignore rules are propagated before client generation"
        );
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
        "-g typescript",
        `-o ${TYPESCRIPT_OUTPUT}`,
        `-t ${TYPESCRIPT_TEMPLATE}`,
        `-c ${TYPESCRIPT_TEMPLATE}/openapi-config.yaml`,
    ].join(" ");

    console.log(`Executing: ${cmd}`);
    execSync(cmd, { stdio: "inherit" });
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
        "--global-property=apis,models,apiTests=false,modelTests=false,supportingFiles",
    ].join(" ");

    console.log(`Executing: ${cmd}`);
    execSync(cmd, { stdio: "inherit" });
    console.log("Python client generated successfully!");
}

function main() {
    try {
        if (target === 'all' || target === 'typescript') {
            prepareOutputDirectory(TYPESCRIPT_OUTPUT);
            generateTypescriptClient();
        }

        if (target === 'all' || target === 'python') {
            prepareOutputDirectory(PYTHON_OUTPUT);
            generatePythonClient();
        }

        if (target !== 'all' && target !== 'typescript' && target !== 'python') {
            console.error(`Invalid target: ${target}. Valid options are 'typescript', 'python', or 'all'`);
            process.exit(1);
        }

        console.log("Client generation completed!");
    } catch (error) {
        console.error("Error generating clients:", error);
        process.exit(1);
    }
}

main();
