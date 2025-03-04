#!/usr/bin/env bash

target="${CARGO_TARGET_DIR:-target}"
input="$target/wasm32-wasip1/release/$1.wasm"
output="$target/wasm32-wasip1/$1.wasm"

echo "Building"

cargo build --target wasm32-wasip1 --release

echo "Optimizing"

# https://github.com/WebAssembly/binaryen
~/Dev/web-assembly-binaryen/bin/wasm-opt -Os "$input" --output "$output"

echo "Stripping"

# https://github.com/WebAssembly/wabt
~/Dev/web-assembly-wabt/bin/wasm-strip "$output"

echo "Generating sha256checksum"

sha256sum "$output" >"$output.sha256"

echo "Done"
