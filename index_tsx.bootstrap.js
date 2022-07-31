"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunktoy_json_wasm_demo"] = self["webpackChunktoy_json_wasm_demo"] || []).push([["index_tsx"],{

/***/ "../pkg/wasm_bg.js":
/*!*************************!*\
  !*** ../pkg/wasm_bg.js ***!
  \*************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"__wbg_error_09919627ac0992f5\": () => (/* binding */ __wbg_error_09919627ac0992f5),\n/* harmony export */   \"__wbg_new_693216e109162396\": () => (/* binding */ __wbg_new_693216e109162396),\n/* harmony export */   \"__wbg_stack_0ddaca5d1abfb52f\": () => (/* binding */ __wbg_stack_0ddaca5d1abfb52f),\n/* harmony export */   \"__wbindgen_json_parse\": () => (/* binding */ __wbindgen_json_parse),\n/* harmony export */   \"__wbindgen_object_drop_ref\": () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   \"__wbindgen_string_new\": () => (/* binding */ __wbindgen_string_new),\n/* harmony export */   \"format\": () => (/* binding */ format),\n/* harmony export */   \"parse\": () => (/* binding */ parse)\n/* harmony export */ });\n/* harmony import */ var _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_bg.wasm */ \"../pkg/wasm_bg.wasm\");\n/* module decorator */ module = __webpack_require__.hmd(module);\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__]);\n_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = new Uint8Array();\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = new Int32Array();\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n/**\n* @param {string} s\n* @returns {any}\n*/\nfunction parse(s) {\n    try {\n        const retptr = _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        const ptr0 = passStringToWasm0(s, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.parse(retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var r2 = getInt32Memory0()[retptr / 4 + 2];\n        if (r2) {\n            throw takeObject(r1);\n        }\n        return takeObject(r0);\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n    }\n}\n\n/**\n* @param {string} s\n* @param {number} indent\n* @returns {string}\n*/\nfunction format(s, indent) {\n    try {\n        const retptr = _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        const ptr0 = passStringToWasm0(s, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.format(retptr, ptr0, len0, indent);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var r2 = getInt32Memory0()[retptr / 4 + 2];\n        var r3 = getInt32Memory0()[retptr / 4 + 3];\n        var ptr1 = r0;\n        var len1 = r1;\n        if (r3) {\n            ptr1 = 0; len1 = 0;\n            throw takeObject(r2);\n        }\n        return getStringFromWasm0(ptr1, len1);\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(ptr1, len1);\n    }\n}\n\nfunction __wbindgen_json_parse(arg0, arg1) {\n    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_string_new(arg0, arg1) {\n    const ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_new_693216e109162396() {\n    const ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {\n    const ret = getObject(arg1).stack;\n    const ptr0 = passStringToWasm0(ret, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    const len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_09919627ac0992f5(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);\n    }\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://toy-json-wasm-demo/../pkg/wasm_bg.js?");

/***/ }),

