# Art Direction — Relight

Reference: the design canvas "Relight — game stills" (7 frames: strategic, island, agent, culture, blessing, mobile strategic, mobile island): https://claude.ai/artifact/N7kSQnXR1JsNT32bzf551Y. Match it.

## Mood
A documentary about an AI civilization. Cinematic sci-fi data visualization. Elegant, mysterious, premium. High contrast, lots of negative space. Never cartoonish.

## Color

| Role | Value | Use |
|---|---|---|
| Void | `#000000` – `#020305` | background; faint dust particles |
| Ground slab | `#0B0D12` glossy | island tops, with mirror reflections of emissive geometry |
| Structure | cyan `#BDEFF7` / `#6BE6F5` at low alpha | wireframes, holographic towers, Beacon |
| Life / energy | gold `#FFC266`, warm white `#FFE3B0` | agents, filaments, roads, blessings, star cracks |
| Windows | warm white, amber, some teal `#73EAE5`, rare orange `#FF7333` | voxel buildings |
| Danger / events | red `#FF5A47` | storms, losses, dissent — and nowhere else |
| Text | `#F4F6F8` primary, `#8C96A3` secondary, `#6C7682` timestamps | HUD |

Civ accents (territory outline glow, labels, tracker dots): Vesper `#6BE6F5`, Ondra `#A88FFF`, Kael `#75F2AE`, Mirra `#FF8FC7`, Solun `#94B8FF`.

## Light
- Bloom everywhere emissive, multi-radius (tight core + wide soft halo). Soft light falloff.
- The star is the only large dim object; it should feel dead but cracked with gold.
- Reflections on island tops are blurred and dim (~20–30%).

## Shapes
- Thin lines (1 px at 1×), additive blending, no solid fills except dark slabs and voxel bodies.
- Agents: tiny glowing points; when they communicate, thin filaments form constellation / neural-network patterns.
- Beacon: tall segmented square tower narrowing upward, flanges at each joint, junction nodes, vertical light strokes inside faces, pyramid cap with a bright spire; unfinished segments shown dashed.

## Typography
- HUD: **IBM Plex Mono** (300/400/500). Uppercase micro-labels, letter-spacing 0.2–0.34em, 9–11 px. Large numerics (countdown) 36 px desktop / 21 px mobile.
- Titles: **Jost** 200–300, very wide tracking (wordmark `RELIGHT` 0.6em).
- No Inter/Roboto/Arial.

## HUD rules
- World ≥ 85% of the screen. HUD only at edges.
- Thin 1 px lines, transparency, text. Panels at most `rgba(3,5,8,0.45)` + 1 px border `rgba(255,255,255,0.14)` + backdrop blur.
- No rounded colorful buttons, no heavy cards, no gradients, no emoji, no left-border accent cards.
- Primary action (BLESS) is a thin gold outline button with a 4-point spark glyph.
- Progress = 1 px line with a glowing fill in the civ color.
- Touch targets ≥ 44 px. Text contrast ≥ 4.5:1 for body.

## Motion
- Slow, weighty camera easing (critically damped spring). Zoom transitions cross-fade HUD per level.
- Agents flow along roads; filaments flicker with traveling pulses.
- Big moments get one orchestrated effect: storm sweep (red arc), Beacon link (beam ignites from spire to star), Rain of Light (gold particle cascade with ground ripples).
- Respect `prefers-reduced-motion`: disable camera shake and particle bursts.
