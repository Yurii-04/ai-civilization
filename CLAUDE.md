# CLAUDE.md — Relight

This file is the entry point for any AI agent working in this repo. Read it fully before changing code. Deeper docs live in `docs/`; read the one that matches your task before starting.

## What we are building

**Relight** is a live, spectator-first simulation game for the web. Five civilizations of tiny neural-network agents evolve on their own, on five floating islands in a black void, and race to reignite a dead star by building Beacon towers toward it. Viewers do not control anything directly: they watch from a free 3D strategy camera, zoom from the whole map down to a single agent, follow the culture and history of each civilization, and can send paid **blessings** (Rain of Light, Inspiration, Vision, Shield) that nudge a civilization. A season ends when the first Beacon reaches the star.

Audience: English-speaking, desktop and mobile web. All UI copy, code, comments and docs are in **English**.

| Doc | Read it when you… |
|---|---|
| `docs/PRODUCT.md` | touch anything user-facing: views, HUD, seasons, blessings, Spark economy |
| `docs/SIMULATION.md` | touch agents, genomes, evolution, culture, world rules, balance |
| `docs/ARCHITECTURE.md` | touch services, protocol, storage, infra, performance |
| `docs/ART_DIRECTION.md` | touch rendering, shaders, HUD styling, colors, typography |
| `docs/ROADMAP.md` | pick the next task or wonder what is in/out of scope |

## Stack

- **Languages:** **Rust** (stable, edition 2021) for the simulation; **TypeScript** (strict, Node 22 LTS) for everything else.
- **Monorepo:** Cargo workspace (`crates/`) + pnpm workspaces with Turborepo (`apps/`, `packages/`). Turborepo also drives cargo tasks through package scripts.
- **Simulation core (`crates/sim-core`):** pure Rust library, data-oriented (struct-of-arrays `Vec<f32>`/`Vec<u32>`), no I/O, no threads inside, no `unsafe` without an ADR. Builds natively (runner, headless tools) and to **WASM** (`crates/sim-wasm`, via `wasm-bindgen`) for browser replays and dev tools.
- **Sim runner (`crates/sim-runner`):** native Rust binary. `tokio` for I/O, one OS thread per island for ticking (`std::thread` / `rayon` scoped), `redis` crate for publishing frames and reading inputs, `aws-sdk-s3`-compatible client (or `object_store`) for snapshots.
- **Protocol:** FlatBuffers schemas in `schemas/*.fbs`, code generated for Rust (`flatc --rust`) and TypeScript (`flatc --ts`). Generated code is committed and checked in CI.
- **Server (TS):** NestJS (REST API: auth, wallet, blessings, payments, admin), a separate realtime **relay** service using `uWebSockets.js` for fan-out to viewers.
- **Data:** PostgreSQL + Prisma (users, wallets, ledger, blessings, season history), Redis (pub/sub between sim and relays, rate limits, ephemeral state). Snapshots/replays in S3-compatible object storage.
- **Client:** Next.js (App Router) + React for the HUD; three.js via `@react-three/fiber` + `@react-three/drei` + `postprocessing` for the world. Zustand for client state.
- **Payments:** Stripe (Spark packs). No app-store billing in v1.
- **Tests:** Rust: `cargo test` (unit, determinism, sim regression), `criterion` benches, `proptest` for invariants; `clippy -D warnings` and `rustfmt`. TS: Vitest, Playwright (smoke e2e).
- **Infra:** Docker; Hetzner dedicated/cloud servers (cheap egress); Cloudflare in front for static assets and CDN.

Do not add a new language, framework, ORM, state library or 3D engine without writing an ADR in `docs/adr/` and getting human approval.

## Repo layout

```
crates/                 Rust (Cargo workspace)
  sim-core/             THE simulation: world, agents, genome, evolution, culture. Pure + deterministic
  sim-runner/           native binary: hosts the live season, ticks islands, publishes to Redis, snapshots
  sim-headless/         CLI: run seasons without rendering, pre-evolve populations, dump stats/reports
  sim-replay/           CLI: load snapshot + input log, replay, verify hashes, export events
  sim-wasm/             wasm-bindgen wrapper of sim-core for browser replays and dev tools
  protocol/             generated FlatBuffers Rust code + frame builders (LOD encoding)
apps/                   TypeScript
  web/                  Next.js client: HUD (React) + world renderer (r3f)
  api/                  NestJS: auth, users, Spark wallet, blessings, Stripe webhooks, admin
  relay/                uWebSockets.js fan-out: subscribes to Redis, streams to viewers
packages/               TypeScript
  protocol/             generated FlatBuffers TS code + decoders, PROTOCOL_VERSION
  shared/               shared types, constants (civ ids, colors, blessing catalog), utils
  ui/                   HUD primitives (mono labels, thin-line panels, progress lines)
schemas/                FlatBuffers .fbs — the single source of truth for the wire format
config/
  balance.toml          every simulation tuning number (embedded into sim-core at build time)
docs/                   product, simulation, architecture, art direction, roadmap, ADRs
```

## Commands

