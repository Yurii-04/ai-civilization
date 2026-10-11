//! Same seed + same inputs => identical world hash, on every target.
//!
//! CI runs this file on `x86_64`, `aarch64` and `wasm32` (`wasm32-wasip1` under
//! wasmtime). `GOLDEN` pins the expected hash, so a target that drifts fails
//! on its own without a cross-job comparison.

use sim_core::hash::world_hash;
use sim_core::{step, Balance, CivId, Input, InputKind, World};

const SEED: u64 = 7;
const TICKS: u64 = 2_400;

/// Hash after `TICKS` ticks of seed `SEED` with `inputs()`.
/// Update only for an intentional sim change, and say so in the PR.
const GOLDEN: u64 = 0x0e53_e7f7_19ec_eba2;

fn inputs() -> Vec<Input> {
    vec![
        Input {
            tick: 100,
            civ: CivId::Kael,
            kind: InputKind::Marker(3),
        },
        Input {
            tick: 1_000,
            civ: CivId::Solun,
            kind: InputKind::Marker(11),
        },
    ]
}

fn run(seed: u64, ticks: u64, inputs: &[Input]) -> u64 {
    let mut world = World::new(seed, Balance::embedded().expect("balance"));
    for _ in 0..ticks {
        let tick = world.tick;
        let due: Vec<Input> = inputs.iter().copied().filter(|i| i.tick == tick).collect();
        step(&mut world, &due);
    }
    world_hash(&world)
}

#[test]
fn matches_golden_hash() {
    let hash = run(SEED, TICKS, &inputs());
    println!("determinism hash seed={SEED} ticks={TICKS}: {hash:#018x}");
    assert_eq!(hash, GOLDEN, "world hash drifted: got {hash:#018x}");
}

#[test]
fn repeated_runs_are_identical() {
    assert_eq!(run(SEED, 500, &inputs()), run(SEED, 500, &inputs()));
}

#[test]
fn seed_and_inputs_change_the_hash() {
    let base = run(SEED, 500, &inputs());
    assert_ne!(base, run(SEED + 1, 500, &inputs()));
    assert_ne!(base, run(SEED, 500, &[]));
}
