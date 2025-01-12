import { get_justified_layout } from '../pkg/justified-layout-wasm.js';

export interface LayoutOptions {
  rowHeight: number;
  rowWidth: number;
  spacing: number;
  heightTolerance: number;
}

export class JustifiedLayout {
  layout: Int32Array;

  constructor(aspectRatios: Float32Array, { rowHeight, rowWidth, spacing, heightTolerance }: LayoutOptions) {
    if (aspectRatios.length === 0) {
      this.layout = Int32Array.of(0, 0, 0, 0);
    } else {
      this.layout = get_justified_layout(aspectRatios, rowHeight, rowWidth, spacing, heightTolerance);
    }
  }

  get containerWidth() {
    return this.layout[0];
  }

  get containerHeight() {
    return this.layout[1];
  }

  getTop(boxIdx: number) {
    return this.layout[boxIdx * 4 + 4]; // the first 4 elements are containerWidth, containerHeight, padding, padding
  }

  getLeft(boxIdx: number) {
    return this.layout[boxIdx * 4 + 5];
  }

  getWidth(boxIdx: number) {
    return this.layout[boxIdx * 4 + 6];
  }

  getHeight(boxIdx: number) {
    return this.layout[boxIdx * 4 + 7];
  }
}
