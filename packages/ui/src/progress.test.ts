import { describe, expect, it } from 'vitest';
import { clampProgress } from './ProgressLine.js';

describe('clampProgress', () => {
  it('clamps to [0, 1]', () => {
    expect(clampProgress(-1)).toBe(0);
    expect(clampProgress(0.42)).toBe(0.42);
    expect(clampProgress(3)).toBe(1);
    expect(clampProgress(Number.NaN)).toBe(0);
  });
});
