# Stage 4 — WIT and the Canonical ABI

**You'll be able to:** write a WIT world by hand with records, variants,
and results, and explain — at a conceptual level — how a value like a
`list<record>` actually crosses the guest/host boundary.

**Time:** 2–4 hours.

**Build:** [`code/04-richer-wit-world`](../code/04-richer-wit-world).

## Read, in this order

1. **["Interfaces"](https://component-model.bytecodealliance.org/design/interfaces.html)**
   and **["WIT Worlds"](https://component-model.bytecodealliance.org/design/worlds.html)**
   (The WebAssembly Component Model docs) — WIT's two organizing
   concepts: an `interface` groups related functions and types; a `world`
   is what a specific component actually imports and exports, built by
   combining interfaces. Every WIT file you've seen so far in this repo
   (`export greet: func...`) is a minimal, single-function world — these
   pages show the fuller shape.
2. **["An Overview of WIT"](https://component-model.bytecodealliance.org/design/wit.html)**
   — you don't need to read this cover to cover now; use it as the
   reference while you read [`wit/world.wit`](../code/04-richer-wit-world/wit/world.wit)
   in this stage's sample. Confirm you can find `record`, `variant`,
   `tuple`, `list`, and `result` in it before moving on.
3. **[Canonical ABI Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md)**
   (the Component Model spec repository) — the actual, authoritative
   definition of how WIT values are lifted and lowered across the
   boundary. This is a specification document, written in pseudocode, not
   a tutorial — read the introduction and the sections on records, lists,
   and variants for the shape of the idea; you do not need to trace every
   line of pseudocode to get value from this stage. **Note:** the
   `component-model.bytecodealliance.org` docs site does not currently
   have its own dedicated Canonical ABI page — this is a real, current
   gap in that documentation, not a broken link in this curriculum. Going
   to the spec directly is the correct move here, not a workaround.

## Do

```bash
cd code/04-richer-wit-world
make demo
```

Read the sample's README fully, then do its "things to try" — especially
adding a new `variant` case and watching what regenerates versus what you
have to write by hand.

## The concept everyone gets wrong here

**"The Canonical ABI is a serialization format, like JSON or Protobuf."**
It's closer to a *calling convention* than a wire format: values are
lifted and lowered through **linear memory** (the same flat byte array a
core Wasm module already has), using rules for exactly which bytes mean
what for each WIT type — there's no length-prefixed self-describing
envelope the way JSON or Protobuf produce. A `list<point>` crosses the
boundary as a pointer-and-length pair into memory the two sides have
agreed how to interpret, not as a blob you could hand to an unrelated
parser. This is *why* WIT bindings generators exist at all: getting this
right by hand, per language, per type, is exactly the tedious and
error-prone work `wit-bindgen`/`cargo-component`/`componentize-py`
automate.

## Checkpoint

- What Rust type did `cargo component` generate for the WIT `variant
  shape { circle(f64), rectangle(tuple<f64, f64>), polygon(list<point>) }`,
  and why is that mapping a natural one for Rust specifically (would it be
  equally natural in a language with no tagged-union type)?
- `result<f64, shape-error>` shows up as an `Err` case, not a panic or
  crash, when you pass a negative radius. Where in `src/lib.rs` is that
  choice actually made — in the WIT, or in the guest code?
- Name one WIT type this stage's sample doesn't use (a `resource` is the
  obvious answer) and say, roughly, why it needs different handling than
  everything this sample does cover.

Next: [Stage 5 — composing components](05-composition.md).
