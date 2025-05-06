// Re-export everything from index.js
module.exports = require("./index");

const mod = require("../pkg/algokit_transact_ffi_bg.wasm");
const { initSync } = require("../pkg/algokit_transact_ffi");

// Initialize WASM synchronously
initSync({ module: mod() });
