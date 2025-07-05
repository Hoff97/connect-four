#!/bin/bash

# Build script for WebAssembly Connect Four game

echo "Building WebAssembly module..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg

echo "Build complete!"
echo ""
echo "To run the web app:"
echo "1. Start a local server in this directory"
echo "2. For example: python3 -m http.server 8000"
echo "3. Or use: npx serve ."
echo "4. Open your browser and navigate to the server address"
echo ""
echo "The game will be available at index.html"