```
pnpm install
pnpm dev                 # web + api + relay + sim-runner (docker compose for postgres/redis)
pnpm codegen             # flatc: schemas/*.fbs -> crates/protocol + packages/protocol
pnpm test                # cargo test --workspace + vitest
pnpm lint && pnpm typecheck   # includes cargo clippy -D warnings, cargo fmt --check, eslint, tsc

# Rust only
cargo test -p sim-core
cargo test -p sim-core --test determinism      # same seed + inputs => identical world hash
cargo bench -p sim-core                        # tick benchmarks (criterion)
cargo run -p sim-headless --release -- --seed 7 --days 30 --report out/
cargo run -p sim-replay --release -- --snapshot <file> --inputs <file> --verify
wasm-pack build crates/sim-wasm --target web   # browser build

pnpm db:migrate          # prisma migrate dev
```

Always run sim binaries with `--release` for anything performance-related. If a command does not exist yet, add it (Cargo alias in `.cargo/config.toml` or a package script + `turbo.json`), then document it here.

## Hard rules

1. **The simulation is deterministic, natively and in WASM.** `crates/sim-core` may not use `rand::thread_rng`, `std::time`, `HashMap`/`HashSet` iteration for logic (use `Vec`, `BTreeMap` or `IndexMap`), threads, or platform-dependent float math. Use the seeded PRNG in `sim_core::rng` (`rand_xoshiro`). Use `libm` for transcendental functions (no `f32::sin`/`exp`/`tanh` from std in logic), no FMA, no fast-math, no SIMD intrinsics in decision paths unless proven bit-identical. Every tick is `fn step(&mut World, &[Input])`. Blessings and admin actions are *inputs* stamped with a tick number. CI runs the determinism test on x86_64, aarch64 and wasm32 and compares hashes.
2. **sim-core has no I/O.** No network, filesystem, `println!`, or async. Diagnostics go through returned stats/event structs. Parallelism lives in `sim-runner`/`sim-headless` (one island per thread); islands never share mutable state within a tick.
3. **Fixed tick.** The world advances at a fixed rate (default 10 ticks/s simulated time, scaled by the season speed multiplier). Rendering never drives simulation.
4. **Money is a ledger.** Spark balances are derived from an append-only `LedgerEntry` table. Never update a balance column in place. Every blessing purchase is idempotent (idempotency key) and is applied to the sim only after the ledger commit.
5. **Fairness guardrails.** Blessings have diminishing returns per civilization and per time window (see `docs/PRODUCT.md`). Never add a blessing or tweak that lets money directly add Beacon segments.
6. **Bandwidth is the main cost.** Do not stream full agent positions to every viewer. Follow the LOD protocol in `docs/ARCHITECTURE.md`.
7. **Performance budgets are tests.** One island tick at 12,000 agents must stay under 5 ms on one core of the reference machine (release build); 60,000 agents across 5 islands under 10 ms wall-clock with one thread per island. The client must hold 60 fps at strategic zoom on a 2020 MacBook Air. Keep criterion benches in `crates/sim-core/benches` and fail CI on >15% regression.
8. **Art direction is strict.** Black void, thin cyan wireframes, warm gold life, red only for danger. No rounded colorful buttons, no heavy cards, no emoji. Read `docs/ART_DIRECTION.md` before touching visuals.

## How to work

- Small, reviewable PRs. One concern per PR. Conventional commits (`feat(sim): …`, `fix(relay): …`).
- Any change to sim rules: add or update a test in `sim-core`, run `sim-headless --release` for at least 3 seeds, and paste the before/after summary (population, Beacon %, extinction count) in the PR.
- Any change to the protocol: edit `schemas/*.fbs` only, run `pnpm codegen`, bump `PROTOCOL_VERSION` (Rust and TS), keep the client tolerant of one previous version.
- Simulation balance numbers live in `config/balance.toml` only (deserialized with `serde` into a typed `Balance` struct and embedded via `include_str!`; its hash is stored in every snapshot). No magic numbers in systems. Viewer-facing catalog data (blessing prices, Spark packs) lives in `packages/shared`.
- Rust style: `clippy::pedantic` where reasonable, no `unwrap()` outside tests and startup, errors via `thiserror` (libs) / `anyhow` (bins), `tracing` for logs in binaries.
- When a decision is ambiguous or product-shaping (pricing, new blessing, season length, monetization), stop and ask the human instead of guessing. Record decisions in `docs/adr/NNNN-title.md`.
- Keep this file and the `docs/` files current. If you change how something works, update the doc in the same PR.

## Glossary

- **Agent:** one simulated being. Has a genome (neural net weights + trait genes), energy, role, memory, links.
- **Civilization (civ):** the population of one island. Ids: `vesper`, `ondra`, `kael`, `mirra`, `solun`.
- **Beacon:** a civ's segmented tower toward the star. Progress 0–100%, 11 segments.
- **Stage:** Tribal → Agrarian → Industrial → Signal Age → Ascendant. Unlocked by emergent behavior thresholds.
- **Meme:** a culturally transmitted unit (glyph, recipe, doctrine) passed between agents during their lifetime.
- **Void storm:** periodic world event that damages island frontiers; countdown shown in the HUD.
- **Blessing:** a paid viewer action, applied to the sim as a timestamped input.
- **Spark:** the soft currency viewers buy in packs and spend on blessings.
- **Season:** one full race, from seeded start to the first Beacon reaching the star.
