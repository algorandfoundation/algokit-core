export * from "./index";

import mod from "../pkg/algokit_transact_ffi_bg.wasm";

import { initSync } from "../pkg/algokit_transact_ffi";

initSync({ module: mod() });
