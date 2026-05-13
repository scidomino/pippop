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
cp target/wasm32-unknown-unknown/release/rust.wasm ../web_dist/pippop.wasm

# 4. Copy the HTML wrapper, manifest, and service worker
cp index.html ../web_dist/index.html
cp manifest.json ../web_dist/manifest.json
cp sw.js ../web_dist/sw.js
cp assets/icon.png ../web_dist/favicon.png
cp assets/og-image.png ../web_dist/og-image.png

# 4.5 Copy all game assets
echo "Copying game assets..."
cp -r assets ../web_dist/assets

# 5. Download the JS glue code
echo "Fetching JS bundle..."
curl -L https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o ../web_dist/mq_js_bundle.js
curl -L https://raw.githubusercontent.com/not-fl3/sapp-jsutils/master/js/sapp_jsutils.js -o ../web_dist/sapp_jsutils.js
curl -L https://raw.githubusercontent.com/optozorax/quad-storage/master/js/quad-storage.js -o ../web_dist/quad-storage.js

echo "----------------------------------------"
echo "Build complete! Files are in 'web_dist/'"
echo "To test locally, run: npx serve ../web_dist"
echo "----------------------------------------"
