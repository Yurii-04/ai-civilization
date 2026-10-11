//! Seeded PRNG streams. The only randomness allowed in the simulation.
//!
//! Each (island, system) pair owns an independent `Xoshiro128StarStar`
//! stream derived from the season seed with `SplitMix64`, so adding a draw in
//! one system never shifts the sequence of another.

use rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128StarStar;

/// Stable ids for per-system streams. Never renumber; append only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Stream {
    /// M0 placeholder walkers.
    Walkers = 0,
}

#[derive(Debug, Clone)]
pub struct SimRng(Xoshiro128StarStar);

impl SimRng {
    /// Derives the stream for `island` / `stream` from the season seed.
    pub fn derive(season_seed: u64, island: u32, stream: Stream) -> Self {
        let mut state = season_seed
            ^ (u64::from(island) << 32)
            ^ u64::from(stream as u32).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let mut seed = [0u8; 16];
        for chunk in seed.chunks_exact_mut(8) {
            chunk.copy_from_slice(&splitmix64(&mut state).to_le_bytes());
        }
        Self(Xoshiro128StarStar::from_seed(seed))
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    /// Uniform in `[0, 1)` with 24 bits of precision (exact in `f32`).
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0)
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::{SimRng, Stream};

    #[test]
    fn same_seed_same_sequence() {
        let mut a = SimRng::derive(7, 0, Stream::Walkers);
        let mut b = SimRng::derive(7, 0, Stream::Walkers);
        for _ in 0..64 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }

    #[test]
    fn islands_get_independent_streams() {
        let mut a = SimRng::derive(7, 0, Stream::Walkers);
        let mut b = SimRng::derive(7, 1, Stream::Walkers);
        let same = (0..64).filter(|_| a.next_u32() == b.next_u32()).count();
        assert!(same < 2);
    }

    #[test]
    fn f32_in_unit_interval() {
        let mut rng = SimRng::derive(1, 0, Stream::Walkers);
        for _ in 0..10_000 {
            let x = rng.next_f32();
            assert!((0.0..1.0).contains(&x));
        }
    }
}
