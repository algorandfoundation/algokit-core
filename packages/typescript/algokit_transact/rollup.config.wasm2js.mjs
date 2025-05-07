// From https://github.com/conda/rattler/blob/5e22731f53afad39bbf9bdd570513849cf419d0c/js-rattler/rollup.config.esm.mjs
/** Build distribution for downstream bundler users */

import typescript from "@rollup/plugin-typescript";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

export default {
  input: "src/index.ts",
  output: {
    file: "dist/algokit_transact.wasm2js.mjs",
    sourcemap: false,
    format: "esm",
  },
  plugins: [
    commonjs(),
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
