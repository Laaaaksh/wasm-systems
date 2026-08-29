# Stage 7 — WASI 0.3 async

**You'll be able to:** read and write WIT using `stream<T>`/`future<T>`,
and give an accurate, current answer to "can I actually run this yet?"

**Time:** 1–2 hours — this stage is shorter than the others because there
is less to *do*; the honest limitation is most of the content.

**Build:** [`code/07-wasi-async`](../code/07-wasi-async) — a real,
validated component with a genuine async export, plus a documented,
reproducible current failure trying to run it.

## Why this stage exists, and why it's different

WASI 0.3.0 shipped 2026-06-11 — [confirmed directly from the release
itself](https://github.com/WebAssembly/WASI/releases/tag/v0.3.0) — about
three months before this repository was written. Its headline feature is
native async: a guest can return a `stream<T>` or `future<T>` instead of
blocking the whole component while a value is produced. Every other stage
in this curriculum curates and sequences material that already exists.
This one mostly can't, yet — there is no accessible third-party tutorial
for building or running a WASI 0.3 async component as of this writing.
What follows is this repository's own direct verification against
current tooling, done specifically so this stage doesn't have to (and
doesn't) pretend otherwise.

## Read, in this order

1. **[The Component Model's Concurrency Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Concurrency.md)**
   — the actual design document introducing `stream<T>` and `future<T>`
   as "unidirectional unbuffered channels" for passing 0..N or exactly 1
   value across a component boundary without blocking. This is a spec
   document, not a tutorial — read it for the concept and the two type
   names, not to memorize every detail.
2. **[The WASI 0.3.0 release itself](https://github.com/WebAssembly/WASI/releases/tag/v0.3.0)**
   — read the release notes for what actually shipped and when.

## Do

```bash
cd code/07-wasi-async
make validate    # this succeeds - a real, valid stream<u32>-exporting component
make try-run     # this is *expected* to fail - read why in the sample's README
```

Read the sample's README in full, especially the versions-pinned-exactly
section — it documents a real breaking API change this repository's own
author hit going from `wit-bindgen` 0.41.0 to 0.46.0 while building this
one sample, which is itself evidence of how unsettled this specific
corner of the ecosystem still is.

## The concept everyone gets wrong here

**"If `wasm-tools validate` says a component is valid, it will run."**
Validity and runnability are different claims. A component can be a
fully spec-conformant binary — this stage's sample is — and still fail to
*instantiate* under a specific runtime's specific embedding mode, because
that runtime hasn't implemented every canonical built-in a valid
component is allowed to use. `wasmtime run`'s synchronous CLI path is
exactly this case for the async built-ins as of `wasmtime` 48.0.1.
"Valid" means "conforms to the spec"; it does not mean "every runtime you
might hand it to has caught up to that spec."

## Where this leaves you

If you need working async behavior across a component boundary *today*,
the honest options are: (1) don't use `stream`/`future` yet, and instead
have a guest export a plain function repeatedly, called by a host loop
(polling, not true async) — least elegant, but works on every tool in
this curriculum right now; (2) write a full custom host using
`wasmtime`'s async embedding API (`add_to_linker_async`, an async
`Store`) — genuinely more work than this repository's other host samples,
and the reason this stage doesn't build one; (3) wait — WASI 0.3 is a
preview release, and per the ecosystem-direction discussion in
[Stage 9](09-production-and-ecosystem.md), tooling catching up to a
three-month-old spec release is a normal, expected lag, not a sign
something is broken.

## Checkpoint

- What does `wasm-tools validate` actually check, and why doesn't passing
  it guarantee `wasmtime run` can execute the component?
- In `code/07-wasi-async/src/lib.rs`, why is `count_to` a plain `fn`
  rather than an `async fn`, even though it deals in async values?
- Name the exact error `wasmtime run --invoke 'count-to(3)'` produces
  against this stage's sample, and which of the two failure sites this
  repository found (CLI value-printing vs. component instantiation) it
  corresponds to.

Next: [Stage 8 — capstone: a host for untrusted plugins](08-plugin-host-capstone.md).
