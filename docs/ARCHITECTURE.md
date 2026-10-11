# Architecture — Relight

## Overview

```
                 ┌───────────── Hetzner (or similar, cheap egress) ─────────────┐
 Stripe ──webhook──▶ apps/api (NestJS) ──▶ PostgreSQL (Prisma)                    │
                 │       │  ▲                                                     │
                 │       │  └── REST: auth, wallet, blessings, season, history    │
                 │       ▼                                                        │
                 │   Redis ◀── inputs (blessings) ──┐                             │
                 │     ▲  │                         │                             │
                 │     │  └── frames/events ──▶ apps/relay ×N (uWebSockets.js) ───┼──▶ viewers (WebSocket)
                 │     │                                                          │
                 │   crates/sim-runner (Rust) ── 5 threads (one per island)       │
                 │     └── snapshots + input log ──▶ object storage (S3-compat)   │
                 └───────────────────────────────────────────────────────────────┘
 apps/web (Next.js static + SSR pages) ──▶ Cloudflare CDN
```

## Services

### `crates/sim-runner` (Rust, native binary)
- Runtime: `tokio` for Redis/S3/HTTP-health I/O; the tick loop runs on dedicated OS threads (one per island) coordinated by a barrier each tick, so I/O never stalls simulation. Frame encoding (FlatBuffers via `crates/protocol`) happens on a separate encoder thread from a copied view of state.
- Crates: `redis` (async, streams + pub/sub), `object_store` or `aws-sdk-s3` for snapshots, `postcard`/`bincode` for snapshot serialization, `zstd` compression, `tracing` + `tracing-opentelemetry`, `axum` for `/health` and `/metrics` (Prometheus).
- Deployed as a single static binary in a minimal Docker image (`FROM gcr.io/distroless/cc`), built with `--release`, `lto = "thin"`, `codegen-units = 1`.
- Loads the season snapshot and `balance.toml` hash; refuses to start if the hash differs from the snapshot's.
- Main loop: pull pending inputs for tick T from Redis stream `sim:inputs`, advance all islands one tick, collect outputs.
- Publishes to Redis:
  - `sim:frame:strategic` at 2 Hz (aggregates per island),
  - `sim:frame:island:<civ>` at 10 Hz (agent positions quantized, building deltas),
  - `sim:events` (chronicle events, stage changes, storms, blessings applied),
  - `sim:agent:<id>` on demand (inspector detail for followed/selected agents).
- Writes snapshot + input log every sim hour; on restart, resumes from latest snapshot + replays inputs.
- Exactly one active runner per season (Redis lock with lease). A warm standby can take over from the snapshot.

### `apps/relay`
- Stateless WebSocket fan-out. Each client subscribes to channels based on its camera: strategic always; one island when zoomed in; one agent when inspecting.
- Holds the latest keyframe per channel to serve new clients instantly.
- Horizontal scale: add relays behind a load balancer; each subscribes to Redis.
- Auth: short-lived JWT from api (viewing is allowed anonymously with a guest token).

### `apps/api` (NestJS)
- Modules: `auth` (email magic link + OAuth Google/Discord), `users`, `wallet` (ledger), `blessings`, `payments` (Stripe Checkout + webhooks), `seasons` (current state, history, hall of fame), `admin` (pause, speed, seed, moderation of display names).
- Blessing flow:
  1. `POST /blessings` with idempotency key → validate (cooldowns, target valid, season live).
  2. Transaction: insert `LedgerEntry(-price)` + `Blessing(pending)`; check balance ≥ 0 via ledger sum (serializable or row lock on wallet).
  3. Compute effective strength (diminishing returns from recent blessings on that civ).
  4. `XADD sim:inputs` with target tick = current tick + small delay.
  5. Runner applies it and emits `blessing_applied` event → api marks `Blessing(applied)`. If not applied within timeout → refund entry.

### `apps/web`
- Next.js App Router. Routes: `/` (live world), `/season/[n]` (recap), `/civ/[id]`, `/agent/[id]` (shareable permalink), `/account`, `/shop`.
- World renderer (r3f) is a client component; HUD is React DOM overlaid with `pointer-events` only on controls.
- State: Zustand stores for camera, selection, latest frames, HUD.

## Network protocol (`schemas/*.fbs` → `crates/protocol` + `packages/protocol`)

