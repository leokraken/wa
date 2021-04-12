
const fs = require("fs");
const { WASI } = require("@wasmer/wasi");
let nodeBindings = require("@wasmer/wasi/lib/bindings/node");

const wasmFilePath = "./opensubtitle-wasm.wasm";

nodeBindings = nodeBindings.default || nodeBindings;

// Instantiate a new WASI Instance
let wasi = new WASI({
  args: [wasmFilePath],
  env: {},
  bindings: nodeBindings,
  preopens: {
    '/sandbox': '/home/granbestiapop/Downloads/Lie to Me Season 2'
  }
});

// Async function to run our Wasm module/instance
const startWasiTask = async pathToWasmFile => {
  // Fetch our Wasm File
  let wasmBytes = new Uint8Array(fs.readFileSync(pathToWasmFile)).buffer;

  // Instantiate the WebAssembly file
  let wasmModule = await WebAssembly.compile(wasmBytes);
  let instance = await WebAssembly.instantiate(wasmModule, {
    ...wasi.getImports(wasmModule),
    //wasi_unstable: () => {},
  });
  console.log(instance.exports)
  const result = instance.exports.hello_world();
  console.log(result)
  const result2 = instance.exports.get_hello();
  console.log(result2)
  // Start the WASI instance
  wasi.start(instance);
};

// Everything starts here
startWasiTask(wasmFilePath);
