# Stage 8 — Capstone: a host for untrusted plugins

**You'll be able to:** design and build a host that loads components it
has no compile-time knowledge of, and explain why this — not a bigger
single program — is the shape the Component Model is actually good for.

**Time:** 3–5 hours.

**Build:** [`code/08-plugin-host-capstone`](../code/08-plugin-host-capstone).

## Read, in this order

1. **[Extism's own site, extism.org](https://extism.org/)** — a real,
   independently-maintained "universal plugin system" built on
   WebAssembly, from [Dylibso](https://dylibso.com/products/extism/).
   This stage's sample is a small, from-scratch version of exactly the
   pattern Extism productizes: a fixed host, an open set of guest
   plugins, a minimal shared contract between them.
2. **Re-read [Stage 6](06-host-embedding-and-capabilities.md)'s "what to
   look for" section** before starting — this capstone is that stage's
   capability-grant pattern, applied to code the host author didn't
   write and has no reason to trust.

## Do

```bash
cd code/08-plugin-host-capstone
make run
```

Read the whole sample README — the "capability story, again" section is
the heart of this stage: it shows a real, current, mildly surprising fact
(every plugin still imports the full `wasi:cli` world, regardless of what
its own WIT declares) and uses it to make the sharpest version of this
curriculum's core security claim.

Then do the "things to try": add a third plugin, and try to break one on
purpose (a wrong-typed `transform`, or an attempted filesystem read) to
see the failure modes directly rather than take this repo's word for
them.

## The concept everyone gets wrong here

**"An untrusted plugin needs to be checked for bad behavior before it's
loaded."** In a capability-secure host, that's backwards: you don't need
to *audit* what a plugin might try to do, because the host already
controls what it's *capable* of doing, before a single instruction of the
plugin's code runs. This is the practical payoff of everything from
Stage 6 onward — the interesting security question for a plugin host
built this way isn't "is this plugin's code safe," it's "did I grant this
plugin only what it actually needs," which is a much smaller, more
answerable question, and one this stage's host answers by granting
essentially nothing.

## Checkpoint

- `host/src/main.rs` never imports type-checked bindings for `shout` or
  `reverse`. What does it use instead to find and call their exports, and
  what's the tradeoff against Stage 3's compile-time-checked approach?
- Both plugins in this sample import the full `wasi:cli` world but are
  granted none of it. Why do they still work?
- If a plugin author wanted their plugin to be able to write a log file,
  what would have to change — in the plugin's code, in the host's code,
  or both?

Next: [Stage 9 — where this sits now](09-production-and-ecosystem.md).
