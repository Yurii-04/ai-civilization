import { describe, expect, it } from 'vitest';
import { BLESSINGS, BLESSING_IDS, CIVS, CIV_IDS, SPARK_PACKS, civIndex, isCivId } from './index.js';

describe('civs', () => {
  it('lists five civs in island order', () => {
    expect(CIV_IDS).toEqual(['vesper', 'ondra', 'kael', 'mirra', 'solun']);
    CIV_IDS.forEach((id, i) => {
      expect(CIVS[id].id).toBe(id);
      expect(civIndex(id)).toBe(i);
    });
  });

  it('validates ids', () => {
    expect(isCivId('kael')).toBe(true);
    expect(isCivId('earth')).toBe(false);
  });
});

describe('catalog', () => {
  it('has a positive integer price for every blessing', () => {
    for (const id of BLESSING_IDS) {
      expect(BLESSINGS[id].id).toBe(id);
      expect(Number.isInteger(BLESSINGS[id].price)).toBe(true);
      expect(BLESSINGS[id].price).toBeGreaterThan(0);
    }
  });

  it('gives more Spark per dollar in bigger packs', () => {
    const rates = SPARK_PACKS.map((p) => p.spark / p.priceCents);
    for (let i = 1; i < rates.length; i++) {
      expect(rates[i]).toBeGreaterThan(rates[i - 1] ?? Infinity);
    }
  });
});