FlatBuffers over WebSocket. Schemas are the single source of truth; `pnpm codegen` runs `flatc --rust` and `flatc --ts`. The Rust runner builds frames, Redis carries the raw bytes, the TS relay forwards them without decoding (it only reads the small header to route), and the browser decodes with the generated TS code (zero-copy reads). Every message is wrapped in an envelope `{ version:u8, type:u8, tick:u32, payload }`.

Messages:
- `StrategicFrame`: per civ → population, stage, Beacon progress (u16 fixed-point), centroid density map (16×16 u8), top events ids.
- `IslandKeyframe`: agent ids + positions quantized to u16 in island space + state byte (role/action); buildings list.
- `IslandDelta`: changed agents only (position delta i8, state), building diffs.
- `AgentDetail`: inspector payload (traits, generation, links, last actions, active-pathway bitmask for the NN view).
- `Event`: chronicle entries (type, civ, params, severity).
- Client → server: `Subscribe{ channels }`, `Ping`.

### Level of detail (the cost rule)
- **Strategic zoom:** client gets only aggregates; it renders agents as particles sampled from the density map with a deterministic per-cell seed. Real positions are NOT sent. ~1–2 KB/s per viewer.
- **Island zoom:** real quantized positions for one island, deltas at 10 Hz, keyframe every 5 s. Cap at 6k visible agents; sample beyond that. ~5–10 KB/s.
- **Agent zoom:** island stream + `AgentDetail` for the selected agent at 2 Hz.
- Client interpolates between frames; never simulates rules.

## Data model (Prisma, simplified)

```
User(id, email, displayName, createdAt, patronCivId?)
Wallet(id, userId)                          // balance = SUM(LedgerEntry.amount)
LedgerEntry(id, walletId, amount, kind[purchase|blessing|refund|grant], refId, createdAt)
Purchase(id, userId, stripeSessionId, pack, amountCents, status)
Blessing(id, userId, seasonId, civ, type, targetAgentId?, price, strength, status, tick?, createdAt)
Season(id, number, seed, startedAt, endedAt?, winnerCiv?, speed)
SeasonEvent(id, seasonId, tick, type, civ, payload jsonb)   // notable events only, for recaps
Snapshot(id, seasonId, tick, objectKey, hash)
```

## Rust ↔ TypeScript boundaries

There are exactly three, and no others may be added without an ADR:
1. **Redis streams/channels** — inputs from `api` to `sim-runner` (`sim:inputs`, JSON-free: a FlatBuffers `Input` message), frames and events from `sim-runner` to `relay`.
2. **FlatBuffers schemas** — shared wire types, generated for both languages.
3. **`sim-wasm`** — `wasm-bindgen` build of `sim-core` used only by dev/replay tools in the browser (e.g. an internal replay viewer and balance sandbox), never in the live game client.

The NestJS api never links Rust code. Diminishing-returns strength for blessings is computed in the api from Postgres history and sent as part of the `Input`, so the sim stays pure.

## Rendering (`apps/web`)

- Islands: extruded slab mesh, glossy dark material with a screen-space reflection approximation (mirror pass of emissive layer, blurred).
- Settlement: one `InstancedMesh` of boxes per island; windows via a shader using per-instance seed (no texture per building). Roof/edge lines as thin emissive lines.
- Holographic towers and Beacon: line segments (`LineSegments2`) + additive vertical "light strokes" in a shader; Beacon segments driven by progress uniform.
- Agents: `Points` with additive gold sprites; filaments as additive `LineSegments` between linked agents (only at island/agent zoom).
- Star: sphere with noise-based crack shader; crack emission tied to total progress.
- Post: bloom (multi-radius), vignette, subtle grain, depth-of-field only at agent zoom.
- Budgets: ≤ 300 draw calls; ≤ 150k points; dynamic resolution scaling on low FPS.

## Infra and cost targets

- Prototype: one box running everything via docker compose (≈ €70–150/mo).
- Launch (1–5k concurrent): 1 dedicated server for sim-runner, 2–3 relay nodes, managed or self-hosted Postgres + Redis, Cloudflare for web (≈ €300–800/mo).
- Avoid hyperscaler egress pricing; bandwidth is the dominant cost (see LOD).
- Observability: OpenTelemetry → Grafana stack; key metrics: tick time p99, agents alive, relay connections, bytes/s per viewer, blessing latency, ledger anomalies.

## Security
- Never trust client for anything except camera/subscription.
- Rate limit blessings per user; Stripe webhooks verified by signature; idempotency on all money endpoints.
- Display names moderated (blocklist + admin tools) because they appear in the public chronicle.
