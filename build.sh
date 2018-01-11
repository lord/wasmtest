cargo +nightly build --release --target=wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/debug/wasmtest.wasm program.wasm