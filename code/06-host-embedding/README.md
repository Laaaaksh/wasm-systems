# 06 — A host that decides what a guest can touch

Every earlier sample ran under the `wasmtime` CLI, which grants a generous
default WASI environment (inherited stdio, and whatever `--dir` you pass).
This sample replaces the CLI with [`runner/`](runner), a small native Rust
program that embeds `wasmtime` directly and makes the capability decision
itself, in code.

Companion to [`curriculum/06-host-embedding-and-capabilities.md`](../../curriculum/06-host-embedding-and-capabilities.md).
Built against `wasmtime` (crate) 48.0.1 and `wasmtime-wasi` (crate) 48.0.1 -
the same major version as the CLI used throughout this repo, so this
sample's embedding API is exactly what the `wasmtime` binary you've been
using is built on top of.

[`guest/`](guest) is an ordinary `wasi:cli/command` component (same shape
as [`02-hello-component-rust`](../02-hello-component-rust)) that tries to
read a file, `secret.txt`, from its current directory, and prints whether
that worked.

## Run it

```bash
make run
```

Expected output:

```
--- host: instantiating with no filesystem grant ---
guest: starting up
guest: could not read secret.txt: No such file or directory (os error 44)
guest: done

--- host: instantiating with 'granted-dir' granted read-only ---
guest: starting up
guest: read secret.txt: the treasure is buried under the oak tree
guest: done
```

**The same `guest.wasm` binary, unmodified, run twice by the same host
program.** The only difference between the two runs is
[`runner/src/main.rs`](runner/src/main.rs)'s `grant_dir` argument - whether
`WasiCtxBuilder::preopened_dir` was called before `build()`. The guest
never had a choice in the matter, and its code doesn't change between
runs: it asked to read a file, and the host either had already decided to
make that file visible to it or hadn't.

## What to look for in `runner/src/main.rs`

- `WasiCtx::builder()` starts from **nothing granted** - no stdio, no
  filesystem, no clock, no random, no network. `.inherit_stdout()` is an
  explicit grant, made once, for both runs. This is the opposite default
  from a typical OS process, which inherits its parent's whole environment
  unless something sandboxes it.
- `.preopened_dir(dir, ".", FsPerms::ReadOnly)` is the entire access-control
  decision for the filesystem. `FsPerms::ReadOnly` is enforced by the
  runtime, not by guest cooperation - the guest can't upgrade it to
  read-write no matter what code it runs, because the write operations
  were never wired into its imports for that descriptor. Try changing it
  to `FsPerms::ReadWrite` and have the guest attempt a write, to see the
  difference.
- `wasi_snapshot_preview1`-level errno `44` (`ENOENT`, "no such file or
  directory") is what a denied-by-absence lookup actually looks like to
  the guest - not a distinguishable "permission denied." From inside the
  sandbox, "this doesn't exist" and "you're not allowed to see this" are
  the same observation. That's a deliberate part of the capability model:
  a component can't even confirm something exists if it wasn't granted a
  path to it.

## Things to try

- Delete the `.inherit_stdout()` call, rebuild the runner, and rerun -
  `println!` in the guest now goes nowhere, the same silent-void failure
  mode as [`01-core-wasm-no-wasi`](../01-core-wasm-no-wasi), but this time
  caused by a host decision instead of a missing WASI import.
- Change `guest/src/main.rs` to also attempt `std::fs::write("new.txt",
  "hi")` inside the granted-but-read-only directory, and see the specific
  error WASI reports for a write against a read-only preopen.
- This is the natural place to introduce a WIT `resource` (a stateful
  handle a host or guest owns and passes by reference) - `granted-dir`'s
  file descriptor is conceptually one, though this sample uses WASI's
  built-in filesystem resource type rather than a custom one.
  [Stage 8](../../curriculum/08-plugin-host-capstone.md) builds a host
  that manages resources of its own design.
