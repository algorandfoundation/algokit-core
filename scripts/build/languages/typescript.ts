import { run } from "..";
import * as fs from "fs";

async function wasmPack(crate: string, target: "web" | "nodejs" | "bundler", cwd: string) {
  const cmd = `npx wasm-pack build --out-dir ../../packages/typescript/algokit_transact/pkg --mode normal --release --target ${target} ../../../crates/${crate}_ffi --no-default-features --features ffi_wasm`;

  await run(cmd, cwd, {
    // Needed due to Rust 1.82+ using LLVM with reference-types enabled by default, which makes the WASM binary incompatible with wasm2js
    // See https://github.com/WebAssembly/binaryen/issues/7358
    RUSTFLAGS: "-C strip=symbols",
  });
}

async function build(crate: string, mode: "esm" | "cjs" | "wasm2js", cwd: string) {
  switch (mode) {
    case "esm":
      await wasmPack(crate, "web", cwd);
      break;
    case "cjs":
      await wasmPack(crate, "nodejs", cwd);
      break;
    case "wasm2js":
      await wasmPack(crate, "bundler", cwd);
      await run(`npx wasm2js -O pkg/${crate}_ffi_bg.wasm -o pkg/${crate}_ffi_bg.wasm.js`, cwd);

      // Replace references to the wasm file with the wasm2js file
      [".js", "_bg.js"].forEach((ext) => {
        const file = `${cwd}/pkg/${crate}_ffi${ext}`;
        const content = fs.readFileSync(file, "utf-8");

        fs.writeFileSync(file, content.replace(`${crate}_ffi_bg.wasm`, `${crate}_ffi_bg.wasm.js`));
      });

      // When decoding, rust is passing numbers when it should be BigInt
      // Could be issue in wasm-bindgen or serde, but for now this fixes it
      const bgJs = `${cwd}/pkg/${crate}_ffi_bg.js`;
      const content = fs.readFileSync(bgJs, "utf-8");

      fs.writeFileSync(bgJs, content.replace("BigInt.asUintN(64, arg0)", "BigInt.asUintN(64, BigInt(arg0))"));

      break;
    default:
      throw new Error(`Unknown mode ${mode}`);
  }

  await run(`npx rollup -c rollup.config.${mode}.mjs`, cwd);
}

export async function buildTypescript(crate: string) {
  const cwd = `packages/typescript/${crate}`;

  await run("npm i", cwd);

  fs.rmSync(`packages/typescript/${crate}/pkg`, {
    recursive: true,
    force: true,
  });
  fs.rmSync(`packages/typescript/${crate}/dist`, {
    recursive: true,
    force: true,
  });
  fs.rmSync(`packages/typescript/${crate}/types`, {
    recursive: true,
    force: true,
  });

  await build(crate, "wasm2js", cwd);
  await build(crate, "esm", cwd);
  await build(crate, "cjs", cwd);

  fs.copyFileSync(`packages/typescript/${crate}/pkg/${crate}_ffi.d.ts`, `packages/typescript/${crate}/dist/index.d.ts`);
}
