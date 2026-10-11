/** Civilization ids in island-index order (must match `sim_core::CivId`). */
export const CIV_IDS = ['vesper', 'ondra', 'kael', 'mirra', 'solun'] as const;

export type CivId = (typeof CIV_IDS)[number];

export interface Civ {
  id: CivId;
  /** Display name. */
  name: string;
  /** Accent color: territory glow, labels, tracker dots. */
  accent: string;
}

export const CIVS: Readonly<Record<CivId, Civ>> = {
  vesper: { id: 'vesper', name: 'Vesper', accent: '#6BE6F5' },
  ondra: { id: 'ondra', name: 'Ondra', accent: '#A88FFF' },
  kael: { id: 'kael', name: 'Kael', accent: '#75F2AE' },
  mirra: { id: 'mirra', name: 'Mirra', accent: '#FF8FC7' },
  solun: { id: 'solun', name: 'Solun', accent: '#94B8FF' },
};

export function isCivId(value: string): value is CivId {
  return (CIV_IDS as readonly string[]).includes(value);
}

export function civIndex(id: CivId): number {
  return CIV_IDS.indexOf(id);
}
