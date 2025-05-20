export * from "./index";

import mod from "../pkg/algokit_transact_ffi_bg.wasm";

//@ts-ignore
import { initSync } from "../pkg/algokit_transact_ffi";

//@ts-ignore
initSync({ module: mod() });