/***/ "./index.tsx":
/*!*******************!*\
  !*** ./index.tsx ***!
  \*******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! react */ \"./node_modules/react/index.js\");\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var react_dom_client__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! react-dom/client */ \"./node_modules/react-dom/client.js\");\n/* harmony import */ var toy_json_wasm__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! toy-json-wasm */ \"../pkg/wasm_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([toy_json_wasm__WEBPACK_IMPORTED_MODULE_2__]);\ntoy_json_wasm__WEBPACK_IMPORTED_MODULE_2__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n\nfunction debounce(f, delay) {\n    let timeout = null;\n    const debounsed = () => {\n        if (timeout !== null) {\n            window.clearInterval(timeout);\n        }\n        window.setTimeout(f, delay);\n    };\n    debounsed.cancel = () => {\n        if (timeout !== null) {\n            window.clearInterval(timeout);\n        }\n    };\n    return debounsed;\n}\nfunction useDebounce(f, delay, deps) {\n    react__WEBPACK_IMPORTED_MODULE_0__.useEffect(() => {\n        const timeout = window.setTimeout(() => f(), delay);\n        return () => {\n            window.clearTimeout(timeout);\n        };\n    }, [delay, ...deps]);\n}\nfunction useToggle(initialValue = false) {\n    const [on, set] = react__WEBPACK_IMPORTED_MODULE_0__.useState(initialValue);\n    const toggle = react__WEBPACK_IMPORTED_MODULE_0__.useCallback(() => {\n        set((prev) => !prev);\n    }, [on, set]);\n    return [on, toggle, set];\n}\nconst SAMPLES = {\n    シンプルオブジェクト: {\n        name: \"シンプルオブジェクト\",\n        json: `{\"Image\": {\"Width\":  800,\"Height\": 600,\"Title\":  \"View from 15th Floor\",\"Thumbnail\": {\"Url\":\"http://www.example.com/image/481989943\",\"Height\": 125,\"Width\":  100},\"Animated\" : false,\"IDs\": [116, 943, 234, 38793]}}`,\n    },\n    文字列: {\n        name: \"文字列\",\n        json: `\"Hello, World!\"`,\n    },\n    数値: {\n        name: \"数値\",\n        json: `3.1415`,\n    },\n    いろいろなデータ型: {\n        name: \"いろいろなデータ型\",\n        json: `{\n      \"number\": 42,\n\"boolean\": true, \"string\":\n                \"Hello, World!\"\n\n                ,\"nullType\": null,\n                \"array\"\n      : [1,\n        2,\n\n    3], \"object\": {\n\n            \"prop1\":\n      \"日本語\", \"prop2\": \"🎯🌀\"\n\n  }}`,\n    },\n    空オブジェクト: {\n        name: \"空オブジェクト\",\n        json: \"{}\",\n    },\n    Unicode: {\n        name: \"Unicode\",\n        json: String.raw `{\"key\": \"\\u30CF\\u30ED\\u30FC\\u30EF\\u30FC\\u30EB\\u30C9\\u2600\\uFE0F\"}`,\n    },\n};\nfunction App() {\n    const [sample, setSample] = react__WEBPACK_IMPORTED_MODULE_0__.useState(Object.keys(SAMPLES)[0]);\n    const [input, setInput] = react__WEBPACK_IMPORTED_MODULE_0__.useState(\"\");\n    const [format, toggleFormat] = useToggle(true);\n    react__WEBPACK_IMPORTED_MODULE_0__.useEffect(() => {\n        if (sample === \"custom\") {\n            return;\n        }\n        setInput(SAMPLES[sample].json);\n    }, [sample]);\n    function handleSampleChange(e) {\n        setSample(e.target.value);\n    }\n    function handleInputChange(e) {\n        console.debug(e.target.value);\n        e.preventDefault();\n        e.stopPropagation();\n        setInput(e.target.value);\n        setSample(\"custom\");\n    }\n    let output;\n    try {\n        if (input === \"\") {\n            output = \"\";\n        }\n        else {\n            output = toy_json_wasm__WEBPACK_IMPORTED_MODULE_2__.format(input, format ? 2 : 0);\n        }\n    }\n    catch (err) {\n        output = `Failed to parse JSON:\\n ${err}`;\n    }\n    // useDebounce(\n    //   () => {\n    //     try {\n    //       if (input === \"\") {\n    //         return \"\";\n    //       }\n    //       setOutput(toyJson.format(input, format ? 2 : 0));\n    //     } catch (err) {\n    //       setOutput(`Failed to parse JSON:\\n ${err}`);\n    //       // throw err;\n    //     }\n    //   },\n    //   1000,\n    //   [input, setOutput, format]\n    // );\n    function handleSubmit(e) {\n        e.preventDefault();\n        e.stopPropagation();\n    }\n    return (react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"main\", { style: { padding: \"0 2rem\" } },\n        react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"h1\", null, \"toy-json-wasm demo\"),\n        react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"form\", { onSubmit: handleSubmit },\n            react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"section\", { style: { display: \"flex\", gap: \"1rem\", padding: \"1rem 0\" } },\n                react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"div\", null,\n                    react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"label\", null,\n                        \"\\u30B5\\u30F3\\u30D7\\u30EB:\",\n                        react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"select\", { value: sample, onChange: handleSampleChange },\n                            react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"option\", { value: \"custom\", disabled: true }, \"custom\"),\n                            Object.values(SAMPLES).map(({ name }) => (react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"option\", { key: name, value: name }, name)))))),\n                react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"div\", null,\n                    react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"label\", null,\n                        \"Format:\",\n                        react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"input\", { type: \"checkbox\", style: { fontFamily: \"monospace;\" }, checked: format, onChange: (e) => toggleFormat() })))),\n            react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"div\", { style: {\n                    display: \"grid\",\n                    gridTemplateColumns: \"minmax(min(100%, 200px), 1fr) minmax(min(100%, 200px), 1fr)\",\n                    margin: \"0 auto\",\n                    minHeight: \"40rem\",\n                    gap: \"1rem\",\n                } },\n                react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"textarea\", { value: input, onInput: handleInputChange, style: {\n                        resize: \"horizontal\",\n                    } }),\n                react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"div\", null,\n                    react__WEBPACK_IMPORTED_MODULE_0__.createElement(\"pre\", { style: {\n                            height: \"100%\",\n                            margin: \"0\",\n                            width: \"100%\",\n                            background: \"#EFEFEF\",\n                            whiteSpace: \"pre-wrap\",\n                        } }, output))))));\n}\nconst $root = document.getElementById(\"root\");\nif ($root === null) {\n    throw Error(\"react root element doesn't exist\");\n}\nconst root = react_dom_client__WEBPACK_IMPORTED_MODULE_1__.createRoot($root);\nroot.render(react__WEBPACK_IMPORTED_MODULE_0__.createElement(App, null));\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://toy-json-wasm-demo/./index.tsx?");

