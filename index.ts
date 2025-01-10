import { get_justified_layout } from "./pkg/justified_layout.js";

export interface LayoutBox {
  top: number;
  left: number;
  width: number;
  height: number;
}

export interface LayoutOptions {
  rowHeight: number;
  rowWidth: number;
  spacing: number;
  heightTolerance: number;
}

export interface Layout {
  boxes: LayoutBox[];
  containerHeight: number;
  containerWidth: number;
}

export function getJustifiedLayout(
  aspectRatios: Float32Array,
  { rowHeight, rowWidth, spacing, heightTolerance }: LayoutOptions
): Layout {
  if (aspectRatios.length === 0) {
    return { boxes: [], containerHeight: 0, containerWidth: 0 };
  }

  const layout = get_justified_layout(
    aspectRatios,
    rowHeight,
    rowWidth,
    spacing,
    heightTolerance
  );
  const boxes: LayoutBox[] = [];
  const containerWidth = layout[0];
  const containerHeight = layout[1];
  for (let i = 4; i < layout.length; i += 4) {
    boxes.push({
      top: layout[i],
      left: layout[i + 1],
      width: layout[i + 2],
      height: layout[i + 3],
    });
  }
  return { boxes, containerHeight, containerWidth };
}
