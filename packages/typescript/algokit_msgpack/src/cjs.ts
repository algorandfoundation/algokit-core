module.exports = require("./index");

const mod = require("../pkg/algokit_msgpack_ffi_bg.wasm");
const { initSync } = require("../pkg/algokit_msgpack_ffi");

initSync({ module: mod() });
