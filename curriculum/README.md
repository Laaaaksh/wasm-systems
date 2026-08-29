# The curriculum

Ten stages, in order. Each one names what you'll be able to do at the end,
what to read or watch, roughly how long it takes, and what to build to
prove it stuck. Do them in order the first time through — later stages
assume earlier ones.

| Stage | You'll be able to... | Time | Build |
|---|---|---|---|
| [0 — Toolchain setup](00-toolchain-setup.md) | Compile a component and run it under `wasmtime` | 30–60 min | Every checkpoint command prints a version |
| [1 — Why WASI exists](01-why-wasi.md) | Explain why raw `.wasm` has no OS interface | 1–2 hr | [`code/01-core-wasm-no-wasi`](../code/01-core-wasm-no-wasi) |
| [2 — Your first component](02-first-component.md) | Build, inspect, and run a real component | 1–2 hr | [`code/02-hello-component-rust`](../code/02-hello-component-rust) |
| [3 — Three languages, one contract](03-same-component-other-languages.md) | Target the same WIT world from Rust, Python, and Go | 2–4 hr | [`code/03-greet-in-three-languages`](../code/03-greet-in-three-languages) |
| [4 — WIT and the Canonical ABI](04-wit-and-canonical-abi.md) | Write a WIT world with records, variants, and results | 2–4 hr | [`code/04-richer-wit-world`](../code/04-richer-wit-world) |
| [5 — Composition](05-composition.md) | Link two components' imports/exports into one | 1–2 hr | [`code/05-composition`](../code/05-composition) |
| [6 — Host embedding & capabilities](06-host-embedding-and-capabilities.md) | Write a host that decides what a guest can touch | 2–4 hr | [`code/06-host-embedding`](../code/06-host-embedding) |
| [7 — WASI 0.3 async](07-wasi-async.md) | Read/write `stream<T>`/`future<T>` WIT and know today's real limits | 1–2 hr | [`code/07-wasi-async`](../code/07-wasi-async) |
| [8 — Capstone: untrusted plugins](08-plugin-host-capstone.md) | Build a host for plugins it has never seen | 3–5 hr | [`code/08-plugin-host-capstone`](../code/08-plugin-host-capstone) |
| [9 — Where this sits now](09-production-and-ecosystem.md) | Choose between Spin/wasmCloud/Extism/a custom host | 2–3 hr | — (decision guide, no new code) |

**Total: roughly 20–28 hours** of focused work, spread over however long
that takes you. There's no clock running.

## Prerequisites

You should be comfortable writing and debugging code in at least one of
Rust, Python, or Go, and comfortable with the command line. You do **not**
need prior WebAssembly, WASI, or distributed-systems experience — Stage 1
starts from "what is a `.wasm` file" and builds up from there. Rust is the
primary language for the host-embedding stages (6 and 8) specifically
because `wasmtime`'s Rust embedding API is the most complete and
best-documented of its language bindings today; you don't need Rust
fluency going in, just willingness to read code in it.

## How each stage is structured

- **Read, in this order** — a short, sequenced list, not an unordered
  pile, with a reason given for each entry and why it comes where it
  does. Full annotated detail on every external resource this curriculum
  cites — what it covers, how current it is, what's aged but still worth
  it — lives in [`resources/curated-resources.md`](../resources/curated-resources.md).
- **Do** — a runnable sample in [`code/`](../code), with its own README
  explaining what to look for and what to try breaking on purpose.
  Building it is not optional: several of this curriculum's central
  claims (that a denied capability looks like a missing file, that
  composition removes an import rather than merely satisfying it) are
  things you're meant to observe happening, not just read about.
- **The concept everyone gets wrong here** — every stage names a specific
  misconception directly, rather than leaving you to infer the correct
  model from correct examples alone.
- **Checkpoint questions** — answer these without looking anything up
  before moving on. They're a way to notice what didn't actually land,
  not a quiz to pass.

## If something stops you

[`resources/common-pitfalls.md`](../resources/common-pitfalls.md) collects
the specific things that stop most people starting out — toolchain version
mismatches, the WIT-resolution errors that look like your fault but
usually aren't, and the misconceptions almost everyone starts with. Check
there before assuming you've found a new problem.

## Re-verification cadence

This curriculum is pinned to specific, dated tool versions throughout
(see [Stage 0](00-toolchain-setup.md)'s table) because the WIT tooling
stack — `wit-bindgen`, `cargo-component`, `wac`, `wkg` — moves in lockstep
with a WIT specification that is itself still moving; [Stage 7](07-wasi-async.md)
documents a real breaking API change hit while writing this repository's
own sample, within one release of a single 0.x tool. The intent is to
re-verify every sample against each new `wasmtime` minor release and each
new WASI point release (0.3.x, then 0.4), and to update
[`resources/curated-resources.md`](../resources/curated-resources.md)'s
dates and verdicts at the same time — not to write this once and leave it
to rot the way the "aged" resources catalogued in that file did.
