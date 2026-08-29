# Stage 6 — A host that decides what a guest can touch

**You'll be able to:** write a native program that embeds `wasmtime`
directly, and explain capability-based security in terms of a concrete
API decision rather than a slogan.

**Time:** 2–4 hours.

**Build:** [`code/06-host-embedding`](../code/06-host-embedding).

## Read, in this order

1. **[wasi.dev's overview, again](https://wasi.dev/)** — this time for
   its explicit framing of WASI as **capability-based**: a component gets
   access to a resource (a directory, a socket) only if the host hands it
   an explicit, unforgeable reference to it — there is no ambient
   filesystem root or environment a component can reach out and grab on
   its own. Everything else in this stage is that one sentence, worked
   through in real code.
2. **[`wasmtime-wasi`'s own crate documentation](https://docs.rs/wasmtime-wasi/latest/wasmtime_wasi/)**
   (docs.rs) — specifically `WasiCtxBuilder`, `preopened_dir`, and the
   `FsPerms` type this stage's sample uses directly. You've been using
   `wasmtime` as a CLI everywhere else in this repo; this is the same
   engine as a Rust library.
3. **[Wasmtime's own "Security" page](https://docs.wasmtime.dev/security.html)**
   — the project's own model for what it does and doesn't guarantee, and
   how that relates to the sandboxing this stage demonstrates.

## Do

```bash
cd code/06-host-embedding
make run
```

Read `runner/src/main.rs` side by side with the sample's README — the
entire capability decision is in about four lines (`WasiCtx::builder()`,
`.inherit_stdout()`, the conditional `.preopened_dir(...)`), which is
itself worth noticing: this is not a large or complex API surface.

## The concept everyone gets wrong here

**"A denied capability produces a 'permission denied' error."** From
inside the sandbox, it doesn't — `01-core-wasm-no-wasi` demonstrated total
silence (no host interface at all), and this stage demonstrates `ENOENT`
("no such file or directory"), the *same* error a real missing file would
produce. This is intentional, not an accident of this repository's
samples: a capability-secure design that let a component distinguish
"this doesn't exist" from "you're not allowed to see this" would leak
information about the host's filesystem to code that was explicitly not
given access to it. Compare this to how a Unix process with an empty
`chroot` jail experiences a missing file, versus a Unix process denied by
file permissions (`EACCES`) — WASI's default posture is closer to the
former: the file might as well not exist, because as far as this
component's world is concerned, it doesn't.

## Checkpoint

- `runner/src/main.rs`'s `WasiCtx::builder()` call, with nothing added,
  denies filesystem, network, clock, and random access, in addition to
  stdio. Why does it explicitly call `.inherit_stdout()` at all, rather
  than defaulting to some access being granted?
- The guest component (`guest/src/main.rs`) is identical between both
  runs in `make run`'s output. What's the only thing that differs, and in
  which file does that difference live?
- If you wanted to grant `guest.wasm` write access instead of read-only,
  what's the one-line, one-argument change, and why is it enforced by the
  runtime rather than something the guest could bypass by writing more
  determined code?

Next: [Stage 7 — WASI 0.3 async](07-wasi-async.md).
