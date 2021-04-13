# Build
cargo build --target wasm32-wasi

# Rustup
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

# Deps
```
cargo install wasm-bindgen-cli
```

# Build nodejs
wasm-bindgen target/wasm32-unknown-unknown/release/opensubtitle-wasm.wasm --nodejs  --out-dir ./pkg
