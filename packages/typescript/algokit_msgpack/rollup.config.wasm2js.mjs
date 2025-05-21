import typescript from "@rollup/plugin-typescript";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

export default {
  input: "src/index.ts",
  output: {
    file: "dist/algokit_msgpack.wasm2js.mjs",
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
