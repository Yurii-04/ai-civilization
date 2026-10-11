#!/usr/bin/env node
// Generates FlatBuffers code from schemas/*.fbs:
//   Rust -> crates/protocol/src/generated
//   TS   -> packages/protocol/src/generated
// flatc is looked up in $FLATC, then .tools/bin/flatc (scripts/install-flatc.sh), then PATH.
import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, rmSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const FLATC_VERSION = '25.9.23';
const root = join(dirname(fileURLToPath(import.meta.url)), '..');

function findFlatc() {
  if (process.env.FLATC) return process.env.FLATC;
  const local = join(root, '.tools', 'bin', 'flatc');
  return existsSync(local) ? local : 'flatc';
}

const flatc = findFlatc();
let version;
try {
  version = execFileSync(flatc, ['--version'], { encoding: 'utf8' }).trim();
} catch {
  console.error(
    `flatc not found. Run scripts/install-flatc.sh or set FLATC (need ${FLATC_VERSION}).`,
  );
  process.exit(1);
}
if (!version.includes(FLATC_VERSION)) {
  console.error(`Expected flatc ${FLATC_VERSION}, got "${version}".`);
  process.exit(1);
}

const schemas = readdirSync(join(root, 'schemas'))
  .filter((f) => f.endsWith('.fbs'))
  .sort()
  .map((f) => join('schemas', f));

const targets = [
  { flag: '--rust', out: 'crates/protocol/src/generated' },
  { flag: '--ts', out: 'packages/protocol/src/generated' },
];

for (const { flag, out } of targets) {
  rmSync(join(root, out), { recursive: true, force: true });
  execFileSync(flatc, [flag, '-o', out, ...schemas], { cwd: root, stdio: 'inherit' });
  console.log(`flatc ${flag} -> ${out}`);
}
