# temporal-availability — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/spec-builtins :: E3/intrinsic-object:temporal :: availability/missing-global-or-binding :: err:ReferenceError-like
```

This coordinate sits at the top of the full-suite Pin-Art matrix
(`pilots/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/matrix.md`
rank #1, **4,152 fails**, ~17.4% of the 23,768 interpreted non-pass
records). Per the new apparatus articulation
(`apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`),
the telos is the explicit DAG coordinate that had to exist for the result
to pass, not the percentage shift itself. Closing this coordinate names
the substrate decision that the engine must make to surface `Temporal`
as a globalThis-bound intrinsic.

Spec anchor: TC39 Proposal-Temporal (Stage 3), candidate for ECMA-262
inclusion via the temporal-objects draft. Surfaces: PlainDate,
PlainTime, PlainDateTime, ZonedDateTime, PlainYearMonth, PlainMonthDay,
Duration, Instant, Now.

## Apparatus

- **Exemplar suite**: `pilots/temporal-availability/exemplars/exemplars.txt`
  — 100 paths stratified-sampled from the 4,152-fixture pool by Temporal
  sub-class (proportional + min 1 per class for coverage).
- **Exemplar runner**: `pilots/temporal-availability/exemplars/run-exemplars.sh`
  — runs the 100 via the test262 harness wrapper against cruft; prints
  aggregate + per-class breakdown of remaining fails.
- **Substrate site** (to be edited): `pilots/rusty-js-runtime/derived/src/` —
  intrinsic-object registration tier. Mirror the existing built-in
  registration pattern (e.g. how `Date`, `Map`, `WeakMap` are bound to
  globalThis).
- **Baseline measurement (FOUNDING)**: PASS=0 / 100 (0.0%), 2026-05-25.
  All 12 Temporal classes uncovered; ZonedDateTime/PlainDateTime/PlainDate
  the largest sub-clusters (20/17/14 respectively).

## Methodology

Standing rule 13 applies here in prospective form. The deeper-layer
closure is **availability** (registering the Temporal namespace as a
global), not implementation completeness of every Temporal method. C1-C4
read:

- **C1 (sibling closure pattern)**: HOLDS — existing intrinsic-object
  registration of Date/Map/Set/WeakRef/AggregateError is the empirical
  anchor. Same shape: register a constructor + prototype, install methods.
- **C2 (shape-compat with substrate APIs)**: HOLDS — Runtime's
  `register_intrinsic` family is the established mechanism.
- **C3 (cost-positive when integrated)**: TO BE VERIFIED at first
  measurement. The exemplar suite is the immediate falsifier; ≥one
  exemplar pass after registration is the C3 confirmation.
- **C4 (bail safety)**: HOLDS at the parser/AST tier (Temporal is
  user-code-accessed via dot-notation off a global; no parser change).

Multi-rung shape per Doc 737 §II:

- **TA-EXT 0** (founding; this rung): name the coordinate, build the
  exemplar suite, baseline.
- **TA-EXT 1** (registration MVP): bind a stub `globalThis.Temporal`
  namespace with at minimum the 9 class constructors as Identifier-typed
  values. Expect partial exemplar yield (constructor-presence tests pass,
  method-call tests still fail).
- **TA-EXT 2+** (class-by-class implementation): largest sub-classes first
  (ZonedDateTime → PlainDateTime → PlainDate → Duration → ...). Per-class
  rounds spawn nested locales if their multi-rung shape warrants.

## Carve-outs

- **Temporal calendar/timezone machinery** is large; the founding telos
  is the availability axis (the single decision to register), not the
  full semantics. Class-by-class semantic rungs land as separate
  trajectory entries (or nested locales) under standing rule 4 (no
  half-landed moves at each tier).
- **Intl402 dependency**: some Temporal operations defer to Intl402
  Calendar/TimeZone identifiers. Intl402 is its own top-coordinate
  (rank #2, 2,008 fails); the cross-tier dependency is named but
  resolved in the Intl402 locale.

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
  — the framing of conformance as decision-basis extraction.
- `pilots/test262-categorize/full-suite/` — the canonical full-suite
  measurement that surfaces this coordinate.
- `pilots/intl402-availability/` (future) — rank #2 sibling coordinate.
- Standing rule 13 prospective application (C1-C4 above).

## Resume protocol

Read `trajectory.md` tail; then run `exemplars/run-exemplars.sh` for
the current cluster yield. Add new rounds as substrate moves land.
