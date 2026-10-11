//! Live season host.
//!
//! M0: runs the world at a fixed tick rate and logs progress. Per-island
//! threads, Redis publishing, inputs and snapshots arrive in M3.

use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use sim_core::balance::balance_hash;
use sim_core::hash::world_hash;
use sim_core::{step, Balance, World};
use tracing::info;

#[derive(Debug, Parser)]
#[command(about = "Relight simulation runner")]
struct Args {
    /// Season seed.
    #[arg(long, default_value_t = 7)]
    seed: u64,
    /// Season speed multiplier (ticks per real second = base rate × speed).
    #[arg(long, default_value_t = 1.0)]
    speed: f64,
    /// Log a status line every N ticks.
    #[arg(long, default_value_t = 100)]
    log_every: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let args = Args::parse();
    anyhow::ensure!(
        args.speed > 0.0 && args.speed.is_finite(),
        "--speed must be positive"
    );
    let balance = Balance::embedded().context("loading balance.toml")?;
    let rate = f64::from(balance.time.ticks_per_second) * args.speed;
    let mut world = World::new(args.seed, balance);
    info!(
        seed = args.seed,
        rate,
        balance_hash = format_args!("{:#018x}", balance_hash()),
        population = world.population(),
        "sim-runner started"
    );

    let mut interval = tokio::time::interval(Duration::from_secs_f64(1.0 / rate));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let out = step(&mut world, &[]);
                if out.tick.is_multiple_of(args.log_every) {
                    info!(
                        tick = out.tick,
                        day = world.day(),
                        hash = format_args!("{:#018x}", world_hash(&world)),
                        "tick"
                    );
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!(tick = world.tick, "shutting down");
                return Ok(());
            }
        }
    }
}
