#!/usr/bin/env bash

wasm-pack build --no-pack --out-name justified-layout-wasm --target web
echo "Uint8Array.fromBase64 ??= (string) => Uint8Array.from(atob(string), (c) => c.charCodeAt(0));" > pkg/justified-layout-wasm-module.js
echo "export const MODULE = Uint8Array.fromBase64('$(cat pkg/justified-layout-wasm_bg.wasm | base64)')" >> pkg/justified-layout-wasm-module.js
