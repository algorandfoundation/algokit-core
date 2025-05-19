export * from "./index";

import mod from "../pkg/algokit_msgpack_ffi_bg.wasm";

//@ts-ignore
import { initSync } from "../pkg/algokit_msgpack_ffi";

//@ts-ignore
initSync({ module: mod() });
