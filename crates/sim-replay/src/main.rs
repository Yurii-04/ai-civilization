//! Snapshot replay and hash verification.
//!
//! The CLI surface is fixed now; snapshots and input logs are implemented in
//! M3 together with the runner's persistence.

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Replay a Relight snapshot and verify hashes")]
struct Args {
    #[arg(long)]
    snapshot: PathBuf,
    #[arg(long)]
    inputs: PathBuf,
    /// Compare recorded hashes against the replay.
    #[arg(long)]
    verify: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    anyhow::bail!(
        "snapshot replay is not implemented yet (M3); got snapshot={} inputs={} verify={}",
        args.snapshot.display(),
        args.inputs.display(),
        args.verify
    )
}
