---
coord: pilots/json-stringify-semantics
tag: json-stringify-semantics
parent_arc: apparatus/arcs/2026-05-28-json-serialization-carve-out
status: IN PROGRESS
---

# JSON.stringify Semantics

## Telos

Close `JSON.stringify` against the local Test262 slice by consuming each
serialization directive at its resolver boundary, not by broad JSON-level
special casing.

## Apparatus

- Test262 local slice: `/Users/jaredfoy/Developer/cruftless-sidecar/test262/test/built-ins/JSON/stringify/*.js`.
- Runner: `legacy/host-rquickjs/tests/test262/runner.mjs`.
- Binary: `target/debug/cruft` or release `cruft`.

## Method

Follow the substrate-shaped pipeline:

1. Inspect the failing row cluster.
2. Identify the resolver boundary the row is actually probing.
3. Move the boundary once, then measure focused rows plus the full stringify
   slice.
4. Record explicit carve-outs when a row fails before reaching JSON.

## Resume

Read `trajectory.md` tail first. The current residual clusters after JSS-EXT 5
are gap/space, BigInt/toJSON, wrapper value substitution, lone surrogate
escaping, and proxy/realm `Proxy.revocable` availability.
