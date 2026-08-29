# Stage 9 — Where this sits now

**You'll be able to:** name the current production options for running
components in real deployments, choose between them for a given
situation, and describe honestly what's still immature about this
ecosystem.

**Time:** 2–3 hours of reading; no new code. This stage is a decision
guide, not a tutorial — the real question at this point isn't "how do I
build one more thing," it's "which of several legitimate, competing
options fits my situation."

## The options, as of August 2026

| Project | What it is | Governance / maturity | Current version |
|---|---|---|---|
| [Wasmtime `serve`](https://docs.wasmtime.dev/cli-options.html) | Wasmtime's own built-in HTTP server, running a `wasi:http/proxy` component | Part of Wasmtime, the Bytecode Alliance's first ["Core Project"](https://bytecodealliance.org/articles/wasmtime-core-project) (2025-04-30) | Wasmtime 48.0.1 |
| [Spin](https://github.com/spinframework/spin) | A CLI and runtime for building and deploying serverless Wasm apps, triggered by HTTP/other events | CNCF Sandbox, accepted 2025-01-21 ([cncf.io/projects/spin](https://www.cncf.io/projects/spin/)) | v4.1.0 (2026-08-26) |
| [wasmCloud](https://github.com/wasmCloud/wasmCloud) | A distributed runtime: components (portable logic) linked to providers (host capabilities like messaging/KV) across a NATS-based mesh ("lattice") | CNCF **Incubating** — a materially higher maturity tier than Spin's, promoted 2024-11-08 after Sandbox acceptance in 2021 ([cncf.io/projects/wasmcloud](https://www.cncf.io/projects/wasmcloud/)) | v2.8.0 (2026-08-25) |
| [Extism](https://extism.org/) | A host-embeddable plugin library (not a standalone server) wrapping Wasmtime and other engines, for adding a plugin system to an existing app | Independent, commercially backed by [Dylibso](https://dylibso.com/products/extism/) — not CNCF/BA-governed | v1.30.0 (2026-06-04) |
| [jco](https://github.com/bytecodealliance/jco) | Transpiles a component to a JS module, for running components in Node or a browser | Bytecode Alliance | jco-v1.32.1 (2026-08-24) |

## A rough decision guide

- **Building an HTTP-triggered function or edge workload, and want the
  simplest path?** Spin. It's the most mature packaged answer for
  exactly this shape of workload, and — per Fermyon's own November 2025
  GA announcement — it underpins commercial deployment at real scale on
  Akamai's edge network after [Akamai's acquisition of
  Fermyon](https://www.akamai.com/newsroom/press-release/akamai-announces-acquisition-of-function-as-a-service-company-fermyon)
  closed on 2025-12-01. Treat any specific throughput figure from a
  vendor's own GA announcement as a vendor claim, not an independently
  audited benchmark.
- **Building a distributed system with multiple services that need to
  discover and call each other, potentially across machines?** wasmCloud
  — its component/provider split and lattice networking model are built
  for exactly this, and it currently sits at a higher CNCF maturity tier
  than Spin. It's also a heavier conceptual lift; don't reach for it for
  a single HTTP function.
- **Adding a plugin system to software you're already building, where
  "plugin" means "a `.wasm` file a user drops in," not "a whole deployed
  service"?** Extism, or the pattern this repository's own
  [Stage 8 capstone](../code/08-plugin-host-capstone) builds by hand — Extism
  packages that same idea as a reusable, multi-language host library
  instead of one you write yourself.
- **Testing or demoing a `wasi:http` component locally?** `wasmtime
  serve` — but its own docs are explicit that it's for local development
  only. It is not a substitute for Spin/SpinKube or wasmCloud in
  production; it has no rate limiting, request-size limits, DDoS
  protection, or TLS termination built in.
- **Need to run a component in a browser, or in a Node.js app, without a
  native runtime?** `jco` — the standard current path, though the
  Bytecode Alliance's own docs currently label its WASI Preview 2 support
  in JS environments "experimental."

## Honest gaps, right now

This isn't a finished ecosystem, and pretending otherwise would waste
your time discovering the gaps yourself.

- **The Component Model proposal itself is still Phase 1** ("Feature
  Proposal," per the [WebAssembly CG's own proposals
  tracker](https://github.com/WebAssembly/proposals), checked live while
  writing this) — despite years of active development, multiple shipped
  WASI preview releases, and everything this curriculum builds on top of
  it. Gerard Gallant's ["The State of WebAssembly – 2025 and
  2026"](https://platform.uno/blog/the-state-of-webassembly-2025-2026/)
  (2026-01-19) noted the proposal might advance to Phase 2 once WASI 0.3
  shipped — WASI 0.3 has since shipped (2026-06-11), so it's worth
  checking that tracker yourself for the current phase rather than
  trusting either Gallant's January prediction or this document's
  August-2026 snapshot. Don't assume "the Component Model" is a finished,
  ratified standard the way Wasm 3.0 itself is (ratified by the W3C in
  September 2025, per the Bytecode Alliance's own ["10 Years of Wasm: A
  Retrospective"](https://bytecodealliance.org/articles/ten-years-of-webassembly-a-retrospective),
  2026-01-22) — it's a fast-moving, still-stabilizing set of proposals,
  including the WASI 0.3 async work in [Stage 7](07-wasi-async.md).
- **Multithreading** is a named, current gap per Gallant's account above
  — enough that a wave of .NET/WebAssembly collaboration in early 2026
  was specifically motivated by developer demand for it.
- **Language support is broad but uneven in depth.** Almost all
  major languages have *some* Wasm story, but quality varies — Gallant
  notes a beta Kotlin/Wasm toolchain arrived only in September 2025.

Be skeptical of specific adoption statistics you see quoted elsewhere
("X% of developers use Wasm outside the browser," "X% of new projects
include a Wasm module") — a number of 2026 "state of WebAssembly"
listicles repeat suspiciously precise-sounding figures with no traceable
survey behind them. This curriculum found no verifiable primary source
for any of those specific numbers and deliberately doesn't repeat them.

## What this curriculum didn't cover, and where to go instead

- **wasmCloud and Spin in depth** — each is a large enough surface for
  its own curriculum. This stage tells you which one fits which
  situation; actually operating one in production is out of scope here.
- **Threading, SIMD, and Wasm GC** — real, current parts of the platform,
  outside a WASI/Component Model-focused path.
- **Non-Rust/Python/Go host embedding** — this repository's host samples
  ([Stage 6](06-host-embedding-and-capabilities.md), [Stage 8](08-plugin-host-capstone.md))
  are all Rust; `wasmtime` also ships embeddings for other languages
  (its own docs list them) with the same concepts but different APIs.

## Checkpoint

- You're building a webhook handler that needs to scale to zero between
  requests. Which option from the table above fits best, and why would
  wasmCloud be over-engineered for it?
- Why is a `wasmtime serve`-hosted component not "the same thing, just
  smaller" as a Spin-hosted one, even though both ultimately run on
  Wasmtime?
- Name one part of the Component Model ecosystem this curriculum built
  code against that is explicitly *not* a finished, ratified standard.

This is the last stage. From here: pick a real, small project — a CLI
tool, a plugin system for something you maintain, an HTTP handler — and
build it as a component, using whichever pieces from Stages 0–8 fit.
