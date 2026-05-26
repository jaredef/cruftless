---
name: instant-static
description: Second sub-rung of temporal-instant. Implements from / fromEpochMilliseconds / fromEpochNanoseconds / compare static methods.
type: project
---

# instant-static — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-instant/`.

Sibling-shape to DStat (Duration.from + compare). Per-class sub-rung at ~150 LOC budget.

## Telos

- `Temporal.Instant.from(item)`:
  - Instant instance → clone via __ti_ns sentinel.
  - String → DEFER (TypeError "ISO 8601 datetime string parsing not yet implemented (Tier-L stub)").
- `Temporal.Instant.fromEpochMilliseconds(ms)`:
  - ToNumber + finite check → trunc → multiply by 10^6 to get ns → construct.
- `Temporal.Instant.fromEpochNanoseconds(ns)`:
  - Must be BigInt (Number → TypeError per spec).
- `Temporal.Instant.compare(a, b)`:
  - Coerce both (Instant or string-defer).
  - Compare epochNanoseconds via f64 (range-safe).

Inline helper `make_instant(rt, proto, ns_value)` shared across these four methods (sibling pattern to Duration's make_duration).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — 4 static methods + make_instant helper inline.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Instant/{from,fromEpochMilliseconds,fromEpochNanoseconds,compare}/`.
- **Exemplar suite**: 81 fixtures.

## Carve-outs (residuals expected)

- ISO 8601 datetime string parsing (24 records: from-string + compare-string) — deferred to temporal-iso-string-parse.
- Calendar annotation in ISO strings (2 records) — same dependency.
- Variant minus sign / RFC 3339 edge cases (3 records) — same.
- Mistaken-callee from sibling tests (2 records) — depends on PlainDate / TimeZone.

## Status

TIS-EXT 1 LANDED 2026-05-26. 28/81 PASS (34.6%) — low because residuals are dominated by ISO-string-parse deferrals (same pattern as DStat).
