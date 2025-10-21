#!/usr/bin/env bash

wasm-pack build --no-pack --out-name justified-layout-wasm --target web
echo "export const MODULE = '$(cat pkg/justified-layout-wasm_bg.wasm | base64)'" > pkg/justified-layout-wasm_bg.wasm.js
