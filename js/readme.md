# NodeJS version
* Use >=15 https://github.com/rustwasm/wasm-bindgen/issues/2274

# Run WASI
node --experimental-wasi-unstable-preview1 --experimental-wasm-bigint runtimes/wasi.js

# Run Bindgen
```
node --experimental-wasm-anyref runtimes/bindgen.js
```
or 
```
npm run wasm
```

# Links
* https://www.secondstate.io/articles/wasi-access-system-resources/
* https://www.secondstate.io/articles/deno-webassembly-rust-wasi/
* https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
* https://github.com/second-state/ssvmup
* https://docs.wasmtime.dev/examples-rust-wasi.html
* https://www.secondstate.io/articles/wasi-access-system-resources/
* https://www.secondstate.io/articles/deno-webassembly-rust-wasi/
* Networking on WASI https://radu-matei.com/blog/towards-sockets-networking-wasi/
* Wasmtime PR networking https://github.com/bytecodealliance/wasmtime/compare/main...Kong:feat/wasi-sockets
* https://radu-matei.com/blog/nodejs-wasi/