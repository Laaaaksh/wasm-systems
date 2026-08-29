# 02 — Your first component

The same idea as [`01-core-wasm-no-wasi`](../01-core-wasm-no-wasi), but built
as a **component** instead of a raw core module, using `cargo component`.

Companion to [`curriculum/02-first-component-rust.md`](../../curriculum/02-first-component-rust.md).
Built and run against `cargo-component` 0.21.1, `wasmtime` 48.0.1, `wasm-tools`
1.258.0, `rustc` 1.98.0.

## Run it

```bash
make run
```

You should see `Hello, WASI!` — `wasmtime run ... WASI` passes `WASI` as a
CLI argument, which the program reads with `std::env::args()`.

## What to look for

- **There's no WIT file in this directory.** `cargo component new --bin`
  targets a default `wasi:cli/command` world, so it wires up
  `wasi:cli/stdout`, `wasi:cli/environment`, `wasi:cli/args`, and a few
  others for you - run `make wit` to see the full imported world a plain
  `println!`-and-`args()` program actually needs. [Stage 4](../../curriculum/04-wit-and-canonical-abi.md)
  is where you write a WIT file by hand instead of accepting the default.
- **`make validate`** runs `wasm-tools validate --features component-model`
  against the output. A component is a different binary shape from the core
  module in sample 01 - it's core Wasm plus a typed, versioned interface
  description embedded in custom sections, per the [Component Model binary
  format](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md).
  This is why `wasmtime run` on this file works with no special flags,
  while `01`'s core module needed `--invoke` and produced two experimental
  warnings for anything beyond the plainest call.
- Compare the import list from `make wit` against `01`'s four raw
  `wasi_snapshot_preview1` imports. This component targets
  `wasm32-wasip1` under the hood (check `cat Cargo.toml` - no explicit
  target is pinned; `cargo component` defaults to it today) and is wrapped
  into a component by `cargo component`'s build step, which is what turns
  the low-level `wasi_snapshot_preview1.fd_write`-style imports from sample
  01 into the structured `wasi:cli/*` and `wasi:io/*` worlds you see here.

## Things to try

- Run `wasmtime run target/wasm32-wasip1/release/hello-component-rust.wasm`
  with no argument - it should print `Hello, world!` using the default.
- Run `make wit` and find the `wasi:filesystem` and `wasi:clocks` imports.
  This program never touches a file or a clock - they're pulled in because
  they're part of the default `wasi:cli/command` world, not because this
  program asked for them. [Stage 6](../../curriculum/06-host-embedding-and-capabilities.md)
  covers building a host that grants only the specific WASI interfaces a
  component actually needs, instead of the whole `wasi:cli` world.
- Try `cargo component build` (no `--release`) and compare binary size and
  build time against `--release`.
