# Stage 1 — Why WASI exists

**You'll be able to:** explain, concretely, why a `.wasm` file can't print
"hello world" on its own, and what WASI actually is (a set of host-provided
function imports, not a runtime or a language feature).

**Time:** 1–2 hours.

**Build:** [`code/01-core-wasm-no-wasi`](../code/01-core-wasm-no-wasi) —
the same nine-line Rust program, compiled twice, once with no host
interface and once against WASI.

## Read, in this order

1. **[WebAssembly's own "Introduction"](https://webassembly.org/)** — the
   homepage, not a random tutorial. Read it for exactly one thing: Wasm is
   defined as a portable **binary instruction format for a stack-based
   virtual machine** — nothing in that definition mentions files, sockets,
   or a clock. Everything this stage is about follows from that one fact.
2. **[WASI's own overview, wasi.dev](https://wasi.dev/)** — current
   framing: WASI is "a modular collection of standardized APIs," designed
   around **capability-based security**, versioned in phases (0.1, 0.2,
   0.3...). Read the front page only; it's a hub, not a tutorial — full
   detail on why that matters for curation is in
   [`resources/curated-resources.md`](../resources/curated-resources.md).
3. **[Bytecode Alliance, "10 Years of Wasm: A Retrospective"](https://bytecodealliance.org/articles/ten-years-of-webassembly-a-retrospective)**
   (published ~January 2026) — the standards body's own account of how
   Wasm went from a browser-only sandbox (Firefox shipped it March 2017)
   to something that needed a WASI at all. You don't need the whole
   history, just the shape of the problem: browsers already give Wasm a
   host (the JS engine and DOM); running Wasm *outside* a browser means
   something has to play that role, and that something needs a standard.

## Do

Build and run [`code/01-core-wasm-no-wasi`](../code/01-core-wasm-no-wasi):

```bash
cd code/01-core-wasm-no-wasi
make run
```

Read its README fully — the "what you'll see" section is the point of
this whole stage, distilled into two `wasm-tools print` outputs you can
compare side by side. Do the "things to try" section, especially deleting
the `println!` and rebuilding — confirming that pure computation needs no
host interface either way is what proves the point isn't "WASI makes Wasm
work," it's "WASI is what supplies one *specific* capability (here,
`fd_write`) that this program asked for and the bare format doesn't have."

## The concept everyone gets wrong here

**"WASI is a runtime" or "WASI is what makes Wasm run outside the
browser."** Neither is quite right. WASI is a *specification* for a set of
host-provided interfaces (originally as raw `wasi_snapshot_preview1`
function imports, now as WIT-defined worlds under the Component Model).
`wasmtime`, `wasmer`, `WAMR`, and others are *runtimes* that each
independently implement WASI's spec. Nothing about Wasm itself changed to
get here — the same core Wasm binary format from 2017 is what both a
browser and `wasmtime` execute; what differs is which host interfaces
each environment chooses to expose to it, and under what names.

## Checkpoint

Answer these without looking anything up:

- Why did `01-core-wasm-no-wasi`'s `wasm32-unknown-unknown` build compile
  successfully, but print nothing when run?
- What specific host function does WASI provide that makes `println!`
  work, and what module is it imported from (`wasm-tools print`'s output
  names it exactly)?
- If a Wasm host wanted to run a `.wasm` file that only does arithmetic —
  no printing, no file access — would it need to implement any WASI
  interfaces at all?

Next: [Stage 2 — your first component](02-first-component.md).
