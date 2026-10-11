# Product — Relight

## One line

Watch five AI civilizations evolve on their own and race to reignite a dead star; back the one you believe in.

## Pillars

1. **It is alive without you.** The world runs 24/7 whether or not anyone is watching. Nothing is scripted; stories emerge from evolution and culture.
2. **Watching is the game.** Spectating must be as engaging as a good documentary or a live sports broadcast: clear stakes (the race), drama (storms, schisms, comebacks), and readable visuals at every zoom level.
3. **Influence, not control.** Viewers nudge civilizations with blessings. They never place buildings, move agents or pick winners directly.
4. **Premium and mysterious.** Cinematic sci-fi data visualization, not a cartoon game. See `ART_DIRECTION.md`.

## Target user

- English-speaking web users who enjoy ambient/idle games, simulation and AI curiosity, and live-stream style participation (Twitch chat energy, prediction markets, fantasy sports).
- Two modes: **lean-back** (leave it open on a second monitor, glance at the race) and **lean-in** (zoom into an island, follow an agent, read the chronicle, send a blessing at a critical moment).

## World

- A black void with 5 floating island-platforms arranged in a ring. Each island hosts one civilization with a signature accent color:
  - Vesper `#6BE6F5` (cyan), Ondra `#A88FFF` (violet), Kael `#75F2AE` (jade), Mirra `#FF8FC7` (rose), Solun `#94B8FF` (ice blue).
- At the center, high above, hangs **the dead star**: a dim grey orb with gold cracks. Cracks brighten as total Beacon progress rises.
- Each island's surface fills over time with a voxel settlement (lit windows, roads of light), holographic towers, and the civ's **Beacon** in the central plaza.
- Starting conditions differ slightly per island (size, resources, storm exposure) so civs have distinct arcs; generated from the season seed.

## Season loop

1. **Seed.** A season starts from a seed and from populations pre-evolved headless (see `SIMULATION.md` → Bootstrapping), so agents can already survive on day 1.
2. **Rise.** Civs grow, specialize, invent language and roles, move through stages.
3. **Race.** Beacon construction becomes the dominant economic activity. Tallest Beacons link to the star with a light beam.
4. **Climax.** The first Beacon to reach 100% relights the star. Cinematic ending, season recap, hall of fame.
5. **Afterlight.** Short break (24–48 h): recap pages, lineage trees, highlights reel; next season seeded.

Season length target: **14–21 real days** at default speed. Sim speed multiplier is shown in the HUD (e.g. ×2.5) and may change during quiet periods.

## Views and zoom levels

The camera is a top-down strategy camera tilted ~50°, free pan/zoom/rotate. Zoom has three semantic levels; HUD adapts to each.

| Level | What you see | HUD focus |
|---|---|---|
| **Strategic** | all 5 islands as glowing clusters, star, Beacon heights comparable | civ name, stage, Beacon % per island; race tracker |
| **Island** | one island fills the screen: voxel city, roads, Beacon, agent flows | civ stats, district labels, local event log |
| **Agent** | individual agents as glowing points joined by communication filaments | agent inspector next to the selected agent |

Extra views:
- **Culture view:** radial map of a civ — agents as dots on a polar grid, trait arcs around the rim (Hierarchy, Shared language, Knowledge transfer, Belief in the Star, Division of labor), cohesion, schism risk, dominant doctrine.
- **Lineage view (later):** family tree of notable agents and how genes/memes spread.

### Agent inspector
ID (6 hex chars, e.g. `23619E`), civ, generation, age, role, lineage, number of links, trait bars, a tiny visualization of its neural net with currently active pathways, and the last ~5 actions as log lines (`RECV`, `SEND`, `MOVE`, `LEARN`, `PRAY`, `BUILD`…). Buttons: Follow (camera tracks the agent), Inspire (opens Inspiration blessing targeted at this agent).

## HUD (desktop)

At least 85% of the screen is world. Thin lines, transparency, monospace text. No heavy panels.

- **Top-left:** wordmark, `SEASON 07 · DAY 23`, view tabs (Map / Culture / Lineage).
- **Top-right:** countdown to next void storm (large mono, `T-23:54:50`), sim speed, viewers online.
- **Right edge:** race tracker — 5 rows: color dot, name, stage, Beacon % and a thin progress line, daily delta.
- **Left edge:** zoom ladder (Strategic / Island / Agent).
- **Bottom-left:** live chronicle (5 lines max; white events, red danger, gold blessings).
- **Bottom-right:** minimap + **BLESS** button that opens the blessing tray.

## Mobile (390×844)

Strategic and island zoom only in v1. Top bar with season + storm countdown. Race tracker and agent inspector are translucent bottom sheets (drag handle, 44 px touch targets). BLESS lives in the sheet.

## Blessings and Spark

Spark is bought in packs via Stripe and spent on blessings. Blessings are applied to the simulation as timestamped inputs and announced in the chronicle with the gifter's display name (opt-out available).

| Blessing | Effect in the sim | Draft price |
|---|---|---|
| **Rain of Light** | energy influx over an area + build speed ×1.4 for 6 h on that island | 120 Spark |
| **Inspiration** | one targeted agent receives a beneficial meme or a guided mutation | 60 Spark |
| **Vision** | reveals where the star's next fracture appears (Beacon targeting bonus for the civ that acts on it) | 200 Spark |
| **Shield** | protects the island frontier from the next void storm | 340 Spark |

Draft Spark packs (to validate): 500 / $4.99, 1,200 / $9.99, 3,000 / $19.99, 8,000 / $49.99. Packs exist to amortize the per-transaction payment fee.

### Fairness rules (non-negotiable)
- **Diminishing returns:** each additional blessing of the same type on the same civ within a rolling window has reduced effect (e.g. 100% → 60% → 35% → 20%).
- **Global cooldown per civ** on Shield (one per storm).
- **No direct Beacon segments** from money, ever. Blessings change conditions; the agents still have to build.
- **Underdog bonus:** blessings on the lowest-ranked civ are slightly stronger. Comebacks are good drama.
- All blessing effects are public and logged; total Spark per civ is visible in the season recap.

## Social and retention (post-MVP candidates)
- Pick a **patron civ** per season; cosmetic badge, season recap shows your impact.
- Predictions ("Who reaches 50% first?") with free points, not money.
- Shareable clips: auto-generated 15 s highlight of big events (storm hits, schism, Beacon link).
- Named agents: viewers who bless can name a newborn agent.

## Success metrics
- Avg session length, D1/D7 return rate, % of viewers who zoom to agent level, concurrent viewers at season climax, payer conversion, ARPPU, refund/chargeback rate.

## Out of scope for v1
Native apps, player-vs-player control, user-created civilizations, chat, NFTs/crypto, real-money prediction.
