//! Headless season runner.
//!
//! M0: runs the placeholder world as fast as possible and reports the final
//! hash and throughput. Population curves and genome stats arrive in M1.

use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use clap::Parser;
use serde::Serialize;
use sim_core::balance::balance_hash;
use sim_core::hash::world_hash;
use sim_core::{step, Balance, World};

#[derive(Debug, Parser)]
#[command(about = "Run Relight seasons headless")]
struct Args {
    #[arg(long, default_value_t = 7)]
    seed: u64,
    /// Sim days to run.
    #[arg(long, default_value_t = 1)]
    days: u64,
    /// Directory to write `report.json` into.
    #[arg(long)]
    report: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct Report {
    seed: u64,
    days: u64,
    ticks: u64,
    population: usize,
    world_hash: String,
    balance_hash: String,
    elapsed_ms: u128,
    mean_tick_us: f64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let balance = Balance::embedded().context("loading balance.toml")?;
    let ticks = args.days * u64::from(balance.time.ticks_per_day);
    let mut world = World::new(args.seed, balance);

    let start = Instant::now();
    for _ in 0..ticks {
        step(&mut world, &[]);
    }
    let elapsed = start.elapsed();

    let report = Report {
        seed: args.seed,
        days: args.days,
        ticks,
        population: world.population(),
        world_hash: format!("{:#018x}", world_hash(&world)),
        balance_hash: format!("{:#018x}", balance_hash()),
        elapsed_ms: elapsed.as_millis(),
        mean_tick_us: elapsed.as_secs_f64() * 1e6 / ticks.max(1) as f64,
    };
    let json = serde_json::to_string_pretty(&report)?;
    println!("{json}");

    if let Some(dir) = args.report {
        std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
        let path = dir.join("report.json");
        std::fs::write(&path, json).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}
