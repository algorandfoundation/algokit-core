'use strict';

var require$$0 = require('util');
var require$$1 = require('path');
var require$$2 = require('fs');

var algo_models_ffi = {exports: {}};

algo_models_ffi.exports;

var hasRequiredAlgo_models_ffi;

function requireAlgo_models_ffi () {
	if (hasRequiredAlgo_models_ffi) return algo_models_ffi.exports;
	hasRequiredAlgo_models_ffi = 1;
	(function (module) {
		let imports = {};
		imports['__wbindgen_placeholder__'] = module.exports;
		let wasm;
		const { TextEncoder, TextDecoder } = require$$0;

		const heap = new Array(128).fill(undefined);

		heap.push(undefined, null, true, false);

		function getObject(idx) { return heap[idx]; }

		let WASM_VECTOR_LEN = 0;

		let cachedUint8ArrayMemory0 = null;

		function getUint8ArrayMemory0() {
		    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
		        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
		    }
		    return cachedUint8ArrayMemory0;
		}

		let cachedTextEncoder = new TextEncoder('utf-8');

		const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
		    ? function (arg, view) {
		    return cachedTextEncoder.encodeInto(arg, view);
		}
		    : function (arg, view) {
		    const buf = cachedTextEncoder.encode(arg);
		    view.set(buf);
		    return {
		        read: arg.length,
		        written: buf.length
		    };
		});

		function passStringToWasm0(arg, malloc, realloc) {

		    if (realloc === undefined) {
		        const buf = cachedTextEncoder.encode(arg);
		        const ptr = malloc(buf.length, 1) >>> 0;
		        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
		        WASM_VECTOR_LEN = buf.length;
		        return ptr;
		    }

		    let len = arg.length;
		    let ptr = malloc(len, 1) >>> 0;

		    const mem = getUint8ArrayMemory0();

		    let offset = 0;

		    for (; offset < len; offset++) {
		        const code = arg.charCodeAt(offset);
		        if (code > 0x7F) break;
		        mem[ptr + offset] = code;
		    }

		    if (offset !== len) {
		        if (offset !== 0) {
		            arg = arg.slice(offset);
		        }
		        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
		        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
		        const ret = encodeString(arg, view);

		        offset += ret.written;
		        ptr = realloc(ptr, len, offset, 1) >>> 0;
		    }

		    WASM_VECTOR_LEN = offset;
		    return ptr;
		}

		let cachedDataViewMemory0 = null;

		function getDataViewMemory0() {
		    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
		        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
		    }
		    return cachedDataViewMemory0;
		}

		let heap_next = heap.length;

		function addHeapObject(obj) {
		    if (heap_next === heap.length) heap.push(heap.length + 1);
		    const idx = heap_next;
		    heap_next = heap[idx];

		    heap[idx] = obj;
		    return idx;
		}

		function dropObject(idx) {
		    if (idx < 132) return;
		    heap[idx] = heap_next;
		    heap_next = idx;
		}

		function takeObject(idx) {
		    const ret = getObject(idx);
		    dropObject(idx);
		    return ret;
		}

		function isLikeNone(x) {
		    return x === undefined || x === null;
		}

		function debugString(val) {
		    // primitive types
		    const type = typeof val;
		    if (type == 'number' || type == 'boolean' || val == null) {
		        return  `${val}`;
		    }
		    if (type == 'string') {
		        return `"${val}"`;
		    }
		    if (type == 'symbol') {
		        const description = val.description;
		        if (description == null) {
		            return 'Symbol';
		        } else {
		            return `Symbol(${description})`;
		        }
		    }
		    if (type == 'function') {
		        const name = val.name;
		        if (typeof name == 'string' && name.length > 0) {
		            return `Function(${name})`;
		        } else {
		            return 'Function';
		        }
		    }
		    // objects
		    if (Array.isArray(val)) {
		        const length = val.length;
		        let debug = '[';
		        if (length > 0) {
		            debug += debugString(val[0]);
		        }
		        for(let i = 1; i < length; i++) {
		            debug += ', ' + debugString(val[i]);
		        }
		        debug += ']';
		        return debug;
		    }
		    // Test for built-in
		    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
		    let className;
		    if (builtInMatches && builtInMatches.length > 1) {
		        className = builtInMatches[1];
		    } else {
		        // Failed to match the standard '[object ClassName]'
		        return toString.call(val);
		    }
		    if (className == 'Object') {
		        // we're a user defined class or Object
		        // JSON.stringify avoids problems with cycles, and is generally much
		        // easier than looping through ownProperties of `val`.
		        try {
		            return 'Object(' + JSON.stringify(val) + ')';
		        } catch (_) {
		            return 'Object';
		        }
		    }
		    // errors
		    if (val instanceof Error) {
		        return `${val.name}: ${val.message}\n${val.stack}`;
		    }
		    // TODO we could test for more things here, like `Set`s and `Map`s.
		    return className;
		}

		let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

		cachedTextDecoder.decode();

		function getStringFromWasm0(ptr, len) {
		    ptr = ptr >>> 0;
		    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
		}

		function passArray8ToWasm0(arg, malloc) {
		    const ptr = malloc(arg.length * 1, 1) >>> 0;
		    getUint8ArrayMemory0().set(arg, ptr / 1);
		    WASM_VECTOR_LEN = arg.length;
		    return ptr;
		}
		/**
		 * Get the transaction type from the encoded transaction.
		 * This is particularly useful when decoding a transaction that has a unknow type
		 * @param {Uint8Array} bytes
		 * @returns {TransactionType}
		 */
		module.exports.getEncodedTransactionType = function(bytes) {
		    try {
		        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
		        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_export_0);
		        const len0 = WASM_VECTOR_LEN;
		        wasm.getEncodedTransactionType(retptr, ptr0, len0);
		        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
		        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
		        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
		        if (r2) {
		            throw takeObject(r1);
		        }
		        return takeObject(r0);
		    } finally {
		        wasm.__wbindgen_add_to_stack_pointer(16);
		    }
		};

		function getArrayU8FromWasm0(ptr, len) {
		    ptr = ptr >>> 0;
		    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
		}
		/**
		 * @param {Transaction} tx
		 * @returns {Uint8Array}
		 */
		module.exports.encodeTransaction = function(tx) {
		    try {
		        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
		        wasm.encodeTransaction(retptr, addHeapObject(tx));
		        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
		        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
		        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
		        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
		        if (r3) {
		            throw takeObject(r2);
		        }
		        var v1 = getArrayU8FromWasm0(r0, r1).slice();
		        wasm.__wbindgen_export_2(r0, r1 * 1, 1);
		        return v1;
		    } finally {
		        wasm.__wbindgen_add_to_stack_pointer(16);
		    }
		};

		/**
		 * @param {Uint8Array} bytes
		 * @returns {Transaction}
		 */
		module.exports.decodeTransaction = function(bytes) {
		    try {
		        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
		        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_export_0);
		        const len0 = WASM_VECTOR_LEN;
		        wasm.decodeTransaction(retptr, ptr0, len0);
		        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
		        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
		        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
		        if (r2) {
		            throw takeObject(r1);
		        }
		        return takeObject(r0);
		    } finally {
		        wasm.__wbindgen_add_to_stack_pointer(16);
		    }
		};

		/**
		 * @param {Uint8Array} encoded_tx
		 * @param {Uint8Array} signature
		 * @returns {Uint8Array}
		 */
		module.exports.attachSignature = function(encoded_tx, signature) {
		    try {
		        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
		        const ptr0 = passArray8ToWasm0(encoded_tx, wasm.__wbindgen_export_0);
		        const len0 = WASM_VECTOR_LEN;
		        const ptr1 = passArray8ToWasm0(signature, wasm.__wbindgen_export_0);
		        const len1 = WASM_VECTOR_LEN;
		        wasm.attachSignature(retptr, ptr0, len0, ptr1, len1);
		        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
		        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
		        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
		        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
		        if (r3) {
		            throw takeObject(r2);
		        }
		        var v3 = getArrayU8FromWasm0(r0, r1).slice();
		        wasm.__wbindgen_export_2(r0, r1 * 1, 1);
		        return v3;
		    } finally {
		        wasm.__wbindgen_add_to_stack_pointer(16);
		    }
		};

		/**
		 * @param {Uint8Array} pub_key
		 * @returns {Address}
		 */
		module.exports.addressFromPubKey = function(pub_key) {
		    try {
		        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
		        const ptr0 = passArray8ToWasm0(pub_key, wasm.__wbindgen_export_0);
		        const len0 = WASM_VECTOR_LEN;
		        wasm.addressFromPubKey(retptr, ptr0, len0);
		        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
		        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
		        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
		        if (r2) {
		            throw takeObject(r1);
		        }
		        return takeObject(r0);
		    } finally {
		        wasm.__wbindgen_add_to_stack_pointer(16);
		    }
		};

		/**
		 * @param {string} address
		 * @returns {Address}
		 */
		module.exports.addressFromString = function(address) {
		    try {
		        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
		        const ptr0 = passStringToWasm0(address, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
		        const len0 = WASM_VECTOR_LEN;
		        wasm.addressFromString(retptr, ptr0, len0);
		        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
		        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
		        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
		        if (r2) {
		            throw takeObject(r1);
		        }
		        return takeObject(r0);
		    } finally {
		        wasm.__wbindgen_add_to_stack_pointer(16);
		    }
		};

		module.exports.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
		    const ret = String(getObject(arg1));
		    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
		    const len1 = WASM_VECTOR_LEN;
		    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
		    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
		};

		module.exports.__wbg_buffer_61b7ce01341d7f88 = function(arg0) {
		    const ret = getObject(arg0).buffer;
		    return addHeapObject(ret);
		};

		module.exports.__wbg_entries_4f2bb9b0d701c0f6 = function(arg0) {
		    const ret = Object.entries(getObject(arg0));
		    return addHeapObject(ret);
		};

		module.exports.__wbg_from_d68eaa96dba25449 = function(arg0) {
		    const ret = Array.from(getObject(arg0));
		    return addHeapObject(ret);
		};

		module.exports.__wbg_get_9aa3dff3f0266054 = function(arg0, arg1) {
		    const ret = getObject(arg0)[arg1 >>> 0];
		    return addHeapObject(ret);
		};

		module.exports.__wbg_getwithrefkey_1dc361bd10053bfe = function(arg0, arg1) {
		    const ret = getObject(arg0)[getObject(arg1)];
		    return addHeapObject(ret);
		};

		module.exports.__wbg_instanceof_ArrayBuffer_670ddde44cdb2602 = function(arg0) {
		    let result;
		    try {
		        result = getObject(arg0) instanceof ArrayBuffer;
		    } catch (_) {
		        result = false;
		    }
		    const ret = result;
		    return ret;
		};

		module.exports.__wbg_instanceof_Uint8Array_28af5bc19d6acad8 = function(arg0) {
		    let result;
		    try {
		        result = getObject(arg0) instanceof Uint8Array;
		    } catch (_) {
		        result = false;
		    }
		    const ret = result;
		    return ret;
		};

		module.exports.__wbg_isArray_1ba11a930108ec51 = function(arg0) {
		    const ret = Array.isArray(getObject(arg0));
		    return ret;
		};

		module.exports.__wbg_isSafeInteger_12f5549b2fca23f4 = function(arg0) {
		    const ret = Number.isSafeInteger(getObject(arg0));
		    return ret;
		};

		module.exports.__wbg_length_65d1cd11729ced11 = function(arg0) {
		    const ret = getObject(arg0).length;
		    return ret;
		};

		module.exports.__wbg_length_d65cf0786bfc5739 = function(arg0) {
		    const ret = getObject(arg0).length;
		    return ret;
		};

		module.exports.__wbg_new_3ff5b33b1ce712df = function(arg0) {
		    const ret = new Uint8Array(getObject(arg0));
		    return addHeapObject(ret);
		};

		module.exports.__wbg_new_688846f374351c92 = function() {
		    const ret = new Object();
		    return addHeapObject(ret);
		};

		module.exports.__wbg_newwithbyteoffsetandlength_ba35896968751d91 = function(arg0, arg1, arg2) {
		    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
		    return addHeapObject(ret);
		};

		module.exports.__wbg_set_23d69db4e5c66a6e = function(arg0, arg1, arg2) {
		    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
		};

		module.exports.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
		    getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
		};

		module.exports.__wbindgen_as_number = function(arg0) {
		    const ret = +getObject(arg0);
		    return ret;
		};

		module.exports.__wbindgen_bigint_from_u64 = function(arg0) {
		    const ret = BigInt.asUintN(64, arg0);
		    return addHeapObject(ret);
		};

		module.exports.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
		    const v = getObject(arg1);
		    const ret = typeof(v) === 'bigint' ? v : undefined;
		    getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
		    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
		};

		module.exports.__wbindgen_boolean_get = function(arg0) {
		    const v = getObject(arg0);
		    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
		    return ret;
		};

		module.exports.__wbindgen_debug_string = function(arg0, arg1) {
		    const ret = debugString(getObject(arg1));
		    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
		    const len1 = WASM_VECTOR_LEN;
		    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
		    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
		};

		module.exports.__wbindgen_error_new = function(arg0, arg1) {
		    const ret = new Error(getStringFromWasm0(arg0, arg1));
		    return addHeapObject(ret);
		};

		module.exports.__wbindgen_in = function(arg0, arg1) {
		    const ret = getObject(arg0) in getObject(arg1);
		    return ret;
		};

		module.exports.__wbindgen_is_bigint = function(arg0) {
		    const ret = typeof(getObject(arg0)) === 'bigint';
		    return ret;
		};

		module.exports.__wbindgen_is_object = function(arg0) {
		    const val = getObject(arg0);
		    const ret = typeof(val) === 'object' && val !== null;
		    return ret;
		};

		module.exports.__wbindgen_is_string = function(arg0) {
		    const ret = typeof(getObject(arg0)) === 'string';
		    return ret;
		};

		module.exports.__wbindgen_is_undefined = function(arg0) {
		    const ret = getObject(arg0) === undefined;
		    return ret;
		};

		module.exports.__wbindgen_jsval_eq = function(arg0, arg1) {
		    const ret = getObject(arg0) === getObject(arg1);
		    return ret;
		};

		module.exports.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
		    const ret = getObject(arg0) == getObject(arg1);
		    return ret;
		};

		module.exports.__wbindgen_memory = function() {
		    const ret = wasm.memory;
		    return addHeapObject(ret);
		};

		module.exports.__wbindgen_number_get = function(arg0, arg1) {
		    const obj = getObject(arg1);
		    const ret = typeof(obj) === 'number' ? obj : undefined;
		    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
		    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
		};

		module.exports.__wbindgen_number_new = function(arg0) {
		    const ret = arg0;
		    return addHeapObject(ret);
		};

		module.exports.__wbindgen_object_clone_ref = function(arg0) {
		    const ret = getObject(arg0);
		    return addHeapObject(ret);
		};

		module.exports.__wbindgen_object_drop_ref = function(arg0) {
		    takeObject(arg0);
		};

		module.exports.__wbindgen_string_get = function(arg0, arg1) {
		    const obj = getObject(arg1);
		    const ret = typeof(obj) === 'string' ? obj : undefined;
		    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
		    var len1 = WASM_VECTOR_LEN;
		    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
		    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
		};

		module.exports.__wbindgen_string_new = function(arg0, arg1) {
		    const ret = getStringFromWasm0(arg0, arg1);
		    return addHeapObject(ret);
		};

		module.exports.__wbindgen_throw = function(arg0, arg1) {
		    throw new Error(getStringFromWasm0(arg0, arg1));
		};

		const path = require$$1.join(__dirname, 'algo_models_ffi_bg.wasm');
		const bytes = require$$2.readFileSync(path);

		const wasmModule = new WebAssembly.Module(bytes);
		const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
		wasm = wasmInstance.exports;
		module.exports.__wasm = wasm; 
	} (algo_models_ffi));
	return algo_models_ffi.exports;
}

requireAlgo_models_ffi();
