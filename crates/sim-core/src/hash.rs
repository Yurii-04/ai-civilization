//! World hash: xxh3 over the exact bits of the simulation state.
//!
//! Hashing raw `f32` bits (rather than quantized values) makes the
//! determinism test as strict as possible: any cross-target float drift fails.

use xxhash_rust::xxh3::Xxh3;

use crate::world::World;

pub fn world_hash(world: &World) -> u64 {
    let mut h = Xxh3::new();
    h.update(&world.tick.to_le_bytes());
    h.update(&world.seed.to_le_bytes());
    for island in &world.islands {
        h.update(&[island.civ as u8]);
        h.update(&island.marker_sum.to_le_bytes());
        h.update(&(island.len() as u64).to_le_bytes());
        for v in island
            .pos_x
            .iter()
            .chain(&island.pos_y)
            .chain(&island.heading)
        {
            h.update(&v.to_bits().to_le_bytes());
        }
    }
    h.digest()
}
