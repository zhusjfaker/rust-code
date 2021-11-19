const fs = require("fs");
const path = require("path");

const wasmpath = path.resolve(__dirname, "../out/wasm_test_bg.wasm");
const wasmBuffer = fs.readFileSync(wasmpath);
WebAssembly.instantiate(wasmBuffer).then(wasmModule => {
    // Exported function live under instance.exports
    const exports_obj = wasmModule.instance.exports;
    console.log(exports_obj.greet(4), exports_obj.test_add(4)); // Outputs: 11
});