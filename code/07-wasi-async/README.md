# 07 — WASI 0.3 async: how far the tooling actually gets

**Read this before running anything.** WASI 0.3 shipped 2026-06-11 - about
three months before this repository was written. Its headline feature is
native async: `stream<T>` and `future<T>` types that let a guest hand
values across the component boundary incrementally, instead of blocking
until a whole result is ready. This is new enough that, as of this
writing, there is no third-party tutorial for it - what follows is this
repository's own verification, done directly against current tooling, not
a curated summary of someone else's write-up.

**What works: this compiles and validates as a real component.**
**What doesn't: running it needs an async-aware host, which `wasmtime
run` currently is not.** Both are demonstrated below, honestly.

## Run it

```bash
make validate
```

This builds [`src/lib.rs`](src/lib.rs) - one exported function,
`count-to: func(n: u32) -> stream<u32>`, whose guest implementation spawns
a task that writes `0..n` into the stream and returns the read half
immediately - and confirms with `wasm-tools validate --features
component-model` that the output is a genuine, spec-valid Wasm component.
It is: this is real, current, working codegen for a real async WASI
export, using [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen)
0.46.0's `StreamReader`/`StreamWriter` API.

```bash
make try-run
```

**This is expected to fail.** It runs:

```
wasmtime run -W component-model-async=y --invoke 'count-to(3)' target/wasm32-wasip2/release/wasi_async_demo.wasm
```

and panics with `unsupported value type` from `wasmtime`'s own
`wasm-wave` crate (the library `wasmtime run --invoke` uses to print a
return value as text). Enabling every related `-W component-model-*`
flag `wasmtime` 48.0.1 exposes doesn't change this. The reason is
architectural, not a missing flag: printing a `stream<u32>` result on the
CLI would require the CLI itself to poll the stream, and `wasmtime run`'s
default execution path is the *synchronous* embedding API, which does not
implement the Component Model's async canonical built-ins
(`stream.new`, `stream.read`, etc.) at all - a separate attempt at making
the exported function *itself* a `run()` entry point that consumes its
own stream internally, sidestepping the CLI's value-printing path
entirely, failed for the same underlying reason: `wasmtime run`
instantiation itself rejects the module with `unknown import:
[export]$root::[stream-new-0]count-to has not been defined`. Consuming a
stream today requires a host built with `wasmtime`'s **async** embedding
API (`wasmtime_pkg::component::Linker` configured for async, an async
`Store`, and `wasmtime_wasi::p2::add_to_linker_async`) - genuinely more
than a CLI invocation, and beyond this sample's scope. If you get a
working async host together, a PR with it would be a real contribution -
see [`CONTRIBUTING.md`](../../CONTRIBUTING.md).

## Versions, pinned exactly, because this is where it matters most

- `wit-bindgen` (crate) `=0.46.0`, pinned in `Cargo.toml` - **not** a
  floating `"0.46"` or the newer `0.61.1` used by the `wit-bindgen-cli`
  tool elsewhere in this repo. While building this sample, the guest-side
  stream API changed shape between `wit-bindgen` 0.41.0 and 0.46.0
  (`StreamWriter` stopped implementing `futures::Sink` and gained a
  direct `.write()` method instead) - a real, observed breaking change
  within one release's worth of a 0.x version, exactly the churn you
  should expect working this close to a three-month-old spec.
- `wasmtime` 48.0.1, `wasm-tools` 1.258.0, `rustc` 1.98.0 (`wasm32-wasip2`
  target).
- `rustup target add wasm32-wasip3` **fails outright** on both stable and
  nightly Rust as of this writing (`"no prebuilt artifacts available for
  target 'wasm32-wasip3'... low-tier target"`) - this sample deliberately
  targets `wasm32-wasip2` instead, since `wit-bindgen`'s async support is
  implemented as a guest-side library on top of the existing Canonical
  ABI, not as a distinct Rust target triple.

## What to look for in `src/lib.rs`

- `count_to` is a plain (non-`async`) function that returns a
  `StreamReader<u32>` - not `impl Future<Output = StreamReader<u32>>`.
  The async work happens *after* the function returns, inside the spawned
  task; the function's job is only to set up the channel and hand back
  the reading end. Writing it as `async fn count_to(...) -> StreamReader<u32>`
  is a natural first instinct and a compile error - the generated `Guest`
  trait's actual signature is the giveaway (see the compiler's own
  suggestion if you try it).
- `wit_stream::new::<u32>()` is generated per-type, specifically because
  this WIT world has a function returning `stream<u32>` - it does not
  exist as a generic helper you can call for an arbitrary `T` without a
  WIT type using it somewhere in the world.

## Things to try

- Change `wit/world.wit`'s stream element type from `u32` to `string` and
  see what regenerates in `wit_stream`.
- Read the Component Model's [Concurrency
  Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Concurrency.md)
  (the actual design doc for `stream<T>`/`future<T>`) and the [WASI 0.3.0
  release](https://github.com/WebAssembly/WASI/releases/tag/v0.3.0)
  (shipped 2026-06-11) for the design this sample's WIT is a tiny instance
  of.
- If you want to go further than this repository does: `wasmtime`'s own
  async component-model tests (in its GitHub repo's test suite, not its
  CLI) are the most current working reference for a full async host as of
  this writing - reading real test code over a written tutorial is
  genuinely the right move here, because the tutorial doesn't exist yet.
