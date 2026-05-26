---
name: temporal-now
description: First nested sub-locale under temporal-implementation. Temporal.Now.{plainDateTimeISO, zonedDateTimeISO, instant} — smallest viable Temporal surface; validates namespace + host-clock plumbing.
type: project
---

# temporal-now — Seed

## Nested sub-locale under `pilots/temporal-implementation/`.

First rung in the Temporal program. 3-test surface; smallest viable subset.

## Telos

Implement `Temporal.Now.plainDateTimeISO()`, `Temporal.Now.zonedDateTimeISO()`, and `Temporal.Now.instant()` sufficient to pass `intl402/Temporal/Now/*` (3 tests). Each returns a Temporal.X instance whose value reflects the current host wall-clock time.

This rung validates two substrate pieces before bigger classes land:
1. The `Temporal` global namespace can be registered and looked up.
2. cruft's host-clock plumbing can produce the nanosecond-precision timestamp Temporal expects (BigInt-backed).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` — Temporal namespace registration + Temporal.Now object.
- New: `pilots/rusty-js-runtime/derived/src/temporal/mod.rs` (or similar) — Instant, PlainDateTime, ZonedDateTime stub types just sufficient to construct.
- Host clock: existing `std::time::SystemTime::now()` plumbing.
- **Exemplar suite**: 3 fixtures in `pilots/temporal-implementation/temporal-now/exemplars/exemplars.txt`.

## Baseline

3/3 SKIP via RFSDO-EXT 2's Temporal deny-list. Pre-RFSDO: 3/3 FAIL with `Temporal is not defined`.

## Methodology

### TN-EXT 1 — Temporal namespace + Now.* skeleton (pending)

1. Register `Temporal` as a global frozen namespace object.
2. Register `Temporal.Now` as a frozen sub-namespace.
3. Implement `Temporal.Now.instant()` returning a stub Temporal.Instant whose epochNanoseconds is the host clock.
4. Implement `Temporal.Now.plainDateTimeISO()` and `Temporal.Now.zonedDateTimeISO()` returning stub instances.

The stub Temporal.Instant / PlainDateTime / ZonedDateTime need just enough methods to satisfy the test assertions (probably `epochMilliseconds`, ISO toString form). Full method coverage belongs to subsequent rungs.

### TN-EXT 2 — RFSDO sync (after TN-EXT 1 lands)

If yield is 2+/3, leave the `Temporal` flag in RFSDO until temporal-instant lands (then the per-class entries decision happens).

## R13 prospective C1-C4

- C1 (sibling): WEAK — no Temporal-shaped sibling in cruft.
- C2 (shape-compat): HOLDS — namespace + simple factory methods.
- C3 (cost-positive): TBV — first Temporal rung will validate.
- C4 (bail-safe): HOLDS — incremental; existing tests SKIPped.

## Carve-outs

- No timezone resolution; `zonedDateTimeISO()` returns UTC.
- No calendar selection; ISO 8601 only.
- Stub class methods that aren't called by the 3 tests can throw "not implemented" — fine for this rung.

## Status

TN-EXT 0 FOUNDED 2026-05-26 alongside parent temporal-implementation. TN-EXT 1 pending — keeper directive was "add a track," not "begin work."
