# Security Policy

wasm-systems is a set of educational docs and small, self-contained
WebAssembly/WASI component samples. Some samples build a host program that
embeds a runtime (`wasmtime`) and load components at runtime - that's the
part of this repo with a real (if narrow) attack surface, since it's
literally demonstrating a sandboxing model.

## What belongs in a report

Worth reporting privately:

- A host-embedding sample that grants a loaded component more capability
  (filesystem, network, environment access) than its README says it grants -
  that would undermine the exact thing the sample is meant to teach.
- A sample that executes something fetched over the network without saying
  so, or that shells out to a command built from unsanitized input.
- Anything in [`code/08-plugin-host-capstone`](code/08-plugin-host-capstone)
  (the untrusted-plugin loader) that doesn't actually sandbox a loaded
  plugin the way its README claims.

Not a security issue, just a normal bug report (open a public issue instead):

- A component that compiles but returns the wrong value.
- A build or version-mismatch error against a newer toolchain release.
- A dead or incorrect link in the curated resources.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting:

> https://github.com/Laaaaksh/wasm-systems/security/advisories/new

That reaches the maintainer privately so any real issue can be fixed before
it's discussed in public.

## Credits

Reporters who wish to be credited may say so in the private report; otherwise
reports are handled without attribution.
