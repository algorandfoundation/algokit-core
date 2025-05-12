// From https://github.com/conda/rattler/blob/5e22731f53afad39bbf9bdd570513849cf419d0c/js-rattler/rollup.config.cjs.mjs
/** Build node cjs distribution */

import { wasm } from "@rollup/plugin-wasm";
import typescript from "@rollup/plugin-typescript";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import esmShim from "@rollup/plugin-esm-shim";

export default {
  input: "src/index.ts",
  output: {
    file: "dist/algokit_transact.node.cjs",
    sourcemap: false,
    format: "commonjs",
  },
  plugins: [
    esmShim(),
    commonjs(),
    wasm({
      targetEnv: "auto-inline",
      sync: ["pkg/algokit_transact_ffi_bg.wasm"],
    }),
    nodeResolve(),
    typescript({
      sourceMap: false,
      declaration: false,
      declarationMap: false,
      inlineSources: false,
      tsconfig: "./tsconfig.rollup.json",
    }),
  ],
};
