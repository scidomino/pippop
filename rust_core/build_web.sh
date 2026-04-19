#!/bin/bash
set -e

# Ensure the wasm target is installed
rustup target add wasm32-unknown-unknown

# 1. Build the release Wasm binary
echo "Building Wasm..."
cargo build --manifest-path Cargo.toml --target wasm32-unknown-unknown --release

# 2. Create/Clean the distribution directory
mkdir -p ../web_dist
rm -f ../web_dist/*

# 3. Copy the binary
cp target/wasm32-unknown-unknown/release/rust_core.wasm ../web_dist/pippop.wasm

# 4. Copy the HTML wrapper
cp index.html ../web_dist/index.html

# 5. Download the JS glue code
echo "Fetching JS bundle..."
curl -L https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o ../web_dist/mq_js_bundle.js

echo "----------------------------------------"
echo "Build complete! Files are in 'web_dist/'"
echo "To test locally, run: npx serve ../web_dist"
echo "----------------------------------------"
