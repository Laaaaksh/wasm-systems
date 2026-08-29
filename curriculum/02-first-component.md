# Stage 2 — Your first component

**You'll be able to:** build, inspect, and run a real WebAssembly
component, and explain what a component actually is as a binary artifact
(not just "a fancier `.wasm` file").

**Time:** 1–2 hours.

**Build:** [`code/02-hello-component-rust`](../code/02-hello-component-rust).

## Read, in this order

1. **["Why the Component Model?"](https://component-model.bytecodealliance.org/design/why-component-model.html)**
   (Bytecode Alliance) — the motivating problem: core Wasm modules can
   only exchange numbers at their boundary, so two modules written in
   different languages (or even the same language, different versions of
   a toolchain) can't safely share a string or a struct without both
   sides hand-rolling the same memory-layout convention. Components fix
   this with a typed interface (WIT) and a defined process for
   translating richer values across the boundary (the Canonical ABI,
   covered in [Stage 4](04-wit-and-canonical-abi.md)).
2. **["Components" — The WebAssembly Component Model](https://component-model.bytecodealliance.org/design/components.html)**
   — what a component actually is, in the docs' own words: "a structure
   that may contain core modules and/or other components," with the
   interfaces of those contained pieces described in WIT. You don't need
   to memorize the binary layout — just come away knowing a component
   isn't a new instruction set, it's structured packaging around core
   modules.
3. **[`cargo-component`'s own README](https://github.com/bytecodealliance/cargo-component)**
   — skim it for the workflow (`cargo component new`, `cargo component
   build`), not the internals.

## Do

```bash
cd code/02-hello-component-rust
make run
```

Then do everything in that sample's README, in order — especially `make
wit`, which is the single most useful habit this stage can give you:
whenever you're unsure what a component actually declares, ask the tool,
don't guess from the source.

## What changed from Stage 1

Stage 1's module imported four bare functions named things like
`wasi_snapshot_preview1.fd_write` — a flat C-style ABI with no type
information beyond integers and pointers. This stage's component
describes its needs as a **world**: a named collection of typed imports
and exports, written in WIT, that a tool can validate, generate bindings
from, and check for compatibility. `cargo component new --bin` picked a
sensible default world (`wasi:cli/command`) for you; you haven't written
any WIT yourself yet — that's [Stage 4](04-wit-and-canonical-abi.md).

## Checkpoint

- What's the difference between the `.wasm` file `01-core-wasm-no-wasi`
  produced and the one `02-hello-component-rust` produced — structurally,
  not just "one works and one doesn't"?
- Why did `wasmtime run` need `--invoke` and print experimental warnings
  for Stage 1's module, but run Stage 2's component directly with no
  flags?
- `make wit` shows imports for `wasi:filesystem` and `wasi:clocks` that
  this program's code never uses. Where did those come from, and whose
  decision was it to include them?

Next: [Stage 3 — the same component, three languages](03-same-component-other-languages.md).
