// From https://github.com/conda/rattler/blob/5e22731f53afad39bbf9bdd570513849cf419d0c/js-rattler/rollup.config.esm.mjs
/** Build distribution for downstream bundler users */

import { wasm } from "@rollup/plugin-wasm";
import typescript from "@rollup/plugin-typescript";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

export default {
  input: "src/esm.ts",
  output: {
    file: "dist/algokit_transact.bundler.mjs",
    sourcemap: false,
    format: "esm",
  },
  plugins: [
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
