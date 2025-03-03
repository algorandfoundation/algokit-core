import { run, getCrateNanme } from ".";
import * as fs from "fs";

const crate = getCrateNanme();

await run(
  `wasm-pack build crates/${crate}_ffi --target web --out-dir ../../packages/typescript/${crate} -- --color always --no-default-features --features ffi_wasm`,
);

// Remove the generated .gitignore file from the pkg directory
if (fs.existsSync(`packages/typescript/${crate}/.gitignore`)) {
  fs.rmSync(`packages/typescript/${crate}/.gitignore`);
}
