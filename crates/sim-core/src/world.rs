//! World and islands.
//!
//! M0 placeholder: each island holds a struct-of-arrays population of random
//! walkers inside a disc. It exists so determinism (native vs WASM) and the
//! runner/headless plumbing can be tested before real agents land in M1.

use crate::balance::Balance;
use crate::civ::CivId;
use crate::events::Event;
use crate::input::{Input, InputKind};
use crate::rng::{SimRng, Stream};

const TURN_JITTER: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct World {
    pub tick: u64,
    pub seed: u64,
    pub balance: Balance,
    pub islands: Vec<Island>,
}

#[derive(Debug, Clone)]
pub struct Island {
    pub civ: CivId,
    pub pos_x: Vec<f32>,
    pub pos_y: Vec<f32>,
    pub heading: Vec<f32>,
    /// Accumulates applied input markers so inputs affect the world hash.
    pub marker_sum: u32,
    rng: SimRng,
}

impl World {
    pub fn new(seed: u64, balance: Balance) -> Self {
        let islands = CivId::ALL
            .iter()
            .map(|&civ| Island::new(seed, civ, &balance))
            .collect();
        Self {
            tick: 0,
            seed,
            balance,
            islands,
        }
    }

    pub fn day(&self) -> u64 {
        self.tick / u64::from(self.balance.time.ticks_per_day)
    }

    pub(crate) fn step(&mut self, inputs: &[Input]) -> crate::StepOutput {
        let tick = self.tick;
        let mut events = Vec::new();
        if tick.is_multiple_of(u64::from(self.balance.time.ticks_per_day)) {
            events.push(Event::DayStarted { day: self.day() });
        }
        for island in &mut self.islands {
            island.step(tick, inputs, &self.balance, &mut events);
        }
        self.tick += 1;
        crate::StepOutput { tick, events }
    }

    /// Total population across islands.
    pub fn population(&self) -> usize {
        self.islands.iter().map(Island::len).sum()
    }
}

impl Island {
    fn new(seed: u64, civ: CivId, balance: &Balance) -> Self {
        let n = balance.world.placeholder_walkers_per_island as usize;
        let radius = balance.world.island_radius;
        let mut rng = SimRng::derive(seed, civ as u32, Stream::Walkers);
        let mut pos_x = Vec::with_capacity(n);
        let mut pos_y = Vec::with_capacity(n);
        let mut heading = Vec::with_capacity(n);
        for _ in 0..n {
            // Uniform in the disc.
            let r = radius * libm::sqrtf(rng.next_f32());
            let a = rng.next_f32() * core::f32::consts::TAU;
            pos_x.push(r * libm::cosf(a));
            pos_y.push(r * libm::sinf(a));
            heading.push(rng.next_f32() * core::f32::consts::TAU);
        }
        Self {
            civ,
            pos_x,
            pos_y,
            heading,
            marker_sum: 0,
            rng,
        }
    }

    pub fn len(&self) -> usize {
        self.pos_x.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pos_x.is_empty()
    }

    fn step(&mut self, tick: u64, inputs: &[Input], balance: &Balance, events: &mut Vec<Event>) {
        for input in inputs
            .iter()
            .filter(|i| i.tick == tick && i.civ == self.civ)
        {
            match input.kind {
                InputKind::Marker(value) => {
                    self.marker_sum = self.marker_sum.wrapping_add(value);
                }
            }
            events.push(Event::InputApplied {
                civ: self.civ,
                kind: input.kind,
            });
        }

        let speed = balance.world.walker_speed;
        let radius_sq = balance.world.island_radius * balance.world.island_radius;
        for i in 0..self.len() {
            let mut h = self.heading[i] + (self.rng.next_f32() - 0.5) * TURN_JITTER;
            let nx = self.pos_x[i] + speed * libm::cosf(h);
            let ny = self.pos_y[i] + speed * libm::sinf(h);
            if nx * nx + ny * ny > radius_sq {
                // Bounce off the island edge.
                h += core::f32::consts::PI;
            } else {
                self.pos_x[i] = nx;
                self.pos_y[i] = ny;
            }
            self.heading[i] = libm::remainderf(h, core::f32::consts::TAU);
        }
    }
}
