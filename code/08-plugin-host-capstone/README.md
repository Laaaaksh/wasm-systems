# 08 — Capstone: a host for plugins it's never seen

Everything before this sample had the host and guest built against a
shared WIT file, known at compile time. This one drops that: [`host/`](host)
loads every `.wasm` file in a directory, calls two functions on each by
**name** (`name() -> string` and `transform(string) -> string`), and never
sees [`plugins/shout`](plugins/shout) or [`plugins/reverse`](plugins/reverse)'s
WIT at its own compile time. This is the [Extism](https://extism.org/)
plugin-host pattern: a fixed host binary, an open-ended set of guest
plugins written by anyone, in any language the Component Model supports.

Companion to [`curriculum/08-plugin-host-capstone.md`](../../curriculum/08-plugin-host-capstone.md).

## Run it

```bash
make run
```

```
reverse: "Hello, plugins!" -> "!snigulp ,olleH"
shout: "Hello, plugins!" -> "HELLO, PLUGINS!!"
```

`host` scanned `plugins-built/` for `.wasm` files, loaded each one, and
called `name()`/`transform()` on each generically - `host/src/main.rs`
never imports type-checked bindings for `shout` or `reverse` specifically,
it calls `instance.get_typed_func(&mut store, "name")` by string.

## The capability story, again, at the point where it matters most

`host/src/main.rs` grants each plugin **`WasiCtx::builder().build()`
with nothing added** - no inherited stdout, no filesystem, nothing. Run
`make wit` and look at `plugins-built/shout.wasm`'s actual imports: it
still lists the entire `wasi:cli` world (`environment`, `stdout`,
`filesystem/*`, and more), because that's what `cargo component build`
attaches to every component today regardless of what its own
[`wit/world.wit`](wit/world.wit) declares (that file exports two plain
functions and imports nothing at all). **The plugins still work,
identically, with every one of those imports satisfied but backed by
nothing.** That's the actual lesson: capability is enforced by what a
component *calls*, not by what it happens to *import* - `shout` and
`reverse` never call `wasi:cli/stdout.get-stdout`, so it not mattering
that this host gave them no real stdout is invisible from the outside.
This is also a good example of why [Stage 0](../../curriculum/00-toolchain-setup.md)
tells you to check `wasm-tools component wit` against a real build rather
than trusting what you wrote in a `.wit` file - the tool's actual current
default behavior and your source file can disagree.

This is the sharpest version of the untrusted-plugin threat model this
repository covers: if you were loading a plugin you didn't write, from
someone you don't fully trust, this is the difference between "it merely
can't do anything," which this host provides, and "it politely chose not
to," which a plugin's own good behavior cannot be relied on for.

## Things to try

- Change `plugins/shout/src/lib.rs`'s `transform` to attempt
  `std::env::var("HOME")` or `std::fs::read_to_string("/etc/passwd")` and
  rebuild - the call will fail (the imports are satisfied by an empty
  `WasiCtx`, so any real use traps or errors), not silently succeed. This
  is the same guarantee as [Stage 6](../06-host-embedding), now applied to
  a plugin the host author didn't write.
- Add a third plugin - anything exporting `name`/`transform` with the
  matching signature, in any language this repository's other samples
  build components in - and drop its `.wasm` into `plugins-built/` (or add
  it as a new crate and wire it into the Makefile). No change to
  `host/src/main.rs` is needed.
- Change one plugin's `transform` to a different signature (e.g. take a
  `u32` instead of a `string`) and rerun - `get_typed_func` performs a
  runtime type check against the actual component, so this fails loudly
  at the `get_typed_func` call, not silently.
