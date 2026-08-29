# Stage 5 — Composing components

**You'll be able to:** compose two independently-built components into
one, and explain the difference between a component that imports an
interface and one that has that import satisfied.

**Time:** 1–2 hours.

**Build:** [`code/05-composition`](../code/05-composition).

## Read, in this order

1. **["Composing Components" — The WebAssembly Component Model](https://component-model.bytecodealliance.org/composing-and-distributing/composing.html)**
   — the concept: linking two components together statically, resolving
   one's imports with another's exports, producing a single new
   component. This is a build-time operation on `.wasm` binaries, not a
   runtime RPC mechanism.
2. **[`wac`'s own README](https://github.com/bytecodealliance/wac)**
   (Bytecode Alliance) — the tool this stage uses. Its README documents
   both `wac plug` (the simpler two-component case this stage's sample
   uses) and the full WAC composition language (for graphs of more than
   two components) — skim the language section for awareness, you won't
   need it for this stage's sample.

## Do

```bash
cd code/05-composition
make run
```

Do the whole sample README, especially `make run-greeter-alone` (which is
*meant* to fail) and the `wit-before`/`wit-after` comparison — seeing an
import disappear after composition, rather than just being told it
happens, is the point of this stage.

## The concept everyone gets wrong here

**"Composition links two components at runtime, the way `dlopen` links a
shared library."** It doesn't — `wac plug` runs once, ahead of time,
producing a new `.wasm` file with both components' code already inside
it. There is no runtime resolution step, no version negotiation at
startup, no way for the composed component to later swap out which
`namer` implementation it's using. If you need to swap implementations at
runtime instead of build time, that's a different problem — closer to
what [Stage 8](08-plugin-host-capstone.md)'s plugin host does, loading
separate components dynamically rather than composing them into one.

## Checkpoint

- What error does `wasmtime` report when you try to run `greeter.wasm`
  alone, and what does that error tell you about how WASI/Component
  Model imports are resolved at instantiation time?
- After composing, `wasm-tools component wit composed.wasm` no longer
  lists `wasm-systems:namer/names` as an import. Where did that
  requirement actually go — is it satisfied by a runtime lookup, or gone
  entirely?
- Could you compose three components instead of two, where `greeter`
  imports from `namer` and `namer` itself imports from a third
  component? What would you expect `wac plug`'s syntax to need to change
  to (hint: check its README for the multi-component composition
  language, `wac compose`, versus the two-component `wac plug` this
  stage's sample uses)?

Next: [Stage 6 — a host that decides what a guest can touch](06-host-embedding-and-capabilities.md).
