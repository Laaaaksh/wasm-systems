# 04 — A richer WIT world

Every sample before this one crossed the guest/host boundary with strings
and `argv`. This one crosses it with the value shapes that actually make
the Canonical ABI worth learning: a `record`, a `variant` whose cases carry
different payloads (including a `list<record>`), a `tuple`, and a
`result<T, E>` used as a real error channel instead of a crash.

Companion to [`curriculum/04-wit-and-canonical-abi.md`](../../curriculum/04-wit-and-canonical-abi.md).

## Run it

```bash
make demo
```

## What to look for

Open [`wit/world.wit`](wit/world.wit) first, then [`src/lib.rs`](src/lib.rs)
- notice `cargo component` generated a plain Rust `enum Shape` and `struct
Point` from the WIT `variant`/`record` (check `src/bindings.rs` after
building), and the guest code you write is just ordinary Rust `match`
arms. That's the Canonical ABI's job: it defines exactly how a WIT
`variant`'s discriminant-plus-payload, or a `record`'s fields, get packed
into linear memory and pointers at the boundary, so every language's code
generator can produce its own idiomatic representation (a Rust `enum`, a
Python `dataclass`-like class, a Go tagged struct) on top of the same wire
format - see the Component Model spec's own [Canonical ABI
Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md)
for the actual lifting/lowering rules this sample's calls follow. (The
bytecodealliance.org docs site doesn't currently have a dedicated
Canonical ABI page of its own - the spec repo is the primary source for
this specific detail.)

Run `make demo` and read each `wasmtime run --invoke` line in the
Makefile against its output:

- `area(circle(2.0))` - a variant case wrapping a single `f64`.
- `area(rectangle((3.0, 4.0)))` - a variant case wrapping a `tuple<f64,
  f64>`.
- `area(circle(-1.0))` - the `result<f64, shape-error>` return type's
  `Err` path, carrying a `shape-error` variant with a `string` payload.
- `area(polygon([{x: 0.0, y: 0.0}, ...]))` - a variant case wrapping a
  `list<point>`, where `point` is itself a `record` - the deepest nesting
  in this sample, and the case that actually exercises `list` marshalling.

`make wit` prints the world back out via `wasm-tools component wit` - a
useful habit any time you want to confirm what a built component actually
exports, independent of what you meant to write.

## Things to try

- Add a `shape-error::empty-polygon` variant case with no payload
  (`empty-polygon,` with no parens) and a matching `Shape::Polygon(v) if
  v.is_empty()` arm - rebuild, and try `area(polygon([]))`.
- Change `point`'s fields from `f64` to `s32` and see exactly which parts
  of `src/bindings.rs` regenerate - the ABI-facing lift/lower code changes,
  the shape of your `match` arms doesn't.
- This sample never uses a WIT `resource` (a handle to guest- or
  host-owned state, the one Canonical ABI concept that isn't just data
  marshalling). [Stage 6](../../curriculum/06-host-embedding-and-capabilities.md)
  and [`code/06-host-embedding`](../06-host-embedding) introduce one, in a
  host program that can actually manage a resource's lifetime - the
  `wasmtime run --invoke` CLI used here isn't a natural fit for that.
