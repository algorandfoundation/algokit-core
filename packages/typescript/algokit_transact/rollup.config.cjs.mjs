import { wasm } from "@rollup/plugin-wasm";
import typescript from "@rollup/plugin-typescript";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

export default {
  input: "src/esm.ts",
  output: {
    file: "dist/algokit_transact.node.cjs",
    sourcemap: false,
    format: "commonjs",
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
