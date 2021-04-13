
## Rustup
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

## Build
cargo build --target wasm32-unknown-unknown


## Install dependencies
Install wasm-bindgen-cli or wasm-pack in order to generate NodeJS bindings. 
```
cargo install wasm-bindgen-cli
```
```
cargo install wasm-pack

```
# Build NodeJS dependency
wasm-bindgen target/wasm32-unknown-unknown/release/opensubtitle-wasm.wasm --nodejs  --out-dir ./pkg

```
wasm-pack build --target nodejs
```


## Links
* https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.rust.en-us.html
* https://dev.to/jor/rust-wasm-browser-nodejs-2bo6
