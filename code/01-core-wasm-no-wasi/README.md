# 01 — Core Wasm has no OS interface

The same nine-line Rust program, compiled twice to two different targets.
Nothing in `src/main.rs` changes between the two builds — only the target
does. That's the entire demonstration.

Companion to [`curriculum/01-why-wasi.md`](../../curriculum/01-why-wasi.md).

## Run it

```bash
make run
```

## What you'll see

**Build 1: `--target wasm32-unknown-unknown`** — "core" WebAssembly, the
bare MVP spec with no OS bindings at all. `wasm-tools print` on the output
shows **zero imports**. Rust's standard library still compiles for this
target (it has a stub `std` for it), but `println!` has nowhere to go:
there is no file descriptor, no `write` syscall, nothing on the other side
of the sandbox boundary to hand bytes to. Running it with `wasmtime run`
exits `0` and prints nothing. Not an error — there is genuinely no
mechanism, in the pure spec, for a Wasm module to reach the outside world.

**Build 2: `--target wasm32-wasip1`** — the identical source, compiled
against WASI Preview 1. `wasm-tools print` now shows four imports from
the `wasi_snapshot_preview1` module: `environ_get`, `environ_sizes_get`,
`fd_write`, `proc_exit`. `fd_write` is what `println!` compiles down to.
Running it prints `sum = 5`.

**The takeaway:** WebAssembly the *format* is a sandboxed instruction set
with no ambient access to anything — no files, no network, no clock, no
environment variables, nothing. Every one of those capabilities is a
function a *host* chooses to provide, under some name, that the guest
module imports and calls by that name. WASI is a standard set of names and
signatures for those functions, so a Wasm binary you build once can run
against any host that implements the same WASI interfaces — instead of
every runtime inventing its own bespoke syscall names, which is exactly
what browser-only Wasm and various pre-WASI runtimes used to do.
[Stage 2](../../curriculum/02-first-component-rust.md) builds on this by
using WASI through the higher-level Component Model instead of raw
`wasi_snapshot_preview1` imports.

## Things to try

- Delete the `println!` line, rebuild, and diff the two `.wasm` files'
  import sections again — with no I/O at all, does the no-WASI build still
  differ from the WASI build? (It shouldn't: pure computation needs no
  host interface either way.)
- Add `std::env::var("HOME")` and print it. Rebuild both targets. The
  `wasm32-unknown-unknown` build will now fail to *run* correctly (or
  return an error from the call) rather than fail to compile — reading
  the environment is exactly the kind of ambient authority raw Wasm
  doesn't have and WASI has to explicitly grant.
- Run `wasmtime run --invoke main target/wasm32-unknown-unknown/release/hello.wasm`
  directly and compare the experimental-invoke warnings wasmtime prints
  for a bare core module against how cleanly [`02-hello-component-rust`](../02-hello-component-rust)
  runs — that gap is part of why the Component Model exists.