/***/ }),

/***/ "../pkg/wasm_bg.wasm":
/*!***************************!*\
  !*** ../pkg/wasm_bg.wasm ***!
  \***************************/
/***/ ((module, exports, __webpack_require__) => {

eval("var __webpack_instantiate__ = ([WEBPACK_IMPORTED_MODULE_0]) => {\n\treturn __webpack_require__.v(exports, module.id, \"d1ded31e433585b3b7b6\", {\n\t\t\"./wasm_bg.js\": {\n\t\t\t\"__wbindgen_json_parse\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_json_parse,\n\t\t\t\"__wbindgen_string_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_new,\n\t\t\t\"__wbg_new_693216e109162396\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_693216e109162396,\n\t\t\t\"__wbg_stack_0ddaca5d1abfb52f\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_0ddaca5d1abfb52f,\n\t\t\t\"__wbg_error_09919627ac0992f5\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_09919627ac0992f5,\n\t\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref\n\t\t}\n\t});\n}\n__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => {\n\ttry {\n\t/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./wasm_bg.js */ \"../pkg/wasm_bg.js\");\n\tvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([WEBPACK_IMPORTED_MODULE_0]);\n\tvar [WEBPACK_IMPORTED_MODULE_0] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__;\n\tawait __webpack_require__.v(exports, module.id, \"d1ded31e433585b3b7b6\", {\n\t\t\"./wasm_bg.js\": {\n\t\t\t\"__wbindgen_json_parse\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_json_parse,\n\t\t\t\"__wbindgen_string_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_new,\n\t\t\t\"__wbg_new_693216e109162396\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_693216e109162396,\n\t\t\t\"__wbg_stack_0ddaca5d1abfb52f\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_0ddaca5d1abfb52f,\n\t\t\t\"__wbg_error_09919627ac0992f5\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_09919627ac0992f5,\n\t\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref\n\t\t}\n\t});\n\t__webpack_async_result__();\n\t} catch(e) { __webpack_async_result__(e); }\n}, 1);\n\n//# sourceURL=webpack://toy-json-wasm-demo/../pkg/wasm_bg.wasm?");

/***/ })

}]);