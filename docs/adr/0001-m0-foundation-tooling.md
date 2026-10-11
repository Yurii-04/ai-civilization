# ADR 0001 — M0 foundation tooling

- Status: accepted (M0)
- Date: 2026-10-11

## Context

M0 sets up the monorepo, codegen, local infra and CI. A few tooling choices
were not fixed by `CLAUDE.md` and are recorded here so later milestones do not
re-litigate them.

## Decisions

1. **WASM determinism runs on `wasm32-wasip1` under wasmtime.**
   The browser build targets `wasm32-unknown-unknown`, but both targets share
   the same IEEE-754 semantics for `f32`/`f64` arithmetic and `libm`. Running
   the *same* `cargo test -p sim-core --test determinism` binary under
   wasmtime gives a real wasm32 execution with no JS harness. The test pins a
   golden world hash (`GOLDEN` in `crates/sim-core/tests/determinism.rs`), so
   every target (x86_64, aarch64, wasm32) is checked independently. A
   separate CI job builds `sim-wasm` with `wasm-pack` for the browser target.
   The hash covers the raw `f32` bits of the state (stricter than quantized
   values).

2. **Pinned toolchain versions.** Rust `1.97.0` (`rust-toolchain.toml`),
   Node 22 (`.nvmrc`), pnpm 10 (`packageManager`), flatc and the `flatbuffers`
   runtime both at `25.9.23` (`scripts/install-flatc.sh`, `Cargo.toml`,
   `packages/protocol`). TypeScript is pinned to `~6.0` because
   `typescript-eslint` does not support TypeScript 7 yet.

3. **Generated FlatBuffers code is committed and not reformatted.**
   The Rust module is included with `#[rustfmt::skip]` and lint allows, the TS
   output is ignored by ESLint/Prettier. CI regenerates it with the pinned
   flatc and fails on any diff (`pnpm codegen:check`).

4. **uWebSockets.js comes from GitHub, loaded via `createRequire`.**
   It is not published on npm; the dependency is
   `github:uNetworking/uWebSockets.js#v20.71.0`. Its ESM wrapper in that
   release imports a missing `./index.js`, so the relay loads the CommonJS
   entry with `createRequire` and takes types from the package.

5. **NestJS dev server uses `@nestjs/cli` (`nest start --watch`).** Nest relies
   on `emitDecoratorMetadata`, which esbuild-based runners (tsx) do not emit.

6. **Prisma is deferred** until the first models (M3 seasons/snapshots or M6
   wallet). `pnpm db:migrate` is added with it.

## Consequences

- Changing a sim rule that affects the placeholder world requires updating
  `GOLDEN` in the same PR, with a note explaining why.
- Upgrading flatc means bumping it in three places and regenerating.
