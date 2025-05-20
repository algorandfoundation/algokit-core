import * as fs from "fs";
import { initSync } from "../pkg/algokit_transact_ffi";

const wasmBuffer = fs.readFileSync("pkg/algokit_transact_ffi_bg.wasm");
const wasmModule = new WebAssembly.Module(wasmBuffer);
initSync({ module: wasmModule });
