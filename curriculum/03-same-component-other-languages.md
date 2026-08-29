# Stage 3 — The same component, three languages

**You'll be able to:** build a component that targets a specific WIT world
in a second and third language, and explain what's actually shared between
them (the WIT contract) versus what isn't (everything else).

**Time:** 2–4 hours — the Go path in particular has real toolchain friction
worth budgeting time for.

**Build:** [`code/03-greet-in-three-languages`](../code/03-greet-in-three-languages) —
Rust, Python, and Go implementations of the identical `wasi:cli/command`
world, run identically.

## Read, in this order

1. **["Creating Components" — The WebAssembly Component Model](https://component-model.bytecodealliance.org/language-support.html)**
   (or the site's per-language pages linked from it) — current language
   support: Rust, Go, Python, C/C++, C#, JavaScript, MoonBit, and hand-written
   WAT are all listed. Skim the Python and Go pages specifically before
   doing this stage's Python and Go work.
2. **[`componentize-py`'s own README](https://github.com/bytecodealliance/componentize-py)**
   — the Python toolchain this stage uses. Note its own stated scope: it
   targets CPython semantics compiled to Wasm, not a from-scratch Python
   interpreter rewrite.
3. **[`go.bytecodealliance.org` (the `go-modules` repo)](https://github.com/bytecodealliance/go-modules)**
   — `wit-bindgen-go`, the tool this stage's Go path uses to generate
   bindings before compiling with TinyGo.

## Do

```bash
cd code/03-greet-in-three-languages
make run
```

Then work through that sample's README in full — the "per-language notes"
section is where the real content of this stage lives, especially the Go
note about `wasi:cli/command` being required even for what should be a
pure library export.

## The concept everyone gets wrong here

**"These three components can call each other directly, like a shared
library."** They can't, not without an explicit host or composition step
([Stage 5](05-composition.md)) — each is a fully separate, sandboxed
`.wasm` binary. What's shared is the WIT world description, which is a
*contract*, not a *link*. Rust's compiler enforces that contract at
compile time (a signature mismatch is a `cargo component build` error);
Python's doesn't (a `Run` class with a wrong method name just fails
however `componentize-py` happens to fail); this asymmetry is real and
worth noticing, not a bug in this repository's samples.

## A real, current toolchain gap, seen firsthand

TinyGo 0.41.1's Wasm component support currently assumes a full
`wasi:cli/command` world is present even for what should be a
library-only ("reactor") component — attempting a pure-export world
without it fails `wasm-tools component new` with a missing
`wasi:cli/environment` import, reproduced in this stage's sample. This
kind of gap is exactly the "things that actually stop people" this
curriculum tries to name explicitly rather than let you discover
alone — see [`resources/common-pitfalls.md`](../resources/common-pitfalls.md).

## Checkpoint

- What does `wit/world.wit` actually guarantee is true about all three
  implementations, and what does it say nothing about?
- Why did the Go implementation need `include wasi:cli/command@0.2.3`
  when the Rust and Python versions could (in principle) have targeted a
  narrower, export-only world?
- If you handed `python/app.py` to someone who had never seen this
  repository, what would they need to know to understand why the class is
  named `Run` and not, say, `Greeter`?

Next: [Stage 4 — WIT and the Canonical ABI](04-wit-and-canonical-abi.md).
