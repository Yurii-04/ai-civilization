# Simulation — Relight

Everything here lives in the Rust crate `crates/sim-core`. It must stay deterministic and I/O-free (see `CLAUDE.md`, hard rules 1–3). Tuning numbers come from `config/balance.toml`.

## Crate structure

```
crates/sim-core/src/
  lib.rs          pub World, Input, StepOutput; fn step(&mut World, &[Input]) -> StepOutput
  balance.rs      Balance struct (serde) loaded from config/balance.toml via include_str!
  rng.rs          seeded xoshiro128** streams, derivation per island/system
  world/          grid, cells, resources, buildings, roads, beacon, star, storms
  agents/         SoA storage (Agents { pos_x: Vec<f32>, energy: Vec<f32>, genome: Vec<f32>, ... })
  nn.rs           forward pass over flat weight slices; tanh via lookup table
  systems/        perception, decision, movement, actions, energy, reproduction, culture, stats
  culture/        memes, lexicon, metrics, roles
  events.rs       chronicle event enum
  hash.rs         world hash (quantized state -> xxh3)
  snapshot.rs     serialize/deserialize World (bincode/postcard, versioned)
benches/          criterion: tick_12k, nn_forward, spatial_hash
tests/            determinism.rs, invariants.rs (proptest), regressions/
```

Systems are plain functions over `&mut Island` executed in a fixed order; no ECS framework (no bevy_ecs) — the data is simple and explicit SoA gives the best control over determinism and cache layout. Islands are independent structs, so the runner can step them on separate threads and merge `StepOutput`s in island-index order.

## Design goals

1. **Emergence over scripting.** Roles, language, cooperation and Beacon building should arise from evolution and incentives, not from hand-written behavior trees.
2. **Watchable from day 1.** Agents must look purposeful early, so we pre-evolve populations and use high-level actions.
3. **Cheap.** Tens of thousands of agents on one server core. Tiny networks, spatial hashing, struct-of-arrays.
4. **Legible.** Every interesting internal state must be summarizable for the HUD (traits, role, last actions, culture metrics).

## World model

- 5 islands, each a 2D grid of cells (≈ 0.5 world units) inside an irregular outline; the 3D view is a presentation of this 2D world plus building heights.
- Cell contents: terrain type, resource stock (food/energy, material), building id, road flag, ownership by civ.
- Per-island: Beacon progress (segments 0–11 + scaffold fraction), stage, culture metrics, storm exposure.
- Global: tick, day, season seed, star state (fracture positions, luminance), storm schedule.
- Islands are separate populations (no migration in v1). This lets cultures diverge.

## Time

- Fixed tick, default **10 ticks/s of sim time**. 1 sim day = 24,000 ticks by default (tunable in `balance.toml`).
- The season speed multiplier (×1–×4) only changes how many ticks per real second the runner executes.
- Expensive systems run on staggered schedules: perception every tick, decision every 2 ticks, reproduction/culture every 50 ticks, stats every 600 ticks.

## Agent state (struct-of-arrays)

```
id: u32, civ: u8, alive: u8
pos: f32x2, vel: f32x2, energy: f32, age: u32, generation: u16
role: u8 (emergent label, computed), status: f32, targetCell: i32
memory: f32[M] (small recurrent state, M = 4)
genome: f32[G]  (NN weights + trait genes)
memes: u16[K]   (ids of known memes, K = 16, LRU)
links: u32[L]   (recent communication partners, L = 6)
lastActions: ring buffer of 8 (tick, actionId, arg)
```

Default caps: 12,000 agents per island, 60,000 total.

## Genome

`genome = [ NN weights | trait genes ]`

- **Network:** inputs 14 → hidden 6 → hidden 6 → outputs 8 (+ memory write 4). Dense, `tanh` hidden, biases included. ~200 floats. The UI shows it as "4·6·6·3" grouped layers; the exact sizes live in `balance.toml` (layer sizes are compile-time consts in `nn.rs`, validated against the TOML at startup).
- **Trait genes** (0–1, clamped): curiosity, cooperation, risk, devotion, fertility, signal frequency. They modulate action costs, mutation rate of memes, mate choice and storm behavior, and they are what the HUD shows as traits.

### Inputs (normalized, all local)
energy, age, local food, local material, distance & direction to plaza/Beacon, crowd density, nearest kin distance, incoming signal (strongest glyph id embedding, 2 dims), storm proximity, own status, 4 memory values.

### Outputs → high-level actions
The net outputs scores; the action is `argmax` over valid actions with a seeded softmax temperature for variety:
`FORAGE`, `BUILD` (house/road/Beacon), `CARRY` (move material to a site), `SIGNAL` (emit a glyph), `TEACH` (copy a meme to a linked agent), `MATE`, `REST`, `PRAY` (face the star; feeds devotion & culture).
Movement is solved by a simple steering/pathfinding layer toward the chosen action's target. The network never outputs raw motor commands.

## Evolution (neuroevolution, no gradients)

- **Energy is fitness.** Every action costs energy; foraging, receiving shared food, and successful building (via civ rewards) return energy. Agents at 0 energy die. Old age: probabilistic death after a genetic lifespan.
- **Reproduction:** when two agents of the same civ choose `MATE`, are adjacent, and both have energy above the threshold, they pay energy and spawn a child.
- **Crossover:** uniform crossover per weight block (layer-wise) of the two parents' genomes.
- **Mutation:** each weight mutates with probability p≈0.05 by Gaussian noise σ≈0.03; rare large mutations (p≈0.001, σ≈0.5). Trait genes mutate the same way and are clamped.
- **Mate choice:** probability weighted by partner `status` and `energy` (sexual selection). This is the main lever that rewards Beacon work.
- **Population control:** carrying capacity via local food regrowth; hard cap per island culls the lowest-energy agents.
- **Speciation guard:** if genome diversity (mean pairwise distance on a sample) falls below a threshold, temporarily raise mutation rate. Prevents a civ from collapsing into clones.

