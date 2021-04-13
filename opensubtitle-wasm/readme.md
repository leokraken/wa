
## Rustup
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

## Build
cargo build --target wasm32-unknown-unknown


## Install dependencies
Install wasm-bindgen-cli in order to generate NodeJS bindings. 
```
cargo install wasm-bindgen-cli
```

# Build NodeJS dependency
wasm-bindgen target/wasm32-unknown-unknown/release/opensubtitle-wasm.wasm --nodejs  --out-dir ./pkg

