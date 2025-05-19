import * as fs from "fs";
import { initSync } from "../pkg/algokit_msgpack_ffi";

const wasmBuffer = fs.readFileSync("pkg/algokit_msgpack_ffi_bg.wasm");
const wasmModule = new WebAssembly.Module(wasmBuffer);
initSync({ module: wasmModule });
