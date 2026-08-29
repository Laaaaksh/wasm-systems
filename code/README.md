# Code samples

Eight small, self-contained programs — components and the hosts that run
them — each isolating one idea. Every sample has its own `Makefile` and
`README.md` explaining what to look for.

| # | Sample | Idea |
|---|--------|------|
| [01](01-core-wasm-no-wasi) | `core-wasm-no-wasi` | Raw Wasm has no OS interface — WASI supplies one |
| [02](02-hello-component-rust) | `hello-component-rust` | A real component, inspected with `wasm-tools` |
| [03](03-greet-in-three-languages) | `greet-in-three-languages` | The same WIT world, three unrelated toolchains |
| [04](04-richer-wit-world) | `richer-wit-world` | Records, variants, and results crossing the Canonical ABI |
| [05](05-composition) | `composition` | Linking one component's import to another's export, with `wac` |
| [06](06-host-embedding) | `host-embedding` | A host that decides what a guest can touch |
| [07](07-wasi-async) | `wasi-async` | WASI 0.3's `stream<T>` — what builds today, what doesn't run yet |
| [08](08-plugin-host-capstone) | `plugin-host-capstone` | A host for plugins it's never seen |

Each pairs with a stage in [`curriculum/`](../curriculum) — see the
"Companion to" link at the top of each sample's README.

## Building

Every sample builds and runs the same way:

```bash
cd code/0N-sample-name
make run     # or `make demo` / `make validate` - check each sample's README
```

[`curriculum/00-toolchain-setup.md`](../curriculum/00-toolchain-setup.md)
covers everything you need installed first.

## Every sample here was actually run, not just compiled

Unlike a curriculum for hardware-gated material, WebAssembly's whole
toolchain runs on an ordinary CPU — there's no GPU-style access gap to
disclose. Every `make run`/`make demo` above is a real, reproduced result
in this environment, pinned to exact tool versions
([Stage 0](../curriculum/00-toolchain-setup.md)'s table: `rustc` 1.98.0,
`cargo-component` 0.21.1, `wasmtime` 48.0.1, `wasm-tools` 1.258.0,
`wit-bindgen-cli` 0.61.1, `wac` 0.10.1, `wkg` 0.16.1, `componentize-py`
0.25.0, `tinygo` 0.41.1). The one deliberate exception is
[`07-wasi-async`](07-wasi-async), which builds and validates a real
component but documents, with exact reproducible error text, exactly
where running it hits a current `wasmtime` CLI limitation — see that
sample's README rather than assuming it fully works.

## `Cargo.lock` files are committed on purpose

This is a teaching repository: the exact dependency versions each sample
was verified against matter more here than they would in a library meant
to float with its dependencies. If you update a sample, update its
`Cargo.lock` deliberately and say what changed, rather than letting it
drift silently.
