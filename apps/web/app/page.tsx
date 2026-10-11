import { CIVS, CIV_IDS } from '@relight/shared';
import { MonoLabel, ProgressLine } from '@relight/ui';
import { RelayStatus } from './RelayStatus';

// M0 holding page: proves the workspace packages, fonts and relay wiring.
// The live world (r3f) replaces it in M4.
export default function Home() {
  return (
    <main
      style={{
        minHeight: '100vh',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'space-between',
        padding: '24px 16px',
      }}
    >
      <header style={{ display: 'flex', justifyContent: 'space-between', gap: 16 }}>
        <MonoLabel>Season 00 · Pre-launch</MonoLabel>
        <RelayStatus />
      </header>

      <section style={{ textAlign: 'center' }}>
        <h1
          style={{
            margin: 0,
            fontFamily: 'var(--font-title), sans-serif',
            fontWeight: 200,
            fontSize: 'clamp(28px, 6vw, 56px)',
            letterSpacing: '0.6em',
            marginRight: '-0.6em',
          }}
        >
          RELIGHT
        </h1>
        <p style={{ marginTop: 16 }}>
          <MonoLabel>The star is dark. Five civilizations are waking.</MonoLabel>
        </p>
      </section>

      <footer style={{ width: '100%', maxWidth: 360, marginLeft: 'auto' }}>
        <ul style={{ listStyle: 'none', margin: 0, padding: 0, display: 'grid', gap: 12 }}>
          {CIV_IDS.map((id) => (
            <li key={id} style={{ display: 'grid', gap: 6 }}>
              <span style={{ display: 'flex', justifyContent: 'space-between' }}>
                <MonoLabel tone="primary" style={{ color: CIVS[id].accent }}>
                  {CIVS[id].name}
                </MonoLabel>
                <MonoLabel tone="timestamp">Beacon 0%</MonoLabel>
              </span>
              <ProgressLine value={0} color={CIVS[id].accent} label={`${CIVS[id].name} Beacon`} />
            </li>
          ))}
        </ul>
      </footer>
    </main>
  );
}
