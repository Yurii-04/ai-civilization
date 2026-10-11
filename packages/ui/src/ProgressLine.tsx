export interface ProgressLineProps {
  /** 0–1; clamped. */
  value: number;
  /** Fill color, usually the civ accent. */
  color: string;
  label?: string;
}

export function clampProgress(value: number): number {
  if (!Number.isFinite(value)) return 0;
  return Math.min(1, Math.max(0, value));
}

/** 1 px track with a glowing fill (docs/ART_DIRECTION.md → HUD rules). */
export function ProgressLine({ value, color, label }: ProgressLineProps) {
  const pct = clampProgress(value) * 100;
  return (
    <div
      role="progressbar"
      aria-label={label}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={Math.round(pct)}
      style={{ position: 'relative', height: 1, background: 'rgba(255,255,255,0.14)' }}
    >
      <div
        style={{
          position: 'absolute',
          inset: 0,
          width: `${pct}%`,
          background: color,
          boxShadow: `0 0 6px ${color}`,
        }}
      />
    </div>
  );
}
