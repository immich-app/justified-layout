import { describe, it } from 'node:test';
import { strict as assert } from 'node:assert';
import { JustifiedLayout } from './index.ts';

describe('JustifiedLayout', () => {
  it('fits perfectly on one row', () => {
    const input = new Float32Array([1.0, 1.0, 1.0]);
    const layout = new JustifiedLayout(input, { rowHeight: 300, rowWidth: 900, spacing: 0, heightTolerance: 0 });

    assert.equal(layout.containerWidth, 900.0);
    assert.equal(layout.containerHeight, 300.0);

    assert.deepEqual(layout.getPosition(0), { top: 0, left: 0, width: 300, height: 300 });
    assert.deepEqual(layout.getPosition(1), { top: 0, left: 300, width: 300, height: 300 });
    assert.deepEqual(layout.getPosition(2), { top: 0, left: 600, width: 300, height: 300 });
  });

  it('applies spacing', () => {
    const input = new Float32Array([1.0, 1.0, 1.0]);
    const layout = new JustifiedLayout(input, { rowHeight: 300, rowWidth: 904, spacing: 2, heightTolerance: 0 });

    assert.equal(layout.containerWidth, 904.0);
    assert.equal(layout.containerHeight, 300.0);

    assert.deepEqual(layout.getPosition(0), { top: 0, left: 0, width: 300, height: 300 });
    assert.deepEqual(layout.getPosition(1), { top: 0, left: 302, width: 300, height: 300 });
    assert.deepEqual(layout.getPosition(2), { top: 0, left: 604, width: 300, height: 300 });
  });

  it('handles empty input', () => {
    const input = new Float32Array([]);
    const layout = new JustifiedLayout(input, { rowHeight: 300, rowWidth: 900, spacing: 0, heightTolerance: 0 });

    assert.equal(layout.containerWidth, 0);
    assert.equal(layout.containerHeight, 0);
  });
});