/**
 * Viewer-facing catalog: blessing prices and Spark packs.
 * Draft values from docs/PRODUCT.md — pricing changes need human approval.
 */

export const BLESSING_IDS = ['rain_of_light', 'inspiration', 'vision', 'shield'] as const;

export type BlessingId = (typeof BLESSING_IDS)[number];

export interface Blessing {
  id: BlessingId;
  name: string;
  /** Price in Spark. */
  price: number;
  /** Whether the blessing targets a single agent rather than a civ. */
  targetsAgent: boolean;
}

export const BLESSINGS: Readonly<Record<BlessingId, Blessing>> = {
  rain_of_light: { id: 'rain_of_light', name: 'Rain of Light', price: 120, targetsAgent: false },
  inspiration: { id: 'inspiration', name: 'Inspiration', price: 60, targetsAgent: true },
  vision: { id: 'vision', name: 'Vision', price: 200, targetsAgent: false },
  shield: { id: 'shield', name: 'Shield', price: 340, targetsAgent: false },
};

export interface SparkPack {
  spark: number;
  /** Price in US cents. */
  priceCents: number;
}

export const SPARK_PACKS: readonly SparkPack[] = [
  { spark: 500, priceCents: 499 },
  { spark: 1_200, priceCents: 999 },
  { spark: 3_000, priceCents: 1_999 },
  { spark: 8_000, priceCents: 4_999 },
];
