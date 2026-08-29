# Curated resources

Every entry here was actually opened and checked — via `gh api`, `WebFetch`,
or a real `cargo install`/`brew install`/`pip install` — at the time of
writing (August 2026), not recalled from memory. Dates are the resource's
own stated publish/update date, or the exact timestamp this repository
fetched it at, where no publish date exists. Where something has aged,
that's said plainly, along with what to read instead or alongside it.
Curriculum stages link the specific entries relevant to them; this page is
the full, browsable list with the reasoning behind each recommendation.

If a link here breaks, or you find something better, please open an issue
using the "Resource suggestion" template — see
[`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Official specs and docs

| Resource | What it covers | Current as of | Verdict |
|---|---|---|---|
| [webassembly.org](https://webassembly.org/) | The core Wasm spec's own framing | Live | Read once, for [Stage 1](../curriculum/01-why-wasi.md) — it's a hub, not a tutorial. |
| [wasi.dev](https://wasi.dev/) | WASI's own overview: modular APIs, phased versioning, capability-based security | Live | Current, and the right one-paragraph definition of WASI to start from — but it's a navigation hub, confirmed by its own structure, not a sequenced guide. |
| [The WebAssembly Component Model docs](https://component-model.bytecodealliance.org/) | Structured docs: Understanding Components, Building Components (per-language), Composing/Running/Distributing | Live; references WASI 0.2.0 (2024-01-25) as a baseline in places | **The strongest official attempt at sequencing that exists** — genuinely organized, and this curriculum's stage-by-stage reading lists lean on its per-page structure directly (`design/why-component-model.html`, `design/components.html`, `design/interfaces.html`, `design/worlds.html`, `design/wit.html`, `composing-and-distributing/composing.html`, `language-support.html`, all confirmed live while writing this). **Its real, current gaps, confirmed via the project's own open issues** ([bytecodealliance/component-docs](https://github.com/bytecodealliance/component-docs), 135★): a tracking issue for WASI P3 documentation updates ([#341](https://github.com/bytecodealliance/component-docs/issues/341), opened 2026-04-22, still open); a broken Python tutorial ([#348](https://github.com/bytecodealliance/component-docs/issues/348), opened 2026-06-12, still open); a maintainer's own plea for documentation help ([#321](https://github.com/bytecodealliance/component-docs/issues/321), opened 2025-10-08, still open ten months later); an acknowledged terminology gap around host-embedding examples ([#342](https://github.com/bytecodealliance/component-docs/issues/342), opened 2026-04-22, still open); and an issue tracking the WIT reference falling behind a spec change ([#362](https://github.com/bytecodealliance/component-docs/issues/362), opened 2026-07-15, still open). It also currently has **no dedicated Canonical ABI page** — this curriculum uses the Component Model spec repository directly for that (next row). |
| [Component Model spec — Canonical ABI Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md) | The authoritative lifting/lowering rules for every WIT value type | Live spec repo | Current and correct, but a specification document with Python pseudocode, not a tutorial — [Stage 4](../curriculum/04-wit-and-canonical-abi.md) tells you what to read for concept vs. detail. |
| [Component Model spec — Binary Format Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md) | The actual binary encoding of a component vs. a core module | Live spec repo | Reference-depth; useful once, not something to read start to end. |
| [Component Model spec — Concurrency Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Concurrency.md) | `stream<T>`/`future<T>` design | Live spec repo | Current — and, as of this writing, essentially the *only* real explanation of this feature, since it's too new (WASI 0.3 shipped 2026-06-11) for third-party material to exist yet. See [Stage 7](../curriculum/07-wasi-async.md). |
| [WASI 0.3.0 release](https://github.com/WebAssembly/WASI/releases/tag/v0.3.0) | The actual shipped release | 2026-06-11 | Primary source for the version this curriculum's async chapter is pinned to. |
| [WebAssembly proposals tracker](https://github.com/WebAssembly/proposals) | Phase status of every active Wasm proposal, including the Component Model itself | Checked live while writing this (August 2026): Component Model is **Phase 1** | The single best way to check "how official is this, really" for any Wasm feature — check it yourself rather than trusting a snapshot, including this document's. |

## Toolchain projects (all confirmed installable and working together)

Every version below is what this repository's samples were actually built
and run against — confirmed via `gh api repos/<org>/<repo>/releases/latest`
at the time of writing, and via real `cargo install --locked`/`brew
install`/`pip install` runs, not assumed.

| Tool | Latest release (Aug 2026) | Verdict |
|---|---|---|
| [wasmtime](https://github.com/bytecodealliance/wasmtime) | v48.0.1, 2026-08-24 | The runtime this whole curriculum runs on. Extremely active release cadence (roughly monthly major versions) — pin the exact version you verify against, per [Stage 0](../curriculum/00-toolchain-setup.md). |
| [wasm-tools](https://github.com/bytecodealliance/wasm-tools) | v1.258.0, 2026-08-24 | Current, actively released alongside wasmtime. |
| [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) | v0.61.1 (CLI), 2026-08-25 | Current — but see [Stage 7](../curriculum/07-wasi-async.md) for a real breaking API change hit going from the *library crate's* 0.41.0 to 0.46.0 while researching this repository. The CLI tool version and the Rust crate version are not the same number and don't need to match. |
| [cargo-component](https://github.com/bytecodealliance/cargo-component) | v0.21.1, released 2025-04-07 | **Works correctly against current wasmtime/wasm-tools** (verified throughout this repo), but hasn't cut a release in over a year as of this writing — the slowest-moving tool in this curriculum's core chain. Worth checking for a newer release before assuming this repo's exact version is still current. |
| [wac](https://github.com/bytecodealliance/wac) | v0.10.1, 2026-06-16 | Current. |
| [wkg](https://github.com/bytecodealliance/wasm-pkg-tools) | v0.16.1, 2026-08-19 | Current — the standard tool for fetching WIT dependencies (like `wasi:cli`) from the WASI package registry; this repo's [Stage 3 sample](../code/03-greet-in-three-languages) vendors its `wit/deps/` this way. |
| [componentize-py](https://github.com/bytecodealliance/componentize-py) | v0.25.0, 2026-07-07 | Current. |
| [go-modules](https://github.com/bytecodealliance/go-modules) (`wit-bindgen-go`) | v0.7.0, 2025-05-25 | Works, but see [Stage 3](../curriculum/03-same-component-other-languages.md)'s TinyGo note — component-model support for pure export-only ("reactor") worlds is currently incomplete. |
| [TinyGo](https://tinygo.org/) | 0.41.1 (verified via `tinygo version`) | The Go compiler this curriculum uses for Wasm components — not upstream Go's own `GOOS=wasip1` support, which targets core Wasm/WASI Preview 1 directly rather than the Component Model. |

## What already exists, and why it isn't a substitute for this curriculum

| Resource | What it is | Current as of | Verdict |
|---|---|---|---|
| [mbasso/awesome-wasm](https://github.com/mbasso/awesome-wasm) | General "awesome" list for the whole Wasm ecosystem | 9,631★, last pushed 2024-11-15 | Aged-but-useful as a raw inventory. Confirmed structurally: 20 topic-grouped sections, zero numbering, zero prerequisites, no Component-Model-era framing. Good for finding one more link; not a path. |
| [wasmerio/awesome-wasi](https://github.com/wasmerio/awesome-wasi) | WASI-specific list | 554★, last pushed 2025-09-30 | The freshest of the two, and WASI-specific — but the same structural gap: 8 sections keyed by category/language, no sequencing, no "read this before that." |
| [component-model.bytecodealliance.org's own tutorial](https://component-model.bytecodealliance.org/tutorial.html) | A single continuous walkthrough: a calculator component, three components, `wac` composition | Targets `wasmtime` ≥v14, WIT @0.2.7 | **The closest thing to a real course that exists today**, and worth doing in addition to this curriculum, not instead of it — it's one continuous document, not gated/checkpointed modules, and (per its own open issues above) doesn't cover host embedding, capability grants, or WASI 0.3 async, all of which are the core of this curriculum's Stages 6–8. |
| [Kubesimplify, "The Complete WebAssembly Course"](https://github.com/kubesimplify/wasm-course) | 14 numbered modules: intro → CNCF landscape → WASI → Preview 2 → Component Model → cloud-native | Published 2024-01-15, 11★, no pushes since | Aged, and genuinely the closest sequenced attempt found anywhere in this research — but no WIT-authoring or Canonical ABI module, lecture/demo style rather than exercise-based, and roughly 2.5 years stale against a WASI 0.2/0.3 world. |
| No Rustlings/Ziglings-style exercise-based path | — | Searched directly (`gh search repos` for "wasi tutorial," "webassembly component model course," "wit-bindgen tutorial," and similar) | **Confirmed absent.** Zero real hits for a sequenced, checkpointed, runnable-exercise curriculum specifically for WASI/the Component Model — this is the gap this repository exists to close. |

## Books

| Resource | What it covers | Current as of | Verdict |
|---|---|---|---|
| ["Server-Side WebAssembly"](https://www.manning.com/books/server-side-webassembly) — Danilo Chiarlone (Manning) | Wasm components, WASI interfacing, production/Kubernetes deployment | Published December 2025 | **The one current book covering this material** — the author is a WASI-proposal contributor, with a foreword by Brendan Burns and an afterword by Luke Wagner (co-chair of the W3C WebAssembly Working Group). Worth reading alongside this curriculum for a deeper, book-length treatment; this curriculum did not independently verify every chapter's technical claims. |
| ["Programming WebAssembly with Rust"](https://pragprog.com/titles/khrust/programming-webassembly-with-rust/) — Kevin Hoffman | Early Rust+Wasm, pre-WASI | Published March 2019 | **Dead as a current reference for this curriculum's material.** Confirmed via direct check: zero mentions of WASI, the Component Model, or WIT anywhere in it — it predates WASI's public existence entirely. |
| "WebAssembly in Action" — Gerard Gallant (Manning) | Browser-era Wasm | Published November 2019 | Aged — WASI is only namechecked as a future direction, not covered as it exists today. |
| "WebAssembly: The Definitive Guide" — Brian Sletten (O'Reilly) | General Wasm | Published December 2021 | Likely aged for this curriculum's purposes — predates the Component Model's 2023–2024 stabilization; its table of contents wasn't independently confirmed in this research due to a paywall, so treat this verdict as date-based inference, not a confirmed gap. |

## Courses and talks

| Resource | What it covers | Current as of | Verdict |
|---|---|---|---|
| [WasmCon 2025](https://colocatedeventsna2025.sched.com/list/descriptions/area/WasmCon) (co-located with KubeCon NA, Atlanta) | Conference-depth talks, including "Does the Component Model Require Extra Copying?" (Luke Wagner, Fastly) and "Composable, Polyglot Concurrency with WASI p3" | 2025-11-10 | Current and primary-source — spec authors talking about exactly the Canonical ABI and async-concurrency material this curriculum's Stages 4 and 7 cover. No recording availability was confirmed in this research; check the Bytecode Alliance's own channels directly. |
| No accredited university course found | — | Searched directly | Unlike CUDA (which has UIUC ECE408, Oxford, CMU offerings), no university course covering WASI/the Component Model in depth was found. |

## Where the ecosystem sits now (production, packaging, direction)

Covered with full citations in [`curriculum/09-production-and-ecosystem.md`](../curriculum/09-production-and-ecosystem.md)
rather than duplicated here — Spin, wasmCloud, Extism, jco, and
`wasmtime serve`, plus an honest read on what's still immature
(multithreading, the Component Model's own standards-track phase, uneven
language support).

## Suggested reading order

This is denser than the curriculum's per-stage lists — use those first;
come back here for the full picture or a next step beyond what a stage
asks for.

1. [webassembly.org](https://webassembly.org/) and [wasi.dev](https://wasi.dev/), for the one-paragraph mental model each provides.
2. This curriculum, Stages 0–9, in order.
3. [component-model.bytecodealliance.org's own tutorial](https://component-model.bytecodealliance.org/tutorial.html), as a second pass covering some of the same ground from a different angle.
4. Danilo Chiarlone's *Server-Side WebAssembly*, for book-length depth on production deployment this curriculum's Stage 9 only sketches.
5. WasmCon 2025's talks, once Stages 4 and 7 have given you the vocabulary to get value from spec-author-level detail.
