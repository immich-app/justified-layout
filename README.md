# Justified Layout

A blazingly fast implementation of the justified layout gallery view popularized by Flickr, written in Rust and exported to WebAssembly. Capable of processing hundreds of thousands of boxes in under a millisecond, or 10 million boxes at under 50ms.

## Install (JS/TS)

```bash
npm i @immich/justified-layout-wasm
```

Note that you will need to have both WebAssembly ESM integration and possibly top-level await configured for your project. For Vite, this means using [vite-plugin-wasm](https://www.npmjs.com/package/vite-plugin-wasm) and possibly [vite-plugin-top-level-await](https://www.npmjs.com/package/vite-plugin-top-level-await).

Additionally, you should exclude this package from dependency optimization as it may interfere with the initialization of the Wasm module. For Vite, this means using the `optimizeDeps.exclude` field in `vite.config.js`:
```js
...
optimizeDeps: {
  exclude: ['@immich/justified-layout-wasm'],
},
...
```

Usage as a native Rust library for non-Wasm targets is not yet supported.

## Build (WebAssembly)

```bash
npm run build
```

## Usage

```ts
import { JustifiedLayout } from '@immich/justified-layout-wasm';

const boxes = [{ width: 160, height: 90 }, { width: 200, height: 100 }, { width: 90, height: 160 }];
const aspectRatios = new Float32Array(boxes.map(({ width, height }) => width / height));

const layout = new JustifiedLayout(aspectRatios, {
  rowHeight: 250, // the target height for each row
  rowWidth: 600, // the target width for each row
  spacing: 4, // spacing between boxes
  heightTolerance: 0.1, // allows increasing the height of a row by a certain percentage (10% here) when it doesn't fill the target row width at the target height
});

// maximum width across all rows, used to determine the width of the component containing these rows
const containerWidth = layout.containerWidth;

// total height needed to display all rows, used to determine the height of the component containing these rows
const containerHeight = layout.containerHeight;

for (let i = 0; i < boxes.length; i++) {
  // you can use these values to position each box accordingly
  const top = layout.getTop(i);
  const left = layout.getLeft(i);
  const width = layout.getWidth(i);
  const height = layout.getHeight(i);
}
```

## Contributing

PR's are welcome! Also feel free to reach out to the team on [Discord](https://discord.immich.app).

## Technology
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html)
