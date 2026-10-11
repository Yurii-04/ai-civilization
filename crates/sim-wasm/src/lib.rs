//! Browser bindings for `sim-core`. Dev/replay tools only, never the live
//! game client (see `docs/ARCHITECTURE.md`).

use sim_core::hash::world_hash;
use sim_core::{step, Balance, World};
use wasm_bindgen::prelude::*;

/// Runs `ticks` ticks of the given seed with no inputs and returns the world
/// hash as a `0x…` hex string (u64 does not fit a JS number).
///
/// # Errors
/// Fails if the embedded balance is invalid.
#[wasm_bindgen(js_name = worldHash)]
pub fn world_hash_after(seed: u64, ticks: u32) -> Result<String, JsError> {
    let balance = Balance::embedded()?;
    let mut world = World::new(seed, balance);
    for _ in 0..ticks {
        step(&mut world, &[]);
    }
    Ok(format!("{:#018x}", world_hash(&world)))
}
