# 05 — Composing two components

Two components: [`namer/`](namer) exports a function that returns a name;
[`greeter/`](greeter) **imports** that exact interface and exports
`greet()`, which calls it. Built separately, by separate `cargo component`
invocations, in separate crates that don't depend on each other at the Rust
level - only at the WIT level (`greeter/wit/world.wit` declares `import
wasm-systems:namer/names@0.1.0;`, matching `namer/wit/world.wit`'s
interface exactly).

Companion to [`curriculum/05-composition.md`](../../curriculum/05-composition.md).
Composed with [`wac`](https://github.com/bytecodealliance/wac) 0.10.1.

## Run it

```bash
make run
```

Expect `"Hello, Ferris!"` - `namer` supplies the name, `greeter` never
hardcodes it.

## What's actually happening

`greeter.wasm`, built alone, is not runnable - try it:

```bash
make run-greeter-alone
```

This fails on purpose: `wasmtime` reports `component imports instance
wasm-systems:namer/names@0.1.0, but a matching implementation was not
found in the linker`. A component with an unsatisfied import isn't
"broken," it's incomplete - the same way a Rust crate that depends on a
trait implementation it doesn't provide won't link until something
supplies one.

The `compose` target in the Makefile runs:

```bash
wac plug greeter/.../greeter.wasm --plug namer/.../namer.wasm -o composed.wasm
```

It takes greeter as the "socket" (the thing with a hole) and namer as the
"plug" (the thing that fills it), and produces a single component with
namer's code and greeter's code both linked inside it, statically, at the
Wasm binary level - not two processes talking over a channel.

Compare the two `wasm-tools component wit` outputs:

```bash
make wit-before   # greeter alone: imports wasm-systems:namer/names@0.1.0
make wit-after    # composed:      that import is gone
```

The `wasm-systems:namer/names` import disappears entirely after
composition - it's been resolved and inlined, not merely "connected."
Nothing at runtime does a lookup; the composed component simply no longer
has that import to satisfy.

## Things to try

- Change `namer`'s `get_name()` to return a different string and rerun
  `make run` - no change needed anywhere in `greeter/`.
- Build a second "plug" component with a different implementation of
  `wasm-systems:namer/names` and compose it against the same `greeter.wasm`
  instead - this is the actual point of composition: `greeter` was written
  against an interface, not an implementation.
- Try composing with a `namer` whose WIT interface has a different function
  signature (e.g. `get-name: func() -> u32`) - `wac plug` should refuse,
  since the plug's export no longer matches the socket's import type.
