import type { CSSProperties, ReactNode } from 'react';

export interface MonoLabelProps {
  children: ReactNode;
  /** Secondary labels are dimmer (#8C96A3). */
  tone?: 'primary' | 'secondary' | 'timestamp';
  style?: CSSProperties;
}

const TONES = {
  primary: '#F4F6F8',
  secondary: '#8C96A3',
  timestamp: '#6C7682',
} as const;

/** Uppercase IBM Plex Mono micro-label (docs/ART_DIRECTION.md → Typography). */
export function MonoLabel({ children, tone = 'secondary', style }: MonoLabelProps) {
  return (
    <span
      style={{
        fontFamily: 'var(--font-mono), "IBM Plex Mono", monospace',
        fontSize: 10,
        fontWeight: 400,
        letterSpacing: '0.24em',
        textTransform: 'uppercase',
        color: TONES[tone],
        ...style,
      }}
    >
      {children}
    </span>
  );
}
