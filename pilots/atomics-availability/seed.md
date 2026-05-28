# atomics-availability — Seed

## Telos

Install the Atomics namespace + the 14 method-slot stubs per ECMA-262 §25.4 so test262 structure/availability tests (`prop-desc`, `proto`, `Symbol.toStringTag`, per-method `length`/`name`/`descriptor`/`not-a-constructor`) pass against cruft. Semantic-heavy paths (real `wait`/`waitAsync`/`notify` agent-cluster semantics) stay deferred until shared-memory substrate lands; the carve-out is recorded explicitly so the locale's close-condition is well-defined.

## Apparatus

- `exemplars/exemplars.txt` — 100 stratified-sample paths drawn from `built-ins/Atomics/{top-level + 14 methods}`.
- `exemplars/run-exemplars.sh` — runner; reports aggregate pass/fail + per-method breakdown.
- Test262 runner allowlist entry under `PARTIALLY_IMPLEMENTED.Atomics` opts the structure tests in; semantic tests stay SKIPped.
- Standing gates: build clean; `scripts/diff-prod/run-all.sh` parity.

## Methodology

Pin-Art rung sequence per Doc 581:

- EXT 0 — founding + baseline measurement against the runnable subset (rule 23).
- EXT 1 — namespace + 14 method-stub install + allowlist opt-in. Stubs satisfy the non-shared-memory degenerate path: arithmetic stubs read+write the typed-array index directly; `wait` returns `"not-equal"`; `notify` returns 0; `isLockFree(n)` returns `n ∈ {1,2,4,8}`.
- Close when residual fails are not addressable by stub-level adjustments (i.e., test depends on real SAB / agent-cluster semantics).

## Carve-outs

- Not in scope: real SharedArrayBuffer concurrency (`Atomics.wait` blocking on a shared int32 slot, agent-cluster wakeups via `notify`, `waitAsync` Promise resolution against a shared int32). Cruft is single-threaded; the semantics degrade to the agent-cluster-free path.
- Not in scope: BigInt64Array atomic precision (`Atomics.{add,sub,...}` on 64-bit integers — the stub uses `f64` arithmetic via `i64` cast which loses precision above 2^53).
- Not in scope: `[[Detached]]` checks on the backing buffer (deferred to the ArrayBuffer-detached locale when it spawns).

## Composes with

- Doc 729 — resolver-instance pattern; Atomics is a runtime-tier intrinsic namespace installed alongside Math / JSON / Reflect.
- Doc 737 — locale-as-coordinate; this locale is rung-1 substrate.
- Predictive-ruleset Rule 23 (founding-baseline-inspection) — applied at EXT 0.
- The test262 runner allowlist mechanism (PARTIALLY_IMPLEMENTED) — standing recurrence per Finding PDTS.5.

## Resume protocol

Read this seed → trajectory.md tail → re-run `exemplars/run-exemplars.sh`. Next rung addresses the dominant fail-method per the runner breakdown.
