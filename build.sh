#!/usr/bin/env bash

set -eu

wasm-pack build --no-pack --out-name justified-layout-wasm --target web
echo "export const MODULE: string;" > pkg/justified-layout-wasm-module.d.ts
echo "Uint8Array.fromBase64 ??= (string) => Uint8Array.from(atob(string), (c) => c.charCodeAt(0));" > pkg/justified-layout-wasm-module.js
echo "export const MODULE = Uint8Array.fromBase64('$(cat pkg/justified-layout-wasm_bg.wasm | base64 -w 0)');" >> pkg/justified-layout-wasm-module.js
