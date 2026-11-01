<div align="center">

# wasm-systems

**A sequenced path from "I've heard of WebAssembly" to "I can build a
capability-secure host and reason about the Canonical ABI" — with
runnable, hands-on-verified code at every step.**

[![CI](https://github.com/Laaaaksh/wasm-systems/actions/workflows/ci.yml/badge.svg)](https://github.com/Laaaaksh/wasm-systems/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-purple.svg)](LICENSE)
[![Wasmtime](https://img.shields.io/badge/wasmtime-48.0.1-654FF0)](code/README.md)

**[Curriculum](curriculum/README.md) • [Code samples](code/README.md) • [Resources](resources/curated-resources.md) • [Common pitfalls](resources/common-pitfalls.md) • [Contributing](CONTRIBUTING.md) • [License](LICENSE)**

</div>

## What this is

WebAssembly outside the browser has no shortage of material — the specs
themselves, a fast-moving official docs site, blog posts, conference
talks. What it lacks is a **path**: something that sequences WASI and the
Component Model from first principles through to a real, capability-secure
host, and says honestly which of the many existing explanations have aged
and which are still the best available. An unordered list of 200 links is
the problem this repository exists to not be — and, checked directly
against the two most popular such lists while building this
([mbasso/awesome-wasm](https://github.com/mbasso/awesome-wasm),
[wasmerio/awesome-wasi](https://github.com/wasmerio/awesome-wasi)), that's
exactly what they still are: real, current, useful inventories with zero
sequencing.

This repo is ten sequenced stages, each with:

- **What you'll be able to do** at the end of it, stated concretely.
- **What to read, in order** — a short, reasoned list, not a pile, with
  the full case for each entry in [`resources/curated-resources.md`](resources/curated-resources.md).
- **A small, complete, hands-on-verified program to build and run**, in
  [`code/`](code) — starting from why a raw `.wasm` file can't even print
  "hello," through building the same component in Rust, Python, and Go,
  writing WIT with real Canonical-ABI value types, composing two
  components with `wac`, embedding `wasmtime` in a Rust host that decides
  exactly what a guest can touch, a from-scratch attempt at WASI 0.3's
  brand-new async support (with an honest account of exactly where
  today's tooling stops), and a capstone plugin host that loads
  components it's never seen.
- **The specific misconception people bring to that stage**, named
  directly, not left for you to infer.
- **Checkpoint questions** to answer before moving on.

Start at [`curriculum/README.md`](curriculum/README.md).

## Why this is worth trusting

**Every sample was actually built and run in this environment, not
compile-checked or asserted.** This repository has no GPU-style access
gap to disclose — WebAssembly and its whole toolchain run on an ordinary
CPU — so unlike a curriculum for hardware-gated material, every `make
run`/`make demo` in [`code/`](code) is a real, reproduced result, pinned
to the exact tool versions in [Stage 0](curriculum/00-toolchain-setup.md)'s
table. The one deliberate exception is [Stage 7](curriculum/07-wasi-async.md)
(WASI 0.3 async), where this repo says plainly, with the exact
reproducible error text, exactly how far current tooling gets — because
WASI 0.3 shipped 2026-06-11, about three months before this was written,
and no third-party material for it exists yet to check against.

**Every external link was actually fetched and confirmed live** — via
`gh api`, `WebFetch`, or a real install command — while writing this, not
recalled from a model's memory. Dates and version numbers throughout are
what those checks returned, not approximations.

## What this doesn't cover

This curriculum takes you from zero to a working capability-secure plugin
host, using WASI 0.2/0.3 and the Component Model. It does **not** cover:
building a production Spin or wasmCloud deployment end to end (Stage 9
tells you which one fits your situation and points you at each project's
own docs); Wasm threading, SIMD, or garbage-collection proposals; running
Wasm in the browser or via `jco`/JavaScript in depth; or non-Rust host
embedding (this repo's host samples are all in Rust, since `wasmtime`'s
Rust embedding API is currently the most complete of its language
bindings — the concepts transfer, the exact API calls won't).

## Repository layout

```
curriculum/   10 sequenced stages - the path itself
code/         8 runnable, hands-on-verified components and hosts
resources/    honest, dated curation of everything external cited above
```

## Contributing

Contributions are welcome — a wrong claim, a stale "current" statement, a
dead link, or a working async host for Stage 7 that gets further than
this repo currently does. See [CONTRIBUTING.md](CONTRIBUTING.md). Please
read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) first.

## Security

Found a security issue? See [SECURITY.md](SECURITY.md) — please don't
open a public issue for it.

## License

MIT — see [LICENSE](LICENSE).
