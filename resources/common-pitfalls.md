# The things that actually stop people

Collected in one place, so you can check here before assuming you've found
a new problem. Every one of these was hit directly while building this
repository, not gathered secondhand — each includes the exact error text
you'll see.

## Toolchain and versions

**A WIT resolution error mentioning a package or version it can't find,
from `cargo component build`, `componentize-py`, or `wit-bindgen-go
generate`.** All of this curriculum's WIT tooling — `wit-bindgen`,
`cargo-component`, `wac`, `wkg` — moves together against a WIT
specification that itself keeps changing. A mismatched pair (an older
tool against a WIT file written for a newer `wasi:cli`, for instance)
produces an error that looks like a mistake in your `.wit` file, not a
version problem. Check `cargo component --version`, `wasm-tools
--version`, and `wkg --version` against [Stage 0](../curriculum/00-toolchain-setup.md)'s
table before assuming your WIT is wrong.

**`cargo component build` fails with `package not found` when you add a
local WIT dependency via `[package.metadata.component.target.dependencies]`
pointing at a `wit/deps/<name>` directory you copied from elsewhere.**
Hit directly while building [Stage 5](../code/05-composition)'s sample: a
plain relative `path = "../other-crate/wit"` pointing at another crate's
own WIT source resolved fine, but a manually restructured `wit/deps/`
tree (the shape `wkg wit fetch` produces) needed the dependency path to
point at the specific vendored subdirectory (`wit/deps/wasi-cli-0.2.3`,
not just `wit/deps`) to resolve. If a path dependency fails to resolve,
try pointing it at the most specific directory that directly contains a
`package ...;` declaration.

**Building a Go component with a pure export-only ("reactor") WIT
world fails during `wasm-tools component new`, with an error like `module
requires an import interface named 'wasi:cli/environment@0.2.0'`, even
though your WIT never mentions `wasi:cli` at all.** This is a real,
current TinyGo limitation (confirmed against TinyGo 0.41.1): its
component-model support currently assumes a full `wasi:cli/command` world
is present, regardless of what your own WIT world declares. [Stage
3](../curriculum/03-same-component-other-languages.md)'s sample works
around this by giving the Go (and, for consistency, Rust and Python)
implementations a `wasi:cli/command`-based world from the start, rather
than fighting this on a per-sample basis.

**Every component `cargo component build` produces imports the full
`wasi:cli` world (`environment`, `stdout`, `filesystem/*`, and more),
even when your own `.wit` file declares zero WASI dependency.** Confirmed
directly, repeatedly, across this repository's samples — including
[Stage 8](../code/08-plugin-host-capstone)'s plugins, whose own WIT
exports two pure functions and imports nothing. Run `wasm-tools component
wit` on anything you build and check it against what you actually wrote —
don't assume your source file and the tool's actual output agree.

**`rustup target add wasm32-wasip3` fails outright**, on both stable and
nightly Rust, with `no prebuilt artifacts available for target
'wasm32-wasip3'... this may happen to a low-tier target`. Confirmed as of
this writing (August 2026), roughly two months after WASI 0.3 shipped.
This isn't a local misconfiguration — [Stage 7](../curriculum/07-wasi-async.md)'s
sample deliberately targets `wasm32-wasip2` instead, since WASI 0.3
async's guest-side support is a library on top of the existing ABI, not a
distinct Rust target.

**A `wit-bindgen` (the Rust crate, not the CLI) minor-version bump breaks
your async guest code.** Hit directly going from `wit-bindgen` 0.41.0 to
0.46.0 while building [Stage 7](../code/07-wasi-async)'s sample:
`StreamWriter<T>` stopped implementing `futures::Sink` and gained a
direct `.write()` method instead — a real, observed breaking change
within a single crate's 0.x version series. Pin this dependency exactly
(`=0.46.0`, not `"0.46"`) in anything touching WASI 0.3 async, and expect
to re-verify it against every new release rather than assuming semver
stability that a pre-1.0 crate never promised.

## The first time you write a host program

**A denied capability doesn't look like "permission denied."** A
component whose host didn't grant it filesystem access sees the *same*
error (`ENOENT`, "no such file or directory") a genuinely missing file
would produce — not a distinguishable "access denied." This is
deliberate: telling a sandboxed component "that exists, you're just not
allowed to see it" would leak information about the host's filesystem to
code explicitly denied access to it. See [Stage 6](../curriculum/06-host-embedding-and-capabilities.md)
for the exact reproduction.

**`wasmtime run --invoke` prints experimental warnings, or panics
outright, for anything beyond the simplest core-module function call.**
Confirmed directly: invoking a function with arguments/return values on a
bare core module (no Component Model) prints "using `--invoke` with a
function that takes arguments is experimental" warnings; invoking a
function returning a `stream<T>` panics inside `wasmtime`'s own
`wasm-wave` crate with `unsupported value type` (see [Stage
7](../curriculum/07-wasi-async.md) for the full reproduction). Neither is a
sign your component is wrong — the CLI's `--invoke` convenience layer has
real, current limits distinct from what the runtime itself can execute
given a proper host.

## Misconceptions almost everyone starts with

**"WASI is a runtime."** It's a specification for host-provided
interfaces; `wasmtime`, `wasmer`, and others are runtimes that each
implement it. See [Stage 1](../curriculum/01-why-wasi.md).

**"A component can call another component directly, like linking a
shared library."** Not without an explicit step — either composition
(build-time, static, via `wac`; see [Stage 5](../curriculum/05-composition.md))
or a host that loads and mediates between them at runtime (see [Stage
8](../curriculum/08-plugin-host-capstone.md)). Two components sitting in
the same process are still two separate sandboxes.

**"If `wasm-tools validate` says a component is valid, it will run
anywhere."** Validity means "conforms to the Component Model
specification." It says nothing about whether a *specific* runtime's
*specific* execution mode has implemented every canonical built-in a
valid component is allowed to use — [Stage 7](../curriculum/07-wasi-async.md)
is a complete, reproducible example of a valid component that
`wasmtime run`'s synchronous CLI path currently cannot instantiate.

**"cargo-component's default WIT world for a `--bin` component is
minimal."** It isn't — it pulls in the entire `wasi:cli` world
(`stdout`, `stdin`, `stderr`, `environment`, `exit`, `filesystem/*`,
`clocks/*`) whether your program uses all of that or not. Run `wasm-tools
component wit` on anything before assuming you know its actual surface;
see [Stage 2](../curriculum/02-first-component.md).