## Pressure toward the star

Without an external goal agents would only learn to survive. Beacon building is wired into the economy:
- **Status:** contributing material/labor to the Beacon raises an agent's status; status decays slowly. High status → more mates → genes spread.
- **Civ dividend:** each completed Beacon segment increases food regrowth and storm resistance for the whole island.
- **Star light:** the higher the Beacon, the more energy the star radiates onto that island (small but compounding).
- **Devotion trait:** `PRAY` near the Beacon slightly boosts nearby builders' efficiency (a reason for "belief" to evolve).

Beacon construction rules: segments require material delivered to the scaffold and a minimum number of simultaneous builders (coordination). Higher segments need more material and coordination. Stage gates also apply (Signal Age required for segments 9–11).

## Culture (fast, lifetime transmission)

- A **meme** is `{ id, kind: glyph|recipe|doctrine, payload, fitnessHint }`. Recipes modify action efficiency (e.g. "mortar ratio v3" → build +8%). Doctrines bias action scores (e.g. "The Star remembers its builders" → +BUILD near Beacon).
- `TEACH` copies a meme to a linked agent; copying can mutate the meme (new variant id).
- **Language emerges as signaling:** a glyph id emitted with `SIGNAL` that precedes successful coordination (e.g. joint building) gets reinforced in both agents' associations; glyphs that coordinate spread. The civ **lexicon** = set of glyphs known by ≥ 5% of agents.
- Memes are inherited from parents at birth (vertical) and copied from peers (horizontal). Their spread is tracked per civ.

## Emergent labels and culture metrics (for the HUD)

Computed every stats pass, never fed back as rules:
- **Role** per agent: clustered from its recent action histogram (Builder, Forager, Scribe/Teacher, Carrier, Devout, Scout).
- **Hierarchy:** Gini coefficient of status.
- **Shared language:** lexicon size × mean agreement.
- **Knowledge transfer:** TEACH events per capita, meme half-life.
- **Division of labor:** entropy of role distribution (normalized, inverted).
- **Belief in the Star:** share of agents with devotion > 0.6 and PRAY frequency.
- **Cohesion / schism risk:** modularity of the communication graph; if two large communities with different dominant doctrines form → schism event (guild split) in the chronicle.

## Stages

Stage transitions are game thresholds over emergent metrics, e.g.:
- Tribal → Agrarian: food stock per capita > X for 2 days and division of labor > 0.3
- Agrarian → Industrial: road network > N cells and recipe count > R
- Industrial → Signal Age: lexicon > 500 glyphs and knowledge transfer > T
Exact thresholds live in `balance.toml` and are tuned with headless runs.

## World events

- **Void storms:** scheduled with jitter from the seed; countdown visible. A storm sweeps an arc of each island's frontier; agents and buildings in the arc take damage unless shielded. Agents with higher risk-aversion learn to move inward when `storm proximity` rises.
- **Star fractures:** new crack points on the star; Beacons aligned with a fracture gain bonus progress (Vision blessing reveals the next one).
- **Discoveries:** first time a civ reaches a metric threshold → chronicle event.

## Blessings as inputs

Blessings arrive as `Input { tick, civ, type, target?, strength }` after the ledger commit. `strength` already includes diminishing returns (computed server-side, deterministic from the input log).
- Rain of Light: food/energy injection over a radius + temporary build multiplier.
- Inspiration: grant a high-value meme or apply a guided mutation (copy weights from the civ's top-status agent with small noise) to one agent.
- Vision: reveals next fracture position to the civ (agents' "Beacon direction" input gets the fracture bias).
- Shield: marks the island as protected for the next storm.

## Bootstrapping a season

1. `sim-headless` (release build) runs each island for thousands of generations with rendering off and speed uncapped (multi-core: one thread per island, plus `rayon` to run many seeds in parallel).
2. Pick populations at a "pre-Tribal" level: agents forage, rest and mate competently but have no Beacon, no lexicon.
3. Freeze as a season snapshot; the live season starts from it with the season seed.

## Determinism and replay

- Single seeded PRNG stream per system per island (`rand_xoshiro::Xoshiro128StarStar`), derived from season seed with SplitMix64.
- Agent iteration order is by index; removal uses tombstones + compaction at fixed intervals.
- Use `libm` or the `nn::tanh_lut` lookup table; never std float transcendental functions in logic. Compile without `target-cpu=native` for the runner so native and WASM results match. CI compares hashes across x86_64, aarch64 and wasm32.
- Every N ticks compute a world hash (xxh3 over quantized positions, energies, Beacon). `cargo test -p sim-core --test determinism` asserts equal hashes across runs and targets.
- Snapshot every sim hour to object storage + append-only input log → full replay and crash recovery.

## Performance notes

- Spatial hash grid per island (cell = perception radius) rebuilt every tick.
- NN forward pass is ~200 multiply-adds per agent per decision; 60k agents × 5 decisions/s ≈ 60M MACs/s — trivial. Budget is dominated by neighbor queries and communication.
- One OS thread per island; the runner merges outputs in island order. Target (release): < 5 ms per tick per island at 12k agents. Keep hot loops branch-light and allocation-free (preallocate, reuse buffers; `#[inline]` on NN forward).

## Open questions (ask the human before deciding)
- Migration between islands later? (breaks population isolation but adds drama)
- Should a civ go extinct, or do we always reseed a minimum population?
- What carries over between seasons (nothing / lineage names / top genomes)?
