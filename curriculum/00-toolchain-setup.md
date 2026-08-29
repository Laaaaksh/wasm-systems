# Stage 0 — Get a toolchain working

**You'll be able to:** compile a Rust program to a WebAssembly component,
run it under `wasmtime`, and diagnose the two or three version-mismatch
errors you're most likely to hit doing that.

**Time:** 30–60 minutes, mostly waiting on installs.

**Build:** nothing yet — this stage ends when the checkpoint commands at
the bottom all print a version number.

## What you need, and why

This curriculum is multi-language on the guest side (Rust, Python, Go) but
single-toolchain on the tooling side: everything routes through the same
WASI Component Model tools regardless of which language you're writing a
guest in. Install these once:

| Tool | What it's for | Version this repo was built and verified against |
|---|---|---|
| [Rust](https://www.rust-lang.org/tools/install) + [`rustup`](https://rust-lang.github.io/rustup/) | The primary guest language | rustc 1.98.0 |
| [`wasmtime`](https://wasmtime.dev/) | The runtime that loads and runs components | 48.0.1 |
| [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) | Inspect, validate, and convert component binaries | 1.258.0 |
| [`cargo-component`](https://github.com/bytecodealliance/cargo-component) | Build Rust components with `cargo component build` | 0.21.1 |
| [`wit-bindgen-cli`](https://github.com/bytecodealliance/wit-bindgen) | Generate guest bindings from a `.wit` file directly (used when you're not going through `cargo component`) | 0.61.1 |
| [`wac`](https://github.com/bytecodealliance/wac) | Compose two or more components into one | 0.10.1 |
| [`wkg`](https://github.com/bytecodealliance/wasm-pkg-tools) | Fetch WIT package dependencies (e.g. the standard `wasi:cli` interfaces) from the WASI registry | 0.16.1 |

Stage 3 also needs, only for that stage:

| Tool | What it's for | Version |
|---|---|---|
| Python 3.9+ and [`componentize-py`](https://github.com/bytecodealliance/componentize-py) | Build Python components | componentize-py 0.25.0 |
| [TinyGo](https://tinygo.org/) and [`wit-bindgen-go`](https://github.com/bytecodealliance/go-modules) | Build Go components | TinyGo 0.41.1, `go.bytecodealliance.org/cmd` 0.7.0 |

Every version above is a real, current release as of August 2026 (see
[`resources/curated-resources.md`](../resources/curated-resources.md) for
release dates) and is the exact version every code sample in this repo was
built and run against. If a command below produces different output than
this document describes, a version drift is the first thing to suspect —
see [Stage 0's own entry in `resources/common-pitfalls.md`](../resources/common-pitfalls.md#toolchain-and-versions).

## Install

**1. Rust**, via `rustup` (not your OS package manager — you need to add
Wasm compilation targets later, which `rustup target add` handles and a
system package usually doesn't):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasip1 wasm32-wasip2
```

You need both `wasm32-wasip1` and `wasm32-wasip2` installed: `cargo
component build` currently targets `wasm32-wasip1` and wraps the result
into a component itself (confirmed against `cargo-component` 0.21.1 — check
`file target/wasm32-wasip1/release/*.wasm` after any build in this repo),
while some lower-level exercises in [Stage 1](01-why-wasi.md) build against
`wasm32-wasip2` and `wasm32-unknown-unknown` directly to show the
difference. This split — a widely-used build tool still emitting Preview 1
underneath a Preview 2/Component Model surface — is a real, current
artifact of how fast this ecosystem is moving, not an error in your setup.

**2. `wasmtime`, `wasm-tools`, `cargo-component`, `wac`, `wkg`** — the
fastest path on macOS is Homebrew (`wasm-tools`, `cargo-component`, and
`wasmtime` are all in `homebrew-core`); on Linux, use each project's
install script or a `cargo install --locked <name>` (all of these are
ordinary Rust binaries you can build from crates.io):

```bash
# macOS
brew install wasmtime wasm-tools cargo-component

# any platform with a Rust toolchain
cargo install wac-cli --locked
cargo install wkg --locked
```

Check each project's own install instructions if you're not on macOS —
[wasmtime.dev](https://wasmtime.dev/), and each tool's GitHub releases page,
are the current sources of truth, not this document.

**3. `wit-bindgen-cli`** (needed directly in Stage 5's Go path and useful
any time you want to inspect generated bindings without a full build):

```bash
cargo install wit-bindgen-cli --locked
```

**4. Python and Go paths (Stage 3 only)** — install when you get there,
not before:

```bash
python3 -m venv .venv && source .venv/bin/activate
pip install componentize-py

brew tap tinygo-org/tools && brew install tinygo   # or see tinygo.org/getting-started
go install go.bytecodealliance.org/cmd/wit-bindgen-go@latest
```

## The version-mismatch failure you're most likely to hit

**`cargo component build` fails with a WIT resolution error mentioning a
package or version it can't find.** The Component Model's WIT tooling
(`wit-bindgen`, `cargo-component`, `wac`, `wkg`) all move together, and a
mismatched pair — an old `cargo-component` against a WIT file written for
a newer `wasi:cli` version, for instance — produces a resolution error
that looks like a mistake in your WIT, not a version problem. If a sample
in this repo fails to build and the error mentions a package name and
version, run `cargo component --version`, `wasm-tools --version`, and `wkg
--version` and compare them against the table above before assuming the
WIT file is wrong. Full detail: [`resources/common-pitfalls.md`](../resources/common-pitfalls.md).

**A Go component built against `wasi:cli` fails with `wasm-tools component
new` complaining about a missing `wasi:cli/environment` import, even for a
component that should only export a plain function.** This is a real,
current TinyGo limitation — as of TinyGo 0.41.1, its component-model
support assumes the full `wasi:cli/command` world is present, even for a
library-style ("reactor") component with no CLI needs. [Stage 3's
sample](../code/03-greet-in-three-languages) works around this by giving
every language the same `wasi:cli/command`-based world rather than a
bespoke export-only one — see that sample's README for the exact error and
why.

## Checkpoint

All of these should print a version, with no error:

```bash
rustc --version
cargo component --version
wasmtime --version
wasm-tools --version
wac --version
wkg --version
```

Then prove the whole pipeline once, end to end:

```bash
cd code/02-hello-component-rust
make run
```

You should see `Hello, WASI!`. If that works, every tool in the chain —
Rust, `cargo-component`, and `wasmtime` — is correctly installed and
talking to the others. Next: [Stage 1 — why WASI exists](01-why-wasi.md).
