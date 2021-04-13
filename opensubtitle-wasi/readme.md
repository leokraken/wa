
## Rustup
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

## Build
cargo build --target wasm32-wasi --release

