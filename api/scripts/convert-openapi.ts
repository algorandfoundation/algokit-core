#!/usr/bin/env bun

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const SPEC_PATH = join(process.cwd(), "specs", "algod.oas2.json");
const OUTPUT_PATH = join(process.cwd(), "specs", "algod.oas3.json");
const CONVERTER_ENDPOINT = "https://converter.swagger.io/api/convert";

console.log(`Converting OpenAPI 2.0 spec at ${SPEC_PATH} to OpenAPI 3.0...`);

try {
    // Read the OpenAPI 2.0 spec
    const spec = JSON.parse(readFileSync(SPEC_PATH, "utf8"));

    // Use the Swagger API converter endpoint
    const main = async () => {
        try {
            const response = await fetch(CONVERTER_ENDPOINT, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    accept: "application/json",
                },
                body: JSON.stringify(spec),
            });

            if (!response.ok) {
                throw new Error(
                    `API responded with status: ${response.status}`
                );
            }

            const openapi3 = await response.json();

            // Format JSON with 2 spaces indentation
            const formatted = JSON.stringify(openapi3, null, 2);

            // Write the output
            writeFileSync(OUTPUT_PATH, formatted, "utf8");

            console.log(
                `Conversion complete! OpenAPI 3.0 spec written to ${OUTPUT_PATH}`
            );
        } catch (error) {
            console.error("Error converting spec:", error);
            process.exit(1);
        }
    };

    main();
} catch (error) {
    console.error("Error processing spec:", error);
    process.exit(1);
}
