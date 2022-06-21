import * as wasm from "json-formatter-wasm";

const s = wasm.format(
  `{"Hello, Wasm!": true, "list"
    : [1,
      2,

  3], "object": {
          "prop1":
    "日本語", "prop2": "🎯🌀"

}}`,
  4
);
console.log(s);
