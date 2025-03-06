export * from "./index";

import mod from "../pkg/algo_models_ffi_bg.wasm";

//@ts-ignore
import { initSync } from "../pkg/algo_models_ffi";

//@ts-ignore
await initSync({ module: await mod() });
