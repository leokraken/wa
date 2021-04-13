# Run WASI
node --experimental-wasi-unstable-preview1 --experimental-wasm-bigint runtimes/wasi.js

# Run Bindgen
node runtimes/bindgen.js


# Links
* https://www.secondstate.io/articles/wasi-access-system-resources/
* https://www.secondstate.io/articles/deno-webassembly-rust-wasi/
* https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
* https://github.com/second-state/ssvmup