
## Rustup
rustup target add wasm32-unknown-unknown

## Build
cargo build --target wasm32-unknown-unknown --release

## Install dependencies
Install wasm-bindgen-cli or wasm-pack in order to generate NodeJS bindings. 
```
cargo install wasm-bindgen-cli
```
```
cargo install wasm-pack
```
# Build NodeJS dependency
Generates JS bridges in order to load wasm file and export functions as module.
```
wasm-bindgen target/wasm32-unknown-unknown/release/opensubtitle_wasm.wasm --nodejs --reference-types --out-dir ./pkg
```
or
```
wasm-pack build --target nodejs
```

## Links
* https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.rust.en-us.html
* https://dev.to/jor/rust-wasm-browser-nodejs-2bo6
* JS Value https://docs.rs/wasm-bindgen/0.2.73/wasm_bindgen/struct.JsValue.html
