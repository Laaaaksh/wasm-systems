# Contributing to wasm-systems

Thanks for considering a contribution. This is a curriculum plus runnable
samples, open source under the MIT license.

## Getting started

```bash
git clone https://github.com/<your-username>/wasm-systems.git   # your fork
cd wasm-systems
```

Install the toolchain in [`curriculum/00-toolchain-setup.md`](curriculum/00-toolchain-setup.md)
before touching anything under `code/`. Unlike a GPU-gated curriculum,
every sample here runs on an ordinary CPU — there's no compile-only
fallback path, you can and should actually run what you change.

```bash
cd code/02-hello-component-rust
make run
```

## Contribution workflow

1. Fork the repo, clone your fork (command above).
2. Create a descriptively named branch off `main`.
3. Make focused commits.
4. If you touched anything under `code/`, run the sample and paste the
   real output in the PR — `make run`/`make demo`/`make validate`,
   whichever that sample's Makefile defines. Never claim a sample works
   without having actually run it.
5. If you touched `resources/curated-resources.md` or added an external
   link anywhere, open every link you're adding or changing and confirm
   it resolves before submitting — via a real fetch, not from memory.
   Never add a link you haven't personally opened. If you're citing a
   version number or a date, get it from the source (a release page, a
   `--version` flag), not a guess.
6. Open a pull request against `main`.

A PR can merge only when CI's checks pass and review feedback is
resolved.

## What contributions are useful

- Fixing a wrong claim, a stale "current" statement, or a dead link.
- **A working async host for [Stage 7](curriculum/07-wasi-async.md)** —
  that stage documents a real, current `wasmtime` CLI limitation running
  WASI 0.3 async components; if you get a custom async-embedding host
  working end to end, that's a genuinely valuable contribution, not a
  nitpick.
- A new sample that isolates one concept the way the existing ones do
  (see "Adding a sample" below) — open an issue first so scope is agreed
  before you write it.
- Corrections from testing against a toolchain version newer than this
  repo has verified against — say exactly which versions you used.
- Curriculum sequencing feedback: if a stage assumes something the
  previous stage didn't actually teach, that's a real bug in a course,
  not a nitpick.

## Adding a sample

Each directory under `code/` demonstrates exactly one idea. Follow the
existing pattern:

- A `Makefile` that builds and runs with `make run` (or a documented
  equivalent target — `make demo`, `make validate`), plus a `README.md`
  explaining what the sample shows, what to look for in the output, and
  how it connects to the curriculum stage that references it.
- Comments in the code explain *why* a line matters for the concept being
  taught, not what a WASI/wasmtime API call does (link to the docs for
  that).
- `Cargo.lock` (and any language-equivalent lockfile) is committed —
  see [`code/README.md`](code/README.md) for why.
- Prefer extending an existing stage's "things to try" over adding a new
  curriculum stage — new stages change the sequencing for everyone.

## Code style

- Comments explain *why*, not *what* — the reader can read the language's
  syntax; they need help with the parts that are non-obvious (capability
  grants, Canonical ABI value shapes, why a particular WIT world was
  chosen).
- Match the pattern already used in sibling samples (Makefile target
  names, error-checking style) rather than inventing a new one.

## Reporting issues

Open a GitHub issue before starting anything larger than a typo fix, so
scope is agreed first. Use the bug report template for something broken,
and the resource suggestion template for anything about
`resources/curated-resources.md`.
