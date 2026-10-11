# Relight

Five civilizations of tiny neural-network agents evolve on floating islands and race to reignite a dead star. Viewers watch and send blessings.

Start with [`CLAUDE.md`](CLAUDE.md) (stack, layout, commands, hard rules) and [`docs/`](docs/).

## Quick start

Requires Rust (toolchain pinned in `rust-toolchain.toml`), Node 22, pnpm 10 and, optionally, Docker.

```
pnpm install
scripts/install-flatc.sh   # only needed to change schemas/
pnpm dev                   # http://localhost:3000 · api :4000 · relay :9001 · sim-runner
pnpm lint && pnpm typecheck && pnpm test
```
