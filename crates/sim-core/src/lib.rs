//! Relight simulation core.
//!
//! Pure and deterministic: no I/O, no threads, no wall clock, no std float
//! transcendentals (see `CLAUDE.md`, hard rules 1–3). Every tick is
//! [`step`]`(&mut World, &[Input])`.
//!
//! M0 ships the skeleton only: balance loading, seeded RNG streams, world
//! hashing and a placeholder population of random walkers that exercises the
//! cross-target determinism harness. Real agents arrive in M1.

#![forbid(unsafe_code)]

pub mod balance;
pub mod civ;
pub mod events;
pub mod hash;
pub mod input;
pub mod rng;
pub mod world;

pub use balance::{Balance, BalanceError};
pub use civ::CivId;
pub use events::Event;
pub use input::{Input, InputKind};
pub use world::{Island, World};

/// Everything a tick produced that the host may want to publish or log.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StepOutput {
    /// Tick that was just simulated.
    pub tick: u64,
    /// Events in deterministic order: island index, then emission order.
    pub events: Vec<Event>,
}

/// Advances the world by exactly one tick.
///
/// Only inputs stamped with the current tick are applied; the caller is
/// responsible for delivering each input exactly once, in a stable order.
pub fn step(world: &mut World, inputs: &[Input]) -> StepOutput {
    world.step(inputs)
}
