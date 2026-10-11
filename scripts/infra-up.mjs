#!/usr/bin/env node
// Starts Postgres + Redis via docker compose. Apps in M0 do not need them yet,
// so a missing Docker daemon is a warning, not a failure.
import { spawnSync } from 'node:child_process';

const result = spawnSync('docker', ['compose', 'up', '-d', '--wait'], { stdio: 'inherit' });
if (result.status !== 0) {
  console.warn(
    '\n[relight] docker compose failed; continuing without Postgres/Redis. Run `pnpm infra:up` once Docker is available.\n',
  );
}
