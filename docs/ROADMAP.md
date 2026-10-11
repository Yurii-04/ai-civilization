# Roadmap — Relight

Work top to bottom. Each milestone ends with a demo the human can run. Do not start a milestone before the previous one's exit criteria pass.

## M0 — Repo foundation (done)
- Cargo workspace (`crates/`) + pnpm/Turborepo (`apps/`, `packages/`) with the layout in `CLAUDE.md`; `rust-toolchain.toml` pinned; strict TS, ESLint, Prettier, Vitest; rustfmt + clippy.
- `schemas/` with a first FlatBuffers envelope + `pnpm codegen`.
- docker compose: Postgres, Redis.
- CI: cargo fmt/clippy/test (x86_64 + aarch64 + wasm32 determinism job), TS lint/typecheck/test.
- **Exit:** `pnpm dev` boots empty apps; CI green on all targets.
- Shipped: placeholder sim-core world (random walkers) with golden-hash determinism test; Ping/Pong/Subscribe envelope end-to-end (Rust fixture decoded in TS, web pings relay); holding page. Decisions in `docs/adr/0001-m0-foundation-tooling.md`. Bench regression gate (>15%) lands with real benches in M1.

## M1 — Headless simulation core (Rust)
- `sim-core`: world grid for one island, agents (SoA), seeded RNG, spatial hash, NN forward pass, actions FORAGE/REST/MATE, energy, death, reproduction with crossover + mutation; `balance.toml`.
- `sim-headless`: run N days, print population curve, mean energy, genome diversity; CSV/JSON report.
- Determinism test (native vs wasm32 hashes) + criterion benchmarks.
- **Exit:** for 3 seeds a population survives 30 sim days without extinction or explosion; release tick < 5 ms at 12k agents.

## M2 — Culture and the Beacon
- Actions BUILD/CARRY/SIGNAL/TEACH/PRAY; memes, lexicon, status, mate choice by status.
- Beacon construction rules, civ dividend, star light.
- Stages and culture metrics; chronicle events.
- 5 islands on parallel threads (`sim-headless` runs many seeds via `rayon`).
- **Exit:** in headless runs at least 3 of 5 civs reach Beacon > 30% within a simulated season; civs differ measurably in culture metrics.

## M3 — Live pipeline
- Rust `sim-runner`: threaded tick loop, Redis publishing, FlatBuffers frames, snapshots (postcard + zstd), input stream, resume from snapshot; `sim-replay` verifies hashes.
- Full FlatBuffers schemas for frames + LOD; generated Rust and TS code.
- `relay` fan-out with subscriptions.
- **Exit:** a CLI client can subscribe, receive strategic frames and an island stream; kill/restart runner resumes with identical hash.

## M4 — Client: strategic and island views
- r3f scene: void, dust, star, islands, settlement instancing, Beacons, agents as points, bloom.
- HUD: top-left, top-right countdown, race tracker, chronicle, minimap; zoom ladder.
- Free camera with three semantic zoom levels.
- **Exit:** 60 fps strategic view on reference laptop; matches design frames 01 and 02.

## M5 — Agent zoom and culture view
- Agent selection, inspector, follow camera, filaments, DOF.
- Culture radial view per civ.
- **Exit:** matches frames 03 and 04; inspector updates live.

## M6 — Accounts, Spark, blessings
- Auth, wallet ledger, Stripe Checkout packs, blessing tray, blessing flow end-to-end, diminishing returns, refunds on timeout.
- Rain of Light visual.
- **Exit:** a test-mode purchase → blessing → visible effect in the world and chronicle; ledger reconciles.

## M7 — Mobile and polish
- Mobile layout (strategic + island, bottom sheets).
- Season end cinematic, recap page, shareable agent permalinks.
- **Exit:** matches mobile frames; Lighthouse performance ≥ 80 on mobile for landing route.

## M8 — Soft launch
- Observability, alerts, load test relays (5k simulated viewers), admin tools, moderation.
- First public season.

## Later
Patron civs, predictions with free points, highlight clips, lineage view, migration between islands, season carry-over.
